# Manifest Schema - Visual Summary

Quick visual reference for the manifest structure and relationships

---

## Manifest Structure (JSON Tree)

```
manifest.json
├── $schema: "https://rusty-shed.app/schemas/manifest/v1.json"
├── version: "1.0"
├── exportedAt: "2026-02-08T14:30:00Z"
├── source: "Rusty Shed v0.1.0"
└── data: {
    ├── manufacturers[] (0..N)
    │   ├── id ✓
    │   ├── name ✓
    │   ├── registeredCompanyName
    │   ├── countryCode
    │   ├── status
    │   └── websiteUrl
    │
    ├── railwayCompanies[] (0..N)
    │   ├── id ✓
    │   ├── name ✓
    │   ├── abbreviation
    │   ├── countryCode
    │   └── status
    │
    ├── railwayModels[] (0..N)
    │   ├── id ✓
    │   ├── manufacturerId ✓ → manufacturers[].id
    │   ├── productCode ✓
    │   ├── description ✓
    │   ├── scale ✓ (H0, N, TT, Z, G, 0, 00, 1)
    │   ├── epoch ✓ (I, II, III, IV, V, VI)
    │   ├── category ✓ {
    │   │   ├── type ✓ (locomotive, passengerCar, etc.)
    │   │   └── subType (steam, electric, etc.)
    │   │ }
    │   ├── powerMethod ✓ (dc, ac, dcc, none)
    │   ├── details
    │   ├── deliveryDate
    │   ├── availabilityStatus
    │   ├── image
    │   └── rollingStocks[] (0..N) {
    │       ├── railwayCompanyId ✓ → railwayCompanies[].id
    │       ├── seriesCode ✓
    │       ├── roadNumber
    │       ├── livery
    │       ├── friendlyName
    │       ├── isDummy
    │       └── lengthOverBuffers
    │     }
    │
    ├── collectionItems[] (0..N)
    │   ├── id ✓
    │   ├── railwayModelId ✓ → railwayModels[].id
    │   ├── addedDate ✓ (YYYY-MM-DD)
    │   ├── removedDate
    │   ├── purchaseCondition
    │   ├── modelCondition
    │   ├── boxCondition
    │   ├── notes
    │   ├── image
    │   └── purchase {
    │       ├── type ✓ (purchased, sold, preordered)
    │       ├── purchaseDate (required if type=purchased|sold)
    │       ├── price {
    │       │   ├── amount ✓
    │       │   └── currency ✓ (EUR, GBP, etc.)
    │       │ }
    │       ├── sellerId → sellers[].id
    │       ├── saleDate (required if type=sold)
    │       ├── salePrice { amount, currency }
    │       ├── depositAmount { amount, currency }
    │       └── expectedDelivery (YYYY-MM-DD)
    │     }
    │
    ├── sellers[] (0..N)
    │   ├── id ✓
    │   ├── name ✓
    │   ├── sellerType ✓ (shop, private, marketplace, auction)
    │   ├── email
    │   ├── phone
    │   ├── websiteUrl
    │   └── address {
    │       ├── street
    │       ├── city
    │       ├── region
    │       ├── postalCode
    │       └── countryCode
    │     }
    │
    └── maintenanceCards[] (0..N)
        ├── id ✓
        ├── collectionItemId ✓ → collectionItems[].id
        ├── lastMaintenanceDate
        ├── nextMaintenanceDate
        └── events[] (0..N) {
            ├── id ✓
            ├── date ✓ (YYYY-MM-DD)
            ├── type ✓ (cleaning, lubrication, repair, modification, inspection)
            ├── description
            └── cost { amount, currency }
          }
```

**Legend**: `✓` = REQUIRED | `[ ]` = ARRAY | `{ }` = OBJECT | `→` = FOREIGN KEY

---

## Entity Relationship Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      ManifestDto (root)                       │
│  version: "1.0", exportedAt, source, schema                  │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                   DataContainerDto                            │
│  Contains 6 collections of entities                          │
└──────────┬────────────┬────────────┬────────────┬────────────┘
           │            │            │            │
    ┌──────┴─┐   ┌─────┴──┐   ┌─────┴──┐   ┌───┴─────┐
    │         │   │        │   │        │   │         │
    ▼         ▼   ▼        ▼   ▼        ▼   ▼         ▼
┌────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│Manufac-│ │RailwayCo │ │RailwayM  │ │Collectio │ │Seller    │
│turer   │ │mpany     │ │odel      │ │nItem     │ │          │
│        │ │          │ │          │ │          │ │          │
│id ✓    │ │id ✓      │ │id ✓      │ │id ✓      │ │id ✓      │
│name ✓  │ │name ✓    │ │name ✓    │ │modelId ✓ │ │name ✓    │
│...     │ │...       │ │modelId ✓ │ │date ✓    │ │type ✓    │
└────────┘ │          │ │          │ │purchase  │ │address   │
           └──────────┘ │category ✓ │ │{ type ✓} │ │{street..}│
                        │rolling    │ │          │ └──────────┘
                        │Stock[] {  │ │          │
                        │ railwayId◆ │ │          │
                        │ ...       │ │          │
                        │}          │ │          │
                        └──────────┘ │          │
                                     │          │
                                     ↓          │
                                  ┌──────────┐  │
                                  │Maintenance
                                  │Card      │  │
                                  │          │  │
                                  │id ✓      │  │
                                  │itemId ◆ ◇─┘
                                  │events[]  │
                                  │{id,date..}
                                  └──────────┘

Legend:
  ✓ = REQUIRED field
  ◆ = FOREIGN KEY (must exist in referenced array)
  ◇ = MATCHES (1:0..1 relationship)
```

---

## Uniqueness & Composite Keys

```
Entity Type           │ Unique Key(s)
──────────────────────┼──────────────────────────────────────
Manufacturer          │ id
RailwayCompany        │ id
RailwayModel          │ id
                      │ manufacturerId + productCode (COMPOSITE)
CollectionItem        │ id
                      │ railwayModelId + addedDate (COMPOSITE)
Seller                │ id
MaintenanceCard       │ id
RollingStock          │ (embedded, no independent uniqueness)
```

---

## Required vs Optional Field Summary

### Always Required (Can't Omit)

```
✓ ManifestDto.version = "1.0"
✓ ManifestDto.data (DataContainerDto)

✓ Manufacturer.id, .name
✓ RailwayCompany.id, .name
✓ RailwayModel.id, .manufacturerId, .productCode, .description,
                    .scale, .epoch, .category, .powerMethod
✓ Category.type
✓ RollingStock.railwayCompanyId, .seriesCode
✓ CollectionItem.id, .railwayModelId, .addedDate
✓ Purchase.type
✓ Seller.id, .name, .sellerType
✓ MaintenanceCard.id, .collectionItemId
✓ MaintenanceEvent.id, .date, .type
✓ Money.amount, .currency
```

### Conditionally Required

```
Purchase type affects requirements:
├─ type="purchased"   → purchaseDate REQUIRED
├─ type="sold"        → purchaseDate + saleDate REQUIRED
└─ type="preordered"  → sellerId REQUIRED
```

### Always Optional (Defaultable)

```
◌ ManifestDto.schema, .exportedAt, .source (all optional)
◌ Address.*  (all optional)
◌ Most details fields (details, notes, description in categories, etc.)
```

---

## Data Type Mapping

```
JSON Type    │ Rust Type  │ Example                 │ Notes
─────────────┼───────────┼─────────────────────────┼──────────────────
string       │ String    │ "Märklin"               │ Use for IDs, names
string (iso) │ String    │ "2026-02-08"            │ Dates: YYYY-MM-DD
string (iso) │ String    │ "2026-02-08T14:30:00Z"  │ Timestamps: RFC3339
string (uri) │ String    │ "https://..."           │ URLs
string (enum)│ String    │ "active", "dc", "mint"  │ Restricted values
integer      │ u64       │ 34999                   │ Cents/smallest unit
number       │ f64       │ 192.5                   │ Length, dimensions
boolean      │ bool      │ true, false             │ Flags
array        │ Vec<T>    │ [ { }, { } ]            │ Collections
object       │ Struct    │ { "id": "...", ... }    │ Complex types
```

---

## Foreign Key Validation Rules

```
When exporting, verify:

RailwayModel.manufacturerId
  ↓
  Must exist in one of manufacturers[].id
  Example: "mfr-001" must be in manufacturers array

RollingStock.railwayCompanyId
  ↓
  Must exist in one of railwayCompanies[].id
  Example: "rc-001" must be in railwayCompanies array

CollectionItem.railwayModelId
  ↓
  Must exist in one of railwayModels[].id
  Example: "rm-001" must be in railwayModels array

Purchase.sellerId (if present)
  ↓
  Must exist in one of sellers[].id
  Example: "seller-001" must be in sellers array

MaintenanceCard.collectionItemId
  ↓
  Must exist in one of collectionItems[].id
  Example: "ci-001" must be in collectionItems array
```

---

## Field Name Mapping (Rust ↔ JSON)

```
Rust Field Name          │ JSON Field Name      │ Type
─────────────────────────┼──────────────────────┼─────────
manufacturer_id          │ manufacturerId       │ string
railway_company_id       │ railwayCompanyId     │ string
railway_model_id         │ railwayModelId       │ string
railway_companies        │ railwayCompanies     │ array
railway_models           │ railwayModels        │ array
collection_items         │ collectionItems      │ array
seller_type              │ sellerType           │ string
seller_id                │ sellerId             │ string
registered_company_name  │ registeredCompanyName│ string
country_code             │ countryCode          │ string
website_url              │ websiteUrl           │ string
product_code             │ productCode          │ string
power_method             │ powerMethod          │ string
availability_status      │ availabilityStatus   │ string
delivery_date            │ deliveryDate         │ string
rolling_stocks           │ rollingStocks        │ array
sub_type                 │ subType              │ string
is_dummy                 │ isDummy              │ boolean
length_over_buffers      │ lengthOverBuffers    │ number
added_date               │ addedDate            │ string
removed_date             │ removedDate          │ string
purchase_condition       │ purchaseCondition    │ string
model_condition          │ modelCondition       │ string
box_condition            │ boxCondition         │ string
purchase_date            │ purchaseDate         │ string
sale_date                │ saleDate             │ string
sale_price               │ salePrice            │ object
deposit_amount           │ depositAmount        │ object
expected_delivery        │ expectedDelivery     │ string
postal_code              │ postalCode           │ string
maintenance_cards        │ maintenanceCards     │ array
collection_item_id       │ collectionItemId     │ string
last_maintenance_date    │ lastMaintenanceDate  │ string
next_maintenance_date    │ nextMaintenanceDate  │ string
exported_at              │ exportedAt           │ string
(Rust uses snake_case, JSON uses camelCase via Serde)
```

---

## Typical Export Sizes

```
Scenario: Small collection
├─ 5 manufacturers
├─ 10 railway companies
├─ 50 railway models (with 2 rolling stocks each)
├─ 100 collection items
├─ 10 sellers
├─ 100 maintenance cards
└─ ~50 images
Result: ~2-5 MB uncompressed ZIP

Scenario: Medium collection
├─ 15 manufacturers
├─ 50 railway companies
├─ 500 railway models
├─ 1000 collection items
├─ 50 sellers
├─ 1000 maintenance cards
└─ ~500 images
Result: ~50-100 MB compressed ZIP

Scenario: Large collection
├─ 30+ manufacturers
├─ 100+ railway companies
├─ 2000+ railway models
├─ 5000+ collection items
├─ 100+ sellers
├─ 5000+ maintenance cards
└─ ~1000+ images
Result: ~200-500 MB compressed ZIP (4-5 sec export time)
```

---

## Implementation Checklist

### Data Gathering

- [ ] Query all Manufacturers
- [ ] Query all RailwayCompanies
- [ ] Query all RailwayModels (with embedded RollingStocks)
- [ ] Query all CollectionItems (with embedded Purchase)
- [ ] Query all Sellers (with embedded Address)
- [ ] Query all MaintenanceCards (with embedded Events)

### Data Validation

- [ ] All RailwayModel.manufacturerId reference valid manufacturers
- [ ] All RollingStock.railwayCompanyId reference valid companies
- [ ] All CollectionItem.railwayModelId reference valid models
- [ ] All Purchase.sellerId reference valid sellers
- [ ] All MaintenanceCard.collectionItemId reference valid items
- [ ] No NULL values in required fields
- [ ] Dates in ISO format (YYYY-MM-DD)
- [ ] Composite keys are unique (manufacturerId+productCode, railwayModelId+addedDate)

### Manifest Building

- [ ] Set version = "1.0"
- [ ] Add exportedAt = current timestamp (RFC3339)
- [ ] Add source = "Rusty Shed v..."
- [ ] Add schema reference URL
- [ ] Populate all 6 data collections

### Serialization

- [ ] Serialize to JSON with pretty-print
- [ ] Validate against manifest_schema.json
- [ ] Check total record count is reasonable
- [ ] Verify no circular references

### Packaging

- [ ] Write manifest.json to ZIP root
- [ ] Include /images/ folder with all referenced images
- [ ] Verify ZIP is readable
- [ ] Test round-trip: export → import → verify
