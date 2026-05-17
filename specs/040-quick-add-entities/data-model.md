# Data Model: On-the-Fly Entity Quick-Add (040)

**Date**: 2026-05-15  
**Feature**: 040-quick-add-entities  
**Scope**: Feature 040 includes both persisted entity creation support and frontend quick-add state. No new tables are introduced, but the feature owns the create command additions and case-insensitive uniqueness migrations required for the Quick-Add flow.

---

## Existing Persisted Entities

### Manufacturer

_Schema source_: `src-tauri/migrations/0001_create_railway_models_and_rolling_stocks.sql`  
_Rust domain_: `src-tauri/src/catalog/domain/manufacturer/manufacturer.rs`  
_Specta binding_: `src/lib/bindings.ts → Manufacturer`

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` PK | UUID as string; pattern `trn:manufacturer:{slug}` |
| `name` | `TEXT NOT NULL` | Unique via case-insensitive index introduced in 040 |
| `registered_company_name` | `TEXT` | Optional formal name |
| `status` | `TEXT NOT NULL DEFAULT 'ACTIVE'` | `ACTIVE \| MERGED \| OUT_OF_BUSINESS` |
| `country_code` | `TEXT` | ISO 3166-1 alpha-2 |
| `website_url` | `TEXT` | Optional URL |
| `created_at` / `updated_at` | `TEXT NOT NULL` | ISO 8601 |
| `version` | `INTEGER NOT NULL DEFAULT 0` | Optimistic concurrency |

**Scope note**:
- `is_system_seeded` is not required for the Quick-Add workflow and is out of scope for Feature 040.

**TypeScript type** (current bindings):
```typescript
type Manufacturer = {
  id: ManufacturerId;               // string
  name: string;
  registeredCompanyName: string | null;
  countryCode: string | null;
  status: ManufacturerStatus;       // "ACTIVE" | "MERGED" | "OUT_OF_BUSINESS"
  websiteUrl: string | null;
};
```

---

### Seller (also used as Buyer)

_Schema source_: `src-tauri/migrations/0002_create_collection_schema.sql`  
_Rust domain_: `src-tauri/src/sellers/domain/seller.rs`  
_Specta binding_: `src/lib/bindings.ts → SellerView`

| Column | Type | Notes |
|---|---|---|
| `id` | `TEXT` PK | UUID slug |
| `name` | `TEXT NOT NULL` | Unique via case-insensitive index introduced in 040 |
| `type` | `TEXT NOT NULL` | `SHOP \| PRIVATE \| MARKETPLACE \| DISTRIBUTOR` |
| `email` | `TEXT` | Optional |
| `phone` | `TEXT` | Optional |
| `website_url` | `TEXT` | Optional |
| `street_address` … `country_code` | `TEXT` | Address fields (all optional) |
| `created_at` / `updated_at` | `TEXT NOT NULL` | ISO 8601 |
| `version` | `INTEGER NOT NULL DEFAULT 0` | Optimistic concurrency |

**"Buyer" mapping**: No separate buyers table exists. Buyers are represented using the Seller persistence model. When the Quick-Add form is opened from a "Buyer" field, the UI labels the flow as "Add Buyer", but the backend command remains `create_seller` with a fixed default `seller_type` suitable for Quick-Add.

**TypeScript type** (current bindings):
```typescript
type SellerView = {
  id: SellerId;
  name: string;
  sellerType: SellerType;           // "SHOP" | "PRIVATE" | "MARKETPLACE" | "DISTRIBUTOR"
  email: string | null;
  phone: string | null;
  websiteUrl: string | null;
  address: Address | null;
};
```

---

## Frontend State Model (new in this feature)

### QuickAddContext

Ephemeral state that exists only during a Quick-Add session. Stored in the parent drawer component via Svelte `$state`, never persisted.

```typescript
type QuickAddTarget =
  | { kind: 'manufacturer' }
  | { kind: 'seller' }
  | { kind: 'buyer' };           // maps to seller creation

type QuickAddState =
  | { status: 'closed' }
  | { status: 'open'; target: QuickAddTarget }
  | { status: 'saving' }
  | { status: 'error'; message: string };
```

**Lifecycle**:
1. `closed` → `open` when user clicks `+` next to a dropdown
2. `open` → `saving` when user submits the quick-add form
3. `saving` → `closed` on success (new entity pushed to parent state; toast shown)
4. `saving` → `error` on failure (drawer stays open, error displayed)
5. Any → `closed` on dismiss/cancel

### QuickAddFormValues

Minimal form data held in the Quick-Add drawer:

```typescript
type QuickAddFormValues = {
  name: string;                   // required; trimmed before comparison
  websiteUrl: string;             // optional
  countryCode: string;            // optional; 2-char ISO code
};
```

**Validation rules** (enforced client-side via Zod schema):
- `name`: `z.string().trim().min(1)` AND not matching any existing name case-insensitively
- `websiteUrl`: `z.string().url().optional().or(z.literal(''))`
- `countryCode`: `z.string().length(2).optional().or(z.literal(''))`

### DuplicateCheckState

Reactive derived from `QuickAddFormValues.name` and the parent's loaded entity list:

```typescript
type DuplicateCheckState =
  | { isDuplicate: false }
  | { isDuplicate: true; existingName: string };  // existing canonical name shown in warning
```

---

## Entity Relationships (unchanged by this feature)

```
Manufacturer ──< AcquisitionItem.manufacturerId
Manufacturer ──< CollectionItem.manufacturerId
Manufacturer ──< WishlistItem.manufacturerId

Seller ──< Acquisition.sellerId
Seller ──< Acquisition.buyerId  (same table, buyer role)
```

All foreign keys are managed by the existing schema and are not affected by this feature.

---

## State Flow Diagram

```
Parent Drawer opens
  └─ loads manufacturers[], sellers[] via Promise.all(commands.*)
       │
       ▼
User clicks [+] next to Manufacturer dropdown
  └─ quickAddState = { status: 'open', target: { kind: 'manufacturer' } }
       │
       ▼
QuickAddShell renders (z-110), scrim renders (z-105), parent dims (opacity-70)
User types name → DuplicateCheckState computed reactively
       │
  [name unique & non-empty]          [name is duplicate / empty]
       │                                      │
  Save button enabled               Save button disabled
       │
  User clicks Save
  └─ quickAddState = { status: 'saving' }
       │
  commands.createManufacturer({ name, websiteUrl, countryCode })
       │
  ┌─────────────────┐         ┌──────────────────────┐
  │ status === 'ok'  │         │ status === 'error'    │
  │ newMfg returned  │         │ quickAddState = error │
  └─────┬───────────┘         └──────────────────────┘
        │
  manufacturers = [...manufacturers, newMfg]
  manufacturerId = newMfg.id   (auto-select)
  quickAddState = { status: 'closed' }
  toaster.success(m.quick_add_manufacturer_success({ name: newMfg.name }))
```
