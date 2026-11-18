# Frontend Design System Migration Plan

## 1. Purpose and scope

This document is a migration audit and phased implementation plan for aligning the existing frontend to the requirements defined in `DESIGN_SYSTEM.md`.

This is a documentation-only analysis. It does not modify application code, configuration, dependencies, tests, or existing documentation beyond this plan.

## 2. Current frontend baseline

### 2.1 Framework and architecture

The current frontend is a SvelteKit application with a Tauri 2 desktop shell and a strong Svelte 5 runes-based codebase.

Evidence from the repo:

- `package.json` shows `svelte` `^5.57.3`, `@sveltejs/kit` `^2.70.4`, `tailwindcss` `^4.3.5`, and `@tauri-apps/cli` `^2.12.4`.
- `src/routes/+layout.svelte` imports the app shell, sidebar navigation, bottom navigation, theme store, and app-level drawer states.
- `src/lib/stores/themeStore.svelte.ts` uses a runes-based singleton (`$state`) and resolves theme preference through `data-theme` on `document.body`.
- The application uses a global shell layout structured around a desktop sidebar and a mobile bottom nav, with a shared app shell and route-level drawers.

### 2.2 Global styling and tokens

The app already contains the key visual primitives the design system expects:

- `src/routes/layout.css` includes Tailwind v4 theme setup with `@theme inline` and the app’s shadcn-style semantic variables.
- `src/lib/themes/steampunk-dark.css` and `src/lib/themes/steampunk-light.css` define the two steampunk palettes.
- `src/app.css` sets typography defaults, global heading styling, and local font-face declarations.
- Theme resolution is already performed by `document.body.dataset.theme` instead of a `.dark` class, which matches the design-system guidance.

### 2.3 Navigation, drawer and shell patterns

The existing app matches the intended cross-platform shell direction in several concrete ways:

- `src/lib/components/navigation/config.ts` explicitly defines four primary mobile items and a secondary “More” menu. This matches the design-system requirement for a limited mobile nav tray and a secondary overflow surface.
- `src/lib/components/BottomNavigation.svelte` implements a 4-item primary tray with a fifth “More” action.
- `src/lib/components/SidebarNavigation.svelte` implements a desktop sidebar with a collapse toggle, preserving the collapsed icon/rail pattern identified in the design spec.
- `src/lib/components/drawer/DrawerShell.svelte` includes a dirty-state guard (`hasChanges`), discard dialog, escape-to-close behavior, and keyboard handling. This is a direct match to the design-system requirement for validation and unsaved-changes handling.

### 2.4 Testing and tooling

The current project already has a strong frontend validation setup:

- `pnpm check` runs Svelte type-checking.
- `pnpm lint` runs ESLint.
- `pnpm test` runs Vitest in a happy-dom environment.
- `pnpm test:e2e` drives Tauri with WebdriverIO.
- Rust checks are split from frontend checks (`cargo fmt`, `cargo check`, `cargo clippy`) and are available via the project scripts.

## 3. What already aligns with the design system

The implementation is already in a good position to support the design-system direction because it has several of the required architectural building blocks in place:

1. Theme model consistency
   - Theme resolution is applied to the document body via `data-theme`.
   - Dark and light modes are already represented as steampunk-specific values instead of a generic `.dark` layout.

2. Navigation consolidation
   - Primary navigation and secondary overflow patterns already exist in code.
   - The app clearly distinguishes mobile, tablet, and desktop shell patterns.

3. Form + drawer discipline
   - Dirty-state management and modal close confirmation are already implemented in the shared drawer shell.
   - This is one of the strongest matches to the design-system specification.

4. Shared visual primitives
   - Typography, font imports, and key theme tokens are defined globally.
   - The app already uses a steampunk aesthetic with copper/brass accents and a shared color palette.

5. Responsive architecture
   - The shell uses a desktop sidebar plus mobile bottom navigation, which is a sound foundation for the cross-platform model described in the design system.

## 4. Migration gaps and implementation drift

Although the repository already matches the design-system direction in several places, the current implementation still has material drift and duplication that should be handled in a staged migration.

### 4.1 Token architecture duplication

The app currently mixes multiple token sources:

- `src/lib/themes/steampunk-dark.css` and `src/lib/themes/steampunk-light.css` define custom color scales such as `--color-surface-*` and `--color-primary-*`.
- `src/routes/layout.css` defines `@theme inline` tokens and `--background`, `--foreground`, `--card`, `--border`, `--primary`, etc.
- `src/app.css` also defines top-level aliases such as `--brass`, `--copper`, and base typography rules.

This creates a real risk of divergence: some components will use semantic shadcn tokens, while others use steampunk-specific scales directly. The design system explicitly requires a single token hierarchy and semantic aliases. The current codebase needs a normalization pass.

Recommendation:

- Keep the theme files as the canonical source for the steampunk palette.
- Make the Tailwind/semantic variables in `src/routes/layout.css` a derived layer, not an independent color source.
- Prefer semantic aliases (`--surface-elevated`, `--surface-hover`, `--border-subtle`, `--border-strong`) in component styling and remove raw direct references where they conflict.

### 4.2 Theme-layer inconsistency between CSS files

The design system’s requirement is explicit: the app should use a shared visual identity with a clear platform-specific shell, not a second or conflicting theme model.

Current drift:

- The root CSS defines a default dark-ish `:root` palette and a `.dark` variant in the same file.
- The runtime theme logic writes `data-theme` on the body.
- The theme files define `[data-theme='steampunk-dark']` and `[data-theme='steampunk-light']` blocks.

This is not a direct contradiction in behavior, but it is a maintenance hazard: CSS references across the repo are split between `:root`, `.dark`, and `[data-theme='...']`. Need a single source of truth for theme activation and token inheritance.

Recommendation:

- Standardize on `data-theme` as the single activation mechanism.
- Remove or deprecate generic `.dark` styling conventions except where compatibility is unavoidable.
- Keep the theme files and `@theme inline` import as the single chain of token resolution.

### 4.3 Mixed component styling patterns

There are currently multiple styling conventions in the frontend:

- Tailwind classes generated from `@theme inline` values.
- Inline CSS variables and custom classes from `src/app.css`.
- Hardcoded hex colors and inline styles in some components.

This is manageable for a small codebase, but it creates drift against the design-system requirements for consistency and implementation safety.

Recommendation:

- Convert shared component styling to a single “base token + semantic alias” model.
- Reduce inline hex-value styling to case-specific exceptions only.
- Encapsulate all repeated steampunk surfaces in shared classes or theme-level CSS variables rather than repeated raw values.

### 4.4 Responsive behavior is partly implemented but not fully standardized

The app already implements:

- a desktop sidebar,
- a mobile bottom nav,
- drawers for forms,
- dense collection/search flows.

However, the design-system requirements call for a more explicit platform-level interaction contract across desktop, tablet, and mobile. The repo’s current code is best described as “desktop-first with mobile adaptations,” not yet a fully standardized platform matrix.

Recommendation:

- Add explicit tablet breakpoints and rules for split views / overlay panels.
- Standardize drawer behavior by device: full-panel desktop drawers, tablet overlay and reduced-width panels, mobile bottom sheets / fullscreen forms.
- Define a common collection view UX pattern: search, filter, sort, result summary, active chips, and action flow.

### 4.5 Accessibility needs to be codified more consistently

The design system emphasizes WCAG AA, touch-safe controls, and effective focus management. The codebase is directionally aligned, but accessibility still appears as a combination of conventions and component-by-component behavior rather than a fully enforced design contract.

Current strengths:

- Drawer close controls and focus logic are present.
- Mobile nav is thumb-reachable and compact.

Current gaps:

- Not all UI shells explicitly enforce consistent touch target sizing across all form factors.
- There is no single design-system-level checklist that all components must satisfy in the same way the document describes.

Recommendation:

- Define and enforce a shared target size and spacing contract across all navigation, form, and action surfaces.
- Standardize the acceptable minimum sizes for toolbar buttons, form controls, and mobile gestures.
- Keep focus-visible styling consistent across browser and Tauri shell implementations.

## 5. Migration plan

### Phase 0 — Baseline audit and freeze

Objective: Establish the design-system contract and freeze the current implementation baseline before any migration work.

Tasks:

- Confirm `DESIGN_SYSTEM.md` as the canonical design authority.
- Build a component inventory by route, shell, and shared library.
- Tag existing patterns that already match the spec (theme store, nav shell, drawer validation, dirty-state guard).
- Identify components that need design-token and shell normalization.

Deliverables:

- a migration backlog grouped by token, shell, form, collection, and accessibility work
- a list of components already conformant vs. needing change

### Phase 1 — Token consolidation and theme normalization

Objective: Reduce multiple color sources to one consistent design-token layer.

Tasks:

- Confirm the theme files as the canonical palette source.
- Make `src/routes/layout.css` a derived semantic token layer rather than a second theme source.
- Standardize all references to `--color-surface-*`, `--surface-elevated`, `--surface-hover`, `--border-subtle`, and `--border-strong`.
- Remove or deprecate raw `.dark` references where they conflict with the `data-theme` system.
- Reconcile any components that directly use raw hex colors with the theme variable system.

Design-system alignment:

- Shared visual identity preserved
- Semantic theme variables consistent across dark and light modes
- No theme inversion risk

### Phase 2 — Global primitives and shared component foundation

Objective: Establish the base component vocabulary before touching route-specific screens.

Tasks:

- Normalize button, input, card, badge, dialog, and sheet styling around the design-system tokens.
- Standardize the shared product primitives against the steampunk copper/brass palette and touch-safe dimensions.
- Ensure form controls and action buttons support consistent focus, hover, disabled, and validation states.
- Align card and panel surfaces with semantic tokens, not raw backgrounds.

Design-system alignment:

- Buttons and form controls match the visual identity
- Validation and state styling are consistent
- The visual identity is preserved while functional controls remain legible and accessible

### Phase 3 — Application shell and navigation migration

Objective: Align the shell and navigation model with the platform contract in the design system.

Tasks:

- Normalize desktop sidebar behavior and collapsed states.
- Enforce the 4-item mobile tray + “More” secondary menu pattern consistently.
- Create a tablet-specific shell strategy for overlays, compact side panels, and split views.
- Ensure the shell layout remains consistent across the app rather than being solved ad hoc by routed components.

Design-system alignment:

- Single navigation vocabulary across form factors
- Desktop and mobile interaction patterns reflect their respective role and input methods
- Navigation density stays within the design-system guidance

### Phase 4 — Drawer and form interaction standardization

Objective: Make the app’s form and drawer shells conform to a single cross-platform interaction model.

Tasks:

- Standardize drawer shell usage across routes and feature modules.
- Enforce desktop right-panel drawers, tablet compact or overlay drawers, and mobile fullscreen/bottom-sheet flows.
- Confirm all drawers use the same safe-area, focus, and modal semantics.
- Preserve the dirty-state pattern (`hasChanges`, confirm-close, validation messaging) in all long-form editors.

Design-system alignment:

- Shared steampunk identity with correct platform-specific shell behavior
- Real workflow semantics remain consistent with the intent of the app
- No hidden destructive close paths

### Phase 5 — Collection and data-heavy workflows

Objective: Make large-model collections and search views behave consistently with the design-system specification.

Tasks:

- Align collection tools around search, filter, sort, result summary, and active filter chips.
- Standardize filter panel patterns for desktop vs. mobile.
- Preserve scannable list layout and predictable “find → filter → act” flow.
- Keep persistent result context visible on desktop and tablet while preserving mobile efficiency.

Design-system alignment:

- Long-list workflows become predictable and consistent
- Large railway collection datasets remain accessible without breaking the design system

### Phase 6 — Accessibility, motion, and QA validation

Objective: Close the implementation gaps around touch targets, motion, and conformance checks.

Tasks:

- Verify all interactive controls meet the intended minimum touch target sizes.
- Keep focus-visible states strong and consistent across desktop and mobile.
- Review animation timing and reduced-motion behavior against the design-system motion contract.
- Run the frontend validation suite to ensure migration work preserves behavior and accessibility requirements.

Design-system alignment:

- WCAG-aligned behavior is preserved
- The app remains comfortable on touch devices while retaining desktop efficiency
- Motion and transition patterns respect user preferences and performance constraints

## 6. Recommended migration order

The highest-value migration order is:

1. theme tokens and color hierarchy
2. shared primitives (buttons, inputs, cards, dialogs)
3. shell navigation (sidebar + mobile nav)
4. drawers and validation flows
5. collection and filter surfaces
6. accessibility and motion QA

This order keeps the migration low-risk because it normalizes the foundation before route-specific screens and data-heavy flows are touched.

## 7. Risks to watch

- Multiple theme definitions may drift over time if token ownership is not centralized.
- Route-specific styling may bypass the shared design-system variables and reintroduce raw colors.
- Mobile and tablet experiences may stay as a patched desktop flow rather than a real platform model.
- Accessibility may be treated as documentation-only instead of being enforced in component patterns.

## 8. Success criteria

The migration should be considered complete when all of the following are true:

- One theme-token model exists and is used across the frontend.
- Desktop, tablet, and mobile interaction shells follow the same steampunk identity but use the correct layout patterns for their form factor.
- Navigation and modals remain purposeful and task-oriented instead of decorative.
- Dirty-state and validation flows are consistent across drawers and forms.
- The frontend passes the project’s existing lint, TypeScript, and test validation.
- The implementation matches the intent of `DESIGN_SYSTEM.md` without inventing an alternate design language.

## 9. Conclusion

The repo already contains a strong starting point: a runes-based Svelte app, explicit theme resolution, a desktop sidebar + mobile nav model, and a dirty-state drawer pattern. The migration should therefore focus on normalization and consistency rather than full redesign.

The largest technical work is not adding new visual features; it is consolidating the app around a single token and interaction model so the design system is implementable, maintainable, and consistent across desktop, tablet, and mobile surfaces.
