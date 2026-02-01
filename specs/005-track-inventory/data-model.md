# Data Model: Track Inventory Feature

**Feature**: 005-track-inventory  
**Created**: 2026-01-30

## Entity Relationship Diagram

```
┌─────────────────────┐       ┌─────────────────────┐
│   TrackInventory    │       │    TrackProduct     │
├─────────────────────┤       ├─────────────────────┤
│ id: TrackInventoryId│       │ track_id: TrackId   │
│ name: string        │       │ manufacturer_id     │──┐
│ description?: string│       │ product_code: string│  │
│ created_at          │       │ description: string │  │
│ updated_at          │       │ track_type: TrackType│  │
└────────┬────────────┘       │ track_code: TrackCode│  │
         │                    │ with_roadbed: bool  │  │
         │ 1:N                │ length?: Length     │  │
         │                    │ radius?: Length     │  │
         ▼                    └──────────┬──────────┘  │
┌─────────────────────┐                  │             │
│ TrackInventoryItem  │◄─────────────────┘             │
├─────────────────────┤       N:1                      │
│ inventory_id        │──┐                             │
│ track_id            │  │                             │
│ quantity: i64       │  │                             │
│ required: i64       │  │    ┌─────────────────────┐  │
└─────────────────────┘  │    │    Manufacturer     │◄─┘
                         │    ├─────────────────────┤
                         │    │ id: ManufacturerId  │
┌─────────────────────┐  │    │ name: string        │
│   TrackPurchase     │  │    └─────────────────────┘
├─────────────────────┤  │
│ id: TrackPurchaseId │  │
│ inventory_id        │──┘
│ track_id            │──────► TrackProduct
│ quantity: i64       │
│ price: MonetaryAmount│
│ seller_id?: SellerId│──────► Seller
│ purchase_date       │
│ created_at          │
└─────────────────────┘
```

## Entities

### TrackInventory (Aggregate Root)

A named collection of track pieces for a specific purpose (e.g., a layout project).

| Field              | Type                          | Required | Description                   |
| ------------------ | ----------------------------- | -------- | ----------------------------- |
| `id`               | `TrackInventoryId`            | ✓        | Unique identifier             |
| `name`             | `string`                      | ✓        | Human-readable name           |
| `description`      | `string`                      |          | Optional detailed description |
| `inventory`        | `Map<TrackId, TrackQuantity>` | ✓        | Current stock by track type   |
| `purchase_history` | `TrackPurchase[]`             | ✓        | Chronological purchase list   |
| `metadata`         | `Metadata`                    | ✓        | Timestamps, version           |

**Invariants**:

- Name must be non-empty
- Quantities must be non-negative

### TrackProduct

A catalog entry for a specific track piece from a manufacturer.

| Field             | Type             | Required | Description                                 |
| ----------------- | ---------------- | -------- | ------------------------------------------- |
| `track_id`        | `TrackId`        | ✓        | TRN identifier (e.g., `trn:track:man:code`) |
| `manufacturer_id` | `ManufacturerId` | ✓        | Reference to manufacturer                   |
| `product_code`    | `string`         | ✓        | Manufacturer's product code                 |
| `description`     | `string`         | ✓        | Human-readable description                  |
| `track_type`      | `TrackType`      | ✓        | Geometric type (NEW)                        |
| `track_code`      | `TrackCode`      | ✓        | Rail profile code                           |
| `with_roadbed`    | `boolean`        | ✓        | Has integrated roadbed                      |
| `length`          | `Length`         |          | Length for straight track                   |
| `radius`          | `Length`         |          | Radius for curved track                     |
| `metadata`        | `Metadata`       | ✓        | Timestamps, version                         |

**Constraints**:

- `manufacturer_id` + `product_code` must be unique

### TrackType (Enum)

Geometric classification of track pieces.

| Value        | Description            |
| ------------ | ---------------------- |
| `STRAIGHT`   | Straight track section |
| `CURVE`      | Curved track section   |
| `TURNOUT`    | Switch/turnout/points  |
| `FLEX_TRACK` | Flexible track         |

### TrackCode (Enum)

Rail profile height classification.

| Value      | Description                  |
| ---------- | ---------------------------- |
| `CODE_70`  | 70-series rail (lightweight) |
| `CODE_75`  | 75-series rail               |
| `CODE_83`  | 83-series rail (common)      |
| `CODE_100` | 100-series rail (heavy)      |

### TrackPurchase

A record of track acquisition.

| Field           | Type               | Required | Description                 |
| --------------- | ------------------ | -------- | --------------------------- |
| `id`            | `TrackPurchaseId`  | ✓        | Unique identifier           |
| `inventory_id`  | `TrackInventoryId` | ✓        | Parent inventory            |
| `track_id`      | `TrackId`          | ✓        | Track product purchased     |
| `quantity`      | `i64`              | ✓        | Number of pieces            |
| `price`         | `MonetaryAmount`   | ✓        | Total purchase price        |
| `seller_id`     | `SellerId`         |          | Shop or collector reference |
| `purchase_date` | `NaiveDate`        | ✓        | When purchased              |
| `created_at`    | `DateTime`         | ✓        | Record creation time        |

### TrackQuantity (Value Object)

Stock level for a specific track type.

| Field      | Type      | Required | Description             |
| ---------- | --------- | -------- | ----------------------- |
| `track_id` | `TrackId` | ✓        | Track product reference |
| `quantity` | `i64`     | ✓        | Current stock count     |

## View Structs (Application Layer)

### TrackInventoryListItem

For displaying inventory list.

| Field            | Type               | Description                   |
| ---------------- | ------------------ | ----------------------------- |
| `id`             | `TrackInventoryId` | Inventory ID                  |
| `name`           | `string`           | Inventory name                |
| `description`    | `string?`          | Optional description          |
| `total_items`    | `i64`              | Count of distinct track types |
| `total_quantity` | `i64`              | Sum of all piece quantities   |

### TrackInventoryView

Full inventory detail for single-inventory view.

| Field         | Type                       | Description                 |
| ------------- | -------------------------- | --------------------------- |
| `id`          | `TrackInventoryId`         | Inventory ID                |
| `name`        | `string`                   | Inventory name              |
| `description` | `string?`                  | Optional description        |
| `items`       | `TrackInventoryItemView[]` | Track items with quantities |
| `purchases`   | `TrackPurchaseView[]`      | Purchase history            |

### TrackInventoryItemView

Single item in an inventory.

| Field           | Type               | Description                  |
| --------------- | ------------------ | ---------------------------- |
| `track_id`      | `TrackId`          | Track product ID             |
| `track_product` | `TrackProductView` | Product details              |
| `quantity`      | `i64`              | Current stock                |
| `required`      | `i64`              | Required quantity (planning) |

### TrackProductView

Track product for display.

| Field               | Type        | Description                      |
| ------------------- | ----------- | -------------------------------- |
| `track_id`          | `TrackId`   | Product ID                       |
| `manufacturer_name` | `string`    | Manufacturer name (denormalized) |
| `product_code`      | `string`    | Product code                     |
| `description`       | `string`    | Description                      |
| `track_type`        | `TrackType` | Geometric type                   |
| `track_code`        | `TrackCode` | Rail profile                     |
| `with_roadbed`      | `boolean`   | Has roadbed                      |
| `length`            | `Length?`   | Length (straight)                |
| `radius`            | `Length?`   | Radius (curved)                  |

### TrackPurchaseView

Purchase history entry.

| Field           | Type               | Description                |
| --------------- | ------------------ | -------------------------- |
| `id`            | `TrackPurchaseId`  | Purchase ID                |
| `track_product` | `TrackProductView` | Product purchased          |
| `quantity`      | `i64`              | Quantity purchased         |
| `price`         | `MonetaryAmount`   | Total price                |
| `seller_name`   | `string?`          | Seller name (denormalized) |
| `purchase_date` | `NaiveDate`        | Purchase date              |

## Database Schema (Existing + Changes)

### Existing Tables (from migration 0006)

- `track_products` - Track product catalog
- `track_inventories` - Inventory headers
- `track_inventory_items` - Inventory line items
- `track_purchases` - Purchase records

### Schema Changes Required

**Migration 0007**: Add `track_type` column

```sql
ALTER TABLE track_products ADD COLUMN track_type TEXT;
UPDATE track_products SET track_type = 'STRAIGHT' WHERE track_type IS NULL;
```

**Future Migration**: Add `required` column to `track_inventory_items`

```sql
ALTER TABLE track_inventory_items ADD COLUMN required INTEGER NOT NULL DEFAULT 0;
```

## Validation Rules

| Entity         | Rule                           | Error Key                          |
| -------------- | ------------------------------ | ---------------------------------- |
| TrackInventory | Name must be 1-100 characters  | `error_inventory_name_required`    |
| TrackInventory | Name must be unique            | `error_inventory_name_duplicate`   |
| TrackProduct   | Product code must be non-empty | `error_product_code_required`      |
| TrackProduct   | Manufacturer must exist        | `error_manufacturer_not_found`     |
| TrackPurchase  | Quantity must be positive      | `error_purchase_quantity_positive` |
| TrackPurchase  | Track product must exist       | `error_track_product_not_found`    |
| TrackPurchase  | Inventory must exist           | `error_inventory_not_found`        |
