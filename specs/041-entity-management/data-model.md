# Data Model: Centralized Entity Management (041)

**Date**: 2026-05-17

## Core Entities

### Manufacturer

| Field | Type | Rules |
|---|---|---|
| id | UUID/string | Immutable primary key |
| name | string | Required, trimmed, case-insensitive unique |
| website | string \| null | Optional URL |
| country | string \| null | Optional ISO-2 code |
| notes | string \| null | Optional free text |
| is_system_seeded | boolean | `true` for protected seed data |
| usage_count | integer | Derived count from linked records |
| created_at | datetime | Server-set |
| updated_at | datetime | Server-set |

### Party (Shared Buyer/Seller Persistence)

Canonical underlying record used by both Buyer and Seller aggregates.

| Field | Type | Rules |
|---|---|---|
| id | UUID/string | Immutable primary key |
| name | string | Required, trimmed, case-insensitive unique in shared table |
| website | string \| null | Optional URL |
| country | string \| null | Optional ISO-2 code |
| notes | string \| null | Optional free text |
| is_system_seeded | boolean | `true` prevents delete and name edit |
| total_usage_count | integer | Derived total refs across buyer + seller contexts |
| buyer_usage_count | integer | Optional derived display value |
| seller_usage_count | integer | Optional derived display value |
| created_at | datetime | Server-set |
| updated_at | datetime | Server-set |

### Buyer Aggregate View

Buyer aggregate projects canonical Party data in buyer context.

| Field | Type | Rules |
|---|---|---|
| party_id | UUID/string | References canonical party |
| role_label | enum | `BUYER` |
| name/website/country/notes | mirror | Sourced from canonical party |
| is_system_seeded | boolean | Sourced from canonical party |
| usage_count | integer | Must equal `total_usage_count` for lock/delete logic |

### Seller Aggregate View

Seller aggregate projects canonical Party data in seller context.

| Field | Type | Rules |
|---|---|---|
| party_id | UUID/string | References canonical party |
| role_label | enum | `SELLER` |
| name/website/country/notes | mirror | Sourced from canonical party |
| is_system_seeded | boolean | Sourced from canonical party |
| usage_count | integer | Must equal `total_usage_count` for lock/delete logic |

## Relationships

- Manufacturer links to collection/acquisition/wishlist items via foreign keys.
- Party links to acquisition and sale records in buyer and seller roles.
- Buyer and Seller aggregate surfaces point to the same canonical party row.

## Validation Rules

- Name must be non-empty after trim.
- Name uniqueness is case-insensitive.
- Country, when present, must be 2-character uppercase ISO code.
- Website, when present, must be valid URL.
- For system-seeded rows (`is_system_seeded=true`):
  - Delete forbidden.
  - Name edit forbidden.
  - Metadata fields may be editable if explicitly allowed by command.
- Delete allowed only when:
  - `is_system_seeded=false`
  - `usage_count=0` (for parties: total across buyer + seller contexts)

## State Transitions

### Create

- Input validated.
- Duplicate name checked (case-insensitive).
- New row inserted with `is_system_seeded=false`.
- Canonical party appears in both Buyers and Sellers tabs immediately.

### Update

- Edit from Buyer or Seller tab resolves to same canonical party row.
- On success, both tabs reflect updated values.
- Name updates reject duplicates and reject system-seeded rows.

### Delete

- UI only shows delete when row appears deletable.
- Backend revalidates protection and usage at execution time.
- Reject with explicit reason if conditions fail.

### Merge

- Exactly two rows of same entity type selected.
- Canonical target chosen.
- All references relinked to target in one transaction.
- Source row deleted only after successful relink.
- For shared Party merges, relink buyer and seller references together atomically.

## Derived View Fields for Library UI

- `origin_badge`: `SYSTEM` or `USER` from `is_system_seeded`.
- `status_badge`:
  - `Protected/System` if system-seeded.
  - `In Use (N)` if usage count > 0.
  - `Unused` if usage count = 0 and user-created.
- `actions`:
  - Edit Name hidden/disabled for system-seeded.
  - Delete shown only when user-created and usage_count=0.
