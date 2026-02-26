# Contract: `upsert_railway_model_translation` (new)

**Command**: `upsert_railway_model_translation`
**Type**: Tauri IPC Command (new)
**Purpose**: Creates or replaces the complete translation for one language on a railway model. Covers both the create form (initial EN translation) and the edit form (add/update IT translation).

## Signature

```typescript
// TypeScript (auto-generated from Rust via specta)
async upsertRailwayModelTranslation(
  args: UpsertRailwayModelTranslationArgs
): Promise<Result<null, CommandError>>
```

```rust
// Rust Args struct
#[derive(Debug, Clone, Deserialize, specta::Type, garde::Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpsertRailwayModelTranslationArgs {
    pub railway_model_id: RailwayModelId,

    /// Language code. Allowed: "en", "it".
    #[garde(pattern(r"^(en|it)$"))]
    pub lang: String,

    /// Description text. Required for "en"; optional for "it".
    /// An empty string for "it" is treated as "remove the IT description".
    pub description: Option<String>,

    /// Details text. Optional for all languages.
    /// An empty string clears the field.
    pub details: Option<String>,
}
```

## Parameters

| Name             | Type             | Required    | Description                                        |
| ---------------- | ---------------- | ----------- | -------------------------------------------------- |
| `railwayModelId` | `RailwayModelId` | Yes         | Target railway model                               |
| `lang`           | `"en" \| "it"`   | Yes         | Language being written                             |
| `description`    | `string \| null` | Conditional | Required non-empty for `"en"`; optional for `"it"` |
| `details`        | `string \| null` | No          | Optional for all languages                         |

## Validation Rules

| Rule                                                                                    | Error                                                  |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `lang` must be `"en"` or `"it"`                                                         | `VALIDATION_ERROR`                                     |
| `description` must be non-empty when `lang === "en"`                                    | `VALIDATION_ERROR` — "English description is required" |
| `description` may be `null` or non-empty for `"it"`                                     | —                                                      |
| Both `description` and `details` being `null` for `"it"` deletes the IT translation row | — (silent removal, not an error)                       |
| Railway model must exist                                                                | `NOT_FOUND`                                            |

## Behaviour

1. Validate `Args` at boundary.
2. Load `RailwayModel` aggregate by `railway_model_id`.
3. If `lang === "it"` and both `description` and `details` are `null`/empty → call `aggregate.remove_translation("it")` which emits `TranslationUpserted` with null fields; repository logic deletes the row.
4. Otherwise → call `aggregate.upsert_translation(lang, description, details)` → emits `TranslationUpserted`.
5. `repository.save()` executes:
   - `INSERT OR REPLACE INTO railway_model_translations (railway_model_id, language_code, description, details, updated_at) VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)`
6. FTS5 `UPDATE` trigger fires automatically.

## Frontend Callsite

```typescript
// Saving from TranslationsSection (create or edit)
await commands.upsertRailwayModelTranslation({
  railwayModelId: modelId,
  lang: 'it',
  description: itDescription || null,
  details: itDetails || null
});
```
