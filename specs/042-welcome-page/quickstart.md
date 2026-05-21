# Quickstart: Welcome Wizard First-Run Onboarding

## Goal

Implement a first-run onboarding wizard that intercepts app shell rendering when onboarding is not complete, then collects settings in 3 steps and exits via import/restore or start-fresh.

## Prerequisites

- `pnpm` installed
- Rust toolchain available for Tauri commands/tests
- Branch: `042-welcome-page`

## Implementation Steps

1. Add guarded shell logic in `src/routes/+layout.svelte`.
1. During startup, await settings initialization and derive onboarding requirement from `settingsState.settings.has_completed_onboarding`.
1. While checking, render a neutral initialization surface (no dashboard mount).
1. If onboarding is required, render `WelcomeWizard` instead of main shell.
1. Create onboarding feature module under `src/lib/features/onboarding/`.
1. Implement Step 1 (language + theme) with shadcn controls and keyboard support.
1. Implement Step 2 (scale + measure unit + power method) with keyboard support.
1. Implement Step 3 action bays:
   - Local archive import via `@tauri-apps/plugin-dialog` for `.json` or `.db` + backend import command.
   - Google Drive restore flow (OAuth, filtered backup selection, restore execution).
   - Skip and start fresh action.
1. Persist selected settings through existing settings update command before completion.
1. Mark onboarding complete by setting `has_completed_onboarding = true`.
1. Return to main shell and continue normal startup flow.
1. Add/update Paraglide keys in both `messages/en.json` and `messages/it.json`.

## Validation Checklist

1. First launch (`has_completed_onboarding=false`) shows onboarding instead of dashboard.
1. Returning launch (`has_completed_onboarding=true`) bypasses onboarding.
1. Step transitions are smooth and keyboard operable (`Enter`, arrow-key groups).
1. During import, action and nav controls are disabled.
1. Skip path always succeeds and enters app without import.
1. No hardcoded user-facing strings.

## Suggested Test Scope

1. Route/layout tests for guard behavior in `src/__tests__/routes/layout.test.ts`.
1. Component tests for wizard steps and busy-state locking.
1. Contract tests for import command success/failure paths.

## Verification Commands

```bash
pnpm svelte-check
pnpm test:unit
pnpm run rust:test
pnpm run rust:clippy -- -D warnings
```

If Rust commands or transport types are changed, regenerate bindings:

```bash
pnpm specta:generate
```

## Implementation Notes

- Canonical onboarding status key is `has_completed_onboarding`; legacy `firstRun` is migration-only input.
- Keep Step 1 focused on language and theme, and Step 2 focused on scale, measure unit, and power method.
- Restrict local archive import formats to `.json` and `.db`.
- Keep all onboarding copy in Paraglide keys for both English and Italian message catalogs.

## Implemented Command Flow Validation

- Local archive path uses `onboarding_import_local` with `{ filePath, formatHint }`.
- Google Drive path resolves OAuth code with `onboarding_get_google_drive_oauth_code`, then lists backups via `onboarding_list_google_drive_backups`, and restores via `onboarding_import_google_drive`.
- Onboarding completion writes `has_completed_onboarding` through settings update input `hasCompletedOnboarding`.