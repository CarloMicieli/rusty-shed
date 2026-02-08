# Manifest Entity Reference - Export Feature

**Complete entity definitions and relationship rules**

---

## Entity Definitions

### 1. ManifestDto (Root)

**Purpose**: Top-level container for export/import packages

```rust
pub struct ManifestDto {
    #[serde(default)]
    pub schema: Option<String>,          // Default: None, Recommended: "https://rusty-shed.app/schemas/manifest/v1.json"

    pub version: String,                 // REQUIRED - Must be "1.0" for v1 schema

    #[serde(default)]
    pub exported_at: Option<String>,     // ISO 8601 timestamp (e.g., "2026-02-08T14:30:00Z")

    #[serde(default)]
    pub source: Option<String>,          // Application that created export (e.g., "Rusty Shed v0.1.0")

    pub data: DataContainerDto,          // REQUIRED - All relational data
}
```

**Usage in Export**:

```rust
let manifest = ManifestDto {
    schema: Some("https://rusty-shed.app/schemas/manifest/v1.json".to_string()),
    version: "1.0".to_string(),
    exported_at: Some(chrono::Utc::now().to_rfc3339()),
    source: Some("Rusty Shed v0.1.0".to_string()),
    data: build_data_container(),
};
```

---

### 2. DataContainerDto (Data Wrapper)

**Purpose**: Container for all entity collections

```rust
pub struct DataContainerDto {
    #[serde(default)]
    pub manufacturers: Vec<ManufacturerRecord>,           // 0..N

    #[serde(rename = "railwayCompanies", default)]
    pub railway_companies: Vec<RailwayCompanyRecord>,     // 0..N

    #[serde(rename = "railwayModels", default)]
    pub railway_models: Vec<RailwayModelRecord>,          // 0..N

    #[serde(rename = "collectionItems", default)]
    pub collection_items: Vec<CollectionItemRecord>,      // 0..N

    #[serde(default)]
    pub sellers: Vec<SellerRecord>,                       // 0..N

    #[serde(rename = "maintenanceCards", default)]
    pub maintenance_cards: Vec<MaintenanceCardRecord>,    // 0..N
}
```

**Notes**:

- All fields default to empty Vec if omitted
- Serde uses camelCase for JSON naming
- Typically populated from database queries

---

### 3. ManufacturerRecord

**Purpose**: Manufacturer/brand information (e.g., Märklin, Roco, Hornby)

```rust
pub struct ManufacturerRecord {
    pub id: String,                           // REQUIRED - External ID (use stable UUID or code)
    pub name: String,                         // REQUIRED - Brand name (e.g., "Märklin")
    pub registered_company_name: Option<String>, // Legal company name
    pub country_code: Option<String>,         // ISO 3166-1 alpha-2 (e.g., "DE", "AT")
    pub status: Option<String>,               // "active" or "outOfBusiness"
    pub website_url: Option<String>,          // Format: URI
}
```

**Example**:

```json
{
  "id": "mfr-001",
  "name": "Märklin",
  "registeredCompanyName": "Märklin GmbH",
  "countryCode": "DE",
  "status": "active",
  "websiteUrl": "https://www.maerklin.de"
}
```

**Uniqueness**: `id` must be unique within manufacturers array
**Referenced by**: RailwayModelRecord.manufacturerId
**Export Logic**: Query all unique manufacturers from RailwayModel records

---

### 4. RailwayCompanyRecord

**Purpose**: Railway operating company (e.g., Deutsche Bahn, French SNCF)

```rust
pub struct RailwayCompanyRecord {
    pub id: String,                    // REQUIRED - External ID
    pub name: String,                  // REQUIRED - Company name (e.g., "Deutsche Bahn")
    pub abbreviation: Option<String>,  // Short code (e.g., "DB")
    pub country_code: Option<String>,  // ISO 3166-1 alpha-2
    pub status: Option<String>,        // "active" or "inactive"
}
```

**Example**:

```json
{
  "id": "rc-001",
  "name": "Deutsche Bahn",
  "abbreviation": "DB",
  "countryCode": "DE",
  "status": "active"
}
```

**Uniqueness**: `id` must be unique within railwayCompanies array
**Referenced by**: RollingStockRecord.railwayCompanyId
**Export Logic**: Query all unique railway companies from RollingStock records

---

### 5. RailwayModelRecord

**Purpose**: A product in the manufacturer's catalog (e.g., Märklin BR 01 locomotive)

```rust
pub struct RailwayModelRecord {
    pub id: String,                        // REQUIRED - External ID
    pub manufacturer_id: String,           // REQUIRED - FK → manufacturers[].id
    pub product_code: String,              // REQUIRED - Manufacturer's catalog code (e.g., "39010")
    pub description: String,               // REQUIRED - Display name (e.g., "BR 01 Electric Locomotive")
    pub scale: String,                     // REQUIRED - Normalized scale (H0, HO, N, TT, Z, G, 0, 00, 1)
    pub epoch: String,                     // REQUIRED - Historical epoch (I, II, III, IV, V, VI)
    pub category: CategoryRecord,          // REQUIRED - Nested: type + optional subType
    pub power_method: String,              // REQUIRED - "dc", "ac", "dcc", or "none"
    pub details: Option<String>,           // Technical details
    pub delivery_date: Option<String>,     // Release date (ISO or "Q1 2024" style)
    pub availability_status: Option<String>, // "available", "announced", or "discontinued"
    pub image: Option<String>,             // Filename in /images/ (e.g., "maerklin-39010.png")
    pub rolling_stocks: Vec<RollingStockRecord>, // 0..N nested rolling stock records
}
```

**Example**:

```json
{
  "id": "rm-001",
  "manufacturerId": "mfr-001",
  "productCode": "39010",
  "description": "BR 01 Electric Locomotive",
  "scale": "H0",
  "epoch": "IV",
  "category": {
    "type": "locomotive",
    "subType": "electric"
  },
  "powerMethod": "ac",
  "deliveryDate": "2024-03-15",
  "availabilityStatus": "available",
  "image": "images/maerklin-39010.png",
  "rollingStocks": [...]
}
```

**Uniqueness**: Composite key: manufacturerId + productCode (must be unique in collection)
**Referenced by**: CollectionItemRecord.railwayModelId
**FK Requirement**: manufacturerId MUST match one of manufacturers[].id
**Export Logic**:

1. Query all RailwayModel domain objects
2. Include embedded RollingStock records
3. Ensure all manufacturerIds reference existing manufacturers
4. Ensure all rollingStockIds reference existing railwayCompanies

---

#### 5.1 CategoryRecord (Nested in RailwayModel)

**Purpose**: Categorize the railway model type

```rust
pub struct CategoryRecord {
    pub r#type: String,              // REQUIRED - Valid values:
                                     //   "locomotive", "passengerCar", "freightCar",
                                     //   "electricMultipleUnit", "railcar", "trainSet"
    pub sub_type: Option<String>,    // Optional: "steam", "electric", "tank", "covered", etc.
}
```

**Examples**:

- `{ "type": "locomotive", "subType": "electric" }`
- `{ "type": "passengerCar", "subType": "covered" }`
- `{ "type": "trainSet" }`

---

#### 5.2 RollingStockRecord (Nested in RailwayModel)

**Purpose**: Individual rolling stock units/cars represented in the model

```rust
pub struct RollingStockRecord {
    pub railway_company_id: String,      // REQUIRED - FK → railwayCompanies[].id
    pub series_code: String,             // REQUIRED - Series identifier (e.g., "BR 01", "VT 12")
    pub road_number: Option<String>,     // Individual unit number (e.g., "01 118")
    pub livery: Option<String>,          // Paint scheme (e.g., "Red/Cream")
    pub friendly_name: Option<String>,   // Display name (e.g., "Mallard")
    pub is_dummy: Option<bool>,          // true for non-powered units (default: false)
    pub length_over_buffers: Option<f64>, // Length in mm
}
```

**Example**:

```json
{
  "railwayCompanyId": "rc-001",
  "seriesCode": "BR 01",
  "roadNumber": "01 118",
  "livery": "Black/Red",
  "friendlyName": "Flying Hamburger",
  "isDummy": false,
  "lengthOverBuffers": 192.5
}
```

**Uniqueness**: No explicit uniqueness; cardinality determined by model composition
**FK Requirement**: railwayCompanyId MUST match one of railwayCompanies[].id
**Export Logic**: Query RollingStock records associated with each RailwayModel; include all

---

### 6. CollectionItemRecord

**Purpose**: A specific instance of a railway model owned by the user

```rust
pub struct CollectionItemRecord {
    pub id: String,                      // REQUIRED - External ID
    pub railway_model_id: String,        // REQUIRED - FK → railwayModels[].id
    pub added_date: String,              // REQUIRED - ISO date (YYYY-MM-DD)
    pub removed_date: Option<String>,    // ISO date (if sold/removed)
    pub purchase_condition: Option<String>, // "new", "used", or "preowned"
    pub model_condition: Option<String>,    // "mint", "excellent", "good", "fair", "poor"
    pub box_condition: Option<String>,      // "mint", "good", "damaged", "missing"
    pub notes: Option<String>,              // Custom notes
    pub image: Option<String>,              // Filename
    pub purchase: Option<PurchaseRecord>,   // Optional purchase details
}
```

**Example**:

```json
{
  "id": "ci-001",
  "railwayModelId": "rm-001",
  "addedDate": "2024-06-15",
  "purchaseCondition": "new",
  "modelCondition": "mint",
  "boxCondition": "mint",
  "notes": "Excellent condition, original box",
  "image": "images/collection-ci-001.jpg",
  "purchase": {
    "type": "purchased",
    "purchaseDate": "2024-06-15",
    "price": { "amount": 34999, "currency": "EUR" },
    "sellerId": "seller-001"
  }
}
```

**Uniqueness**: Composite key: railwayModelId + addedDate (must be unique)
**Referenced by**: MaintenanceCardRecord.collectionItemId
**FK Requirements**:

- railwayModelId MUST match one of railwayModels[].id
- purchase.sellerId (if present) MUST match one of sellers[].id
  **Export Logic**:

1. Query all CollectionItem domain objects
2. Include purchase information
3. Ensure all railwayModelIds reference existing railway models
4. Ensure all sellerIds reference existing sellers

---

#### 6.1 PurchaseRecord (Nested in CollectionItem)

**Purpose**: Acquisition or sale details for a collection item

```rust
pub struct PurchaseRecord {
    pub r#type: String,                  // REQUIRED - "purchased", "sold", or "preordered"
    pub purchase_date: Option<String>,   // ISO date (REQUIRED for "purchased" and "sold")
    pub price: Option<MoneyRecord>,      // Purchase price
    pub seller_id: Option<String>,       // FK → sellers[].id (REQUIRED for "preordered")
    pub sale_date: Option<String>,       // ISO date (REQUIRED for "sold")
    pub sale_price: Option<MoneyRecord>, // Sale price
    pub deposit_amount: Option<MoneyRecord>, // For preorders
    pub expected_delivery: Option<String>, // Expected delivery date (ISO)
}
```

**Conditional Requirements** (enforced by JSON Schema):

- Type "purchased" → purchaseDate is REQUIRED
- Type "sold" → purchaseDate and saleDate are REQUIRED
- Type "preordered" → sellerId is REQUIRED

**Examples**:

```json
// Purchased
{
  "type": "purchased",
  "purchaseDate": "2024-06-15",
  "price": { "amount": 34999, "currency": "EUR" },
  "sellerId": "seller-001"
}

// Sold
{
  "type": "sold",
  "purchaseDate": "2024-06-15",
  "price": { "amount": 34999, "currency": "EUR" },
  "saleDate": "2026-02-08",
  "salePrice": { "amount": 45000, "currency": "EUR" }
}

// Preordered
{
  "type": "preordered",
  "sellerId": "seller-002",
  "depositAmount": { "amount": 10000, "currency": "EUR" },
  "expectedDelivery": "2026-03-01"
}
```

---

#### 6.2 MoneyRecord (Nested in Purchase and MaintenanceEvent)

**Purpose**: Currency-aware monetary value

```rust
pub struct MoneyRecord {
    pub amount: u64,              // REQUIRED - Amount in smallest unit (cents, satoshis, etc.)
    pub currency: String,         // REQUIRED - ISO 4217 code (e.g., "EUR", "GBP", "JPY")
}
```

**Examples**:

- EUR 349.99 → `{ "amount": 34999, "currency": "EUR" }`
- GBP 250.00 → `{ "amount": 25000, "currency": "GBP" }`
- JPY 50000 → `{ "amount": 50000, "currency": "JPY" }`

**ISO 4217 Codes**: Use 3-letter uppercase codes (e.g., "EUR", "USD", "GBP", "JPY")

---

### 7. SellerRecord

**Purpose**: Where/from whom items were purchased

```rust
pub struct SellerRecord {
    pub id: String,                    // REQUIRED - External ID
    pub name: String,                  // REQUIRED - Seller name (e.g., "Model Train Shop")
    pub seller_type: String,           // REQUIRED - "shop", "private", "marketplace", "auction"
    pub email: Option<String>,         // Contact email (format: email)
    pub phone: Option<String>,         // Phone number
    pub website_url: Option<String>,   // Website URL (format: URI)
    pub address: Option<AddressRecord>, // Full address details
}
```

**Example**:

```json
{
  "id": "seller-001",
  "name": "Model Train Shop Munich",
  "sellerType": "shop",
  "email": "info@modeltrainshop.de",
  "phone": "+49 89 123456",
  "websiteUrl": "https://modeltrainshop.de",
  "address": {
    "street": "Bahnhofstraße 1",
    "city": "Munich",
    "region": "Bavaria",
    "postalCode": "80335",
    "countryCode": "DE"
  }
}
```

**Uniqueness**: `id` must be unique within sellers array
**Referenced by**: PurchaseRecord.sellerId
**Export Logic**: Query all unique sellers from Purchase records in CollectionItems

---

#### 7.1 AddressRecord (Nested in Seller)

**Purpose**: Full address information

```rust
pub struct AddressRecord {
    pub street: Option<String>,        // Street address
    pub city: Option<String>,          // City name
    pub region: Option<String>,        // State/province/region
    pub postal_code: Option<String>,   // ZIP/postal code
    pub country_code: Option<String>,  // ISO 3166-1 alpha-2
}
```

---

### 8. MaintenanceCardRecord

**Purpose**: Maintenance history and schedule for a collection item

```rust
pub struct MaintenanceCardRecord {
    pub id: String,                     // REQUIRED - External ID
    pub collection_item_id: String,     // REQUIRED - FK → collectionItems[].id
    pub last_maintenance_date: Option<String>, // ISO date
    pub next_maintenance_date: Option<String>, // ISO date
    pub events: Vec<MaintenanceEventRecord>,   // 0..N maintenance events
}
```

**Example**:

```json
{
  "id": "mc-001",
  "collectionItemId": "ci-001",
  "lastMaintenanceDate": "2026-01-15",
  "nextMaintenanceDate": "2026-04-15",
  "events": [
    {
      "id": "me-001",
      "date": "2025-10-12",
      "type": "cleaning",
      "description": "General cleaning and inspection"
    },
    {
      "id": "me-002",
      "date": "2025-10-13",
      "type": "lubrication",
      "cost": { "amount": 5000, "currency": "EUR" }
    }
  ]
}
```

**Uniqueness**: `id` must be unique within maintenanceCards array
**Cardinality**: 0..1 per CollectionItem (typically 1:1)
**FK Requirement**: collectionItemId MUST match one of collectionItems[].id
**Export Logic**: Query MaintenanceCard for each CollectionItem; include nested events

---

#### 8.1 MaintenanceEventRecord (Nested in MaintenanceCard)

**Purpose**: Individual maintenance action

```rust
pub struct MaintenanceEventRecord {
    pub id: String,                // REQUIRED - External ID
    pub date: String,              // REQUIRED - ISO date (YYYY-MM-DD)
    pub r#type: String,            // REQUIRED - "cleaning", "lubrication", "repair",
                                   //           "modification", or "inspection"
    pub description: Option<String>, // Details of the maintenance
    pub cost: Option<MoneyRecord>,  // Cost of maintenance
}
```

---

## Relationship Validation Rules

### During Export

Verify before writing to manifest:

| Relationship                     | Rule                                                                       | Example                                                                     |
| -------------------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| RailwayModel → Manufacturer      | Every railwayModel.manufacturerId must exist in manufacturers array        | If RM references "mfr-001", check manufacturers contains id="mfr-001"       |
| RollingStock → RailwayCompany    | Every rollingStock.railwayCompanyId must exist in railwayCompanies array   | If RS references "rc-001", check railwayCompanies contains id="rc-001"      |
| CollectionItem → RailwayModel    | Every collectionItem.railwayModelId must exist in railwayModels array      | If CI references "rm-001", check railwayModels contains id="rm-001"         |
| Purchase → Seller                | If purchase.sellerId is present, must exist in sellers array               | If purchase references "seller-001", check sellers contains id="seller-001" |
| MaintenanceCard → CollectionItem | Every maintenanceCard.collectionItemId must exist in collectionItems array | If MC references "ci-001", check collectionItems contains id="ci-001"       |

### Uniqueness Constraints

| Entity                | Uniqueness Key               | Rule                                               |
| --------------------- | ---------------------------- | -------------------------------------------------- |
| ManufacturerRecord    | id                           | id must be unique within manufacturers array       |
| RailwayCompanyRecord  | id                           | id must be unique within railwayCompanies array    |
| RailwayModelRecord    | id                           | id must be unique within railwayModels array       |
| RailwayModelRecord    | manufacturerId + productCode | Composite key must be unique (duplicate detection) |
| CollectionItemRecord  | id                           | id must be unique within collectionItems array     |
| CollectionItemRecord  | railwayModelId + addedDate   | Composite key must be unique (duplicate detection) |
| SellerRecord          | id                           | id must be unique within sellers array             |
| MaintenanceCardRecord | id                           | id must be unique within maintenanceCards array    |

---

## Field Naming Convention

**Rust → JSON Mapping** (via Serde camelCase):

| Rust Field              | JSON Field            |
| ----------------------- | --------------------- |
| manufacturer_id         | manufacturerId        |
| railway_model_id        | railwayModelId        |
| railway_companies       | railwayCompanies      |
| railway_models          | railwayModels         |
| collection_items        | collectionItems       |
| seller_type             | sellerType            |
| seller_id               | sellerId              |
| registered_company_name | registeredCompanyName |
| country_code            | countryCode           |
| website_url             | websiteUrl            |
| product_code            | productCode           |
| power_method            | powerMethod           |
| availability_status     | availabilityStatus    |
| delivery_date           | deliveryDate          |
| rolling_stocks          | rollingStocks         |
| sub_type                | subType               |
| is_dummy                | isDummy               |
| length_over_buffers     | lengthOverBuffers     |
| added_date              | addedDate             |
| removed_date            | removedDate           |
| purchase_condition      | purchaseCondition     |
| model_condition         | modelCondition        |
| box_condition           | boxCondition          |
| purchase_date           | purchaseDate          |
| sale_date               | saleDate              |
| sale_price              | salePrice             |
| deposit_amount          | depositAmount         |
| expected_delivery       | expectedDelivery      |
| postal_code             | postalCode            |
| maintenance_cards       | maintenanceCards      |
| collection_item_id      | collectionItemId      |
| last_maintenance_date   | lastMaintenanceDate   |
| next_maintenance_date   | nextMaintenanceDate   |
| exported_at             | exportedAt            |

---

## Quick Export Checklist

When building a manifest:

### Pre-Export Validation

- [ ] Query all manufacturers, railway companies, railway models, collection items, sellers, maintenance cards
- [ ] Verify no NULL/missing required fields
- [ ] Check all foreign keys reference existing records
- [ ] Validate monetary amounts as u64 (in smallest units)
- [ ] Ensure dates are ISO format (YYYY-MM-DD)

### Build ManifestDto

- [ ] Set version = "1.0"
- [ ] Add exportedAt = current timestamp (ISO 8601)
- [ ] Add source = app name/version
- [ ] Add schema reference (optional but recommended)

### Collections

- [ ] manufacturers: unique IDs, non-empty
- [ ] railway_companies: unique IDs, non-empty
- [ ] railway_models: check FK integrity to manufacturers
- [ ] rolling_stocks: check FK integrity to railway companies
- [ ] collection_items: check FK integrity to railway models
- [ ] sellers: unique IDs, referenced by purchases
- [ ] maintenance_cards: check FK integrity to collection items

### Before Serialization

- [ ] Serialize to JSON with pretty-print (for readability)
- [ ] Validate JSON against manifest_schema.json
- [ ] Check total record count is reasonable
- [ ] Verify no circular references

### Packaging

- [ ] Write manifest.json to ZIP root
- [ ] Include /images/ folder with referenced images
- [ ] Set appropriate file compression level
- [ ] Verify ZIP is readable before handing to user
