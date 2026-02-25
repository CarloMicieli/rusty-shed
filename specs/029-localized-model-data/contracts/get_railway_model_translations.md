# Contract: `get_railway_model_translations` (new)

**Command**: `get_railway_model_translations`
**Type**: Tauri IPC Query (new)
**Purpose**: Returns all stored translations for a railway model, used to pre-populate the EN/IT input fields on the edit form.

## Signature

```typescript
// TypeScript (auto-generated from Rust via specta)
async getRailwayModelTranslations(
  railwayModelId: RailwayModelId
): Promise<Result<RailwayModelTranslations | null, CommandError>>
```

```rust
// Rust command handler
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_translations(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
) -> Result<Option<RailwayModelTranslations>, CommandError>
```

## Parameters

| Name | Type | Required | Description |
| ---- | ---- | -------- | ----------- |
| `railway_model_id` | `RailwayModelId` (String) | Yes | TRN identifier of the railway model |

## Response: `RailwayModelTranslations`

```typescript
export type RailwayModelTranslations = {
  railwayModelId: RailwayModelId;
  en: RailwayModelTranslationEntry | null;  // null if no EN translation stored
  it: RailwayModelTranslationEntry | null;  // null if no IT translation stored
}

export type RailwayModelTranslationEntry = {
  description: string | null;
  details: string | null;
}
```

## Behaviour

1. Query `SELECT language_code, description, details FROM railway_model_translations WHERE railway_model_id = ?` — all rows for the model.
2. Map each row to the corresponding `en` / `it` field.
3. Absent language rows produce `null` in the response (not an error).
4. If the model itself does not exist, return `null`.

## Frontend Callsite

```typescript
// In model edit form / TranslationsSection
const translations = await commands.getRailwayModelTranslations(modelId);
// Pre-populate EN and IT form fields from translations.en / translations.it
```
