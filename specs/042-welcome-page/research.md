# Phase 0 Research: Welcome Wizard First-Run Onboarding

## Decision 1: Use guarded root layout for first-run interception

- Decision: Implement onboarding gating in `src/routes/+layout.svelte` before rendering the main app shell.
- Rationale: The root layout already performs async startup (`settingsState.initialize`, DB init, preload). Extending this to branch into onboarding avoids dashboard flash and keeps startup logic centralized.
- Alternatives considered:
  - Dedicated `/welcome` route redirect: rejected because route transition can still render shell briefly and increases navigation edge cases.
  - Modal on top of dashboard: rejected because heavyweight dashboard initialization should not run before first-run decision.

## Decision 2: Use `has_completed_onboarding` as canonical source of truth

- Decision: Use `has_completed_onboarding` as the canonical onboarding status key and set it to `true` only after onboarding completion succeeds.
- Rationale: The key has explicit semantic intent and avoids ambiguity of execution count style flags.
- Runtime rule: after migration, application reads and writes MUST use only `has_completed_onboarding`; `firstRun` is migration-input only and must not drive runtime branching.
- Alternatives considered:
  - Keep `firstRun` as canonical: rejected because it is semantically ambiguous for interrupted onboarding sessions.
  - Frontend localStorage-only flag: rejected because this app uses backend settings persistence and typed commands.
  - Compatibility note: if legacy `firstRun` is present, map it once during initialization to `has_completed_onboarding` and then persist only the canonical key.

## Decision 3: Persist Step 1 and Step 2 data via existing update settings command

- Decision: Save `language`, `theme`, `measureUnit`, `favouriteScale`, and `powerMethod` through `update_settings` invoke path.
- Rationale: Preserves architectural law of transport boundary and central validation; avoids untyped ad-hoc storage.
- Alternatives considered:
  - Save per-step directly in component-local storage: rejected due to fragmented state and no backend validation.
  - Delay all persistence until final step only: rejected because import step can branch into long-running operations and should not risk loss of prior user choices.

## Decision 4: Step 3 import actions are async action bays with state lock

- Decision: Provide three exit paths in Step 3: local archive import, Google Drive restore, and skip/start-fresh. Lock navigation and action buttons while an import task is running.
- Rationale: Prevents duplicate imports and inconsistent state transitions during heavy I/O operations.
- Alternatives considered:
  - Allow free step navigation during import: rejected due to race-condition risk.
  - Remove skip path: rejected as UX anti-pattern; users must never be blocked from entering app.

## Decision 5: Local import uses Tauri dialog plugin and backend parsing command

- Decision: Use `@tauri-apps/plugin-dialog` for file selection and pass path to backend import command for parsing and persistence. Supported formats are `.json` and `.db` only.
- Rationale: Native picker and backend parsing avoid large file handling inside webview and align with Tauri security/performance practices.
-- Alternatives considered:
  - Web `input type=file` with client parsing: rejected for memory/performance limitations and weaker desktop-native UX.
  - Frontend parsing: rejected because data integrity logic belongs in Rust domain/application layers.
  - CSV format: rejected because it poorly maps to relational collection structures.
  - ZIP archives: rejected to avoid extra decompression complexity during onboarding.

## Decision 6: Keyboard-first navigation and deterministic completion state

- Decision: `Enter` advances primary CTA on Steps 1-2; arrow keys control scale/power selection groups; completion always sets `has_completed_onboarding=true` after successful settings save.
- Rationale: Meets accessibility and deterministic completion expectations.
- Alternatives considered:
  - Mouse-only controls: rejected for accessibility and desktop usability.
  - Completion flag set before persistence: rejected to avoid marking onboarding complete if save fails.