# Data Model: Digital Rolling Stock Management

**Feature**: 006-digital-rolling-stocks  
**Date**: 2026-01-30

## Overview

This feature leverages existing database tables with no schema changes required. The implementation focuses on enhanced queries and new view types to expose the required data.

---

## Existing Entities (No Changes)

### DigitalRollingStock (Aggregate)

**Table**: `digital_rolling_stocks`

| Field                  | Type      | Description                                    |
| ---------------------- | --------- | ---------------------------------------------- |
| id                     | TEXT (PK) | URN format: `trn:digital-rolling-stock:{uuid}` |
| owned_rolling_stock_id | TEXT (FK) | References `owned_rolling_stocks.id`           |
| dcc_address            | INTEGER   | DCC address (1-9999)                           |
| installed_decoder_id   | TEXT (FK) | References `decoders.id`                       |

**Domain Events**:

- `DecoderChanged { decoder_id }`
- `DccAddressChanged { dcc_address }`

### Decoder (Master Record)

**Table**: `decoders`

| Field             | Type      | Description                                     |
| ----------------- | --------- | ----------------------------------------------- |
| id                | TEXT (PK) | URN format: `trn:decoder:{manufacturer}:{code}` |
| manufacturer_id   | TEXT (FK) | References `manufacturers.id`                   |
| product_code      | TEXT      | Manufacturer's product code                     |
| decoder_type      | TEXT      | PLAIN, SOUND, FUNCTION, MULTI_PROTOCOL          |
| protocol          | TEXT      | DCC, MFX, SELECTRIX, MOTOROLA, FMZ, NEXT_18     |
| decoder_interface | TEXT      | NEM651, NEM652, PLUX_8, PLUX_16, PLUX_22, etc.  |

### OwnedRollingStock (from Collecting Domain)

**Table**: `owned_rolling_stocks`

| Field                | Type      | Description                                  |
| -------------------- | --------- | -------------------------------------------- |
| id                   | TEXT (PK) | URN format: `trn:owned-rolling-stock:{uuid}` |
| collection_item_id   | TEXT (FK) | References `collection_items.id`             |
| rolling_stock_id     | TEXT (FK) | References `rolling_stocks.id`               |
| notes                | TEXT      | User notes                                   |
| dcc_address          | INTEGER   | (Legacy) DCC address                         |
| installed_decoder_id | TEXT      | (Legacy) Decoder reference                   |

### RollingStock (from Catalog Domain)

**Table**: `rolling_stocks`

| Field              | Type      | Description                                  |
| ------------------ | --------- | -------------------------------------------- |
| id                 | TEXT (PK) | Rolling stock identifier                     |
| railway_model_id   | TEXT (FK) | References `railway_models.id`               |
| category           | TEXT      | LOCOMOTIVE, PASSENGER_CAR, FREIGHT_CAR, etc. |
| railway_company_id | TEXT (FK) | References `railway_companies.id`            |
| series_code        | TEXT      | Series/class code                            |
| road_number        | TEXT      | Road number                                  |
| control            | TEXT      | DCC_READY, DCC_FITTED, DCC_SOUND, NO_DCC     |
| is_dummy           | BOOLEAN   | True if non-motorized display model          |

---

## New/Extended View Types

### EnrichedDigitalRollingStockView (Extended)

Extends existing `DigitalRollingStockView` with catalog data:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DigitalRollingStockView {
    // Existing fields
    pub id: DigitalRollingStockId,
    pub owned_rolling_stock_id: OwnedRollingStockId,
    pub dcc_address: DccAddress,
    pub decoder: DecoderView,

    // NEW: Catalog enrichment
    pub category: RollingStockCategory,
    pub railway_company_name: Option<String>,
    pub scale: Option<Scale>,
    pub power_method: Option<PowerMethod>,
    pub road_number: Option<String>,
    pub series_code: Option<String>,
    pub description: Option<String>,
}
```

### DigitalSummary (New)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DigitalSummary {
    /// Total number of non-dummy rolling stocks
    pub total_non_dummy: u32,
    /// Number of digital rolling stocks (factory or user-installed)
    pub digital_count: u32,
    /// Percentage of digital rolling stocks (0.0 - 100.0)
    pub percentage: f32,
}
```

### CheckDuplicateAddressResult (New)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CheckDuplicateAddressResult {
    /// True if the address is already in use
    pub is_duplicate: bool,
    /// The rolling stock using this address (if any)
    pub existing_rolling_stock_id: Option<DigitalRollingStockId>,
}
```

---

## Query Patterns

### Get Digital Rolling Stocks (Enhanced)

```sql
SELECT
    drs.id,
    drs.owned_rolling_stock_id,
    drs.dcc_address,
    drs.installed_decoder_id,
    d.id AS decoder_id,
    d.manufacturer_id,
    d.product_code,
    d.decoder_type,
    d.protocol,
    d.decoder_interface,
    m.name AS manufacturer_name,
    rs.category,
    rs.road_number,
    rs.series_code,
    rc.name AS railway_company_name,
    rm.scale,
    rm.power_method
FROM digital_rolling_stocks drs
JOIN decoders d ON drs.installed_decoder_id = d.id
JOIN manufacturers m ON d.manufacturer_id = m.id
JOIN owned_rolling_stocks ors ON drs.owned_rolling_stock_id = ors.id
LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
LEFT JOIN railway_companies rc ON rs.railway_company_id = rc.id
LEFT JOIN railway_models rm ON rs.railway_model_id = rm.id
WHERE d.decoder_type != 'FUNCTION'
ORDER BY drs.dcc_address ASC
```

### Get Digital Summary

```sql
SELECT
    COALESCE(SUM(CASE WHEN rs.is_dummy = 0 OR rs.is_dummy IS NULL THEN 1 ELSE 0 END), 0) as total_non_dummy,
    COALESCE(SUM(
        CASE
            WHEN (rs.is_dummy = 0 OR rs.is_dummy IS NULL)
            AND (rs.control IN ('DCC_SOUND', 'DCC_FITTED') OR drs.id IS NOT NULL)
            THEN 1
            ELSE 0
        END
    ), 0) as digital_count
FROM owned_rolling_stocks ors
LEFT JOIN rolling_stocks rs ON ors.rolling_stock_id = rs.id
LEFT JOIN digital_rolling_stocks drs ON drs.owned_rolling_stock_id = ors.id
JOIN collection_items ci ON ors.collection_item_id = ci.id
WHERE ci.removed_date IS NULL
```

### Check Duplicate Address

```sql
SELECT id
FROM digital_rolling_stocks
WHERE dcc_address = ?
AND id != COALESCE(?, '')
LIMIT 1
```

---

## Entity Relationships

```
┌─────────────────────┐
│  digital_rolling_   │
│      stocks         │
├─────────────────────┤
│ id (PK)             │
│ owned_rolling_      │──────┐
│   stock_id (FK)     │      │
│ dcc_address         │      │
│ installed_decoder_  │──┐   │
│   id (FK)           │  │   │
└─────────────────────┘  │   │
                         │   │
┌─────────────────────┐  │   │   ┌─────────────────────┐
│      decoders       │◄─┘   └──►│  owned_rolling_     │
├─────────────────────┤          │      stocks         │
│ id (PK)             │          ├─────────────────────┤
│ manufacturer_id (FK)│──┐       │ id (PK)             │
│ product_code        │  │       │ rolling_stock_id    │──┐
│ decoder_type        │  │       │ collection_item_id  │  │
│ protocol            │  │       │ notes               │  │
│ decoder_interface   │  │       └─────────────────────┘  │
└─────────────────────┘  │                                │
                         │                                │
┌─────────────────────┐  │       ┌─────────────────────┐  │
│   manufacturers     │◄─┘       │   rolling_stocks    │◄─┘
├─────────────────────┤          ├─────────────────────┤
│ id (PK)             │          │ id (PK)             │
│ name                │          │ category            │
└─────────────────────┘          │ railway_company_id  │──┐
                                 │ railway_model_id    │──┼─┐
                                 │ road_number         │  │ │
                                 │ control             │  │ │
                                 │ is_dummy            │  │ │
                                 └─────────────────────┘  │ │
                                                          │ │
                                 ┌─────────────────────┐  │ │
                                 │ railway_companies   │◄─┘ │
                                 ├─────────────────────┤    │
                                 │ id (PK)             │    │
                                 │ name                │    │
                                 └─────────────────────┘    │
                                                            │
                                 ┌─────────────────────┐    │
                                 │  railway_models     │◄───┘
                                 ├─────────────────────┤
                                 │ id (PK)             │
                                 │ scale               │
                                 │ power_method        │
                                 └─────────────────────┘
```

---

## Validation Rules

| Entity              | Field                  | Rule                                           |
| ------------------- | ---------------------- | ---------------------------------------------- |
| DccAddress          | value                  | 1 ≤ value ≤ 9999                               |
| DigitalRollingStock | owned_rolling_stock_id | Must reference existing owned rolling stock    |
| DigitalRollingStock | installed_decoder_id   | Must reference existing decoder                |
| DigitalRollingStock | dcc_address            | Unique with soft warning (not hard constraint) |

---

## State Transitions

### DigitalRollingStock Lifecycle

```
[Not Exists] ──(install_decoder)──► [Active]
                                        │
                                        ├──(change_address)──► [Active]
                                        │
                                        ├──(change_decoder)──► [Active]
                                        │
                                        └──(remove)*──► [Removed]

* Note: Remove is not in current scope but implied for completeness
```
