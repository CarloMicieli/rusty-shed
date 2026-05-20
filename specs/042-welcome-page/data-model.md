# Data Model: Welcome Wizard First-Run Onboarding

## Entity: OnboardingPreferenceSet

Represents user selections captured in Steps 1 and 2.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `language` | `"en" \| "it"` | Yes | Must be one of supported Paraglide locales | Step 1 |
| `theme` | `AppTheme` | Yes | Must match supported theme tokens | Step 1 |
| `measureUnit` | `"Metric" \| "Imperial"` | Yes | Enum constraint | Step 2 |
| `favouriteScale` | `Scale` | Yes | Must match backend-supported scale enum/string set | Step 2 |
| `powerMethod` | `"DC" \| "DCC" \| "AC"` | Yes | Enum constraint; map to backend variant names as needed | Step 2 |

Relationships:

- Persisted to existing `UserSettings` aggregate through `update_settings` command.

## Entity: OnboardingStatus

Tracks whether onboarding must be shown.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `has_completed_onboarding` | `boolean` | Yes | Boolean only | `true` means onboarding is complete |
| `completedAt` | `string \| null` | No | ISO-8601 datetime if present | Optional audit metadata (future-compatible) |

Relationships:

- Derived from settings state loaded at app startup.
- Transitioned when onboarding completes successfully.

## Entity: ImportActionRequest

Represents Step 3 chosen restore path.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `sourceType` | `"local_archive" \| "google_drive" \| "skip"` | Yes | Enum constraint | Step 3 action selection |
| `filePath` | `string \| null` | Conditional | Required when `sourceType=local_archive` | Provided by Tauri dialog plugin |
| `startedAt` | `string` | Yes | ISO-8601 datetime | Telemetry/debug value |

Relationships:

- `local_archive` delegates to backend import command(s).
- `google_drive` delegates to OAuth + restore command(s).
- `skip` bypasses import and completes onboarding.

## Entity: OnboardingFlowState (frontend transient)

Svelte state for wizard rendering and interaction locks.

| Field | Type | Required | Validation | Notes |
| --- | --- | --- | --- | --- |
| `currentStep` | `1 \| 2 \| 3` | Yes | Bounded integer | Controls visible panel |
| `isBusy` | `boolean` | Yes | Boolean | Locks nav/actions during async work |
| `errorMessage` | `string \| null` | No | Localized message key or mapped string | Error feedback |

## State Transitions

1. `StartupChecking` -> `NeedsOnboarding` when `has_completed_onboarding=false` or `null`.
2. `StartupChecking` -> `AppReady` when `has_completed_onboarding=true`.
3. `NeedsOnboarding` + valid Step 1/2 selections -> `Step3Ready`.
4. `Step3Ready` + import action -> `ImportInProgress`.
5. `ImportInProgress` + success -> `Completing` (persist settings + set `has_completed_onboarding=true`).
6. `Step3Ready` + skip -> `Completing` (persist settings + set `has_completed_onboarding=true`).
7. `Completing` -> `AppReady`.
8. Any async failure -> `NeedsOnboarding` with actionable error and `isBusy=false`.

## Validation Rules Summary

- Onboarding cannot complete unless required Step 1 and Step 2 fields are valid.
- `has_completed_onboarding` flips to `true` only after settings persistence succeeds.
- During `isBusy=true`, back/continue/import buttons are disabled.
- Import operations must be idempotent or guarded against duplicate trigger from UI.