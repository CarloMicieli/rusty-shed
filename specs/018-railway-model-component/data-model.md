# Data Model: Reusable Railway Model Component

**Feature**: 018-railway-model-component
**Date**: 2026-02-11

## Overview

This document defines the entities, value objects, and data structures used by the Railway Model component. The component primarily displays existing data from the catalog domain with the addition of image upload functionality in the media domain.

---

## Entities

### Railway Model (Product)

Represents a boxed railway model product containing one or more rolling stock units.

**Source**: Existing `catalog` domain aggregate

**Fields**:

| Field           | Type                | Required | Validation     | Description                                            |
| --------------- | ------------------- | -------- | -------------- | ------------------------------------------------------ |
| `id`            | `i64`               | Yes      | -              | Unique identifier                                      |
| `manufacturer`  | `String`            | Yes      | Non-empty      | Manufacturer name (e.g., "Rivarossi")                  |
| `product_code`  | `String`            | Yes      | Non-empty      | Product code (e.g., "HR2906")                          |
| `scale`         | `String`            | Yes      | Non-empty      | Model scale (e.g., "H0", "N", "TT")                    |
| `era`           | `Option<String>`    | No       | -              | Railway era (e.g., "IIIb", "IV", "V")                  |
| `power_method`  | `Option<String>`    | No       | -              | Power type (e.g., "DC", "AC", "Steam")                 |
| `category`      | `Option<String>`    | No       | -              | Product category (e.g., "Locomotive", "Passenger Set") |
| `description`   | `Option<String>`    | No       | Max 5000 chars | Detailed product description                           |
| `image_path`    | `Option<String>`    | No       | Valid path     | Relative path to product image                         |
| `status`        | `String`            | Yes      | Enum           | "InCollection" or "Wishlist"                           |
| `rolling_stock` | `Vec<RollingStock>` | No       | -              | List of rolling stock units (1-20)                     |

**Invariants**:

- `manufacturer`, `product_code`, and `scale` are always present (required for display)
- At least one rolling stock unit should exist for a complete model
- `image_path` is relative to the media storage directory

---

### Rolling Stock Unit

Represents an individual piece of rolling stock (locomotive, passenger car, freight car, etc.).

**Source**: Existing `catalog` domain entity

**Fields**:

| Field              | Type             | Required | Validation  | Description                                          |
| ------------------ | ---------------- | -------- | ----------- | ---------------------------------------------------- |
| `id`               | `i64`            | Yes      | -           | Unique identifier                                    |
| `railway_model_id` | `i64`            | Yes      | Foreign key | Reference to parent railway model                    |
| `series_code`      | `String`         | Yes      | Non-empty   | Series code (e.g., "E.656")                          |
| `series_name`      | `Option<String>` | No       | -           | Series name (e.g., "I Serie")                        |
| `category`         | `Option<String>` | No       | -           | Rolling stock category (e.g., "Electric Locomotive") |
| `subcategory`      | `Option<String>` | No       | -           | Subcategory (e.g., "Bo-Bo-Bo")                       |
| `road_number`      | `Option<String>` | No       | -           | Road/running number                                  |
| `depot`            | `Option<String>` | No       | -           | Depot assignment                                     |
| `livery`           | `Option<String>` | No       | -           | Livery/paint scheme                                  |
| `control_type`     | `Option<String>` | No       | -           | Control type (e.g., "Digital", "Analog")             |
| `dcc_interface`    | `Option<String>` | No       | -           | DCC interface (e.g., "21-pin MTC", "PluX22")         |
| `coupling_type`    | `Option<String>` | No       | -           | Coupling type (e.g., "NEM 362", "Kadee")             |

**Invariants**:

- `series_code` is always present (minimum identification)
- Optional fields are hidden in UI when null (no empty placeholders)

---

## Value Objects

### Model Image

Represents a validated image file for a railway model.

**Source**: New value object in `media` domain

**Fields**:

| Field        | Type      | Description                                              |
| ------------ | --------- | -------------------------------------------------------- |
| `path`       | `PathBuf` | Absolute path to stored image file                       |
| `mime_type`  | `String`  | MIME type (image/jpeg, image/png, image/webp, image/gif) |
| `size_bytes` | `u64`     | File size in bytes                                       |
| `width`      | `u32`     | Image width in pixels                                    |
| `height`     | `u32`     | Image height in pixels                                   |

**Validation Rules** (enforced in `ModelImage::validate()`):

- **MIME Type**: Must be one of `image/jpeg`, `image/png`, `image/webp`, `image/gif` (detected via magic numbers, not extension)
- **File Size**: 0 < size ≤ 10 MB (10,485,760 bytes)
- **Dimensions**: 100 ≤ width/height ≤ 4096 pixels
- **File Exists**: Path must point to readable file

**Construction**:

```rust
impl ModelImage {
    pub fn validate(file_path: &Path) -> Result<Self, ImageValidationError>;
}
```

---

### Status Badge

Represents the collection status of a railway model.

**Source**: Existing enum in `catalog` domain

**Variants**:

- `InCollection` - Model is physically owned
- `Wishlist` - Model is desired but not yet acquired

**Display**:

- `InCollection`: Green badge with checkmark icon
- `Wishlist`: Blue badge with heart/bookmark icon

---

## Component Props Interface

### RailwayModelCard Props

The component receives the following props:

```typescript
interface RailwayModelCardProps {
  /** The railway model to display */
  model: RailwayModel;

  /** Whether to enable editing features (image upload) */
  editable?: boolean;

  /** Optional class for styling customization */
  class?: string;

  /** Callback when image is uploaded (if editable=true) */
  onImageUploaded?: (imagePath: string) => void;

  /** Callback when errors occur */
  onError?: (error: string) => void;
}
```

**TypeScript Types** (generated from Rust via specta):

```typescript
// Generated by specta from Rust types
export interface RailwayModel {
  id: number;
  manufacturer: string;
  product_code: string;
  scale: string;
  era: string | null;
  power_method: string | null;
  category: string | null;
  description: string | null;
  image_path: string | null;
  status: 'InCollection' | 'Wishlist';
  rolling_stock: RollingStock[];
}

export interface RollingStock {
  id: number;
  railway_model_id: number;
  series_code: string;
  series_name: string | null;
  category: string | null;
  subcategory: string | null;
  road_number: string | null;
  depot: string | null;
  livery: string | null;
  control_type: string | null;
  dcc_interface: string | null;
  coupling_type: string | null;
}
```

---

## State Management

### Component State (Local)

The component maintains the following local state using Svelte 5 runes:

```typescript
// Active tab selection (for multi-unit models)
let activeTab = $state<'details' | 'rolling-stock'>('details');

// Expanded rolling stock rows (for multi-unit models)
let expandedRows = $state<Set<number>>(new Set());

// Drag-and-drop visual feedback
let isDragging = $state(false);

// Image upload progress
let isUploading = $state(false);
let uploadProgress = $state(0);

// Derived/computed state
let isSingleUnit = $derived(model.rolling_stock?.length === 1);
let showTabs = $derived(!isSingleUnit);
```

**Rationale**: State is entirely local to the component since it only manages UI presentation concerns (tab selection, expand/collapse state, upload feedback). No global state management is needed.

---

## Data Flow

### Read Path (Display)

1. **Parent Page** → loads `RailwayModel` from backend (existing query use cases)
2. **Parent Page** → passes `model` prop to `<RailwayModelCard>`
3. **Component** → renders model data from props
4. **Component** → derives UI state (tabs, single-unit mode) from props

### Write Path (Image Upload)

1. **User** → selects or drops image file
2. **Component** → validates file type client-side (immediate feedback)
3. **Component** → calls Tauri command `upload_model_image`
4. **Backend** → validates file (type, size, dimensions)
5. **Backend** → saves file to media storage
6. **Backend** → returns relative image path
7. **Component** → calls `onImageUploaded(imagePath)` callback
8. **Parent Page** → updates model data (triggers re-render)

---

## Database Schema (Existing)

No new migrations are required. The component uses existing tables:

```sql
-- Existing railway_model table
CREATE TABLE railway_model (
    id INTEGER PRIMARY KEY,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    scale TEXT NOT NULL,
    era TEXT,
    power_method TEXT,
    category TEXT,
    description TEXT,
    image_path TEXT,
    status TEXT NOT NULL,
    -- ... other fields
);

-- Existing rolling_stock table
CREATE TABLE rolling_stock (
    id INTEGER PRIMARY KEY,
    railway_model_id INTEGER NOT NULL,
    series_code TEXT NOT NULL,
    series_name TEXT,
    category TEXT,
    subcategory TEXT,
    road_number TEXT,
    depot TEXT,
    livery TEXT,
    control_type TEXT,
    dcc_interface TEXT,
    coupling_type TEXT,
    FOREIGN KEY (railway_model_id) REFERENCES railway_model(id)
);
```

**Image Storage**: Images are stored in the filesystem (not database) with paths relative to the application's media directory. The `image_path` field stores the relative path.

---

## Validation Summary

### Frontend Validation (Immediate UX Feedback)

- **File Type**: Check `file.type` starts with "image/"
- **File Size**: Check < 10 MB before upload

**Purpose**: Provide instant feedback without backend round-trip.

### Backend Validation (Authoritative)

- **MIME Type**: Detect via magic numbers (not extension)
- **File Size**: Enforce 0 < size ≤ 10 MB
- **Dimensions**: Enforce 100 ≤ width/height ≤ 4096
- **File Integrity**: Verify file can be decoded as image

**Purpose**: Enforce domain invariants and prevent invalid data persistence.

---

## Error Handling

### Frontend Errors

| Error Type                 | User Message (i18n key)      | Action                   |
| -------------------------- | ---------------------------- | ------------------------ |
| Invalid file type (client) | `error_invalid_image_format` | Show toast, don't upload |
| File too large (client)    | `error_image_too_large`      | Show toast, don't upload |
| Upload failed (network)    | `error_upload_failed`        | Show toast, retry option |

### Backend Errors

| Error Type              | Error Variant                              | HTTP Equivalent           |
| ----------------------- | ------------------------------------------ | ------------------------- |
| Unsupported MIME type   | `ImageValidationError::UnsupportedFormat`  | 400 Bad Request           |
| File too large          | `ImageValidationError::FileTooLarge`       | 413 Payload Too Large     |
| Invalid dimensions      | `ImageValidationError::DimensionsTooLarge` | 400 Bad Request           |
| File not found/readable | `ImageValidationError::FileNotFound`       | 404 Not Found             |
| I/O error               | `ImageValidationError::IoError`            | 500 Internal Server Error |

---

## Relationships

```
RailwayModel (1) ──< (Many) RollingStock
       │
       │ has
       │
       └──> (0..1) ModelImage (value object)
```

- A `RailwayModel` contains 1 to many `RollingStock` units
- A `RailwayModel` optionally has one `ModelImage`
- `RollingStock` entities belong to exactly one `RailwayModel`

---

## Summary

- **No new database entities**: Component uses existing `railway_model` and `rolling_stock` tables
- **One new value object**: `ModelImage` for validated image uploads
- **Clear separation**: Display logic in component, validation logic in backend
- **Type safety**: TypeScript types generated from Rust via specta
- **Domain-driven**: Validation rules enforced in backend per constitutional principles
