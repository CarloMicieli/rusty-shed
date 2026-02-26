# Data Model: Global Search

**Branch**: `030-global-search` | **Date**: 2026-02-26

---

## Overview

Global Search is a read-only cross-domain query. It does not introduce new persistent aggregates. The feature:

1. **Extends** the existing `railway_model_search_idx` FTS5 virtual table (new migration).
2. **Adds** a new read-only domain value object `GlobalSearchResult` in a new `search` domain.
3. **Joins** FTS5 results with `collection_items` and `wishlist_items` at query time to attach source context.

---

## Database Changes

### Migration 0014: Extend FTS5 Search Index

**File**: `src-tauri/migrations/0014_extend_railway_model_search_idx.sql`

#### Drop and Recreate FTS5 Virtual Table

FTS5 virtual tables do not support `ALTER TABLE`. The existing table must be dropped and recreated with the extended schema. The existing triggers from migration 0013 are also dropped here — index maintenance is handled by the Rust domain layer, not by database triggers.

```sql
-- Drop old triggers from migration 0013 (replaced by domain-event indexing)
DROP TRIGGER IF EXISTS tr_rmt_fts_insert;
DROP TRIGGER IF EXISTS tr_rmt_fts_update;
DROP TRIGGER IF EXISTS tr_rmt_fts_delete;

-- Drop old FTS5 table (FTS5 does not support ALTER TABLE)
DROP TABLE IF EXISTS railway_model_search_idx;

-- Recreate with extended columns
CREATE VIRTUAL TABLE railway_model_search_idx USING fts5 (
    railway_model_id    UNINDEXED,   -- PK reference, not tokenised
    language_code       UNINDEXED,   -- language tag, not tokenised
    description,                     -- from railway_model_translations.description
    details,                         -- from railway_model_translations.details
    manufacturer_name,               -- NEW: from manufacturers.name
    rolling_stocks_text,             -- NEW: concat of road_number, series_code, livery, depot
    tokenize = 'unicode61'
);
```

#### Initial Population (migration-time only)

The migration populates the index from existing data. After this, all updates are driven by domain events in Rust.

```sql
INSERT INTO railway_model_search_idx (
    railway_model_id,
    language_code,
    description,
    details,
    manufacturer_name,
    rolling_stocks_text
)
SELECT
    rmt.railway_model_id,
    rmt.language_code,
    COALESCE(rmt.description, ''),
    COALESCE(rmt.details, ''),
    COALESCE(m.name, ''),
    COALESCE(
        (SELECT group_concat(
            COALESCE(rs.road_number, '') || ' ' ||
            COALESCE(rs.series_code, '') || ' ' ||
            COALESCE(rs.livery, '') || ' ' ||
            COALESCE(rs.depot, ''),
            ' '
         )
         FROM rolling_stocks rs
         WHERE rs.railway_model_id = rmt.railway_model_id),
        ''
    )
FROM railway_model_translations rmt
JOIN railway_models rm ON rm.id = rmt.railway_model_id
JOIN manufacturers m   ON m.id  = rm.manufacturer_id;
```

#### No Database Triggers

No triggers are created. The FTS5 index is updated exclusively by the Rust infrastructure layer after domain events are drained (see "Domain-Event Index Update" section below).

---

## Domain-Event Index Update

### Principle

The FTS5 index is rebuilt by the Rust infrastructure layer, within the **same transaction** used to persist domain events. No SQLite triggers are involved.

### Rebuild SQL (reusable per model)

This SQL runs after any domain event that mutates a `RailwayModel` or its `rolling_stocks`. It is a complete replace: DELETE all rows for the model, then INSERT fresh rows for every available language.

```sql
-- Step 1: remove all existing FTS5 rows for this model
DELETE FROM railway_model_search_idx
WHERE railway_model_id = ?1;

-- Step 2: re-insert one row per language translation
INSERT INTO railway_model_search_idx (
    railway_model_id,
    language_code,
    description,
    details,
    manufacturer_name,
    rolling_stocks_text
)
SELECT
    rmt.railway_model_id,
    rmt.language_code,
    COALESCE(rmt.description, ''),
    COALESCE(rmt.details, ''),
    COALESCE(m.name, ''),
    COALESCE(
        (SELECT group_concat(
            COALESCE(rs.road_number, '') || ' ' ||
            COALESCE(rs.series_code, '') || ' ' ||
            COALESCE(rs.livery, '') || ' ' ||
            COALESCE(rs.depot, ''),
            ' '
         )
         FROM rolling_stocks rs
         WHERE rs.railway_model_id = ?1),
        ''
    )
FROM railway_model_translations rmt
JOIN railway_models rm ON rm.id  = rmt.railway_model_id
JOIN manufacturers m   ON m.id   = rm.manufacturer_id
WHERE rmt.railway_model_id = ?1;
```

### Rust Integration Point

The rebuild is called from `SqliteRailwayModelRepository::save()`, after all event SQL mutations are applied and before the transaction commits:

```rust
// Pseudocode — exact method names follow repo conventions
async fn save(&mut self, aggregate: &mut RailwayModel) -> Result<(), DomainError> {
    for event in aggregate.drain_events() {
        match event {
            RailwayModelEvent::Created(data)           => { /* INSERT railway_models row */ }
            RailwayModelEvent::TranslationUpserted(..) => { /* UPSERT railway_model_translations */ }
            RailwayModelEvent::RollingStockAdded(..)   => { /* INSERT rolling_stocks row */ }
            RailwayModelEvent::RollingStockUpdated(..) => { /* UPDATE rolling_stocks row */ }
            RailwayModelEvent::RollingStockRemoved(..) => { /* DELETE rolling_stocks row */ }
            RailwayModelEvent::Deleted               => { /* DELETE railway_models row — FTS5 rows cascade */ }
        }
    }
    // After all mutations, rebuild the search index for this model
    self.rebuild_search_index(aggregate.id(), &mut tx).await?;
    tx.commit().await?;
    Ok(())
}
```

For `ManufacturerNameUpdated`, the manufacturer repository calls the same `rebuild_search_index` SQL for every `railway_model_id` associated with that manufacturer.

### Delete Handling

When a `RailwayModel` is deleted, `railway_model_translations` rows cascade-delete (FK with `ON DELETE CASCADE`). The FTS5 rows must be explicitly deleted before or within the same transaction:

```sql
DELETE FROM railway_model_search_idx WHERE railway_model_id = ?1;
```

This runs as part of the `Deleted` event handler, before the `railway_models` row is removed.

---

## Rust Domain Model

### `GlobalSearchResult` (domain/global_search_result.rs)

```rust
/// A single search hit returned by the global search use case.
/// Each item carries enough information for the frontend to display
/// a result card and route the user to the correct detail page.
#[derive(Debug, Clone)]
pub struct GlobalSearchResult {
    pub railway_model_id: RailwayModelId,
    pub source: SearchSource,
    /// The ID of the collection_item or wishlist_item (not the railway model).
    pub item_id: String,
    pub display_name: String,
    pub manufacturer_name: String,
}

/// Where this result comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchSource {
    Collection,
    Wishlist,
}
```

### `GlobalSearchRepository` trait (domain/repository.rs)

```rust
#[async_trait]
pub trait GlobalSearchRepository {
    async fn search(
        &mut self,
        query: &str,
        lang: &str,
    ) -> Result<Vec<GlobalSearchResult>, DomainError>;
}
```

### Use-case input (application/global_search.rs)

```rust
pub struct GlobalSearchInput {
    /// Raw query string, already validated (min 2, max 500 chars).
    pub query: String,
    /// BCP-47 language tag, e.g. "en", "it".
    pub lang: String,
}
```

---

## Transport DTOs (interface layer)

### `GlobalSearchArgs` (command_args.rs)

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, Validate)]
pub struct GlobalSearchArgs {
    #[garde(length(min = 2, max = 500))]
    pub query: String,
    /// BCP-47 language tag forwarded from the frontend locale.
    #[garde(length(min = 2, max = 10))]
    pub lang: String,
}
```

### `GlobalSearchResultView` (command_args.rs)

```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct GlobalSearchResultView {
    pub railway_model_id: String,
    /// "collection" | "wishlist"
    pub source: String,
    /// collection_item.id or wishlist_item.id
    pub item_id: String,
    pub display_name: String,
    pub manufacturer_name: String,
}
```

---

## Query Design

### Core Global Search SQL

```sql
SELECT
    si.railway_model_id,
    ci.id              AS collection_item_id,
    wi.id              AS wishlist_item_id,
    -- Use the language-resolved description as display name
    COALESCE(
        (SELECT description
         FROM railway_model_translations
         WHERE railway_model_id = si.railway_model_id
           AND language_code = ?2
         LIMIT 1),
        (SELECT description
         FROM railway_model_translations
         WHERE railway_model_id = si.railway_model_id
           AND language_code = 'en'
         LIMIT 1),
        rm.description
    )                  AS display_name,
    m.name             AS manufacturer_name,
    bm25(railway_model_search_idx) AS rank
FROM railway_model_search_idx si
JOIN railway_models rm   ON rm.id = si.railway_model_id
JOIN manufacturers m     ON m.id  = rm.manufacturer_id
LEFT JOIN collection_items ci
    ON ci.railway_model_id = si.railway_model_id
    AND ci.removed_date IS NULL
LEFT JOIN wishlist_items wi
    ON wi.railway_model_id = si.railway_model_id
    AND wi.removed_date IS NULL
WHERE railway_model_search_idx MATCH ?1
  AND si.language_code = ?2
  AND (ci.id IS NOT NULL OR wi.id IS NOT NULL)
ORDER BY rank   -- BM25: lower = more relevant
LIMIT 50
```

The Rust mapper iterates over rows: if a row has both `collection_item_id` and `wishlist_item_id` set, it emits two `GlobalSearchResult` values (one per source). Deduplication of `railway_model_id` within the same source is handled by the query returning at most one collection item and one wishlist item per model (DISTINCT is implicit via the LEFT JOIN structure given the primary keys).

### Query Term Transformation

The raw user query must be appended with `*` to enable prefix matching:

- Input: `"A.C.M.E"` → FTS5 query: `"A.C.M.E"*` (quoted phrase + prefix)
- Input: `"diesel loco"` → FTS5 query: `"diesel loco"*`
- Input: `"class 47"` → FTS5 query: `"class 47"*`

This transformation happens in the Rust use-case layer before calling the repository, not in SQL.

---

## Frontend State Shape

```typescript
// Derived from specta-generated bindings
interface GlobalSearchResultView {
  railwayModelId: string;
  source: 'collection' | 'wishlist';
  itemId: string;
  displayName: string;
  manufacturerName: string;
}

interface SearchPageState {
  query: string; // from URL ?q=
  results: GlobalSearchResultView[];
  isLoading: boolean;
  hasSearched: boolean; // true after first query completes
}
```

---

## Entities Not Changed

| Entity                       | Change                    |
| ---------------------------- | ------------------------- |
| `railway_models`             | None                      |
| `railway_model_translations` | None (triggers updated)   |
| `rolling_stocks`             | None (new triggers added) |
| `manufacturers`              | None                      |
| `collection_items`           | None (read-only join)     |
| `wishlist_items`             | None (read-only join)     |
| `collections`                | None                      |
| `wishlists`                  | None                      |
