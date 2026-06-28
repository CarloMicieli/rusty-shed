# Quickstart: Mobile Redesign

## Goal

Deliver the mobile redesign for viewports below 768 px with zero desktop regressions, bounded sheet interactions, robust camera fallback, and smooth bottom-sheet performance.

## Prerequisites

- Branch: `043-mobile-redesign`
- `pnpm` installed
- Rust toolchain available for Tauri checks
- Existing i18n message catalogs (`messages/en.json`, `messages/it.json`)

## Implementation Sequence

1. Foundation setup
1. Add safe-area and touch-hover Tailwind v4 utilities in `src/routes/layout.css`.
1. Update root/main mobile bottom padding calculations to include safe-area inset.
1. Add shared media-query helper for non-CSS toggles only.

1. Drawer/sheet state guardrails
1. Create bounded `DrawerRegistry` state utility with max depth 2.
1. Wire sheet dismissal order for button, gesture, and mobile back behavior.
1. Ensure route/back navigation pops overlays before page-level navigation.

1. Component-level migration
1. Update `DrawerShell` to mobile bottom-sheet pattern under `<768px` while preserving desktop side-panel behavior from `md` upward.
1. Update high-traffic preview card and filter/toggle touch targets to policy thresholds.
1. Apply compositor-safe sheet animation classes and reduced-motion fallback.

1. Route/milestone rollout
1. App shell and mobile navigation: keep 4 primary tabs + More, expose Settings/Debug at top of More.
1. Collection route: enforce `itemMinWidth=320`, show contextual FAB, hide mobile-inappropriate table toggle.
1. Detail/edit routes: disable inline mobile edits and route edits through sheet flow.
1. Supplementary sheets: convert MoreMenu to full-width bottom sheet and preserve desktop behavior.

1. Media fallback hardening
1. Probe camera capability in guarded flow where available.
1. If capability is absent/denied/error, switch UX to upload/gallery mode and remove capture-specific attributes.
1. Keep user state intact across camera fallback transitions.

1. Localization and regression verification
1. Add/update all new text keys in both `messages/en.json` and `messages/it.json`.
1. Validate Italian truncation and spacing at 375 px.
1. Run desktop parity checks at `>=768px` across affected routes.

## Suggested Test Scope

1. Unit tests
1. `DrawerRegistry` stack bounds, dismiss order, and back behavior.
1. Media capability fallback state transitions.
1. Touch target policy helpers.

1. Component tests
1. `DrawerShell` mobile/desktop variant behavior by viewport.
1. MoreMenu structure with Settings/Debug top actions.
1. Collection card/filter controls meeting touch thresholds.

1. Route/integration tests
1. Collection mobile view enforces one-column and contextual FAB.
1. Detail edit flow uses sheets on mobile and preserves desktop behavior.
1. Hardware-back simulation closes child then parent sheet.

## Verification Commands

```bash
pnpm svelte-check
pnpm test:unit
pnpm lint
pnpm run rust:test
pnpm run rust:clippy -- -D warnings
```

If Tauri command or specta types are changed:

```bash
pnpm specta:generate
```

## Exit Criteria

- All FR-001 to FR-018 are covered by planned tasks and tests.
- Mobile behavior is active only below 768 px.
- No desktop-severity regressions are present.
- i18n and touch-target audits pass on 375 px baseline.

## Feature Execution Notes

- Phase 1 setup assets:
	- Test planning matrix in `docs/testing/mobile-redesign-test-matrix.md`.
	- Mobile viewport helper in `src/__tests__/helpers/mobileViewport.ts`.
	- Mobile interaction helper in `src/__tests__/helpers/mobileInteractions.ts`.
- Foundational tasks (safe-area and drawer registry) must complete before story-level work.
- At every story checkpoint, run targeted tests first, then update localized catalogs in both `messages/en.json` and `messages/it.json`.
