# Implementation Plan: Rolling Stock Progressive Editing

**Branch**: `024-rolling-stock-edit` | **Date**: 2026-02-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/024-rolling-stock-edit/spec.md`

## Summary

Transform the railway model and rolling stock detail views from read-only displays into fully interactive management interfaces. Three interaction tiers are layered progressively: (1) **in-place text editing** (click-to-edit / blur-to-save / Escape-to-cancel) for free-text fields on both the Railway Model detail page (description, details) and Rolling Stock cards (series code, road number, livery, depot); (2) **constrained selection** via badge-click popover for classification fields (Scale, Era on the Railway Model; Railway Company on rolling stock cards); (3) a **structured side drawer** for full technical specification of individual rolling stock units across four form sections (Identification, Technical, Control, Coupling).

All mutations are routed through new Tauri IPC commands following ADR 8 conventions, backed by new domain methods on the `RailwayModel` aggregate and a new `RollingStockUpdated` domain event variant. No database migration is required — the existing `rolling_stocks` schema already contains all needed columns. The `RailwayModelRepository.save()` implementation is extended to handle the new event and generate the correct SQL UPDATE.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust 1.93.0 / edition 2024 (backend)
**Primary Dependencies**: SvelteKit (Svelte 5.48.2), Tauri 2.9.x, shadcn-svelte, Paraglide 2.7.1, tauri-specta, sqlx, validator
**Storage**: SQLite — existing `railway_models` and `rolling_stocks` tables (migration 0001). No new migration required; all columns for spec drawer fields (`technical_flywheel_fitted`, `technical_body_shell`, `technical_chassis`, `technical_interior_lights`, `technical_coupling_socket`, `technical_coupling_close_couplers`, `technical_coupling_digital_shunting`, `dcc_interface`, `control`) already present.
**Testing**: Vitest 4.0.18 + happy-dom (frontend unit/component tests); cargo test (Rust backend)
**Target Platform**: Desktop (Linux/Windows/macOS) via Tauri 2.9.x
**Project Type**: Tauri 2 desktop hybrid — SvelteKit frontend + Rust backend in `src-tauri/`
**Performance Goals**: In-place save reflected on screen in <3 seconds after blur (SC-001); read queries <200ms (Constitution SLO); badge selection completed in <5 seconds (SC-002); drawer save in <4 minutes (SC-003)
**Constraints**: Single-user desktop app — no offline queue, no conflict resolution, no image attachments in this feature scope

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Gates (Phase 0)

| Principle                                    | Status         | Assessment                                                                                                                                                                                  |
| -------------------------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS        | New shared primitives (`InPlaceEdit`, `BadgePicker`) plus isolated `rolling-stock-edit` feature module; each component independently testable                                               |
| **Deterministic Interfaces & Observability** | ✅ PASS        | All mutations via Tauri `invoke`; Args types derived via tauri-specta; structured error types serialized as `CommandError`                                                                  |
| **Test-First Emphasis**                      | ⚠️ CONDITIONAL | Must include: domain method unit tests (Rust), repository save-event integration tests (Rust), Vitest component tests for `InPlaceEdit` and `BadgePicker`, and drawer form validation tests |
| **Code Quality**                             | ✅ PASS        | `pnpm lint`, `pnpm check`, `cargo clippy`, `cargo fmt` enforced per CLAUDE.md                                                                                                               |
| **Testing Standards**                        | ⚠️ CONDITIONAL | Coverage targets: domain layer 80%+, new Tauri commands 80%+, frontend components 60%+; tests must be deterministic with no network dependencies                                            |
| **UX Consistency**                           | ✅ PASS        | Paraglide for all strings; shadcn-svelte + Skeleton UI 4.x components; hover affordance and error states follow design tokens                                                               |
| **Performance Requirements**                 | ✅ PASS        | SC-001/SC-002/SC-003 define measurable targets; read commands must satisfy <200ms Constitution SLO                                                                                          |
| **Safe Rust Practices**                      | ✅ PASS        | `Result<T, E>` throughout; no panics in production flows; `cargo clippy -D warnings` enforced                                                                                               |
| **Database Law — REQUIRED**                  | ✅ PASS        | SQLite via sqlx; no new migration (all columns exist in 0001); foreign key enforcement already active on pool init                                                                          |
| **State Management Law — REQUIRED**          | ✅ PASS        | New `RollingStockUpdated` event added to `RailwayModelEvent`; repository drains all pending events atomically inside a transaction                                                          |
| **API Design Law — REQUIRED**                | ✅ PASS        | 5 new commands follow ADR 8: Args structs derive `Debug, Clone, Validate, Type, Deserialize`; boundary validates args before invoking use case                                              |
| **Domain Logic Location — REQUIRED**         | ✅ PASS        | All field validation (non-empty series_code, valid enum values) and mutation logic lives in Rust domain / application layers                                                                |

**GATE RESULT**: CONDITIONAL PASS — proceed to Phase 0 with the following requirements:

- Define test coverage strategy per layer before implementation begins
- Ensure `series_code` non-empty validation is enforced in the Rust domain (not just frontend)
- Confirm `railway_company_id` is always a valid FK reference (use `getRailwayCompanies` list for picker)

### Post-Design Gates (Phase 1)

| Principle                                    | Status  | Assessment                                                                                                                                                          |
| -------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS | `InPlaceEdit.svelte` and `BadgePicker.svelte` are prop-driven, stateless primitives; `RollingStockSpecsDrawer.svelte` is self-contained; all independently testable |
| **Deterministic Interfaces & Observability** | ✅ PASS | 5 Tauri commands fully documented in `contracts/`; specta generates TypeScript bindings automatically                                                               |
| **Test-First Emphasis**                      | ✅ PASS | Test requirements defined per contract; domain unit tests for all 5 new aggregate methods; Vitest for shared components                                             |
| **Code Quality**                             | ✅ PASS | Design follows all formatting and lint requirements; no `any` types; all strings via Paraglide                                                                      |
| **Testing Standards**                        | ✅ PASS | Coverage targets defined: domain 80%+, commands 80%+, UI components 60%+; no external network dependencies in tests                                                 |
| **UX Consistency**                           | ✅ PASS | Hover affordance, error state, and drawer unsaved-changes guard all use existing design tokens; Paraglide i18n keys defined in research.md                          |
| **Performance Requirements**                 | ✅ PASS | Read path (fetch railway model for drawer) stays on existing `get_railway_model_by_id` command (<200ms); write commands are lightweight SQL UPDATEs                 |
| **Safe Rust Practices**                      | ✅ PASS | All new domain methods return `Result`; no panics; event drain uses `std::mem::take` consistent with existing pattern                                               |
| **Database Law — REQUIRED**                  | ✅ PASS | No new migration; existing columns map 1:1 to drawer sections; FK constraint on `railway_company_id` enforced by existing schema                                    |
| **State Management Law — REQUIRED**          | ✅ PASS | `RollingStockUpdated { event_id, railway_model_id, rolling_stock_id, changed: serde_json::Value }` added; repository maps event to SQL UPDATE rolling_stocks        |
| **API Design Law — REQUIRED**                | ✅ PASS | All Args types in `contracts/`; all commands validate at boundary via `args.validate()` before use case invocation                                                  |
| **Domain Logic Location — REQUIRED**         | ✅ PASS | Business rules (series_code non-empty, valid DccInterface/Control enums, valid RailwayCompanyId existence) enforced in Rust; frontend only provides UX hints        |

**GATE RESULT**: ✅ ALL PASS

**Design Compliance Summary**:

- ✅ No new DB migration — all needed columns already in migration 0001
- ✅ New `RollingStockUpdated` domain event drains correctly through repository
- ✅ 5 focused Tauri commands avoid `Option<Option<T>>` serialization pitfalls
- ✅ Reusable `InPlaceEdit` and `BadgePicker` primitives serve all three user stories
- ✅ Full Paraglide i18n coverage; zero hardcoded strings
- ✅ All test layers specified (domain unit, command integration, UI component)

**Ready for Phase 2** (tasks generation via `/speckit.tasks` command)

## UI Design Specifications

> Derived from UX Expert + Designer skill review (2026-02-18). All implementers MUST follow these token and behaviour contracts — they override any defaults from shadcn-svelte or Skeleton UI where they conflict.

### Design Tokens

| Token | Value | Usage |
| --- | --- | --- |
| Surface (cards/modals) | `#0F0F0F` | `bg-[#0F0F0F]` — all drawer panels, pickers |
| Border | `#1F1F1F` | `border-[#1F1F1F]` — card, picker, input borders |
| Accent (amber) | `#D48A42` | Primary actions, focus rings, active states |
| Accent-muted | `rgba(212,138,66,0.15)` | Hover backgrounds on interactive elements |
| Text-main | `#E0E0E0` | Field values |
| Text-muted | `#808080` | Field labels, placeholders |

---

### InPlaceEdit.svelte — Visual State Contract

#### Idle state (read-only display)
- Render value as plain text; no border; no background.
- Empty field: show placeholder `"Click to add..."` as `text-[#808080] italic text-sm`.

#### Hover state (field is editable, cursor over it)
```
bg-[rgba(212,138,66,0.15)]
border border-dashed border-[#D48A42]/40
rounded
cursor-pointer
transition-colors duration-150
```

#### Active edit state (input is focused)
```
border border-[#D48A42]
ring-1 ring-[#D48A42]/30
bg-[#0F0F0F]
rounded
outline-none
```

#### Floating Save / Cancel pill
- When the input is active, render a `position: absolute` pill **below** the input (`top-full left-0 mt-1 z-50`) — never inline — to avoid card layout shift.
- Save: `<Button size="xs" class="bg-[#D48A42] text-black hover:bg-[#D48A42]/90">Save</Button>`
- Cancel: `<Button size="xs" variant="ghost">Cancel</Button>`
- Both wrapped in a `flex gap-1 bg-[#0F0F0F] border border-[#1F1F1F] rounded px-2 py-1 shadow-lg` container.

---

### BadgePicker.svelte — Visual State Contract

#### Badge / trigger element (hover)
- Apply amber-muted background + `cursor-pointer` to the badge container on hover:
  ```
  hover:bg-[rgba(212,138,66,0.15)] cursor-pointer transition-colors duration-150
  ```
- Show a `✏` micro-icon (lucide `Pencil`, size 10px, `text-[#808080]`) to the right of the badge text on hover — hidden at rest (`opacity-0 group-hover:opacity-100`).

#### Picker panel
```
bg-[#0F0F0F]
border border-[#1F1F1F]
rounded-[8px]
shadow-xl
min-w-[160px]
```

#### Option rows
- Default: `px-3 py-2 text-sm text-[#E0E0E0] hover:bg-[rgba(212,138,66,0.15)] cursor-pointer`
- Selected / current value: `bg-[rgba(212,138,66,0.15)] text-[#D48A42] font-medium`

#### Keyboard navigation (REQUIRED)
The `BadgePicker` component MUST handle:
- `ArrowDown` / `ArrowUp` — move focus through options
- `Enter` — confirm selection (same as click)
- `Escape` — dismiss without selecting (existing requirement)

---

### RollingStockSpecsDrawer.svelte — Visual State Contract

#### Backdrop overlay
```
bg-black/80 backdrop-blur-sm
```
> Previous `ItemDrawer.svelte` uses `bg-black/40` — this drawer MUST use `bg-black/80` to satisfy the Anti-Bleed rule.

#### Drawer panel surface
```
bg-[#0F0F0F]
border-l border-[#1F1F1F]
```

#### Section field labels (all four sections: Identification, Technical, Control, Coupling)
```
text-[10px] uppercase tracking-wider text-[#808080]
```

#### Section field values
```
text-[12px] text-[#E0E0E0]
```

#### Constrained selectors inside drawer (Control Type, DCC Interface, Coupling Socket)
- **Do NOT use a native `<select>`** — OS-native styling breaks the charcoal aesthetic.
- Use the shadcn-svelte `Select` component with these overrides:
  ```
  bg-[#0F0F0F] border-[#1F1F1F] text-[#E0E0E0]
  focus:ring-[#D48A42] focus:border-[#D48A42]
  ```
- Alternatively, use `BadgePicker` if the field is displayed in a badge context.

#### Unsaved-changes confirmation Dialog
- This dialog sits **above** the drawer — its backdrop MUST use `bg-black/90` (higher than the drawer's `bg-black/80`) to maintain clear z-order.

---

### "Edit Specs" Button — Component Contract

FR-021 requires an "Edit Specs" action on each rolling stock card. This MUST be rendered as:

```svelte
<Button variant="outline" size="sm" class="gap-1.5 border-[#1F1F1F] text-[#E0E0E0] hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]">
  <PencilLine size={14} />
  {m.rolling_stock_edit_specs_button()}
</Button>
```

- Never a plain text link, bare `<button>`, or icon-only affordance.
- The `PencilLine` icon is from `lucide-svelte`.

---

## Project Structure

### Documentation (this feature)

```text
specs/024-rolling-stock-edit/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── update_railway_model_text.md
│   ├── update_railway_model_classification.md
│   ├── update_rolling_stock_identification.md
│   ├── update_rolling_stock_railway_company.md
│   └── update_rolling_stock_specifications.md
└── tasks.md             # Phase 2 output (/speckit.tasks command — NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Backend (Rust/Tauri)
src-tauri/src/catalog/
├── domain/railway_model/
│   ├── railway_model.rs              # ADD: update_scale, update_epoch, update_rolling_stock_* methods
│   ├── railway_model_event.rs        # ADD: RollingStockUpdated { event_id, railway_model_id, rolling_stock_id, changed } variant
│   └── rolling_stock.rs             # ADD: apply_identification_patch, apply_railway_company, apply_specifications methods
├── application/
│   ├── update_railway_model_text.rs          # NEW: UpdateRailwayModelText use case
│   ├── update_railway_model_classification.rs # NEW: UpdateRailwayModelClassification use case
│   ├── update_rolling_stock_identification.rs # NEW: UpdateRollingStockIdentification use case
│   ├── update_rolling_stock_railway_company.rs # NEW: UpdateRollingStockRailwayCompany use case
│   └── update_rolling_stock_specifications.rs  # NEW: UpdateRollingStockSpecifications use case
├── infrastructure/railway_model/
│   └── sqlite_railway_model_repository.rs    # UPDATE: handle RollingStockUpdated in save(); add update_rolling_stock SQL
└── interface/
    └── command_handlers.rs                   # ADD: 5 new #[tauri::command] handlers + Args types

# Frontend (SvelteKit)
src/lib/
├── components/
│   ├── InPlaceEdit.svelte             # NEW: reusable click-to-edit / blur-to-save primitive
│   └── BadgePicker.svelte             # NEW: constrained selection popover anchored near trigger
├── features/rolling-stock-edit/
│   ├── RollingStockEditState.svelte.ts     # NEW: per-card edit state (active field, pending value, error)
│   └── components/
│       └── RollingStockSpecsDrawer.svelte  # NEW: 4-section technical spec side drawer
└── components/model-details/
    └── RollingStockCard.svelte              # UPDATE: in-place fields + Edit Specs button + BadgePicker for railway company

# Railway model detail page (where Scale/Era badge pickers live):
src/lib/components/
└── RailwayModelCard.svelte                  # UPDATE (or detail page component): InPlaceEdit for description/details + BadgePicker for scale/era

messages/                                    # ADD: new Paraglide message keys (see research.md)
src/lib/bindings.ts                          # AUTO-UPDATED: by cargo build (tauri-specta generates new command types)
```

**Structure Decision**: Single Tauri 2 desktop project. Backend follows existing clean-architecture layout within the `catalog` bounded context. Frontend places new reusable primitives in `src/lib/components/` and feature-specific state + drawer in a new `rolling-stock-edit` feature module. No new backend bounded context is required — all mutations extend the existing `RailwayModel` aggregate.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations detected** — all constitutional principles and architectural laws are satisfied by this design.
