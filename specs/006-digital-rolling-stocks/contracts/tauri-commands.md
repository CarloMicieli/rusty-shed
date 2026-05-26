# API Contracts: Digital Rolling Stock Management

**Feature**: 006-digital-rolling-stocks  
**Date**: 2026-01-30  
**Protocol**: Tauri IPC (invoke commands)

## Overview

This document defines the Tauri command contracts for the Digital Rolling Stock Management feature. All commands follow ADR 8 conventions using specta-generated TypeScript bindings.

---

## Existing Commands (No Changes)

These commands already exist and will be used as-is:

### `new_digital_rolling_stock`

Creates a new digital rolling stock entry.

**Request Args**:

```typescript
interface NewDigitalRollingStockArgs {
  ownedRollingStockId: string; // OwnedRollingStockId URN
  dccAddress: number; // 1-9999
  decoderId: string; // DecoderId URN
}
```

**Response**:

```typescript
interface ResponseNewDigitalRollingStock {
  id: DigitalRollingStockId; // Created entry ID
}
```

### `change_dcc_address`

Updates the DCC address of an existing digital rolling stock.

**Request Args**:

```typescript
interface ChangeDccAddressArgs {
  id: string; // DigitalRollingStockId URN
  newDccAddress: number; // 1-9999
}
```

**Response**: `null` (success) or `CommandError`

### `change_decoder`

Replaces the decoder on an existing digital rolling stock.

**Request Args**:

```typescript
interface ChangeDecoderArgs {
  id: string; // DigitalRollingStockId URN
  decoderId: string; // DecoderId URN
}
```

**Response**: `null` (success) or `CommandError`

---

## Enhanced Commands

### `get_digital_rolling_stocks` (Enhanced)

Retrieves all digital rolling stocks with enriched catalog data.

**Request Args**: None

**Response**:

```typescript
interface DigitalRollingStockView {
  // Existing fields
  id: DigitalRollingStockId;
  ownedRollingStockId: OwnedRollingStockId;
  dccAddress: DccAddress; // number (1-9999)
  decoder: DecoderView;

  // NEW: Enriched catalog data
  category: RollingStockCategory;
  railwayCompanyName: string | null;
  scale: Scale | null;
  powerMethod: PowerMethod | null;
  roadNumber: string | null;
  seriesCode: string | null;
  description: string | null;
}

interface DecoderView {
  id: DecoderId;
  manufacturer: string;
  productCode: string;
  decoderType: DecoderType;
  protocol: DigitalProtocol;
  decoderInterface: DccInterface;
}
```

**Behavior Changes**:

- Now excludes digital rolling stocks with `Function` type decoders
- Returns enriched view with catalog data

---

## New Commands

### `get_digital_summary`

Retrieves the digital rolling stock summary statistics.

**Request Args**: None

**Response**:

```typescript
interface DigitalSummary {
  /** Total number of non-dummy rolling stocks in collection */
  totalNonDummy: number;
  /** Number of digital rolling stocks (factory-fitted or user-installed) */
  digitalCount: number;
  /** Percentage of digital rolling stocks (0.0 - 100.0) */
  percentage: number;
}
```

**Rust Handler**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_digital_summary(
    state: tauri::State<'_, AppState>,
) -> Result<DigitalSummary, CommandError>
```

---

### `get_decoders`

Retrieves all available decoders for the installation dropdown.

**Request Args**: None

**Response**:

```typescript
interface Decoder {
  id: DecoderId;
  manufacturerId: ManufacturerId;
  productCode: string;
  decoderType: DecoderType;
  protocol: DigitalProtocol;
  decoderInterface: DccInterface;
}
```

**Rust Handler**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_decoders(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Decoder>, CommandError>
```

---

### `check_dcc_address_duplicate`

Checks if a DCC address is already in use.

**Request Args**:

```typescript
interface CheckDccAddressDuplicateArgs {
  /** The DCC address to check (1-9999) */
  dccAddress: number;
  /** Optional: exclude this ID from the check (for edit mode) */
  excludeId?: string;
}
```

**Response**:

```typescript
interface CheckDuplicateAddressResult {
  /** True if the address is already in use by another rolling stock */
  isDuplicate: boolean;
  /** The ID of the existing rolling stock using this address, if any */
  existingRollingStockId: DigitalRollingStockId | null;
}
```

**Rust Handler**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn check_dcc_address_duplicate(
    state: tauri::State<'_, AppState>,
    args: CheckDccAddressDuplicateArgs,
) -> Result<CheckDuplicateAddressResult, CommandError>
```

---

### `get_installable_rolling_stocks`

Retrieves rolling stocks from the collection that can have a decoder installed.

**Request Args**: None

**Response**:

```typescript
interface InstallableRollingStockView {
  id: OwnedRollingStockId;
  seriesCode: string;
  roadNumber: string | null;
  railwayCompanyName: string | null;
  category: RollingStockCategory;
  /** True if this rolling stock already has a decoder installed */
  hasDecoder: boolean;
  /** Current DCC address if decoder is installed */
  currentDccAddress: number | null;
}
```

**Rust Handler**:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_installable_rolling_stocks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<InstallableRollingStockView>, CommandError>
```

**Notes**:

- Excludes rolling stocks with `is_dummy = true`
- Includes rolling stocks that already have decoders (for replacement flow)
- Used to populate the rolling stock dropdown in decoder installation form

---

## Error Handling

All commands return `Result<T, CommandError>` where:

```typescript
interface CommandError {
  kind: 'Validation' | 'NotFound' | 'Internal' | 'Conflict';
  message: string;
  field?: string; // For validation errors
}
```

### Error Cases

| Command                     | Error      | Condition                                  |
| --------------------------- | ---------- | ------------------------------------------ |
| `new_digital_rolling_stock` | Validation | Invalid DCC address (< 1 or > 9999)        |
| `new_digital_rolling_stock` | NotFound   | Rolling stock or decoder not found         |
| `change_dcc_address`        | Validation | Invalid DCC address                        |
| `change_dcc_address`        | NotFound   | Digital rolling stock not found            |
| `change_decoder`            | NotFound   | Digital rolling stock or decoder not found |

---

## Type Definitions (Shared)

```typescript
// Strongly-typed ID strings (URN format)
type DigitalRollingStockId = string; // trn:owned-rolling-stock:{uuid}
type OwnedRollingStockId = string; // trn:owned-rolling-stock:{uuid}
type DecoderId = string; // trn:decoder:{manufacturer}:{code}
type ManufacturerId = string; // trn:manufacturer:{name}

// Value type
type DccAddress = number; // 1-9999

// Enums
type DecoderType = 'PLAIN' | 'SOUND' | 'FUNCTION' | 'MULTI_PROTOCOL';
type DigitalProtocol = 'DCC' | 'MFX' | 'SELECTRIX' | 'MOTOROLA' | 'FMZ' | 'NEXT_18';
type DccInterface =
  | 'NEM651'
  | 'NEM652'
  | 'PLUX_8'
  | 'PLUX_16'
  | 'PLUX_22'
  | 'NEXT_18'
  | 'MTC_21'
  | 'PluX_22';
type RollingStockCategory =
  | 'LOCOMOTIVE'
  | 'PASSENGER_CAR'
  | 'FREIGHT_CAR'
  | 'TRAIN_SET'
  | 'RAILCAR'
  | 'ELECTRIC_MULTIPLE_UNIT'
  | 'STARTER_SET';
type Scale = 'H0' | 'N' | 'TT' | 'Z' | 'O' | 'G' | '1';
type PowerMethod = 'DC' | 'AC' | 'BATTERY' | 'STEAM';
```

---

## Command Registration

New commands must be registered in `src-tauri/src/lib.rs`:

```rust
// Add to command list
tauri::generate_handler![
    // ... existing commands ...
    dcc_inventory::interface::command_handlers::get_digital_summary,
    dcc_inventory::interface::command_handlers::get_decoders,
    dcc_inventory::interface::command_handlers::check_dcc_address_duplicate,
    dcc_inventory::interface::command_handlers::get_installable_rolling_stocks,
]
```

And in specta type generation for TypeScript bindings.
