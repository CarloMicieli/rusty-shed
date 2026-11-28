# Frontend Technical Debt Implementation Backlog

## A. Executive Summary

This document replaces the earlier speculative audit with an implementation backlog
verified against the current frontend source. It is documentation-only: none of the
tasks below require application source changes before the task itself is started.

The codebase has a strong architectural foundation:

- Svelte 5 runes are used consistently in the audited application code.
- Frontend work is organized by feature, with shared services and UI primitives.
- `tauri-specta` generated bindings provide the primary typed IPC contract.
- Existing infrastructure already includes `safeInvoke`, `TauriAdapter`, `DrawerShell`,
  `FormInput`, `FormSelect`, `VirtualGrid`, and feature-level state classes.
- Paraglide-JS, Tailwind CSS 4, shadcn-svelte composition, and the steampunk design
  system should be preserved.

The verified backlog contains **18 independently actionable tasks**. Seven proposed
abstractions from the first draft are not backlog tasks:

| Proposed abstraction | Decision | Rationale |
|---|---|---|
| `StateContainer` | Not justified | No concrete initialization defect was established; feature state classes and Svelte context are already the established patterns. |
| `TauriCommandService` | Replace with smaller consolidation | `safeInvoke` and `TauriAdapter` already normalize IPC. A class would duplicate these functions unless they are first reconciled. |
| `FormFieldWrapper` | Not needed | `FormInput`, `FormSelect`, `FormPrice`, and shared `FormField.svelte` already cover field composition; the missing work is consistency and adoption. |
| `useDrawerForm` | Not needed initially | `DrawerShell` already owns discard confirmation and change detection is local to individual form state. A hook should be added only if repeated lifecycle defects are demonstrated. |
| `ModelDetailsState` | Not justified | `useRollingStockEditor.svelte.ts` already owns rolling-stock editing behavior and is used by the model-details components. |
| `TauriBindingValidator` | Not justified | The generated bindings are the source of truth. Validation belongs in response-specific logic or Rust, not in a blanket runtime validator. |
| Generic `DataTable` | Not justified | Lists use distinct card/table layouts. `Table`, `VirtualGrid`, and feature-specific views already solve the proven use cases; a generic table would add indirection. |

## B. Verification Method

Status definitions:

- **Verified**: the source contains the cited problem and a specific measurable or
  maintainable improvement can be demonstrated.
- **Partially verified**: a real problem exists, but only in a subset of affected code
  or the proposed scope must be narrowed.
- **Requires further investigation**: evidence is insufficient for a refactor; a
  benchmark or usage audit must precede implementation.
- **Not justified**: the current source already solves the problem or an abstraction
  would add more complexity than it removes.

The audit distinguished line count from actual responsibility. Large line count alone
is not treated as evidence of a defect. Performance work requires an existing
bottleneck or a repeatable measurement.

## C. Consolidated Findings

### C1. Collection state has two authoritative caches

`CollectionState.svelte.ts` and `src/lib/state/collection.svelte.ts` maintain separate
copies of collection data. Layout and route code call `collectionStore`, while feature
components call `CollectionState`, and mutation methods in `CollectionState` also
refresh `collectionStore`. This is verified duplication.

Affected call sites include:

- `src/routes/+layout.svelte`
- `src/routes/collection/[itemId]/+page.svelte`
- `src/lib/components/SearchBar.svelte`
- `src/lib/features/collection/CollectionState.svelte.ts`

### C2. Tauri integration has two normalization implementations

`src/lib/services/tauri.ts` and `src/lib/shared/services/TauriAdapter.ts` contain
overlapping error normalization. They are in separate clean-architecture locations,
but some application code uses generated `commands`, while one service still calls
`@tauri-apps/api/core` directly.

### C3. Search requires cancellation and stale-response protection

`src/lib/components/SearchBar.svelte` already debounces input, but its debounce only
cancels the timer: an already-running search can finish after a newer query and replace
its results. It also manually inspects generated command result shapes and contains
hardcoded English “No results found” text.

### C4. Existing UI primitives should be standardized, not replaced

The drawer package already includes field wrappers and `DrawerShell` provides one
location for close/discard behavior. The actual debt is inconsistent adoption:
components combine direct markup, bindable fields, callbacks, and custom error markup.

### C5. Responsibility has already been partially extracted

`RollingStockCard.svelte` is 666 lines, but the source also contains specialized
components such as `RollingStockCardHeader`, `RollingStockIdentificationFields`,
`RollingStockTechnicalSpecs`, and `useRollingStockEditor.svelte.ts`. `RailwayModelCard`,
`RailwayModelPreviewCard`, and the model-details tabs already separate several concerns.
Further splitting should be narrow and driven by defects, not by a generic line target.

### C6. Virtualization is already in the main card grids

`CollectionDashboard.svelte` and `WishlistItems.svelte` use `VirtualGrid`. A second
generic virtualization abstraction is not currently justified. Remaining work is to
benchmark whether any additional list is a measured bottleneck.

### C7. Accessibility needs concrete fixes

`DrawerShell.svelte` implements Escape and discard confirmation but has no explicit
focus trap or focus restoration. Several custom dialogs also use overlay markup rather
than the existing shadcn dialog primitives. These are verified, but the fix scope
should start with the shared drawer shell and the highest-impact dialogs.

### C8. Internationalization has residual hardcoded UI text

Search contains hardcoded English text, and model-details option lists contain English
labels. Before introducing broader i18n automation, the backlog should remove verified
user-facing literals in audited components.

## D. Refactoring Tasks

### FE-001: Remove the collection store/cache duplicate

- **Priority:** Critical
- **Verification status:** Verified
- **Category:** State management
- **Affected files:** `src/lib/state/collection.svelte.ts`,
  `src/lib/features/collection/CollectionState.svelte.ts`,
  `src/routes/+layout.svelte`, `src/routes/collection/[itemId]/+page.svelte`,
  `src/lib/components/SearchBar.svelte`
- **Problem description:** Collection data and loading state are stored in both
  `CollectionState` and the module-level `collectionStore`. Mutations refresh the
  second store, but layout and route code can read stale or separately cached data.
- **Reason for change:** A single collection cache reduces synchronization bugs and
  removes repeated IPC and item lookup logic.
- **Suggested implementation:** Choose one existing owner: the feature
  `CollectionState` context for feature views and route state, or a thin module-level
  cache for global lookups. Do not create a new container. Migrate layout, route,
  SearchBar, and mutations one call at a time, then remove the duplicate owner and
  update tests. Preserve route refresh and cache-hit behavior.
- **Expected outcome:** One collection fetch/cache owner, consistent item lookup, and
  no mutation that silently updates only one copy.
- **Acceptance criteria:** No production component reads or writes both collection
  owners; route refresh/re-entry behavior passes existing tests; collection mutations
  update every view observing the collection; no stale item remains after refresh.
- **Dependencies:** None.
- **Estimated effort:** L

### FE-002: Consolidate duplicate Tauri normalization into the existing adapters

- **Priority:** High
- **Verification status:** Verified
- **Category:** Tauri integration
- **Affected files:** `src/lib/services/tauri.ts`,
  `src/lib/shared/services/TauriAdapter.ts`, `src/lib/services/index.ts`,
  `src/lib/shared/services/index.ts`
- **Problem description:** Two modules implement similar `normalizeError` and
  `SafeResult` behavior. Application code may receive subtly different normalized
  fields from different import paths.
- **Reason for change:** Reconciliation is smaller and safer than introducing a new
  service or validator. It removes drift in error normalization and gives one
  documented import path.
- **Suggested implementation:** Decide one canonical module aligned with the existing
  Clean Architecture adapter location. Re-export compatibility types/functions only
  if needed during migration, update imports, and delete duplicate implementation
  after all call sites move. Keep generated `commands` and tauri-specta types intact.
- **Expected outcome:** One implementation of normalization and `safeInvoke`;
  existing tests continue to pass with the same public behavior.
- **Acceptance criteria:** Only one production implementation contains
  `normalizeError`; all production call sites use the canonical export; test coverage
  includes the full `NormalizedError` union; no direct `invoke` use remains outside
  the canonical adapter and generated bindings.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-003: Migrate remaining direct Tauri adapter use

- **Priority:** Medium
- **Verification status:** Verified
- **Category:** Tauri integration
- **Affected files:** `src/lib/features/track-inventory/services/TrackInventoryService.svelte.ts`,
  `src/lib/state/finance.svelte.ts`, any remaining direct-import search results
- **Problem description:** A few production files import `invoke` directly instead of
  using the repository's generated bindings or safe IPC adapter.
- **Reason for change:** Direct calls bypass consistent error normalization and make
  command contracts harder to discover.
- **Suggested implementation:** Replace direct calls with existing generated
  `commands` methods where generated methods exist. Where custom `invoke` behavior
  is required, route it through the canonical adapter selected in FE-002.
- **Expected outcome:** IPC calls are typed and normalized consistently.
- **Acceptance criteria:** Production source outside generated bindings and the
  canonical adapter has no `@tauri-apps/api/core` `invoke` import; relevant service
  tests pass.
- **Dependencies:** FE-002.
- **Estimated effort:** M

### FE-004: Add stale-response protection to SearchBar

- **Priority:** High
- **Verification status:** Verified
- **Category:** Performance and correctness
- **Affected files:** `src/lib/components/SearchBar.svelte`,
  `src/__tests__/lib/components/SearchBar.test.ts`
- **Problem description:** The debounce cancels only the pending timer. A slow
  response for an earlier query can overwrite newer results, and rapid changes can
  still launch work for stale values.
- **Reason for change:** Stale results are an observable correctness bug and frequent
  typing makes it likely.
- **Suggested implementation:** Add a monotonically increasing request sequence or
  invalidate a pending request when input changes. Ignore obsolete completions.
  Clear stale UI state on close/Escape. Keep the 300 ms debounce and 10-result limit.
  Reuse generated command methods rather than a new service.
- **Expected outcome:** The displayed results always correspond to the latest query
  submitted by the user.
- **Acceptance criteria:** Tests prove an older promise cannot replace a newer result;
  a cleared/closed query cannot restore results; debounce behavior and 10-result limit
  remain unchanged.
- **Dependencies:** None.
- **Estimated effort:** S

### FE-005: Replace manual SearchBar result parsing

- **Priority:** Medium
- **Verification status:** Verified
- **Category:** Type safety
- **Affected files:** `src/lib/components/SearchBar.svelte`
- **Problem description:** SearchBar manually inspects result objects and casts data
  to `RailwayModelView`, despite generated command return types.
- **Reason for change:** Manual parsing duplicates the generated contract and can
  diverge after bindings are regenerated.
- **Suggested implementation:** Use the actual return shape of `commands.searchRailwayModels`
  and `commands.getRailwayModelById`; adjust only after generated binding types are
  inspected. Remove casts and defensive branches that do not correspond to a real
  generated result variant.
- **Expected outcome:** Search is compiler-checked against tauri-specta and contains
  no manual duplicate response validation.
- **Acceptance criteria:** SearchBar has no `as RailwayModelView` or `unknown` response
  parsing for these commands; `pnpm svelte-check` passes; search tests cover success,
  empty, and error states.
- **Dependencies:** FE-004.
- **Estimated effort:** S

### FE-006: Localize remaining SearchBar literals

- **Priority:** Medium
- **Verification status:** Verified
- **Category:** Internationalization
- **Affected files:** `src/lib/components/SearchBar.svelte`
- **Problem description:** “No results found” is hardcoded, while surrounding UI uses
  Paraglide.
- **Reason for change:** It is a direct i18n violation visible to every locale.
- **Suggested implementation:** Add matching English and Italian Paraglide keys and
  replace both desktop and mobile strings. Reuse an existing key if one already
  expresses the same message.
- **Expected outcome:** SearchBar contains no user-facing unlocalized result text.
- **Acceptance criteria:** Both key files define the key; English and Italian builds
  contain no missing key; no search UI branch contains the hardcoded English message.
- **Dependencies:** None.
- **Estimated effort:** S

### FE-007: Standardize existing drawer field primitives

- **Priority:** High
- **Verification status:** Verified
- **Category:** Forms and UI consistency
- **Affected files:** `src/lib/components/drawer/FormInput.svelte`,
  `src/lib/components/drawer/FormSelect.svelte`,
  `src/lib/components/drawer/FormPrice.svelte`,
  `src/lib/components/drawer/FormFieldsRenderer.svelte`,
  `src/lib/features/` drawer implementations
- **Problem description:** Repeated field markup remains even though field primitives
  exist. Some components expose bindable values, others callbacks or direct state
  mutation. Some select fields have `span` labels rather than associated `label`
  elements.
- **Reason for change:** Consistency can be improved without a new wrapper abstraction.
- **Suggested implementation:** Define the existing primitives as the canonical
  choices, add only props needed to correct concrete gaps (such as consistent
  `id`/`aria-describedby`/`aria-invalid` behavior), and migrate repeated markup in
  a targeted set of drawers. Do not add `FormFieldWrapper` or a general form DSL.
- **Expected outcome:** Drawers compose existing primitives with consistent labels,
  errors, disabled state, and IDs.
- **Acceptance criteria:** Migrated drawers have associated labels and error text;
  no duplicated label/error markup remains in those drawers; component tests cover
  required and invalid states.
- **Dependencies:** FE-008 and FE-009 may use the same primitives.
- **Estimated effort:** L

### FE-008: Complete shared DrawerShell adoption

- **Priority:** Medium
- **Verification status:** Verified
- **Category:** Drawer consistency
- **Affected files:** `src/lib/components/drawer/DrawerShell.svelte` and the
  `<DrawerShell>` implementations under `src/lib/features/`
- **Problem description:** Nine feature drawers use `DrawerShell`, but adoption and
  close behavior differ. The shell accepts `hasChanges` and owns discard UI, but
  individual drawers decide when that state is true.
- **Reason for change:** It is low-risk to improve existing shell usage rather than
  introduce `useDrawerForm`.
- **Suggested implementation:** Audit each `DrawerShell` caller for accurate
  `hasChanges`, close callbacks, submission pending state, and translated discard
  text. Fix only concrete mismatches. Do not create a drawer form composable.
- **Expected outcome:** Closing or pressing Escape has consistent dirty-state behavior
  across existing drawers.
- **Acceptance criteria:** Each audited drawer passes the correct dirty state and
  close callback; dirty and clean close behavior is tested in at least representative
  drawers; no drawer bypasses the shell solely to reproduce discard behavior.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-009: Add focus management to DrawerShell

- **Priority:** High
- **Verification status:** Verified
- **Category:** Accessibility
- **Affected files:** `src/lib/components/drawer/DrawerShell.svelte`,
  `src/__tests__/` drawer component tests
- **Problem description:** The drawer has dialog semantics and Escape handling but
  does not explicitly trap focus or restore focus to the opening control.
- **Reason for change:** Focus loss is a concrete accessibility regression for keyboard
  and assistive-technology users.
- **Suggested implementation:** Use Svelte DOM lifecycle APIs and existing shadcn
  dialog primitives if compatible; otherwise implement a small scoped focus trap in
  the shell. Save the active element when opened and restore it when closed.
- **Expected outcome:** Keyboard focus remains within the open drawer and returns to
  the opener after close.
- **Acceptance criteria:** Automated tests verify initial focus, Tab cycling, Escape,
  close, and focus restoration; no scroll-lock leak remains after close/unmount.
- **Dependencies:** FE-008.
- **Estimated effort:** M

### FE-010: Localize model-details control labels

- **Priority:** Medium
- **Verification status:** Verified
- **Category:** Internationalization
- **Affected files:** `src/lib/components/model-details/RailwayModelTabsContainer.svelte`,
  `src/lib/components/model-details/RollingStockCard.svelte`
- **Problem description:** Option lists include English labels such as DCC Ready,
  Analogue, DCC interface names, and coupler sockets.
- **Reason for change:** Paraglide is the established localization framework and
  these values are user-facing.
- **Suggested implementation:** Distinguish protocol/model identifiers, which should
  remain technical constants, from translated labels. Add keys for translated labels
  and pass localized text into existing option structures. Avoid changing persisted
  enum values.
- **Expected outcome:** UI labels follow the active locale while data values remain
  stable.
- **Acceptance criteria:** No user-facing label in the audited lists is hardcoded;
  keys exist in English and Italian; saving unchanged technical values produces the
  same backend payload.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-011: Narrow the RollingStockCard responsibility split

- **Priority:** Medium
- **Verification status:** Partially verified
- **Category:** Component responsibility
- **Affected files:** `src/lib/components/model-details/RollingStockCard.svelte`,
  `src/lib/components/model-details/components/`,
  `src/lib/components/model-details/useRollingStockEditor.svelte.ts`
- **Problem description:** The card is large and mixes orchestration with some UI,
  but several concerns have already been extracted. A second large split is not
  justified without identifying a specific failing test or duplicated state path.
- **Reason for change:** Targeted extraction can improve maintainability while
  avoiding component churn.
- **Suggested implementation:** Identify only the remaining duplicated state or
  rendering branch in one focused pass. Prefer moving a complete behavior to an
  existing child or `useRollingStockEditor` before creating a new component. Do not
  introduce a generic model-details state class.
- **Expected outcome:** The card delegates its existing editor behavior cleanly and
  no unrelated responsibilities remain in its markup.
- **Acceptance criteria:** At least one measurable responsibility is removed (for
  example loading/spec state or identification rendering); existing component tests
  pass; no new abstraction exceeds the complexity it replaces.
- **Dependencies:** FE-010 only if option labels are touched.
- **Estimated effort:** M

### FE-012: Prevent stale model-details overrides after refresh

- **Priority:** Medium
- **Verification status:** Requires further investigation
- **Category:** State synchronization
- **Affected files:** `src/lib/components/model-details/RailwayModelTabsContainer.svelte`,
  `src/lib/components/model-details/RollingStockCard.svelte`,
  `src/lib/components/model-details/useRollingStockEditor.svelte.ts`
- **Problem description:** Optimistic or local overrides exist for rolling-stock edits
  and deletions. It is unclear whether refresh, remount, or failed reload consistently
  clears them.
- **Reason for change:** If stale overrides survive a refresh, they can display deleted
  or outdated entities.
- **Suggested implementation:** Reproduce refresh/remount cases in tests. First fix
  only the incorrect ownership or reset behavior in `useRollingStockEditor` or the
  existing tab component. Do not add `ModelDetailsState`.
- **Expected outcome:** Local overrides are temporary and cannot outlive authoritative
  model updates.
- **Acceptance criteria:** Tests cover create/update/delete followed by parent refresh
  and failure/reload; stale units are not rendered after authoritative data changes.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-013: Add image upload/crop lifecycle tests before refactoring

- **Priority:** Low
- **Verification status:** Requires further investigation
- **Category:** Code quality
- **Affected files:** `src/lib/components/model-details/ImageUpload.svelte`,
  `src/lib/components/model-details/ImageCropDialog.svelte`,
  `src/lib/components/model-details/__tests__/ImageUpload.test.ts`,
  `src/lib/components/model-details/__tests__/ImageCropDialog.test.ts`,
  `src/lib/shared/services/ImageService.svelte.ts`
- **Problem description:** The image components are substantial and existing tests
  cover parts of upload/crop behavior, but there is no demonstrated duplicate
  implementation to justify a new `ImageUploadService`.
- **Reason for change:** A service should follow a concrete repeated behavior or
  lifecycle defect, not component size alone.
- **Suggested implementation:** Expand tests around object URL cleanup, crop
  cancellation, invalid files, Tauri file conversion, and remount. Refactor only if
  tests or duplicate logic identify a shared responsibility.
- **Expected outcome:** Image behavior is verified and any later extraction is based
  on demonstrated reuse.
- **Acceptance criteria:** Tests cover success, cancel, invalid input, unmount, and
  repeated crop/upload operations; no URL or file-reader resource leak is observed.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-014: Document and audit the edit context contract

- **Priority:** Low
- **Verification status:** Partially verified
- **Category:** State management
- **Affected files:** `src/lib/components/model-details/editContext.svelte.ts`,
  model-details components that call `getEditContext` or `setEditContext`
- **Problem description:** `editContext.svelte.ts` provides an isolated fallback when
  no provider exists. This can hide missing provider wiring, and the active-edit
  invariant is not documented.
- **Reason for change:** A small documentation and test improvement can prevent
  inconsistent edit locking without moving state into a larger container.
- **Suggested implementation:** Document provider ownership and fallback behavior,
  strengthen tests for nested/provider and fallback cases, and migrate only callers
  that demonstrably use mismatched context instances.
- **Expected outcome:** The active-edit context has one clear ownership rule and is
  easy to debug.
- **Acceptance criteria:** Tests cover provider, nested provider, and no-provider
  fallback; documentation identifies exactly which component establishes the context.
- **Dependencies:** None.
- **Estimated effort:** S

### FE-015: Add feature-boundary lint rules with baseline

- **Priority:** Medium
- **Verification status:** Requires further investigation
- **Category:** Architecture governance
- **Affected files:** `eslint.config.js`, `src/lib/features/*`,
  `src/lib/components/*`, `src/lib/shared/*`
- **Problem description:** Feature imports may cross feature boundaries, but a full
  dependency map is not yet established. Blanket rules could break legitimate shared
  imports.
- **Reason for change:** Enforcing boundaries can protect the existing feature-based
  architecture once exceptions are understood.
- **Suggested implementation:** Generate a direct-import report first. Add narrow
  rules for shared code not importing features and features not importing private
  paths of other features. Allow explicit public exports through feature indexes.
  Run in warning mode briefly if the existing tooling supports it, then enforce only
  the verified violations.
- **Expected outcome:** New cross-feature private imports are blocked without
  rewriting intentional shared usage.
- **Acceptance criteria:** A dependency report is reviewed; lint configuration contains
  no broad “ban every cross-feature import” rule; intentional public imports pass;
  private/relative cross-feature violations fail lint.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-016: Add a repeatable list performance baseline

- **Priority:** Medium
- **Verification status:** Requires further investigation
- **Category:** Performance
- **Affected files:** `src/lib/features/collection/CollectionDashboard.svelte`,
  `src/lib/features/wishlists/components/WishlistItems.svelte`,
  `src/lib/components/VirtualGrid.svelte`, relevant Vitest or Playwright harnesses
- **Problem description:** The virtual grids exist and are likely useful for large
  lists, but no baseline proves that other lists need virtualization or that changing
  derived state improves an observed bottleneck.
- **Reason for change:** Performance optimization should be gated by data and
  measurement rather than component count.
- **Suggested implementation:** Add a small synthetic dataset benchmark or Playwright
  scenario that reports mounted row count and interaction timing for collection and
  wishlist views. Audit remaining lists only if the baseline identifies high mounted
  element count or interaction delay.
- **Expected outcome:** A reproducible measurement identifies the next real list
  bottleneck.
- **Acceptance criteria:** Baseline is documented in test output or a test asserts an
  existing virtual-grid invariant; no speculative virtualizer or generic `DataTable`
  is added; one measured bottleneck is recorded before optimization.
- **Dependencies:** None.
- **Estimated effort:** M

### FE-017: Remove derived-state speculation until profiling shows a cost

- **Priority:** Low
- **Verification status:** Not justified
- **Category:** Performance
- **Affected files:** Potential: `RollingStockCard.svelte`,
  `RailwayModelCard.svelte`, model-details components
- **Problem description:** The previous plan assumed some `$derived` chains recalculate
  unnecessarily, but no benchmark or reactive dependency trace supports that claim.
- **Reason for change:** Svelte runes are already fine-grained; premature memoization
  would add state and debugging risk.
- **Suggested implementation:** No refactor by default. If FE-016 or application
  profiling identifies a hot derived computation, profile that exact expression and
  simplify or split it. Preserve Svelte 5 runes.
- **Expected outcome:** Avoided unnecessary reactivity work, with no added memoization
  layers.
- **Acceptance criteria:** No task implementation changes `$derived` solely for
  line count or architectural preference; any future change cites a recorded
  bottleneck and before/after measurement.
- **Dependencies:** FE-016.
- **Estimated effort:** S

### FE-018: Audit bundle icons and table usage with build evidence

- **Priority:** Low
- **Verification status:** Requires further investigation
- **Category:** Build and performance
- **Affected files:** `vite.config.ts` or `vite.config.js`,
  `package.json`, components importing `@lucide/svelte`
- **Problem description:** Lucide imports are mostly direct icon imports, and the
  repository already includes table primitives. The first draft proposed icon barrels
  and a generic table without build or usage evidence.
- **Reason for change:** The existing build may already tree-shake direct imports;
  a barrel or sprite could regress loading and maintainability.
- **Suggested implementation:** Generate a production bundle report and inspect
  chunk composition. Audit direct icon imports and table usage, but change code only
  if a repeatable large icon chunk or unused table implementation is measured.
- **Expected outcome:** A measured decision on whether icon or table code contributes
  meaningful bundle weight.
- **Acceptance criteria:** Bundle report is attached or recorded; no new icon library,
  sprite, or generic `DataTable` is introduced; a follow-up task exists only for a
  measured optimization.
- **Dependencies:** None.
- **Estimated effort:** S

## E. Abstraction Review

The following abstractions are explicitly rejected from the implementation backlog:

- **StateContainer:** no concrete defect and unnecessary centralization over feature
  state and Svelte context.
- **TauriCommandService:** the existing adapter pair can be reconciled; a class adds
  another lifecycle without a demonstrated need.
- **FormFieldWrapper:** existing `FormInput`, `FormSelect`, `FormPrice`, and shared
  `FormField` are sufficient; consistency should be fixed by migration and prop
  normalization.
- **useDrawerForm:** `DrawerShell` already owns discard confirmation and form state
  differs substantially enough that a hook would couple submission, validation, and
  feature-specific data.
- **ModelDetailsState:** `useRollingStockEditor` already owns behavior; a state wrapper
  would duplicate existing runes and callbacks.
- **TauriBindingValidator:** generated bindings are compile-time contracts. Blanket
  runtime validation adds maintenance and may duplicate Rust validation.
- **Generic DataTable:** cards, tables, and virtual grids already serve distinct use
  cases. Shared primitives should remain composable rather than becoming a generic
  data framework.

## F. Implementation Roadmap

### Phase 1: State and IPC correctness

- **Tasks:** FE-001, FE-002, FE-003
- **Objective:** Establish one collection state owner and one canonical Tauri
  normalization path.
- **Dependencies:** FE-003 depends on FE-002.
- **Estimated effort:** 1 L + 1 M + 1 M
- **Validation strategy:** Run collection, layout, route, service, and tauri adapter
  tests; run Svelte and TypeScript checks; verify refresh and cache behavior manually.

### Phase 2: Search and immediate i18n correctness

- **Tasks:** FE-004, FE-005, FE-006
- **Objective:** Prevent stale search results, remove manual response casting, and
  localize the remaining verified string.
- **Dependencies:** FE-005 follows FE-004 only for sequencing, not code ownership.
- **Estimated effort:** 3 S
- **Validation strategy:** Add stale-promise tests and run SearchBar tests; verify
  English and Italian message generation.

### Phase 3: Drawer and form consistency

- **Tasks:** FE-007, FE-008, FE-009
- **Objective:** Improve existing field primitives and shared drawer behavior before
  any new form abstraction.
- **Dependencies:** FE-009 follows FE-008; FE-007 can proceed independently.
- **Estimated effort:** 1 L + 1 M + 1 M
- **Validation strategy:** Run drawer/component tests, keyboard focus tests,
  svelte-check, and an accessibility review of representative desktop/mobile drawers.

### Phase 4: Model-details responsibility and localization

- **Tasks:** FE-010, FE-011, FE-012, FE-014
- **Objective:** Fix verified localization/state context issues and remove only
  demonstrated responsibility problems.
- **Dependencies:** FE-011 should follow FE-012 where refresh ownership overlaps.
- **Estimated effort:** 3 M + 1 S
- **Validation strategy:** Run model-details tests, edit-context tests, save/delete
  refresh tests, and locale checks.

### Phase 5: Verification before further extraction

- **Tasks:** FE-013, FE-016, FE-018
- **Objective:** Establish lifecycle and performance evidence before image, list,
  derived-state, or bundle refactors.
- **Dependencies:** FE-017 can be considered only after FE-016 yields evidence.
- **Estimated effort:** 2 M + 1 S
- **Validation strategy:** Run image tests; produce a list benchmark and production
  bundle report; record any follow-up implementation only from observed bottlenecks.

### Phase 6: Architecture governance

- **Tasks:** FE-015
- **Objective:** Add narrow, evidence-based feature boundary enforcement.
- **Dependencies:** None, but should occur after Phase 1 import ownership is stable.
- **Estimated effort:** M
- **Validation strategy:** Generate a dependency report, run lint, and verify that
  public imports and known exceptions remain valid.

### Deferred: Evidence-required optimization

- **Tasks:** FE-017
- **Objective:** No implementation until profiling proves reactive recomputation is
  expensive.
- **Dependencies:** FE-016.
- **Estimated effort:** S only if a concrete defect is confirmed.
- **Validation strategy:** Before/after benchmark for the exact derived expression.

## G. Task Count and Consolidation

The previous summary of 56 tasks was not supported by 56 detailed entries. This
backlog lists 18 tasks, and every entry contains the required implementation fields.
Overlapping recommendations were consolidated or removed:

- Component-size recommendations FE-002/FE-003/FE-012 were reduced to one targeted
  responsibility task and one model-details stale-state investigation.
- `TauriCommandService`, `TauriBindingValidator`, and runtime response-validation
  recommendations became one adapter consolidation and one direct-import migration.
- Drawer abstractions became existing-primitive and existing-shell improvements.
- Generic virtualization and `DataTable` work became a measurement task.
- Derived-state and bundle-size recommendations remain deferred until measured.

## H. Quality Assurance Checklist

For each completed task:

- [ ] Only task-related files changed.
- [ ] Existing Clean Architecture and feature boundaries remain intact.
- [ ] Svelte 5 runes and generated tauri-specta bindings remain the source patterns.
- [ ] English and Italian Paraglide keys are present for every new user-facing string.
- [ ] The implementation includes or updates targeted tests.
- [ ] No generic abstraction is introduced without a concrete repeated defect.
- [ ] Performance claims include before/after or a reproducible baseline.
- [ ] `pnpm svelte-check` passes.
- [ ] Relevant Vitest suites pass.
- [ ] Relevant Cargo tests pass only when backend behavior is part of the task.
- [ ] Lint and format checks pass.
