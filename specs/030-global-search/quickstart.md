# Quickstart: Global Search — Developer Guide

**Branch**: `030-global-search` | **Date**: 2026-02-26

---

## Prerequisites

- Rust toolchain pinned to `1.93.0` (check `rust-toolchain.toml`)
- `pnpm` installed globally
- SQLite 3.35+ (bundled with sqlx via Tauri)

---

## Step 0: Branch Setup

```bash
git checkout 030-global-search
pnpm install          # install frontend deps
```

---

## Step 1: Database Migration

Add the new migration file:

```
src-tauri/migrations/0014_extend_railway_model_search_idx.sql
```

See [data-model.md](./data-model.md) for the full SQL. After saving the file:

```bash
pnpm rust:build       # sqlx validates + embeds all migrations at compile time
```

If SQLx offline mode is active, regenerate the `.sqlx` cache:

```bash
cd src-tauri
cargo sqlx prepare    # only needed if SQLX_OFFLINE=true in CI
```

---

## Step 2: Rust Backend — New `search` Domain

### 2a. Create Domain Layer

File: `src-tauri/src/search/domain/global_search_result.rs`

- Define `GlobalSearchResult` struct and `SearchSource` enum (see [data-model.md](./data-model.md)).

File: `src-tauri/src/search/domain/repository.rs`

- Define `GlobalSearchRepository` async trait.

### 2b. Create Application Layer

File: `src-tauri/src/search/application/global_search.rs`

- `GlobalSearchInput { query: String, lang: String }`
- `GlobalSearch::execute(uow, input)` — transforms query to FTS5 prefix format, calls repository.

### 2c. Create Infrastructure Layer

File: `src-tauri/src/search/infrastructure/sqlite_global_search_repository.rs`

- Implement `GlobalSearchRepository` for `SqliteUnitOfWork`.
- Use the SQL from [data-model.md](./data-model.md) (`MATCH ?1` with prefix-transformed query).
- Map rows to `GlobalSearchResult` — emit two items per row when both collection and wishlist IDs are non-null.

### 2d. Create Interface Layer

File: `src-tauri/src/search/interface/command_args.rs`

- `GlobalSearchArgs` (garde + specta derivations).
- `GlobalSearchResultView` (serde Serialize + specta).

File: `src-tauri/src/search/interface/command_handlers.rs`

- `global_search` Tauri command.

### 2e. Wire into `lib.rs`

```rust
// In src-tauri/src/lib.rs — add to the .invoke_handler() call:
search_command_handlers::global_search,

// Add to specta type collection:
.commands(tauri_specta::collect_commands![
    // ... existing commands ...
    search_command_handlers::global_search,
])
```

### 2f. Extend Unit of Work

In `core/infrastructure/unit_of_work.rs`, add:

```rust
pub trait GlobalSearchUowExt {
    fn global_search_repo(&mut self) -> &mut impl GlobalSearchRepository;
}
```

Implement for `SqliteUnitOfWork`.

---

## Step 3: Verify Rust Backend

```bash
pnpm rust:fmt          # format
pnpm rust:build        # compile + migrate
pnpm rust:clippy       # must pass (warnings = errors)
pnpm rust:test         # all tests must pass
```

Write `#[sqlx::test(migrations = "./migrations")]` tests for:

- `rebuild_search_index` correctly populates FTS5 rows for a model with translations and rolling stocks
- Search matching by description, road_number, and manufacturer name
- Empty result when no collection/wishlist item exists for a matched model
- Query with `removed_date` set (soft-deleted) should not appear
- Delete event removes FTS5 rows before the model row is deleted

---

## Step 4: Regenerate TypeScript Bindings

```bash
pnpm tauri dev         # starts the app; specta writes updated bindings.ts on startup
# Ctrl+C after bindings are generated
```

Verify `src/lib/bindings.ts` now contains `globalSearch` and `GlobalSearchResultView`.

---

## Step 5: Frontend — `/search` Page

### 5a. Create Page Route

```
src/routes/search/
├── +page.ts      ← load function: reads ?q=, calls commands.globalSearch(...)
└── +page.svelte  ← renders results grouped by source
```

See [contracts/global_search.md](./contracts/global_search.md) for the load function pattern.

### 5b. Create Feature Module

```
src/lib/features/search/
├── SearchService.svelte.ts   ← wraps state if needed (may just use page data)
├── components/
│   ├── SearchResultCard.svelte
│   └── SearchEmptyState.svelte
└── index.ts
```

### 5c. Update `SearchBar.svelte`

Change the Enter key handler to navigate instead of inline-resolve:

```typescript
function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && query.trim().length >= 2) {
    goto(`/search?q=${encodeURIComponent(query.trim())}`);
  }
}
```

### 5d. Add Paraglide Message Keys

Add keys listed in [contracts/global_search.md](./contracts/global_search.md) to:

- `messages/en.json`
- `messages/it.json`

Then run:

```bash
pnpm prepare    # recompiles Paraglide messages
```

---

## Step 6: Frontend Quality

```bash
pnpm format    # Prettier
pnpm lint      # ESLint (must pass)
pnpm check     # svelte-check + TypeScript (must pass)
pnpm test      # Vitest
```

Write Vitest tests for:

- `SearchResultCard` renders source badge correctly for `"collection"` and `"wishlist"`
- `SearchEmptyState` shows the add-model CTA
- `/search` page shows results when load returns data
- `/search` page shows empty state when load returns `[]`

---

## Step 7: Manual Smoke Test

1. Run `pnpm tauri dev`
2. In the header search bar, type "A.C.M.E" (or any brand in your test DB) and press Enter
3. Verify `/search?q=A.C.M.E` route loads with results
4. Click a Collection result → verify navigation to `/collection/{id}`
5. Click a Wishlist result → verify navigation to `/wishlists/{wishlistId}/items/{itemId}`
6. Search for a term that matches no items → verify "No models found" empty state
7. Search with 1 character → verify no navigation (button/Enter disabled or query too short)

---

## Useful References

| Artifact                 | Path                                                           |
| ------------------------ | -------------------------------------------------------------- |
| Feature spec             | [spec.md](./spec.md)                                           |
| Research decisions       | [research.md](./research.md)                                   |
| Data model + SQL         | [data-model.md](./data-model.md)                               |
| IPC contract             | [contracts/global_search.md](./contracts/global_search.md)     |
| Existing search use-case | `src-tauri/src/catalog/application/search_railway_models.rs`   |
| Existing FTS5 migration  | `src-tauri/migrations/0013_add_railway_model_translations.sql` |
| Existing search command  | `src-tauri/src/catalog/interface/command_handlers.rs`          |
| SearchBar component      | `src/lib/components/SearchBar.svelte`                          |
| Bindings (after regen)   | `src/lib/bindings.ts`                                          |
