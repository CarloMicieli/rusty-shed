# Tasks: Welcome Wizard First-Run Onboarding

**Input**: Design documents from `/specs/042-welcome-page/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/onboarding.openapi.yaml, quickstart.md

**Tests**: Tests are required for this feature per constitution and are included below by user story.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare onboarding module structure and documentation scaffolding.

- [X] T001 Create onboarding feature directories in src/lib/features/onboarding/steps and src/lib/services/import/
- [X] T002 Create onboarding state scaffold in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T003 [P] Add onboarding message key placeholders in messages/en.json and messages/it.json
- [X] T004 [P] Add onboarding implementation notes section in specs/042-welcome-page/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared onboarding shell, canonical status handling, and common state primitives.

**CRITICAL**: Complete this phase before implementing story-specific behavior.

- [X] T005 Implement guarded startup state branching in src/routes/+layout.svelte
- [X] T006 Add onboarding completion derivation from settings has_completed_onboarding in src/routes/+layout.svelte
- [X] T007 Implement shared onboarding flow state and validation helpers in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T008 Create wizard frame component with step progress shell in src/lib/features/onboarding/WelcomeWizard.svelte
- [X] T009 [P] Add settings persistence wrapper for onboarding payloads in src/lib/services/settings.ts
- [X] T010 [P] Add onboarding completion handler for has_completed_onboarding in src/lib/features/settings/SettingsState.svelte.ts

**Checkpoint**: Root app can choose onboarding shell vs main shell without rendering the dashboard first.

---

## Phase 3: User Story 1 - First-Run Detection (Priority: P1) 🎯 MVP

**Goal**: Show welcome wizard when onboarding is incomplete and bypass it after completion.

**Independent Test**: Launch with has_completed_onboarding=false and verify wizard renders; launch with has_completed_onboarding=true and verify main shell renders.

### Tests for User Story 1

- [X] T011 [P] [US1] Add unit test for onboarding route guard defaulting to wizard when has_completed_onboarding=false in src/__tests__/routes/layout.test.ts
- [X] T012 [P] [US1] Add integration test for Skip and Start Fresh setting has_completed_onboarding=true and mounting main shell in src/__tests__/features/onboarding/onboarding-gate.test.ts

### Implementation for User Story 1

- [X] T013 [US1] Mount WelcomeWizard from guarded branch in src/routes/+layout.svelte
- [X] T014 [US1] Wire onComplete callback to clear onboarding branch in src/routes/+layout.svelte
- [X] T015 [US1] Implement onboarding status bootstrap from settings initialize result in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T016 [US1] Add fallback error view state for failed onboarding status read in src/routes/+layout.svelte

**Checkpoint**: First-run detection and interception are functional and independently demonstrable.

---

## Phase 4: User Story 2 - Regional & Appearance Setup (Priority: P1)

**Goal**: Capture language and theme in Step 1 and persist choices.

**Independent Test**: Complete Step 1 with non-default values and confirm persisted settings are applied after onboarding exit.

### Tests for User Story 2

- [X] T017 [P] [US2] Add component test for Step 1 language/theme interactions in src/__tests__/features/onboarding/WelcomeWizard.test.ts

### Implementation for User Story 2

- [X] T018 [US2] Implement Basics step UI (language + theme cards) in src/lib/features/onboarding/steps/BasicsStep.svelte
- [X] T019 [US2] Bind Step 1 controls to onboarding state with runes in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T020 [US2] Add Enter-key advance behavior for Step 1 in src/lib/features/onboarding/WelcomeWizard.svelte
- [X] T021 [US2] Persist Step 1 preferences through update_settings path in src/lib/services/settings.ts
- [X] T022 [P] [US2] Add localized Step 1 labels and helper text in messages/en.json and messages/it.json

**Checkpoint**: Step 1 captures and persists regional and appearance preferences.

---

## Phase 5: User Story 3 - Collector's Core Setup (Priority: P2)

**Goal**: Capture favorite scale, measurement unit, and power method with keyboard-friendly controls and persist selections.

**Independent Test**: Complete Step 2 and verify scale, measurement unit, and power method are saved and available in settings.

### Tests for User Story 3

- [X] T023 [P] [US3] Add component test for Step 2 keyboard selection flow in src/__tests__/features/onboarding/WelcomeWizard.test.ts

### Implementation for User Story 3

- [X] T024 [US3] Implement Modeling step UI (scale grid, unit selector, power method group) in src/lib/features/onboarding/steps/ModelingStep.svelte
- [X] T025 [US3] Add arrow-key navigation support for scale and power selections in src/lib/features/onboarding/steps/ModelingStep.svelte
- [X] T026 [US3] Add Step 2 validation and continue guard in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T027 [US3] Persist Step 2 preferences through update_settings path in src/lib/services/settings.ts
- [X] T028 [P] [US3] Add localized Step 2 labels, ratios, and hints in messages/en.json and messages/it.json

**Checkpoint**: Step 2 reliably captures collector configuration and persists it.

---

## Phase 6: User Story 4 - Archive/Sync Setup (Priority: P3)

**Goal**: Provide local archive import, full Google Drive restore path, and skip/start-fresh exit.

**Independent Test**: On Step 3, local import, drive restore, and skip each complete safely with busy-state locking and inline error feedback.

### Tests for User Story 4

- [X] T029 [P] [US4] Add integration test for Step 3 busy-lock behavior during import in src/__tests__/features/onboarding/WelcomeWizard.test.ts
- [X] T030 [P] [US4] Add integration test for inline Google Drive failure banner without wizard reset in src/__tests__/features/onboarding/WelcomeWizard.test.ts

### Implementation for User Story 4

- [X] T031 [US4] Implement Import step action bays and busy lock UI in src/lib/features/onboarding/steps/ImportStep.svelte
- [X] T032 [US4] Implement local archive picker integration for .json/.db using dialog plugin in src/lib/services/import/localImport.ts
- [X] T033 [US4] Implement Google Drive OAuth launch and callback token handling in src/lib/services/import/googleDriveImport.ts
- [X] T034 [US4] Implement Google Drive backup listing filtered to supported backup extensions in src/lib/services/import/googleDriveImport.ts
- [X] T035 [US4] Implement inline Step 3 failure banner for auth/download/restore failures in src/lib/features/onboarding/steps/ImportStep.svelte
- [X] T036 [US4] Wire Import step actions and disabled/back lock behavior in src/lib/features/onboarding/WelcomeWizard.svelte
- [X] T037 [US4] Complete onboarding by setting has_completed_onboarding=true after successful save in src/lib/features/onboarding/onboarding-state.svelte.ts
- [X] T038 [P] [US4] Add localized Step 3 action copy and error messages in messages/en.json and messages/it.json

**Checkpoint**: Step 3 supports import/restore/skip paths and always allows safe entry into app shell.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, performance verification, and workflow validation.

- [X] T039 [P] Verify startup gating performance (<100ms initialization surface mount) in src/__tests__/routes/layout.test.ts
- [X] T040 Verify dashboard remains unmounted until onboarding status resolves in src/__tests__/routes/layout.test.ts
- [X] T041 Apply transform-based step transition mechanics (transition-transform duration-150 ease-out) in src/lib/features/onboarding/WelcomeWizard.svelte
- [X] T042 Validate transition smoothness and no stutter under normal interaction in src/__tests__/features/onboarding/WelcomeWizard.test.ts
- [X] T043 [P] Align onboarding contract notes with implemented commands in specs/042-welcome-page/contracts/onboarding.openapi.yaml
- [X] T044 Validate quickstart steps against final implementation in specs/042-welcome-page/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup (Phase 1): No dependencies.
- Foundational (Phase 2): Depends on Setup completion; blocks all user stories.
- User Stories (Phases 3-6): Depend on Foundational completion.
- Polish (Phase 7): Depends on completion of all targeted user stories.

### User Story Dependencies

- US1 (P1): Starts after Foundational; no user-story dependency.
- US2 (P1): Starts after Foundational; independent of US1 behavior except shared wizard shell.
- US3 (P2): Starts after Foundational; independent of US2 except shared state container.
- US4 (P3): Starts after Foundational; depends on shared state and wizard frame from Phase 2.

### Within Each User Story

- Tests are authored before implementation and should fail first.
- Build step component structure before wiring persistence.
- Add keyboard and validation behavior before completion flow wiring.
- Finalize localization keys after UI shape is stable.

## Parallel Opportunities

- Setup: T003 and T004 can run in parallel after T001/T002.
- Foundational: T009 and T010 can run in parallel while T008 is in progress.
- US1 tests: T011 and T012 can run in parallel.
- US2: T017 and T022 can run in parallel with T018-T021.
- US3: T023 and T028 can run in parallel with T024-T027.
- US4: T029, T030, and T038 can run in parallel with T031-T037.

---

## Parallel Example: User Story 4

```bash
Task: "T033 [US4] Implement Google Drive OAuth launch and callback token handling in src/lib/services/import/googleDriveImport.ts"
Task: "T034 [US4] Implement Google Drive backup listing filtered to supported backup extensions in src/lib/services/import/googleDriveImport.ts"
Task: "T038 [P] [US4] Add localized Step 3 action copy and error messages in messages/en.json and messages/it.json"
```

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2.
2. Deliver Phase 3 (US1) first-run gating with mandatory tests.
3. Validate startup interception and post-completion bypass.

### Incremental Delivery

1. Add US2 for regional/appearance preferences.
2. Add US3 for collector-specific defaults.
3. Add US4 for import/restore/skip exits.
4. Finish with Phase 7 performance and contract validation.
