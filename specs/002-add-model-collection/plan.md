# Implementation Plan: Add Railway Model to Collection

**Branch**: `002-add-model-collection` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-add-model-collection/spec.md`

## Summary

Implement an enhanced "Add Railway Model to Collection" side-panel drawer that replaces the current simplified popup. The new form captures detailed railway model information (manufacturer, product code, description, category, scale, power method, epoch), dynamic rolling stock management (railway company, series code, category, road number), and optional purchase information (seller, price, conditions, notes). The backend already provides the `add_railway_model_to_collection` command with `AddRailwayModelToCollectionArgs` which accepts `SimplifiedRailwayModelArgs` - **no Rust changes required**.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust 1.93.0 (backend - no changes needed)  
**Primary Dependencies**: Svelte 5.48.2, SvelteKit, Tailwind CSS 4.x, Skeleton UI 4.x, Tauri IPC  
**Storage**: SQLite via Rust backend (existing commands handle persistence)  
**Testing**: Vitest with happy-dom for frontend unit tests  
**Target Platform**: Desktop (Tauri - Windows, macOS, Linux)  
**Project Type**: Web frontend + Rust backend (Tauri app)  
**Performance Goals**: Form interactions < 100ms, dropdown population < 200ms  
**Constraints**: All user strings via Paraglide-JS, type-safe bindings from specta  
**Scale/Scope**: Single feature affecting 2-3 Svelte components, ~8 new form fields, 1 dynamic list

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                                    | Status  | Evidence                                                                                  |
| -------------------------------------------- | ------- | ----------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS | Feature is self-contained in `src/lib/features/collection/`, form components reusable     |
| **Deterministic Interfaces & Observability** | ✅ PASS | Uses existing `add_railway_model_to_collection` Tauri command with specta-generated types |
| **Test-First Emphasis**                      | ✅ PASS | Will add Vitest tests for form validation and controller logic                            |
| **Code Quality**                             | ✅ PASS | TypeScript strict mode, ESLint/Prettier enforced, no `any` types                          |
| **Testing Standards**                        | ✅ PASS | Unit tests for form state, integration tests for command invocation                       |
| **User Experience Consistency**              | ✅ PASS | All strings via Paraglide, Skeleton UI components, consistent drawer pattern              |
| **Performance Requirements**                 | ✅ PASS | Reference data loaded via existing commands, no new backend queries                       |
| **Database (Persistence)**                   | ✅ N/A  | No schema changes - uses existing tables/migrations                                       |
| **State Management / Persistence Strategy**  | ✅ N/A  | Backend handles domain events - frontend is presentation only                             |
| **API Design & Transport Boundary**          | ✅ PASS | Uses existing `AddRailwayModelToCollectionArgs` with generated TypeScript types           |
| **Domain Logic Location**                    | ✅ PASS | All validation/business rules in Rust; frontend does UX validation hints only             |

**All gates pass. No violations requiring justification.**

## Project Structure

### Documentation (this feature)

```text
specs/002-add-model-collection/
├── plan.md              # This file
├── research.md          # Phase 0 output - reference data analysis
├── data-model.md        # Phase 1 output - form state model
├── quickstart.md        # Phase 1 output - implementation guide
├── contracts/           # Phase 1 output - component contracts
│   └── AddModelDrawer.contract.md
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── lib/
│   ├── bindings.ts                          # Generated - AddRailwayModelToCollectionArgs, SimplifiedRailwayModelArgs
│   ├── data/constants/                      # Static reference data (scales, epochs, categories, powerMethods)
│   │   ├── categories.json
│   │   ├── epochs.json
│   │   ├── powerMethods.json
│   │   └── scales.json
│   ├── features/
│   │   └── collection/
│   │       ├── CollectionDashboard.svelte   # MODIFY - add drawer trigger, integrate new drawer
│   │       ├── CollectionState.svelte.ts    # MODIFY - add addRailwayModel method
│   │       └── components/
│   │           ├── ItemDrawer.svelte        # REPLACE - new comprehensive form drawer
│   │           ├── RollingStockEntry.svelte # NEW - dynamic rolling stock form row
│   │           └── PurchaseSection.svelte   # NEW - optional purchase info section
│   └── paraglide/messages.js                # Generated from messages/*.json
├── __tests__/
│   └── lib/
│       └── features/
│           └── collection/
│               └── AddModelForm.test.ts     # NEW - form validation tests
└── routes/my-collection/+page.svelte        # No changes (uses CollectionDashboard)

messages/
├── en.json                                  # ADD new message keys for form labels
└── it.json                                  # ADD Italian translations
```

**Structure Decision**: Frontend-only changes within existing feature module structure. No new routes, no backend modifications.

## Complexity Tracking

> No violations requiring justification - all gates pass.

| Aspect              | Complexity | Justification                                                          |
| ------------------- | ---------- | ---------------------------------------------------------------------- |
| Form fields         | Medium     | 8 model fields + dynamic rolling stock list + 6 purchase fields        |
| Rolling stock list  | Medium     | Dynamic add/remove with validation per entry                           |
| Reference data      | Low        | Already available in constants/\*.json and via existing Tauri commands |
| Backend integration | Low        | Existing command matches spec requirements exactly                     |

---

## Phase 0: Research

### R1: Reference Data Sources

**Decision**: Use existing static JSON constants + Tauri commands for dynamic data

| Data                     | Source                                                               | Status              |
| ------------------------ | -------------------------------------------------------------------- | ------------------- |
| Manufacturers            | `commands.getManufacturers()` → `Manufacturer[]`                     | ✅ Available        |
| Railway Companies        | `commands.getRailwayCompanies()` → `RailwayCompany[]`                | ✅ Available        |
| Sellers                  | `commands.getSellers()` → `SellerView[]`                             | ✅ Available        |
| Scales                   | `src/lib/data/constants/scales.json`                                 | ✅ Available        |
| Epochs                   | `src/lib/data/constants/epochs.json`                                 | ✅ Available        |
| Power Methods            | `src/lib/data/constants/powerMethods.json`                           | ✅ Available        |
| Categories               | `src/lib/data/constants/categories.json`                             | ✅ Available        |
| Rolling Stock Categories | `src/lib/data/constants/rollingStockCategories.json`                 | ✅ Available        |
| Purchase Conditions      | Static values: NEW, PRE_OWNED                                        | ✅ From bindings.ts |
| Model Conditions         | MINT, NEAR_MINT, EXCELLENT, VERY_GOOD, GOOD, FAIR, POOR, FOR_PARTS   | ✅ From bindings.ts |
| Box Conditions           | ORIGINAL_MINT, ORIGINAL_GOOD, ORIGINAL_WORN, REPLACEMENT_BOX, NO_BOX | ✅ From bindings.ts |

**Rationale**: Static constants reduce IPC calls. Dynamic entities (manufacturers, railway companies, sellers) must be fetched since they're user-managed.

### R2: Existing Command Analysis

**Decision**: Use `addRailwayModelToCollection` command as-is

The `AddRailwayModelToCollectionArgs` type in bindings.ts provides:

```typescript
{
  railwayModel: SimplifiedRailwayModelArgs; // manufacturerId, productCode, description, category, scale, epoch, powerMethod, rollingStocks[]
  priceAmount: bigint;
  priceCurrency: string;
  sellerId: string | null;
  addedDate: string;
  purchaseDate: string;
  purchaseCondition: string | null;
  modelCondition: string | null;
  boxCondition: string | null;
  notes: string | null;
}
```

This matches the spec requirements. No backend changes needed.

### R3: Form State Pattern

**Decision**: Use Svelte 5 `$state` runes in a controller class

Following existing pattern from `CollectionState.svelte.ts`:

- Create form state class with `$state` for each field
- Use `$derived` for computed validation
- Expose `submit()` method that calls Tauri command

**Alternatives considered**:

- Separate form store: Rejected - adds complexity, existing pattern works
- Reactive form library: Rejected - Svelte 5 runes are sufficient

---

## Phase 1: Design

### Data Model

See [data-model.md](data-model.md) for complete form state types.

Key types:

- `AddModelFormState` - form state with manufacturer, productCode, etc.
- `RollingStockFormEntry` - individual rolling stock in the list
- `PurchaseFormState` - optional purchase information section

### Component Contracts

See [contracts/AddModelDrawer.contract.md](contracts/AddModelDrawer.contract.md)

Component hierarchy:

```
CollectionDashboard.svelte
└── AddModelDrawer.svelte (replaces ItemDrawer.svelte)
    ├── Railway Model Fields (inline)
    ├── RollingStockEntry.svelte (repeated, dynamic)
    └── PurchaseSection.svelte (collapsible)
```

### Quickstart

See [quickstart.md](quickstart.md) for step-by-step implementation order.

---

## Backend Confirmation

**Question for user**: I've analyzed the existing Rust backend and confirmed that:

1. ✅ `add_railway_model_to_collection` command exists and accepts all required fields
2. ✅ `SimplifiedRailwayModelArgs` includes rolling stocks array
3. ✅ `getManufacturers`, `getRailwayCompanies`, `getSellers` commands exist for dropdowns
4. ✅ All condition types (`PurchaseCondition`, `ModelCondition`, `BoxCondition`) are defined

**No Rust changes are required for this feature.** The implementation is purely frontend work:

- New drawer component with comprehensive form
- Form state management with validation
- Paraglide message keys for all labels
- Integration with existing Tauri commands

Do you confirm this assessment, or would you like me to investigate any specific backend aspect further?
