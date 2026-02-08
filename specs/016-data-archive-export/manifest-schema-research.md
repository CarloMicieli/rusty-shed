# Manifest Schema Structure Research

## Data Archive Export Feature (Spec 016)

**Research Date**: February 8, 2026  
**Status**: Complete  
**Objective**: Document manifest schema from import feature (spec 010) for reuse in export feature

---

## Executive Summary

The manifest schema is **dual-defined** as both:

1. **JSON Schema** (`manifest_schema.json`) - Formal validation specification
2. **Rust Structs** (`manifest.rs`) - Type-safe deserialization DTOs

The schema is **fully reusable** for the export feature with **no modifications required**. Both files are identical across the codebase and version-locked to `v1.0`.

---

## 1. Manifest Definition Location

### Primary Files

| File                                                                                                                      | Purpose                         | Type         | Lines |
| ------------------------------------------------------------------------------------------------------------------------- | ------------------------------- | ------------ | ----- |
| [src-tauri/src/import/domain/manifest.rs](../../../src-tauri/src/import/domain/manifest.rs)                               | Rust DTOs for deserialization   | Rust Structs | 216   |
| [src-tauri/src/import/domain/manifest_schema.json](../../../src-tauri/src/import/domain/manifest_schema.json)             | JSON Schema v2020-12 validation | JSON Schema  | 489   |
| [specs/010-data-import-utility/contracts/manifest.schema.json](../010-data-import-utility/contracts/manifest.schema.json) | Canonical schema reference      | JSON Schema  | 489   |

### Reference Files

| File                                                                                                  | Purpose                        |
| ----------------------------------------------------------------------------------------------------- | ------------------------------ |
| [src-tauri/fixtures/test_import_manifest.json](../../../src-tauri/fixtures/test_import_manifest.json) | Test fixture with example data |
| [specs/010-data-import-utility/data-model.md](../010-data-import-utility/data-model.md)               | Schema documentation           |
| [specs/010-data-import-utility/spec.md](../010-data-import-utility/spec.md)                           | Feature requirements           |

### Schema Namespace & Versioning

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://rusty-shed.app/schemas/manifest/v1.json",
  "title": "Rusty Shed Import Manifest",
  "description": "Schema for the manifest.json file within Rusty Shed import packages"
}
```

**Current Version**: 1.0 (stable, backward-compatible)

---

## 2. Complete Entity List

### Top-Level Manifest Structure

```typescript
interface ManifestDto {
  $schema?: string; // Schema reference URL
  version: '1.0'; // REQUIRED - Semantic version
  exportedAt?: string; // ISO 8601 timestamp
  source?: string; // App/tool that created export
  data: DataContainerDto; // REQUIRED - All relational data
}
```

### Core Entities in Data Container

The manifest contains six primary entity types:

#### 1. **Manufacturer** (RailwayModel dependency)

```rust
pub struct ManufacturerRecord {
    pub id: String,                        // REQUIRED - External ID
    pub name: String,                      // REQUIRED - e.g., "Märklin"
    pub registered_company_name: Option<String>,
    pub country_code: Option<String>,      // ISO 3166-1 alpha-2
    pub status: Option<String>,            // "active" | "outOfBusiness"
    pub website_url: Option<String>,
}
```

**Cardinality**: 0..N | **Referenced by**: RailwayModel

#### 2. **RailwayCompany** (RollingStock dependency)

```rust
pub struct RailwayCompanyRecord {
    pub id: String,                        // REQUIRED - External ID
    pub name: String,                      // REQUIRED - e.g., "Deutsche Bahn"
    pub abbreviation: Option<String>,      // e.g., "DB"
    pub country_code: Option<String>,      // ISO 3166-1 alpha-2
    pub status: Option<String>,            // "active" | "inactive"
}
```

**Cardinality**: 0..N | **Referenced by**: RollingStock

#### 3. **RailwayModel** (Primary catalog entity)

```rust
pub struct RailwayModelRecord {
    pub id: String,                        // REQUIRED - External ID
    pub manufacturer_id: String,           // REQUIRED - FK → Manufacturer.id
    pub product_code: String,              // REQUIRED - Manufacturer's product code
    pub description: String,               // REQUIRED - Human-readable description
    pub scale: String,                     // REQUIRED - H0, HO, N, TT, Z, G, 0, 00, 1
    pub epoch: String,                     // REQUIRED - III, IV, V, VI, etc.
    pub category: CategoryRecord,          // REQUIRED - Nested object
    pub power_method: String,              // REQUIRED - dc | ac | dcc | none
    pub details: Option<String>,
    pub delivery_date: Option<String>,     // ISO format or "Q1 2024" style
    pub availability_status: Option<String>, // available | announced | discontinued
    pub image: Option<String>,             // Filename in /images/ folder
    pub rolling_stocks: Vec<RollingStockRecord>, // Default: []
}
```

**Cardinality**: 0..N | **Uniqueness**: manufacturer_id + product_code | **Referenced by**: CollectionItem

**Nested Type - Category**:

```rust
pub struct CategoryRecord {
    pub r#type: String,                    // REQUIRED - locomotive, passengerCar, etc.
    pub sub_type: Option<String>,          // steam, electric, tank, covered, etc.
}
```

#### 4. **RollingStock** (Embedded in RailwayModel)

```rust
pub struct RollingStockRecord {
    pub railway_company_id: String,        // REQUIRED - FK → RailwayCompany.id
    pub series_code: String,               // REQUIRED - e.g., "BR 01"
    pub road_number: Option<String>,       // e.g., "01 118"
    pub livery: Option<String>,
    pub friendly_name: Option<String>,
    pub is_dummy: Option<bool>,            // Default: false
    pub length_over_buffers: Option<f64>,  // Length in mm
}
```

**Cardinality**: 0..N (embedded within RailwayModel)

#### 5. **CollectionItem** (User's owned items)

```rust
pub struct CollectionItemRecord {
    pub id: String,                        // REQUIRED - External ID
    pub railway_model_id: String,          // REQUIRED - FK → RailwayModel.id
    pub added_date: String,                // REQUIRED - ISO date format
    pub removed_date: Option<String>,      // ISO date format
    pub purchase_condition: Option<String>, // new | used | preowned
    pub model_condition: Option<String>,   // mint | excellent | good | fair | poor
    pub box_condition: Option<String>,     // mint | good | damaged | missing
    pub notes: Option<String>,
    pub image: Option<String>,             // Filename
    pub purchase: Option<PurchaseRecord>,  // Nested object
}
```

**Cardinality**: 0..N | **Uniqueness**: railway_model_id + added_date (if purchase present)

**Nested Type - Purchase**:

```rust
pub struct PurchaseRecord {
    pub r#type: String,                    // REQUIRED - purchased | sold | preordered
    pub purchase_date: Option<String>,     // REQUIRED for purchased/sold
    pub price: Option<MoneyRecord>,
    pub seller_id: Option<String>,         // FK → Seller.id
    pub sale_date: Option<String>,         // REQUIRED for sold
    pub sale_price: Option<MoneyRecord>,
    pub deposit_amount: Option<MoneyRecord>,
    pub expected_delivery: Option<String>, // ISO date format
}
```

**Conditional Requirements** (per JSON Schema):

- Type "purchased" → REQUIRES purchaseDate
- Type "sold" → REQUIRES purchaseDate + saleDate
- Type "preordered" → REQUIRES sellerId

#### 6. **Seller** (Collection purchase reference)

```rust
pub struct SellerRecord {
    pub id: String,                        // REQUIRED - External ID
    pub name: String,                      // REQUIRED
    pub seller_type: String,               // REQUIRED - shop | private | marketplace | auction
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub address: Option<AddressRecord>,    // Nested object
}
```

**Cardinality**: 0..N | **Referenced by**: Purchase

**Nested Type - Address**:

```rust
pub struct AddressRecord {
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,     // ISO 3166-1 alpha-2
}
```

#### 7. **MaintenanceCard** (Maintenance history)

```rust
pub struct MaintenanceCardRecord {
    pub id: String,                        // REQUIRED - External ID
    pub collection_item_id: String,        // REQUIRED - FK → CollectionItem.id
    pub last_maintenance_date: Option<String>, // ISO date format
    pub next_maintenance_date: Option<String>, // ISO date format
    pub events: Vec<MaintenanceEventRecord>, // Default: []
}
```

**Cardinality**: 0..N | **Referenced by**: CollectionItem (implicit)

**Nested Type - MaintenanceEvent**:

```rust
pub struct MaintenanceEventRecord {
    pub id: String,                        // REQUIRED
    pub date: String,                      // REQUIRED - ISO date format
    pub r#type: String,                    // REQUIRED - cleaning | lubrication | repair | modification | inspection
    pub description: Option<String>,
    pub cost: Option<MoneyRecord>,
}
```

**Value Type - Money** (Used in Purchase and MaintenanceEvent):

```rust
pub struct MoneyRecord {
    pub amount: u64,                       // REQUIRED - Smallest currency unit (e.g., cents)
    pub currency: String,                  // REQUIRED - ISO 4217 code (e.g., "EUR")
}
```

---

## 3. Relationship Patterns

### Relationship Model (Conceptual Diagram)

```
Manufacturer (0..N)
    ↑
    | manufacturerId (FK)
    |
RailwayModel (0..N)
    |
    ├─ contains─→ Category (1)
    ├─ contains─→ RollingStocks[] (0..N)
    |                   |
    |                   └─ railwayCompanyId (FK)
    |                           ↑
    |                      RailwayCompany (0..N)
    |
    ← railwayModelId (FK)
    |
CollectionItem (0..N)
    |
    ├─ contains─→ Purchase (0..1)
    |                 |
    |                 └─ sellerId (FK)
    |                         ↑
    |                      Seller (0..N)
    |
    ← collectionItemId (FK)
    |
MaintenanceCard (0..N)
    └─ contains─→ MaintenanceEvents[] (0..N)
```

### Relationship Rules

| Source           | Target         | Cardinality | Field            | Type  | Notes                                     |
| ---------------- | -------------- | ----------- | ---------------- | ----- | ----------------------------------------- |
| RailwayModel     | Manufacturer   | N:1         | manufacturerId   | FK    | REQUIRED; matched during import           |
| RollingStock     | RailwayCompany | N:1         | railwayCompanyId | FK    | REQUIRED; validated against company list  |
| CollectionItem   | RailwayModel   | N:1         | railwayModelId   | FK    | REQUIRED; matched during import           |
| Purchase         | Seller         | N:1         | sellerId         | FK    | OPTIONAL; only for certain purchase types |
| MaintenanceCard  | CollectionItem | N:1         | collectionItemId | FK    | REQUIRED; 1:1 cardinality in practice     |
| MaintenanceEvent | Money          | 1:1         | cost             | VALUE | OPTIONAL; embedded object                 |
| Category         | RailwayModel   | 1:1         | (embedded)       | VALUE | REQUIRED; embedded object, no FK          |

### Foreign Key Validation Rules

**During Import Validation**:

1. **Referential Integrity**: Every FK must reference an existing ID in the manifest
   - RailwayModel.manufacturerId → one of manufacturers[].id
   - RollingStock.railwayCompanyId → one of railwayCompanies[].id
   - CollectionItem.railwayModelId → one of railwayModels[].id
   - Purchase.sellerId → one of sellers[].id (if present)
   - MaintenanceCard.collectionItemId → one of collectionItems[].id

2. **Duplicate Detection**: By composite keys
   - RailwayModel: manufacturerId + productCode (must be unique)
   - CollectionItem: railwayModelId + addedDate (must be unique)

3. **Conditional Requirements**: Purchase type determines required fields
   - "purchased" → purchaseDate required
   - "sold" → purchaseDate + saleDate required
   - "preordered" → sellerId required

---

## 4. Reusability Assessment for Export Feature

### ✅ Full Reusability - No Modifications Required

**Conclusion**: The manifest schema is **production-ready for export** with zero changes needed.

#### Why It's Reusable

1. **Bidirectional Structure**: The DTOs are design-neutral; they work equally well for:
   - **Import**: JSON → Rust structs (deserialization)
   - **Export**: Rust domain models → JSON (serialization)

2. **Schema Completeness**: The JSON Schema is comprehensive and allows for optional fields, making it flexible for partial exports.

3. **Version Lock**: Both schema and DTOs reference v1.0 consistently across the codebase.

4. **No Logic Coupling**: The manifest schema contains only data structure, no business logic.

#### Export Strategy

For the export feature (spec 016), follow this approach:

```rust
// Pseudo-code: Export process
1. Query domain entities from database (RailwayModel, CollectionItem, etc.)
2. Map database domain models → ManifestDto entities
3. Serialize ManifestDto to JSON
4. Validate output against manifest_schema.json
5. Package JSON + images in .zip archive
```

#### Potential Enhancements (Future, Not Required)

While reuse is 100% compatible, consider these optional enhancements for v1.1:

| Enhancement                 | Benefit                         | Complexity |
| --------------------------- | ------------------------------- | ---------- |
| Add `exportedAt` timestamp  | Track export origin             | Minimal    |
| Add `source` field          | Document exporter tool          | Minimal    |
| Add versioning metadata     | Support future schema evolution | Minimal    |
| Compress images in manifest | Archive optimization            | High       |

**Recommendation**: Use schema v1.0 unchanged. Plan versioning strategy for export if future domain requirements change.

---

## 5. Manifest Schema Definition Files

### Core Definition Files

#### [src-tauri/src/import/domain/manifest.rs](../../../src-tauri/src/import/domain/manifest.rs)

**Type**: Rust DTOs  
**Size**: 216 lines  
**Derives**: Debug, Clone, Serialize, Deserialize, Type (Specta)  
**Serde Config**: camelCase naming

**Key Points**:

- Uses `#[serde(rename_all = "camelCase")]` for JSON compatibility
- Optional fields use `Option<T>` with `#[serde(default)]`
- Includes Specta `Type` derive for TypeScript bindings
- All structs defined as public for cross-module access

**Structure**:

```
ManifestDto (top-level)
├── DataContainerDto (data field)
│   ├── manufacturers: Vec<ManufacturerRecord>
│   ├── railway_companies: Vec<RailwayCompanyRecord>
│   ├── railway_models: Vec<RailwayModelRecord>
│   ├── collection_items: Vec<CollectionItemRecord>
│   ├── sellers: Vec<SellerRecord>
│   └── maintenance_cards: Vec<MaintenanceCardRecord>
├── ManufacturerRecord
├── RailwayCompanyRecord
├── RailwayModelRecord
│   └── CategoryRecord
│   └── RollingStockRecord
├── CollectionItemRecord
│   └── PurchaseRecord
│       └── MoneyRecord
├── SellerRecord
│   └── AddressRecord
├── MaintenanceCardRecord
│   └── MaintenanceEventRecord
│       └── MoneyRecord
└── MoneyRecord
```

#### [src-tauri/src/import/domain/manifest_schema.json](../../../src-tauri/src/import/domain/manifest_schema.json)

**Type**: JSON Schema (Draft 2020-12)  
**Size**: 489 lines  
**Namespace**: `https://rusty-shed.app/schemas/manifest/v1.json`  
**Usage**: Runtime validation via `jsonschema` crate

**Key Features**:

- Complete JSON Schema specification language
- Supports `$ref` for type reuse
- Defines `$defs` for all entity types
- Includes validation rules (e.g., country codes as `^[A-Z]{2}$` pattern)
- Conditional validation for Purchase type-specific requirements
- `additionalProperties: false` prevents unexpected fields

**Validation Rules**:

```json
// Example: Conditional requirement for Purchase type
"allOf": [
  {
    "if": { "properties": { "type": { "const": "purchased" } } },
    "then": { "required": ["purchaseDate"] }
  },
  {
    "if": { "properties": { "type": { "const": "sold" } } },
    "then": { "required": ["purchaseDate", "saleDate"] }
  },
  {
    "if": { "properties": { "type": { "const": "preordered" } } },
    "then": { "required": ["sellerId"] }
  }
]
```

#### [specs/010-data-import-utility/contracts/manifest.schema.json](../010-data-import-utility/contracts/manifest.schema.json)

**Type**: JSON Schema (canonical reference)  
**Status**: Master schema definition  
**Relationship**: Identical to `src-tauri/src/import/domain/manifest_schema.json`

**Purpose**: Contract specification for stakeholders and external systems

---

### Test Fixture

#### [src-tauri/fixtures/test_import_manifest.json](../../../src-tauri/fixtures/test_import_manifest.json)

**Type**: Real-world example manifest  
**Contains**:

- 2 manufacturers (Märklin, Roco)
- 2 railway companies (DB, ÖBB)
- 2 railway models (ICE 3, Taurus locomotive)
- 2 collection items
- 1 seller

**Usage**:

```rust
// In tests
let manifest: ManifestDto = serde_json::from_str(TEST_MANIFEST)?;
// Validate against schema
jsonschema::validate(&manifest, &schema)?;
```

---

## 6. Implementation Guidance for Export

### Step 1: Prepare Data from Database

```rust
// Query domain models
let railway_models = repository.get_all_railway_models();
let collection_items = repository.get_all_collection_items();
let sellers = repository.get_all_sellers();
let maintenance_cards = repository.get_all_maintenance_cards();
```

### Step 2: Map to Manifest DTOs

```rust
let manufacturers: Vec<ManufacturerRecord> = railway_models
    .iter()
    .filter_map(|m| m.manufacturer())
    .map(|mfr| ManufacturerRecord {
        id: mfr.id().to_string(),
        name: mfr.name().to_string(),
        // ... other fields
    })
    .collect();
```

### Step 3: Assemble Manifest

```rust
let manifest = ManifestDto {
    schema: Some("https://rusty-shed.app/schemas/manifest/v1.json".to_string()),
    version: "1.0".to_string(),
    exported_at: Some(Utc::now().to_rfc3339()),
    source: Some("Rusty Shed v0.1.0".to_string()),
    data: DataContainerDto {
        manufacturers,
        railway_companies,
        railway_models,
        collection_items,
        sellers,
        maintenance_cards,
    },
};
```

### Step 4: Validate Output

```rust
// Load schema
let schema_str = include_str!("../src-tauri/src/import/domain/manifest_schema.json");
let schema = jsonschema::from_str(schema_str)?;

// Validate
let manifest_json = serde_json::to_value(&manifest)?;
schema.validate(&manifest_json)?;
```

### Step 5: Serialize and Package

```rust
// Serialize to JSON
let json = serde_json::to_string_pretty(&manifest)?;

// Write to archive
zip.start_file("manifest.json")?;
zip.write_all(json.as_bytes())?;

// Copy images
for image_path in image_files {
    zip.start_file(format!("images/{}", image_path.file_name()))?;
    zip.write_all(&std::fs::read(image_path)?)?;
}
```

---

## 7. Related Code Locations

### Import Module Structure

| Path                                                                                  | Purpose                                                                            |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| [src-tauri/src/import/mod.rs](../../../src-tauri/src/import/mod.rs)                   | Module root; re-exports public types                                               |
| [src-tauri/src/import/domain/](../../../src-tauri/src/import/domain/)                 | Domain layer (manifest, validation, entities)                                      |
| [src-tauri/src/import/application/](../../../src-tauri/src/import/application/)       | Use cases (validate_package, execute_import, id_mapper)                            |
| [src-tauri/src/import/infrastructure/](../../../src-tauri/src/import/infrastructure/) | Technical implementations (archive_extractor, schema_validator, duplicate_checker) |
| [src-tauri/src/import/interface/](../../../src-tauri/src/import/interface/)           | Tauri command interface and response types                                         |

### Key Implementation Files

| File                                                                                                                          | Purpose                                   |
| ----------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| [src-tauri/src/import/infrastructure/schema_validator.rs](../../../src-tauri/src/import/infrastructure/schema_validator.rs)   | JSON Schema validation logic              |
| [src-tauri/src/import/infrastructure/archive_extractor.rs](../../../src-tauri/src/import/infrastructure/archive_extractor.rs) | ZIP/TAR.GZ extraction                     |
| [src-tauri/src/import/application/execute_import.rs](../../../src-tauri/src/import/application/execute_import.rs)             | Import execution and database persistence |
| [src-tauri/src/import/application/validate_package.rs](../../../src-tauri/src/import/application/validate_package.rs)         | Package validation pipeline               |

---

## 8. Schema Validation Implementation

### Using jsonschema Crate

```rust
use jsonschema::JSONSchema;
use serde_json::json;

// Load schema
let schema_json = serde_json::json!(/* manifest schema */);
let schema = JSONSchema::compile(&schema_json)?;

// Validate manifest
let manifest_json = serde_json::to_value(&manifest)?;
match schema.validate(&manifest_json) {
    Ok(_) => println!("Valid manifest"),
    Err(e) => eprintln!("Validation error: {}", e),
}
```

### Validation Error Handling

The schema validator produces detailed error messages including:

- Path to invalid field (e.g., `data.railwayModels[0].scale`)
- Expected vs actual value
- Schema constraint violated
- Suggested fixes

---

## 9. Key Design Decisions

### Why JSON Schema + Rust Structs (Dual Definition)?

**Rationale**:

1. **JSON Schema** provides language-neutral validation and documentation
2. **Rust Structs** provide type safety and IDE support
3. **Serde** derives ensure serialization/deserialization compatibility
4. **Specta** generates TypeScript bindings automatically

This approach enables:

- Server-side validation before database write
- Client-side validation in TypeScript (future)
- Clear contract specification for external systems
- Full type safety in Rust code

### Why camelCase in JSON?

**Rationale**:

- JSON convention in web APIs (JavaScript ecosystem)
- Easier for JavaScript/TypeScript consumers
- Rust structs use snake_case (idiomatic Rust)
- Serde handles conversion automatically

### Why Embedded vs Normalized Objects?

**Decision**: Keep embedded objects (Category, Purchase, Address, Money, MaintenanceEvent) as nested objects in manifest.

**Rationale**:

- Simpler manifest structure for export/import
- No need for additional array lookups
- Natural representation of containment relationships
- Easier for users to understand the manifest structure

---

## 10. Summary Table

| Aspect                     | Details                                                                                                             |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| **Current Location**       | `/src-tauri/src/import/domain/`                                                                                     |
| **Primary Files**          | `manifest.rs`, `manifest_schema.json`                                                                               |
| **Current Version**        | 1.0 (stable)                                                                                                        |
| **Entities Included**      | 7 primary types (Manufacturer, RailwayCompany, RailwayModel, RollingStock, CollectionItem, Seller, MaintenanceCard) |
| **Relationship Count**     | 6 foreign keys + 4 embedded types                                                                                   |
| **Reusable for Export**    | ✅ YES - 100% compatible, no changes needed                                                                         |
| **Validation Method**      | JSON Schema (jsonschema crate) + Rust deserialization                                                               |
| **Backward Compatibility** | Version-locked to v1.0; optional fields support evolution                                                           |
| **Test Coverage**          | Fixture available in `/fixtures/test_import_manifest.json`                                                          |

---

## Appendix: Quick Reference

### Manifest Structure Overview

```json
{
  "$schema": "https://rusty-shed.app/schemas/manifest/v1.json",
  "version": "1.0",
  "exportedAt": "2026-02-08T...",
  "source": "Rusty Shed v0.1.0",
  "data": {
    "manufacturers": [{ id, name, ... }],
    "railwayCompanies": [{ id, name, ... }],
    "railwayModels": [{ id, manufacturerId, productCode, ... }],
    "collectionItems": [{ id, railwayModelId, addedDate, ... }],
    "sellers": [{ id, name, sellerType, ... }],
    "maintenanceCards": [{ id, collectionItemId, events, ... }]
  }
}
```

### Entity Dependency Graph

```
Manufacturer ← RailwayModel ← CollectionItem ← MaintenanceCard
    ↓              ↓
    └─ RollingStock ← RailwayCompany

CollectionItem.purchase ← Seller
```

### Required Field Summary

| Entity           | Required Fields                                                                   |
| ---------------- | --------------------------------------------------------------------------------- |
| Manufacturer     | id, name                                                                          |
| RailwayCompany   | id, name                                                                          |
| RailwayModel     | id, manufacturerId, productCode, description, scale, epoch, category, powerMethod |
| RollingStock     | railwayCompanyId, seriesCode                                                      |
| CollectionItem   | id, railwayModelId, addedDate                                                     |
| Seller           | id, name, sellerType                                                              |
| MaintenanceCard  | id, collectionItemId                                                              |
| MaintenanceEvent | id, date, type                                                                    |
| Purchase         | type (+ conditional: purchaseDate, saleDate, sellerId)                            |
| Money            | amount, currency                                                                  |
