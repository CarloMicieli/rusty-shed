# Contract: `update_railway_model_text` (updated)

**Command**: `update_railway_model_text`
**Type**: Tauri IPC Command
**Change**: Breaking — adds required `lang` parameter. Used for in-place single-field editing (e.g., inline description edit in `RailwayModelCard`).

## Signature

```typescript
// TypeScript (auto-generated from Rust via specta)
async updateRailwayModelText(
  args: UpdateRailwayModelTextArgs
): Promise<Result<null, CommandError>>
```

```rust
// Updated Args struct
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRailwayModelTextArgs {
    pub railway_model_id: RailwayModelId,

    /// Which field to update.
    pub field: RailwayModelTextField,

    /// The new value. Empty string for Details clears the field.
    /// Empty string for Description is rejected when lang == "en".
    pub value: String,

    /// Language code for which this update applies. Allowed: "en", "it".
    #[garde(pattern(r"^(en|it)$"))]
    pub lang: String,              // NEW
}
```

## Parameters

| Name | Type | Required | Description |
| ---- | ---- | -------- | ----------- |
| `railwayModelId` | `RailwayModelId` | Yes | Target railway model |
| `field` | `"Description" \| "Details"` | Yes | Which field to update |
| `value` | `string` | Yes | New text value (empty string clears `Details`; rejected for `Description` + `"en"`) |
| `lang` | `"en" \| "it"` | Yes | Language for which the field is updated |

## Validation Rules

| Rule | Error |
| ---- | ----- |
| `lang` must be `"en"` or `"it"` | `VALIDATION_ERROR` |
| `value` empty + `field === "Description"` + `lang === "en"` | `VALIDATION_ERROR` — "English description must not be empty" |

## Behaviour

1. Validate `Args`.
2. Load aggregate.
3. Call `aggregate.upsert_translation(lang, resolved_description, resolved_details)` where the non-updated field is loaded from the existing translation (requires `find_translations` first, or load aggregate with translations attached).
4. Emit `TranslationUpserted`.
5. `repository.save()` upserts into `railway_model_translations`.

## Migration Note

Existing frontend callers of `updateRailwayModelText` must be updated to pass `lang`. The default should be `getLocaleService().currentLocale`.

## Frontend Callsite

```typescript
// In-place description edit (RailwayModelCard)
await commands.updateRailwayModelText({
  railwayModelId: modelId,
  field: 'Description',
  value: newDescription,
  lang: getLocaleService().currentLocale,
});
```
