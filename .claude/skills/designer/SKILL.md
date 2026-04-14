---
name: designer
description: Use this skill when building Tauri 2 views or Svelte 5 components, or when auditing UI code for design-system consistency. It enforces a Mechanical Precision steampunk aesthetic using project Tailwind tokens, shadcn composition patterns, and Svelte 5 runes.
---

## Purpose
Use this skill to design or audit Svelte 5 UI code against the Rusty Shed design system.

Primary goals:
1. Enforce token-first styling (Tailwind theme variables, not ad-hoc values).
2. Preserve shadcn-style composition (`cn`, variants, prop passthrough, accessibility).
3. Keep Svelte 5 runes patterns idiomatic and predictable.
4. Maintain the Mechanical Precision visual language.

## When To Apply
Apply this skill for:
1. New pages, cards, drawers, forms, dialogs, tables, or navigation components.
2. Refactoring legacy or inconsistent classes.
3. Code review and design-system audits.
4. Any task that touches `.svelte` UI files.

## Audit Workflow (Mandatory)
Run this four-part check every time.

### 1. Token Analysis
Verify all visual values map to tokens already defined in `src/routes/layout.css`, `src/lib/themes/*.css`, or utility classes.

Hard rules:
1. No hex/rgb/hsl literals in component markup or local style blocks.
2. No random radii, spacing, or font-size magic numbers unless a documented exception is required.
3. Prefer semantic classes (`bg-card`, `border-border`, `text-muted-foreground`, `ring-primary`) over hardcoded values.
4. Prefer existing steampunk utility variants (`variant-steampunk-riveted`, `variant-steampunk-lever`, `variant-steampunk-gauge`, `variant-steampunk-valve`) before inventing new effects.

Magic-number flags to report:
1. Arbitrary text sizes such as `text-[9px]` and `text-[10px]` when `text-xs`/`text-sm` can work.
2. Arbitrary dimensions such as `w-[72px]` unless tied to a true asset constraint.
3. Hardcoded color classes outside the token set (example: `bg-amber-600`, `focus:border-primary-500`).
4. Local CSS with hardcoded borders/radius (example: `border: 1px solid #...`, `border-radius: 8px`).

### 2. Composition And Reuse
Check whether class composition is override-safe and reusable.

Required pattern:
1. Use `cn()` or `twMerge()` to merge base classes with external `class` overrides.
2. For multi-variant components, use variant factories (`tv`/CVA-style) instead of branching class strings inline.
3. Keep components focused; extract repeatable sections into snippets/subcomponents when they combine structure + styling + behavior.
4. Avoid direct string interpolation for classes like `class="... {className}"` in reusable primitives; use merge utilities.

Refactor signal:
1. Over-engineered if state and props exceed what the visual primitive should own.
2. Under-engineered if layout and style primitives are copy-pasted across files.

### 3. Svelte 5 Runes Patterns
Ensure reactive behavior is runes-native and stable.

Required pattern:
1. Use `$props()` for incoming props and prop defaults.
2. Use `$state` for mutable local UI state.
3. Use `$derived` for computed values/class outputs.
4. Use `$effect` only for true side effects (subscriptions, sync, async boundaries), not routine derivation.
5. Use `{#snippet}` for repeated UI fragments (rows, metadata cells, pill badges).

Performance and correctness checks:
1. No unnecessary `$effect` chains that mutate data derived from props.
2. Avoid derived values recreated inside event handlers when they can be declared once.
3. Keep async UI flows explicit with loading/error states and predictable cleanup.

### 4. shadcn Adherence
Maintain standard shadcn-svelte conventions.

Required checks:
1. Variants are centralized and named (`variant`, `size`, state classes) rather than ad-hoc per call site.
2. Base primitives keep accessibility states (`focus-visible`, `disabled`, `aria-*`, keyboard navigation).
3. Wrapper components forward rest props and support external class overrides.
4. Interactive primitives use semantic elements (`button`, `a`, form controls) and proper attributes.

## Mechanical Precision Visual Direction
Use a three-panel dashboard shell when creating full views:
`[Sidebar | Main Content | Command Center]`

Surface system:
1. Base app: `bg-background`.
2. Elevated panels: `bg-card border border-border rounded-sm`.
3. Nested groups: `bg-background/50 border border-border`.

Typography:
1. Headings: `font-bebas uppercase tracking-widest`.
2. Data values: `font-mono`.
3. Labels: `text-xs uppercase tracking-wider text-muted-foreground`.

Interaction language:
1. Primary actions: `bg-primary text-primary-foreground`.
2. Secondary actions: `border border-primary text-primary hover:bg-primary/10`.
3. Active navigation rows: `bg-primary/15` plus left accent border in `border-primary`.
4. Motion: `transition-all duration-150 ease-out`.

## Accessibility Guardrails
1. Icon-only controls must have accessible labels.
2. Focus states must remain visible in both themes.
3. Contrast must be checked for text on `bg-card`, `bg-background`, and `bg-primary`.
4. Empty states should communicate next action clearly, not just decorative text.

## Output Expectations For Audits
When returning audit feedback, structure findings in this order:
1. Token drift issues (highest severity first).
2. Composition/reuse issues.
3. Svelte 5 runes issues.
4. shadcn/accessibility issues.

For each finding include:
1. What is wrong.
2. Why it violates the system.
3. Exact replacement pattern using existing tokens/utilities.

## Repo-Specific Drift To Catch Early
Prioritize detection of these recurring issues:
1. Hardcoded color values in CSS utility classes for feedback components.
2. Arbitrary tiny typography values for labels where token sizes should be used.
3. Mixed radius language (`rounded-sm` mixed with `rounded-lg/xl/2xl`) inside the same visual family.
4. Token bypass classes like `*-500` not present in the current semantic theme model.
