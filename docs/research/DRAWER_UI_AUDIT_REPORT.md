# Drawer UI Audit Report

## Scope

Systemic audit of:

- **Add Railway Model** drawer (`RollingStockCreateDrawer.svelte`)
- **Add Item to Wishlist** drawer (`AddWishlistItemDrawer.svelte`)

Objective: identify refactoring opportunities aligned with the **Mechanical Precision** aesthetic and Svelte 5 architecture.

---

## Component Tree

```text
DrawerShell (shared shell)
├── DrawerHeader [icon, title, subtitle, onClose]
├── Scrollable Body
│   ├── RollingStockCreateDrawer
│   │   └── form (superForm + use:enhance)
│   │       ├── Category/Type row (FormSelect + conditional FormSelect)
│   │       ├── RollingStockPrototypeSection
│   │       │   └── Company, SeriesCode, Series, FriendlyName, RoadNumber, Livery, Depot
│   │       └── RollingStockTechnicalFields
│   │           └── DCC, Control, Coupling, Feature Flags, Length, etc.
│   └── AddWishlistItemDrawer
│       └── form (superForm + validateForm)
│           ├── WishlistPickerSection
│           ├── ModelInfoSection
│           │   ├── Manufacturer, ProductCode
│           │   ├── Description
│           │   ├── Category
│           │   ├── Scale + Power Method
│           │   └── EpochPicker
│           └── WishlistPreferencesSection
│               ├── Priority toggle
│               └── Desired price
└── Footer
    ├── RollingStockCreateDrawer: DrawerFooter
    └── AddWishlistItemDrawer: inline footer (not DrawerFooter)
```

---

## Atomic Units (Mechanical Precision)

### Positive atomic units already present

- `DrawerShell` with solid panel, clear border, and high-opacity backdrop
- `DrawerHeader` icon box + title/subtitle architecture
- `FormInput`, `FormSelect`, `EpochPicker` as reusable form atoms
- `DrawerFooter` steampunk primary action style

### Structural duplication candidates

- Repeated section-card wrapper:
  - `overflow-hidden rounded-sm border border-border bg-card p-4`
- Repeated section-label typography:
  - `text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase`
- Repeated form-row patterns:
  - Manufacturer + Product Code row
  - Scale + Power Method row

---

## Logic Duplication Audit (Model Details)

The **Model Details** block is functionally mirrored across collection/wishlist flows and conceptually shared with rolling stock onboarding:

- Manufacturer
- Product Code
- Description
- Scale
- Power
- Epoch

Current implementation for wishlist uses `ModelInfoSection.svelte`, but similar field architecture appears elsewhere with slight UI token and composition drift.

Refactor opportunity: promote these shared fields into a reusable **ModelDetails** section contract and compose per-context extensions (e.g., rolling stock-specific fields).

---

## Consistency Audit

### Header Architecture

- Both drawers use `DrawerHeader` with icon, title, subtitle, close action ✅
- Visual architecture mostly consistent ✅

### Footer Actions

- Railway drawer uses shared `DrawerFooter` ✅
- Wishlist drawer inlines custom footer markup ❌
  - Spacing mismatch (`gap-2 p-4` vs `gap-3 px-6 py-4`)
  - Missing standardized footer border wrapper
  - Divergent loading affordance

### Form Hints and Required Asterisks

- `FormInput` and `FormSelect` apply required marker consistently ✅
- Manufacturer field in `ModelInfoSection` uses inline `Select.Root` and custom label flow (not `FormSelect`) ⚠️
- Label sizing/token consistency drifts in some primitives (`FormBooleanSelect`, `FormPrice`) ❌

---

## Desktop Affordance Check

### Good desktop affordances

- Clear field boundaries on `FormInput`/`FormSelect`
- Explicit hover/focus states on core inputs/buttons
- Overlay + panel separation communicates modal context

### Areas to improve

- Standardize section-bar contrast (`DrawerSectionBar`) to avoid low-contrast legacy tokens
- Remove legacy rounded-md/rounded-lg where precision aesthetic expects `rounded-sm`
- Normalize footer layout and elevation cues via universal `DrawerFooter`

---

## Token Consistency Report (Hardcoded Drift)

| File                                | Drift                                                                         | Should be                                                              |
| ----------------------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `DrawerSectionBar.svelte`           | `border-white/10`, `text-zinc-500`, `hover:bg-white/5`                        | `border-border`, `text-muted-foreground`, `hover:bg-muted`             |
| `FormBooleanSelect.svelte`          | `text-zinc-400`, `border-layout-border`, `bg-layout-surface`, `text-zinc-500` | DS label + `border-border` + `bg-background` + `text-muted-foreground` |
| `FormPrice.svelte`                  | `text-zinc-400` label style                                                   | DS label style (`text-[10px] ...`)                                     |
| `WishlistPickerSection.svelte`      | `text-zinc-500`, `rounded-md`                                                 | `text-muted-foreground`, `rounded-sm`                                  |
| `WishlistPreferencesSection.svelte` | `rounded-md`, `border-layout-border`                                          | `rounded-sm`, `border-border`                                          |
| `RollingStockSection.svelte`        | `rounded-lg`, `border-white/10`, `text-zinc-500`                              | `rounded-sm`, `border-border`, `text-muted-foreground`                 |
| `AddWishlistItemDrawer.svelte`      | inline custom footer spacing/tokens                                           | shared `DrawerFooter`                                                  |

---

## Gap Analysis

| Area                | Current Implementation                         | Design System Standard                             | Gap                           |
| ------------------- | ---------------------------------------------- | -------------------------------------------------- | ----------------------------- |
| Footer composition  | Wishlist drawer uses inline footer             | All drawers use `DrawerFooter`                     | Inconsistent CTA architecture |
| Section cards       | Wrapper duplicated in multiple files           | Shared section-card component                      | Repetition + drift risk       |
| Label typography    | Mixed (`text-xs`, `zinc-*`) in some atoms      | Standard uppercase micro-label style               | Visual inconsistency          |
| Token system        | Legacy `zinc-*`, `layout-*`, `white/*` remains | Semantic DS tokens (`border-border`, etc.)         | Token drift                   |
| Corner radius       | `rounded-md`/`rounded-lg` in places            | `rounded-sm` for mechanical precision              | Shape inconsistency           |
| Manufacturer select | Inline custom select in model info             | Use shared form atoms or dedicated reusable select | Atomicity break               |

---

## Refactor Candidates

Promote these to standalone/reused components:

1. `DrawerSectionCard.svelte`  
   Shared card shell + section label.
2. `ModelDetails.svelte`  
   Unified manufacturer/product/description/category/scale/power/epoch block.
3. `PriorityToggle.svelte`  
   Reusable tri-state wishlist priority control.
4. `ManufacturerSelect.svelte` (or enhance `FormSelect`)  
   Handles loading + required + errors consistently.
5. Standardize all drawers to `DrawerFooter.svelte`  
   Remove custom inline footers.
6. Optional: `DrawerFieldLabel.svelte`  
   Centralize label typography and asterisk behavior.

---

## Svelte 5 State Plan (`$props()` + `$bindable()`)

1. Keep **superform state in parent drawer** as single source of truth.
2. Pass each field to subcomponents as explicit bindable props:
   - `manufacturerId = $bindable<string | null>()`
   - `productCode = $bindable('')`, etc.
3. Pass flattened `errors` objects to each section (avoid exposing full form internals).
4. Keep async lookups (manufacturers/wishlists) in parent or section owning the fetch boundary.
5. Emit section-level callbacks only for non-field events (e.g., prototype selected, open picker), not raw store mutation.

This preserves strict rune patterns while keeping subcomponents stateless and reusable.

---

## Svelte 5 Rune Recommendations

- Use `$props()` in every extracted section and atom.
- Use `$bindable()` for each editable field prop; avoid passing a mutable object blob.
- Use `$derived()` for computed labels/options/selection state.
- Use `$effect.pre()` only for controlled reset behavior on drawer open.
- Keep validation orchestration in parent `superForm`, and keep section components presentation-focused.

---

## Refactor Roadmap (Prioritized)

### P1 — High Impact Consistency

1. Replace wishlist inline footer with shared `DrawerFooter`.
2. Normalize token drift in `DrawerSectionBar`, `FormBooleanSelect`, `FormPrice`.
3. Align rounded tokens to `rounded-sm` where required.

### P2 — Structural Reuse

4. Introduce `DrawerSectionCard` and migrate section wrappers.
5. Extract `PriorityToggle`.

### P3 — Form Architecture Unification

6. Extract/standardize `ManufacturerSelect` and migrate `ModelInfoSection`.
7. Define a reusable `ModelDetails` contract for wishlist/collection/add-model contexts.

---

## Conclusion

The current drawer foundation is strong (`DrawerShell`, `DrawerHeader`, shared form atoms), but consistency is weakened by footer divergence, token drift in primitive components, and repeated section wrappers. A focused refactor around shared section composition and strict semantic tokens will produce a cohesive **Mechanical Precision** UI system with lower maintenance cost.
