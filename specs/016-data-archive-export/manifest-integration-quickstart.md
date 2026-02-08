# Export Feature - Manifest Integration Quick Start

**For Spec 016 Implementation Team**

---

## TL;DR - The Short Answer

✅ **YES** - You can reuse the manifest schema from import (spec 010) **as-is with zero modifications**.

**What you get**:

- Rust DTOs: `src-tauri/src/import/domain/manifest.rs`
- JSON Schema: `src-tauri/src/import/domain/manifest_schema.json`
- Example: `src-tauri/fixtures/test_import_manifest.json`

---

## Copy-Paste Implementation

### 1. Import the Manifest Types

```rust
use crate::import::domain::manifest::{
    ManifestDto, DataContainerDto, ManufacturerRecord, RailwayModelRecord,
    RailwayCompanyRecord, RollingStockRecord, CollectionItemRecord,
    PurchaseRecord, SellerRecord, AddressRecord, MaintenanceCardRecord,
    MaintenanceEventRecord, MoneyRecord, CategoryRecord,
};
```

### 2. Build Manifest from Database

```rust
// Example: Create manifest from database
let manifest = ManifestDto {
    schema: Some("https://rusty-shed.app/schemas/manifest/v1.json".to_string()),
    version: "1.0".to_string(),
    exported_at: Some(chrono::Utc::now().to_rfc3339()),
    source: Some(format!("Rusty Shed v{}", env!("CARGO_PKG_VERSION"))),
    data: DataContainerDto {
        manufacturers: /* query & map */,
        railway_companies: /* query & map */,
        railway_models: /* query & map */,
        collection_items: /* query & map */,
        sellers: /* query & map */,
        maintenance_cards: /* query & map */,
    },
};
```

### 3. Serialize to JSON

```rust
let json = serde_json::to_string_pretty(&manifest)?;
// Write to file or ZIP archive
```

### 4. (Optional) Validate Output

```rust
use jsonschema::JSONSchema;

let schema_json = serde_json::from_str(
    include_str!("../import/domain/manifest_schema.json")
)?;
let schema = JSONSchema::compile(&schema_json)?;
let manifest_json = serde_json::to_value(&manifest)?;

schema.validate(&manifest_json)?; // Raises error if invalid
```

---

## File Locations Cheat Sheet

| Need                       | File Path                                          |
| -------------------------- | -------------------------------------------------- |
| Rust types for manifest    | `src-tauri/src/import/domain/manifest.rs`          |
| JSON Schema for validation | `src-tauri/src/import/domain/manifest_schema.json` |
| Example manifest           | `src-tauri/fixtures/test_import_manifest.json`     |
| Feature specification      | `specs/010-data-import-utility/spec.md`            |
| Data model docs            | `specs/010-data-import-utility/data-model.md`      |

---

## Entity Relationship Diagram

```
Your Database → Query Domain Models → Map to ManifestDto → Serialize JSON → Package ZIP
```

**Mapping Direction for Export** (Opposite of Import):

| Database Table  | →   | Manifest DTO          |
| --------------- | --- | --------------------- |
| RailwayModel    | →   | RailwayModelRecord    |
| Manufacturer    | →   | ManufacturerRecord    |
| RailwayCompany  | →   | RailwayCompanyRecord  |
| CollectionItem  | →   | CollectionItemRecord  |
| Seller          | →   | SellerRecord          |
| MaintenanceCard | →   | MaintenanceCardRecord |

---

## Key Data Types Reference

### Top-Level

```rust
ManifestDto {
    version: "1.0",              // REQUIRED
    data: DataContainerDto,      // REQUIRED
    exportedAt: Some(timestamp), // OPTIONAL - ISO 8601
    source: Some("App v1.0"),    // OPTIONAL - your app name/version
    schema: Some(URL),           // OPTIONAL - schema reference
}
```

### Collections Inside `data`

```rust
DataContainerDto {
    manufacturers: Vec<ManufacturerRecord>,
    railway_companies: Vec<RailwayCompanyRecord>,
    railway_models: Vec<RailwayModelRecord>,
    collection_items: Vec<CollectionItemRecord>,
    sellers: Vec<SellerRecord>,
    maintenance_cards: Vec<MaintenanceCardRecord>,
}
```

### RailwayModel Has Embedded Collections

```rust
RailwayModelRecord {
    // ... required fields ...
    category: CategoryRecord,           // REQUIRED - nested object
    rolling_stocks: Vec<RollingStockRecord>, // Can be empty
}
```

### CollectionItem Has Optional Purchase

```rust
CollectionItemRecord {
    id: String,
    railway_model_id: String,
    added_date: String,
    purchase: Option<PurchaseRecord>,   // Can be None
    // ... other optional fields ...
}
```

### Purchase Type Determines Requirements

```rust
// Type "purchased" → must have purchaseDate
// Type "sold"      → must have purchaseDate + saleDate
// Type "preordered"→ must have sellerId
```

---

## Validation Checklist

Before packaging the manifest:

- [ ] All RailwayModel.manufacturerId reference valid manufacturers
- [ ] All RollingStock.railwayCompanyId reference valid railway companies
- [ ] All CollectionItem.railwayModelId reference valid railway models
- [ ] All Purchase.sellerId (if present) reference valid sellers
- [ ] All MaintenanceCard.collectionItemId reference valid collection items
- [ ] All required fields are present (see manifest.rs)
- [ ] Manifest validates against JSON Schema (use jsonschema crate)
- [ ] Dates are in ISO format (YYYY-MM-DD)
- [ ] Currency codes are ISO 4217 (e.g., "EUR", "GBP")
- [ ] Country codes are ISO 3166-1 alpha-2 (e.g., "DE", "AT")

---

## Common Pitfalls to Avoid

❌ **Don't**: Use database UUIDs directly in manifest  
✅ **Do**: Generate or map to stable external IDs

❌ **Don't**: Include circular references  
✅ **Do**: Use string IDs for relationships (no actual pointers)

❌ **Don't**: Skip validation before export  
✅ **Do**: Validate against schema to catch issues early

❌ **Don't**: Forget to include `exportedAt` and `source`  
✅ **Do**: Add metadata for audit trail

❌ **Don't**: Assume all fields are required  
✅ **Do**: Check manifest.rs for Optional fields

---

## Testing Your Export

```rust
#[test]
fn test_export_manifest_format() {
    let manifest = create_export_manifest();

    // Serialize
    let json = serde_json::to_string_pretty(&manifest).unwrap();

    // Can deserialize back
    let deserialized: ManifestDto = serde_json::from_str(&json).unwrap();

    // Validate against schema
    let schema = JSONSchema::compile(SCHEMA).unwrap();
    schema.validate(&serde_json::to_value(&manifest)).unwrap();
}
```

---

## Import Feature Reference (for context)

The import feature that uses this manifest:

- Located in: `src-tauri/src/import/`
- Validates using: `schema_validator.rs`
- Extracts archives: `archive_extractor.rs`
- Maps IDs: `id_mapper.rs`
- Spec: `specs/010-data-import-utility/`

You're implementing the **inverse operation** - creating manifests instead of reading them.

---

## Next Steps

1. **Read**: [Manifest Schema Structure Research](./manifest-schema-research.md) (detailed)
2. **Reference**: [specs/010-data-import-utility/data-model.md](../010-data-import-utility/data-model.md)
3. **Test**: Use [src-tauri/fixtures/test_import_manifest.json](../../../src-tauri/fixtures/test_import_manifest.json) as reference
4. **Validate**: Run schema validation before writing to ZIP
5. **Document**: Add your export process to `docs/tauri-commands.md`

---

**Questions?** Check the detailed research document for schema specifications, relationship rules, and design rationale.
