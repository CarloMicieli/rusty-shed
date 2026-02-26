# Contract: `search_railway_models` (new)

**Command**: `search_railway_models`
**Type**: Tauri IPC Query (new)
**Purpose**: Full-text search over railway model descriptions and details across both EN and IT translations. Returns matching model IDs, allowing the frontend to display results using existing card components.

## Signature

```typescript
// TypeScript (auto-generated from Rust via specta)
async searchRailwayModels(
  args: SearchRailwayModelsArgs
): Promise<Result<RailwayModelId[], CommandError>>
```

```rust
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchRailwayModelsArgs {
    /// The search query. FTS5 MATCH syntax supported (e.g., "steam loco*").
    /// Minimum 2 characters.
    #[garde(length(min = 2, max = 500))]
    pub query: String,
}
```

## Parameters

| Name    | Type     | Required | Description                                                                              |
| ------- | -------- | -------- | ---------------------------------------------------------------------------------------- |
| `query` | `string` | Yes      | Search term (minimum 2 chars). Matches against description and details in all languages. |

## Response

`RailwayModelId[]` — ordered by FTS5 relevance rank (BM25).

## Behaviour

1. Validate `query` length.
2. Execute FTS5 MATCH query (runtime `sqlx::query()` — see research R-001):
   ```sql
   SELECT DISTINCT railway_model_id
   FROM railway_model_search_idx
   WHERE railway_model_search_idx MATCH ?1
   ORDER BY rank
   LIMIT 200
   ```
3. Return the list of matching `railway_model_id` strings.
4. If no matches, return empty list (not an error).

## Notes

- The search is cross-language: a query in Italian returns models with Italian translations; a query in English returns models with English translations. Both are indexed in the same FTS5 table.
- The result does not include the resolved text — callers use the returned IDs with `get_railway_model_by_id` (passing `lang`) to load display data.
- FTS5 `rank` ordering provides relevance-based results at no additional cost.
- Result cap of 200 prevents unbounded payloads; UI should prompt users to refine their query if results are capped.

## Frontend Callsite

```typescript
// In a search bar controller
const result = await commands.searchRailwayModels({ query: searchTerm });
if (result.status === 'ok') {
  const ids = result.data;
  // Load model views for display
}
```
