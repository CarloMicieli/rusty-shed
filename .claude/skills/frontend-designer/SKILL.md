---
name: frontend-design
description: Use when creating, modifying, refactoring, or auditing Svelte 5 UI components and views in the Rusty Shed Tauri 2 application. Covers design-system compliance, Tailwind tokens, shadcn-svelte composition, component architecture, accessibility, and idiomatic Svelte 5 implementation.
---

# Frontend Design

## 1. Purpose

Implement maintainable, accessible, reusable frontend UI that conforms to the Rusty Shed design system and frontend architecture.

This skill governs:
- Svelte 5 component implementation.
- Tailwind CSS and design-token usage.
- shadcn-svelte composition and extension.
- Component APIs, variants, and reuse.
- UI-level accessibility.
- UI refactoring and design-system audits.

This skill does not independently define the visual identity or desktop interaction strategy. Consult the specialist skills when those concerns are central:
- `desktop-ux` for desktop usability and interaction design.
- `drawer-design` for drawer-specific structure and behavior.
- `web-form` for form architecture and validation.

## 2. Mandatory Project References

Before implementing or auditing UI, read:

1. `docs/ux/design-system.md`
2. `docs/frontend-architecture.md`

Authority:
- `design-system.md` is the single source of truth for visual decisions: colors, typography, spacing, radii, surfaces, borders, shadows, states, motion, and component appearance.
- `frontend-architecture.md` is the source of truth for application structure, routing, component organization, Svelte conventions, and frontend integration patterns.
- This skill defines implementation practices, not a competing visual system.

Inspect the relevant existing components, utilities, theme files, and package versions before choosing an implementation. Do not assume a component, token, utility, or dependency exists without checking.

If a skill instruction conflicts with either project document, follow the project document and report the conflict when relevant.

## 3. Token-First Styling

All visual styling must use the existing design system.

Requirements:
1. Prefer semantic theme utilities such as `bg-background`, `bg-card`, `text-foreground`, `text-muted-foreground`, `border-border`, and `ring-ring` when they are defined by the project.
2. Use existing project-specific utilities and component variants before creating new ones.
3. Do not introduce literal hex, RGB, HSL, or OKLCH colors in component markup or local styles unless `design-system.md` explicitly documents the exception.
4. Do not invent arbitrary spacing, dimensions, typography, radii, borders, shadows, or opacity values when an established token or component pattern exists.
5. Do not assume a token exists based only on its name. Verify it in the theme and project styles.
6. Do not copy visual values from screenshots or unrelated components when they conflict with the design-system definitions.

During an audit, flag:
- Hardcoded colors or token-bypass utility classes.
- Arbitrary font sizes or dimensions without a documented need.
- Inconsistent radii, spacing, borders, or surface treatments within the same component family.
- Local CSS that duplicates an existing token or utility.
- One-off visual effects where an existing project variant should be reused.

## 4. Component Composition and Reuse

Build components with clear responsibilities and predictable APIs.

Requirements:
1. Reuse existing primitives from the project's component library.
2. Use the project's `cn()` or equivalent class-merging utility when combining base classes with consumer-provided classes.
3. Ensure external class overrides are merged safely; avoid fragile string concatenation in reusable primitives.
4. Centralize meaningful visual variants using the project's established variant approach, such as `tailwind-variants` or CVA, if already installed.
5. Keep primitive components focused. Do not make a visual primitive own unrelated business state or domain workflows.
6. Extract repeated structure and behavior into components or Svelte snippets when doing so improves consistency and readability.
7. Avoid abstraction for a single trivial use when it would make the code harder to understand.
8. Preserve existing component APIs unless a deliberate migration is required.

When extending shadcn-svelte components:
- Follow the installed version's conventions and local project wrappers.
- Preserve expected props and rest-prop forwarding.
- Preserve class customization and composition behavior.
- Do not replace accessible primitives with custom elements without a compelling reason.

## 5. Svelte 5 Implementation

Use idiomatic Svelte 5 patterns, consistent with `frontend-architecture.md`.

Requirements:
1. Use `$props()` for component props and defaults.
2. Use `$state()` for mutable component-local state.
3. Use `$derived()` or `$derived.by()` for values computed from state or props.
4. Use `$effect()` only for genuine side effects, such as synchronizing with an external system, managing subscriptions, or interacting with browser APIs.
5. Do not use `$effect()` as a substitute for ordinary derivation.
6. Use `{#snippet}` and `{@render ...}` for repeated markup where snippets are the established or clearest pattern.
7. Keep event handling explicit and avoid unnecessary reactive indirection.
8. Clean up subscriptions, listeners, timers, and other resources created by a component.
9. Model asynchronous UI states explicitly, including loading, success, empty, and error states where applicable.

Avoid:
- Legacy Svelte reactivity patterns in new code unless required for compatibility with existing code.
- Effect chains that mutate values derived from props.
- Duplicated state that can become inconsistent.
- Recreating stable derived values inside event handlers.
- Browser-only APIs during module initialization when they may run outside the browser.

## 6. shadcn-svelte and Accessibility

Use semantic HTML and the project's shadcn-svelte primitives.

Requirements:
1. Prefer semantic elements (`button`, `a`, `form`, `label`, `input`, and appropriate landmarks).
2. Use the correct interactive primitive for the task; do not make a generic `div` behave like a button.
3. Preserve keyboard interaction and focus management supplied by accessible primitives.
4. Keep visible focus indicators in all supported themes.
5. Provide accessible names for icon-only controls.
6. Associate labels, descriptions, and errors with their controls.
7. Use ARIA only when native semantics are insufficient; do not add redundant or contradictory attributes.
8. Preserve disabled, invalid, selected, expanded, and other relevant states.
9. Do not rely on color alone to communicate meaning.
10. Ensure overlays, dialogs, popovers, and menus follow the accessibility behavior of their underlying primitive.

## 7. Layout and View Implementation

Before building a page or major view:
1. Inspect the existing route and layout structure.
2. Review `frontend-architecture.md` for the intended page composition.
3. Identify shared shell components and existing patterns.
4. Determine whether the view needs navigation, contextual panels, command areas, or other application-level regions.
5. Use the layout defined by the project; do not impose a new shell or panel arrangement merely because it appears in an older skill or example.

For full views, preserve the application's established Mechanical Precision visual language as specified in `design-system.md`.

## 8. Performance and Robustness

- Avoid unnecessary component state and reactive work.
- Use keyed each blocks when stable identity is available and relevant.
- Avoid expensive calculations in frequently re-rendered template expressions.
- Keep large or complex components decomposed along meaningful responsibilities.
- Handle asynchronous operations and rejected promises explicitly.
- Avoid introducing dependencies for functionality already provided by the project.
- Preserve predictable behavior during loading, empty, error, and boundary conditions.

## 9. Audit Workflow

When auditing UI, report findings in this order:

1. **Design-system violations:** token drift, hardcoded visual values, inconsistent visual language.
2. **Component architecture:** duplication, unclear responsibilities, brittle APIs, unsafe class composition.
3. **Svelte 5 correctness:** incorrect rune usage, unnecessary effects, duplicated state, lifecycle/resource issues.
4. **shadcn-svelte and accessibility:** semantic HTML, keyboard behavior, focus, labels, accessible states.
5. **Performance and robustness:** unnecessary work, missing async states, error handling, and edge cases.

For every finding, include:
- What is wrong.
- Why it matters.
- The file or component affected, when known.
- A concrete replacement pattern that uses existing project conventions.

Do not report stylistic preferences as violations unless they conflict with the project documents or create a concrete usability, accessibility, or maintainability problem.

## 10. Completion Checklist

Before considering UI work complete, verify:
- [ ] `design-system.md` was followed.
- [ ] `frontend-architecture.md` was followed.
- [ ] Existing tokens and primitives were reused.
- [ ] No undocumented hardcoded visual values were introduced.
- [ ] Svelte 5 runes are used appropriately.
- [ ] Component APIs and class overrides are predictable.
- [ ] Keyboard and focus behavior work.
- [ ] Icon-only controls have accessible names.
- [ ] Loading, empty, and error states are handled where relevant.
- [ ] No unnecessary dependency or abstraction was introduced.
