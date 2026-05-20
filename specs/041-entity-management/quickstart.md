# Quickstart: Centralized Entity Management (041)

**Date**: 2026-05-17

## Goal

Deliver Settings > Library CRUD for Manufacturers, Sellers, and Buyers with:
- canonical shared Buyer/Seller party records,
- strong system-seeded and usage-based protection,
- atomic merge semantics,
- shared entity form modes (`QUICK` and `FULL`).

## Preconditions

1. Work on branch `041-entity-management`.
2. Ensure quick-add baseline from feature 040 is present.
3. Confirm no pending migration conflicts.

## Step-by-step Implementation

### 1) Backend data and migration groundwork

1. Add/extend migrations to support:
- `is_system_seeded` for relevant entities,
- case-insensitive unique indexes (`LOWER(name)`),
- query support for usage counts.
2. Verify foreign keys and usage queries cover:
- manufacturer references,
- buyer + seller references for shared party records.

### 2) Backend command surfaces and application logic

1. Manufacturer commands:
- list/search,
- create,
- update,
- delete with protection revalidation,
- merge.
2. Seller commands (distinct surface over shared table):
- list/search,
- create,
- update,
- delete with total-usage revalidation,
- merge on canonical party.
3. Buyer commands (distinct surface over shared table):
- list/search,
- create,
- update,
- delete with total-usage revalidation,
- merge on canonical party.
4. Ensure buyer/seller create or update in either tab reflects same canonical party record.

### 3) Frontend Settings Library UI

1. Add Library section in Settings route.
2. Implement tabs: Manufacturers, Sellers, Buyers.
3. Implement shared table/card renderer with responsive behavior.
4. Add real-time search by name/country.
5. Add row badges:
- Protected/System,
- In Use (N),
- Unused.
6. Add action availability rules:
- hide/disable name edit + delete for system-seeded,
- delete only for user-created + usage=0.

### 4) Shared form modes

1. Extend existing shared entity form component contract:
- `mode=QUICK` for feature 040 flows,
- `mode=FULL` for Settings Library CRUD.
2. Preserve existing quick-add behavior without regressions.

### 5) i18n and bindings

1. Add all new user-facing strings to:
- `messages/en.json`
- `messages/it.json`
2. Regenerate messages:
```bash
pnpm prepare
```
3. Regenerate specta bindings after Rust command/type changes:
```bash
pnpm specta:generate
```

## Verification

Run all quality gates:

```bash
pnpm format
pnpm lint
pnpm check
pnpm test
pnpm rust:fmt
pnpm rust:clippy
pnpm rust:test
```

## Targeted functional checks

1. Create from Buyers tab appears immediately in Sellers tab (same record).
2. Edit from Sellers tab updates Buyers tab view instantly.
3. Delete blocked when total usage (buyer + seller) > 0.
4. Delete blocked for system-seeded entities.
5. Merge relinks references across buyer and seller contexts atomically.
6. Duplicate warning appears quickly and prevents save.

## Notes

- Runtime API transport remains Tauri IPC; OpenAPI contract is planning/test documentation.
- Buyer and Seller remain distinct command surfaces while sharing repository and storage logic.
