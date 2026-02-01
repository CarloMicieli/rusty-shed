# Data Model: Data Import Utility

**Feature**: 010-data-import-utility  
**Created**: January 30, 2026  
**Status**: Complete

---

## Overview

This document defines the data model for the import utility, including:

1. **Manifest Schema** - The structure of `manifest.json` within import packages
2. **Domain Entities** - Internal entities for managing import sessions
3. **Value Objects** - Supporting types for validation and results

---

## 1. Manifest Schema

The `manifest.json` file is the core data structure within import packages. It contains all relational data for migration.

### 1.1 Top-Level Structure

```json
{
  "$schema": "https://rusty-shed.app/schemas/manifest/v1.json",
  "version": "1.0",
  "exportedAt": "2026-01-30T14:30:00Z",
  "source": "External Application v2.0",
  "data": {
    "manufacturers": [...],
    "railwayCompanies": [...],
    "railwayModels": [...],
    "collectionItems": [...],
    "sellers": [...],
    "maintenanceCards": [...]
  }
}
```

### 1.2 Manufacturer Record

```typescript
interface ManufacturerRecord {
  // Required
  id: string; // External ID for internal references
  name: string; // e.g., "Märklin"

  // Optional
  registeredCompanyName?: string; // e.g., "Gebr. Märklin & Cie. GmbH"
  countryCode?: string; // ISO 3166-1 alpha-2, e.g., "DE"
  status?: 'active' | 'outOfBusiness'; // Defaults to "active"
  websiteUrl?: string; // Valid URL
}
```

### 1.3 Railway Company Record

```typescript
interface RailwayCompanyRecord {
  // Required
  id: string;
  name: string; // e.g., "Deutsche Bahn"

  // Optional
  abbreviation?: string; // e.g., "DB"
  countryCode?: string; // ISO 3166-1 alpha-2
  status?: 'active' | 'inactive';
}
```

### 1.4 Railway Model Record

```typescript
interface RailwayModelRecord {
  // Required
  id: string; // External ID for internal references
  manufacturerId: string; // References ManufacturerRecord.id
  productCode: string; // e.g., "39010"
  description: string; // e.g., "BR 01 Steam Locomotive"
  scale: string; // e.g., "H0", "HO", "N" (normalized during import)
  epoch: string; // e.g., "III", "IV", "V"
  category: CategoryRecord; // See below
  powerMethod: 'dc' | 'ac' | 'dcc' | 'none';

  // Optional
  details?: string;
  deliveryDate?: string; // ISO date or "Q1 2024" format
  availabilityStatus?: 'available' | 'announced' | 'discontinued';
  image?: string; // Filename in /images/ folder
  rollingStocks?: RollingStockRecord[];
}

interface CategoryRecord {
  type:
    | 'locomotive'
    | 'passengerCar'
    | 'freightCar'
    | 'electricMultipleUnit'
    | 'railcar'
    | 'trainSet';
  subType?: string; // e.g., "steam", "electric", "tank", "covered"
}

interface RollingStockRecord {
  // Required
  railwayCompanyId: string; // References RailwayCompanyRecord.id
  seriesCode: string; // e.g., "BR 01"

  // Optional
  roadNumber?: string; // e.g., "01 118"
  livery?: string; // e.g., "Red/Black"
  friendlyName?: string;
  isDummy?: boolean; // For non-powered units in sets
  lengthOverBuffers?: number; // In mm
}
```

### 1.5 Collection Item Record

```typescript
interface CollectionItemRecord {
  // Required
  id: string;
  railwayModelId: string; // References RailwayModelRecord.id
  addedDate: string; // ISO date YYYY-MM-DD

  // Optional
  removedDate?: string; // ISO date if sold/removed
  purchaseCondition?: 'new' | 'used' | 'preowned';
  modelCondition?: 'mint' | 'excellent' | 'good' | 'fair' | 'poor';
  boxCondition?: 'mint' | 'good' | 'damaged' | 'missing';
  notes?: string;
  image?: string; // Filename in /images/ folder

  // Purchase Information
  purchase?: PurchaseRecord;
}

interface PurchaseRecord {
  type: 'purchased' | 'sold' | 'preordered';

  // For purchased/sold
  purchaseDate?: string; // ISO date
  price?: MoneyRecord;
  sellerId?: string; // References SellerRecord.id

  // For sold items
  saleDate?: string;
  salePrice?: MoneyRecord;

  // For preorders
  depositAmount?: MoneyRecord;
  expectedDelivery?: string;
}

interface MoneyRecord {
  amount: number; // In smallest unit (cents)
  currency: string; // ISO 4217, e.g., "EUR", "USD"
}
```

### 1.6 Seller Record

```typescript
interface SellerRecord {
  // Required
  id: string;
  name: string;
  sellerType: 'shop' | 'private' | 'marketplace' | 'auction';

  // Optional
  email?: string;
  phone?: string;
  websiteUrl?: string;
  address?: AddressRecord;
}

interface AddressRecord {
  street?: string;
  city?: string;
  region?: string;
  postalCode?: string;
  countryCode?: string; // ISO 3166-1 alpha-2
}
```

### 1.7 Maintenance Card Record

```typescript
interface MaintenanceCardRecord {
  // Required
  id: string;
  collectionItemId: string; // References CollectionItemRecord.id (via owned rolling stock)

  // Optional
  lastMaintenanceDate?: string; // ISO date
  nextMaintenanceDate?: string; // ISO date
  events?: MaintenanceEventRecord[];
}

interface MaintenanceEventRecord {
  id: string;
  date: string; // ISO date
  type: 'cleaning' | 'lubrication' | 'repair' | 'modification' | 'inspection';
  description?: string;
  cost?: MoneyRecord;
}
```

---

## 2. Domain Entities

### 2.1 ImportSession (Aggregate Root)

The `ImportSession` manages the state of an import operation from analysis through execution.

```rust
/// Represents an active import session.
/// This is the aggregate root for import operations.
pub struct ImportSession {
    /// Unique session identifier
    pub id: ImportSessionId,

    /// Path to the original archive file
    pub source_path: PathBuf,

    /// Archive format detected
    pub format: ArchiveFormat,

    /// Current state of the session
    pub state: ImportState,

    /// Parsed manifest data
    pub manifest: Option<Manifest>,

    /// Validation results
    pub validation: ValidationResult,

    /// Preview statistics
    pub preview: ImportPreview,

    /// Execution results (populated after import)
    pub result: Option<ImportResult>,

    /// Session timestamps
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub enum ImportState {
    /// Initial state after file selection
    Pending,
    /// Manifest extracted and parsed
    Analyzed,
    /// Schema validation complete
    Validated,
    /// Preview generated with duplicate detection
    Previewed,
    /// Import in progress
    Importing,
    /// Import completed successfully
    Completed,
    /// Import failed or aborted
    Failed { reason: String },
}
```

### 2.2 ImportPreview (Value Object)

Statistics and data shown to user before confirmation.

```rust
/// Preview summary for user confirmation.
pub struct ImportPreview {
    /// Total records found in manifest
    pub total_records: RecordCounts,

    /// Valid records that can be imported
    pub valid_records: RecordCounts,

    /// Records that will be skipped (duplicates)
    pub duplicate_records: RecordCounts,

    /// Validation errors (blocking)
    pub errors: Vec<ValidationError>,

    /// Warnings (non-blocking, e.g., missing images)
    pub warnings: Vec<ImportWarning>,
}

/// Counts per entity type.
pub struct RecordCounts {
    pub manufacturers: u32,
    pub railway_companies: u32,
    pub railway_models: u32,
    pub collection_items: u32,
    pub sellers: u32,
    pub maintenance_cards: u32,
}
```

### 2.3 ImportResult (Value Object)

Final result after import execution.

```rust
/// Result of a completed import operation.
pub struct ImportResult {
    /// Session that was executed
    pub session_id: ImportSessionId,

    /// Records successfully added
    pub added: RecordCounts,

    /// Records skipped (duplicates)
    pub skipped: RecordCounts,

    /// Images successfully imported
    pub images_imported: u32,

    /// Images that failed (with reasons)
    pub images_failed: Vec<ImageFailure>,

    /// Total execution duration
    pub duration: Duration,

    /// Any warnings during import
    pub warnings: Vec<ImportWarning>,
}

pub struct ImageFailure {
    pub filename: String,
    pub reason: String,
}
```

### 2.4 ValidationError (Value Object)

Structured validation errors for user feedback.

```rust
/// A validation error that blocks import.
pub struct ValidationError {
    /// Path in manifest (e.g., "data.railwayModels[3].productCode")
    pub path: String,

    /// Error code for i18n lookup
    pub code: ValidationErrorCode,

    /// Human-readable message
    pub message: String,

    /// Additional context
    pub context: Option<serde_json::Value>,
}

pub enum ValidationErrorCode {
    // Schema errors
    MissingRequiredField,
    InvalidDataType,
    InvalidEnumValue,
    InvalidFormat,

    // Relationship errors
    OrphanedReference,
    CircularReference,
    DuplicateId,

    // Business rule errors
    InvalidScale,
    InvalidEpoch,
    InvalidCurrency,
}
```

---

## 3. Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        Import Package                            │
│  ┌──────────────┐  ┌─────────────────────────────────────────┐  │
│  │ manifest.json│  │ /images/                                 │  │
│  │              │  │   br01_loco.jpg                          │  │
│  │  - data      │──│   collection_item_1.png                  │  │
│  │              │  │   ...                                    │  │
│  └──────────────┘  └─────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Manifest Structure                          │
│                                                                  │
│  manufacturers ◄──────────┐                                     │
│       │                   │                                     │
│       ▼                   │                                     │
│  railwayModels ───────────┤                                     │
│       │                   │                                     │
│       │  ◄── rollingStocks ──► railwayCompanies                │
│       │                                                         │
│       ▼                                                         │
│  collectionItems ─────► purchase ──► sellers                   │
│       │                                                         │
│       ▼                                                         │
│  maintenanceCards                                               │
│       │                                                         │
│       └──► maintenanceEvents                                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Duplicate Detection Keys

| Entity           | Duplicate Key                    | Comparison                      |
| ---------------- | -------------------------------- | ------------------------------- |
| Manufacturer     | `name` (normalized)              | Case-insensitive                |
| Railway Company  | `name` (normalized)              | Case-insensitive                |
| Railway Model    | `(manufacturerId, productCode)`  | Exact match after ID resolution |
| Collection Item  | `(railwayModelId, purchaseDate)` | Exact match after ID resolution |
| Seller           | `name` (normalized)              | Case-insensitive                |
| Maintenance Card | `collectionItemId`               | One-to-one relationship         |

---

## 5. State Transitions

```
Pending ──► Analyzed ──► Validated ──► Previewed ──► Importing ──► Completed
   │           │            │             │              │
   └───────────┴────────────┴─────────────┴──────────────┴──────► Failed
```

| Transition            | Trigger                          | Validation                             |
| --------------------- | -------------------------------- | -------------------------------------- |
| Pending → Analyzed    | `analyze_package`                | Archive readable, manifest.json exists |
| Analyzed → Validated  | `validate_manifest`              | Schema validation passes               |
| Validated → Previewed | `generate_preview`               | Duplicate detection complete           |
| Previewed → Importing | `execute_import` (user confirms) | User clicked "Confirm"                 |
| Importing → Completed | All writes succeed               | Transaction committed                  |
| \* → Failed           | Any error                        | Transaction rolled back                |

---

## 6. ID Mapping Strategy

Import packages use external IDs that must be mapped to internal database IDs.

```rust
/// Maps external manifest IDs to internal database IDs.
pub struct IdMapping {
    manufacturers: HashMap<String, ManufacturerId>,
    railway_companies: HashMap<String, RailwayCompanyId>,
    railway_models: HashMap<String, RailwayModelId>,
    collection_items: HashMap<String, CollectionItemId>,
    sellers: HashMap<String, SellerId>,
}
```

### Mapping Rules

1. **New Records**: Generate new UUID-based IDs using existing domain ID constructors
2. **Duplicate Records**: Map to existing database ID (record is skipped but ID is available for relationships)
3. **Reference Resolution**: All relationships use external IDs in manifest; resolve to internal IDs during import

---

## 7. Validation Rules Summary

| Rule                    | Type         | Error Code             | Description                                 |
| ----------------------- | ------------ | ---------------------- | ------------------------------------------- |
| Required fields present | Schema       | `MissingRequiredField` | All required fields in type definitions     |
| Valid data types        | Schema       | `InvalidDataType`      | String, number, boolean, array as specified |
| Valid enum values       | Schema       | `InvalidEnumValue`     | e.g., scale must be known value             |
| Valid date format       | Schema       | `InvalidFormat`        | ISO 8601 date/datetime                      |
| Valid URL format        | Schema       | `InvalidFormat`        | websiteUrl fields                           |
| Valid country code      | Schema       | `InvalidFormat`        | ISO 3166-1 alpha-2                          |
| Valid currency          | Schema       | `InvalidFormat`        | ISO 4217                                    |
| Reference exists        | Relationship | `OrphanedReference`    | manufacturerId → manufacturers[].id         |
| No duplicate IDs        | Relationship | `DuplicateId`          | Each ID unique within its collection        |
| Images exist            | Asset        | (Warning)              | Referenced images in /images/ folder        |
