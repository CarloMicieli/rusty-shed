# Contract: `get_railway_model_by_id` (updated)

**Command**: `get_railway_model_by_id`
**Type**: Tauri IPC Query
**Change**: Breaking — adds required `lang` parameter; `RailwayModelView` gains two new fields.

## Signature

```typescript
// TypeScript (auto-generated from Rust via specta)
async getRailwayModelById(
  railwayModelId: RailwayModelId,
  lang: string                     // "en" | "it"
): Promise<Result<RailwayModelView | null, CommandError>>
```

```rust
// Rust command handler
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_by_id(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
    lang: String,
) -> Result<Option<RailwayModelView>, CommandError>
```

## Parameters

| Name | Type | Required | Description |
| ---- | ---- | -------- | ----------- |
| `railway_model_id` | `RailwayModelId` (String) | Yes | TRN identifier of the railway model |
| `lang` | `String` | Yes | Requested language code (`"en"` or `"it"`). Unknown values fall back to `"en"`. |

## Response: `RailwayModelView` (updated)

Two new fields are added. All existing fields are unchanged.

```typescript
export type RailwayModelView = {
  id: RailwayModelId;
  manufacturer: RailwayModelManufacturer;
  productCode: ProductCode;
  description: string;              // resolved text (may be EN fallback)
  descriptionLang: string;          // NEW: "en" | "it" — actual resolved language
  details: string | null;
  detailsLang: string | null;       // NEW: "en" | "it" | null (null if no details)
  powerMethod: PowerMethod;
  scale: Scale;
  epoch: Epoch;
  category: Category;
  deliveryDate: DeliveryDate | null;
  availabilityStatus: AvailabilityStatus | null;
  metadata: Metadata;
}
```

## Behaviour

1. Validate `lang` against allowed set `["en", "it"]`; map unknown values to `"en"`.
2. Execute COALESCE double-join SQL (see `data-model.md`).
3. Populate `descriptionLang` and `detailsLang` from the `resolved_lang` column in the query result.
4. If no record found, return `null`.

## Frontend Callsite

```typescript
// In CatalogueController or model detail page
const lang = getLocaleService().currentLocale;
const result = await commands.getRailwayModelById(modelId, lang);
```

## Fallback Indicator Logic

The frontend uses `descriptionLang` to decide whether to render a `LanguageFallbackBadge`:

```svelte
{#if model.descriptionLang !== currentLocale}
  <LanguageFallbackBadge lang={model.descriptionLang} />
{/if}
```
