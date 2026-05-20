# Phase 0 Research: Welcome Wizard First-Run Onboarding

## Decision 1: Use guarded root layout for first-run interception

- Decision: Implement onboarding gating in `src/routes/+layout.svelte` before rendering the main app shell.
- Rationale: The root layout already performs async startup (`settingsState.initialize`, DB init, preload). Extending this to branch into onboarding avoids dashboard flash and keeps startup logic centralized.
- Alternatives considered:
  - Dedicated `/welcome` route redirect: rejected because route transition can still render shell briefly and increases navigation edge cases.
  - Modal on top of dashboard: rejected because heavyweight dashboard initialization should not run before first-run decision.

## Decision 2: Use existing `firstRun` settings field as source of truth

- Decision: Treat existing `settingsState.settings.firstRun` as canonical first-run flag, and map completion by setting it to `false`.
- Rationale: Existing settings model and backend commands already include `firstRun`; avoids transport/schema drift and extra migration risk.
- Alternatives considered:
  - Introduce new `has_completed_onboarding` key: rejected for now because it duplicates existing semantics and increases compatibility overhead.
  - Frontend localStorage-only flag: rejected because this app already uses backend settings persistence and typed commands.

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

- Decision: Use `@tauri-apps/plugin-dialog` for file selection and pass path to backend import command for parsing and persistence.
- Rationale: Native picker and backend parsing avoid large file handling inside webview and align with Tauri security/performance practices.
- Alternatives considered:
  - Web `input type=file` with client parsing: rejected for memory/performance limitations and weaker desktop-native UX.
  - Frontend CSV/JSON parsing: rejected because data integrity logic belongs in Rust domain/application layers.

## Decision 6: Keyboard-first navigation and deterministic completion state

- Decision: `Enter` advances primary CTA on Steps 1-2; arrow keys control scale/power selection groups; completion always sets `firstRun=false` after successful settings save.
- Rationale: Meets accessibility and deterministic completion expectations.
- Alternatives considered:
  - Mouse-only controls: rejected for accessibility and desktop usability.
  - Completion flag set before persistence: rejected to avoid marking onboarding complete if save fails.