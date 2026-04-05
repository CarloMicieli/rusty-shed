# Bugs

## Master Table

| id  | title                                                           | status  | priority |
| --- | --------------------------------------------------------------- | ------- | -------- |
| 1   | remove the road number from the acquisition railway model cards | Pending | Medium   |

---

id: 1
title: "remove the road number from the acquisition railway model cards"
status: Pending
priority: Medium

---

### 🔍 Problem Analysis

- **Observed:** the road number is displayed on railway model cards in the Acquisition drawer/component.
- **Expected:** no road number is displayed (only epoch and scale should be visible).
- **Target Files:**
  - `src/lib/features/acquisition/AcquisitionDrawer.svelte`
  - `src/lib/components/drawer/ModelInfoSection.svelte`
  - `src/lib/features/depot/DepotState.svelte`
  - `src/lib/features/depot/services/DepotService.svelte.ts`
  - `src/lib/schemas/railway-model.ts`
  - `src/lib/schemas/rolling-stock-form.ts`
  - `src-tauri/src/collecting/infrastructure/mappers.rs` (for backend awareness)

### 🛠 Technical Requirements

- **Validation:** `pnpm format && pnpm lint --fix && pnpm check && pnpm test`
- **Tauri 2 Check:** Verify capabilities in `src-tauri/capabilities/`.

### Notes

- Removing the `roadNumber` from the acquisition card is a frontend change; schema/serialisation and backend mappings should be reviewed for compatibility. Unit tests referencing the road number will need updates.
