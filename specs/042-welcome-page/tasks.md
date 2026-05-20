# Tasks: Welcome Wizard First-Run Onboarding

**Input**: Design documents from `/specs/042-welcome-page/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/onboarding.openapi.yaml, quickstart.md

**Tests**: Test-specific tasks are intentionally omitted because the specification does not explicitly request TDD or dedicated new test suites.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare onboarding module structure and contract alignment docs.

- [ ] T001 Create onboarding feature directories in src/lib/features/onboarding/steps and src/lib/services/import/
- [ ] T002 Create onboarding state scaffold in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T003 [P] Add onboarding message key placeholders in messages/en.json and messages/it.json
- [ ] T004 [P] Add onboarding implementation notes section in specs/042-welcome-page/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared onboarding shell, persistence adapter, and navigation primitives used by all stories.

**CRITICAL**: Complete this phase before implementing story-specific behavior.

- [ ] T005 Implement guarded startup state branching in src/routes/+layout.svelte
- [ ] T006 Add onboarding completion derivation from settings firstRun in src/routes/+layout.svelte
- [ ] T007 Implement shared onboarding flow state and validation helpers in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T008 Create wizard frame component with step progress shell in src/lib/features/onboarding/WelcomeWizard.svelte
- [ ] T009 [P] Add settings persistence wrapper for onboarding payloads in src/lib/services/settings.ts
- [ ] T010 [P] Add onboarding completion handler to settings flow in src/lib/features/settings/SettingsState.svelte.ts

**Checkpoint**: Root app can choose onboarding shell vs main shell without rendering the dashboard first.

---

## Phase 3: User Story 1 - First-Run Detection (Priority: P1) 🎯 MVP

**Goal**: Show welcome wizard on first launch and bypass it after completion.

**Independent Test**: Launch with firstRun=true and verify welcome wizard renders; launch with firstRun=false and verify normal shell renders.

### Implementation for User Story 1

- [ ] T011 [US1] Mount WelcomeWizard from guarded branch in src/routes/+layout.svelte
- [ ] T012 [US1] Wire onComplete callback to clear onboarding branch in src/routes/+layout.svelte
- [ ] T013 [US1] Implement onboarding status bootstrap from settings initialize result in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T014 [US1] Add fallback error view state for failed onboarding status read in src/routes/+layout.svelte

**Checkpoint**: First-run detection and interception are functional and independently demonstrable.

---

## Phase 4: User Story 2 - Regional & Appearance Setup (Priority: P1)

**Goal**: Capture language, theme, and measurement unit in Step 1 and persist choices.

**Independent Test**: Complete Step 1 with non-default values and confirm persisted settings are applied after onboarding exit.

### Implementation for User Story 2

- [ ] T015 [US2] Implement Basics step UI (language + theme cards + unit selector) in src/lib/features/onboarding/steps/BasicsStep.svelte
- [ ] T016 [US2] Bind Step 1 controls to onboarding state with runes in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T017 [US2] Add Enter-key advance behavior for Step 1 in src/lib/features/onboarding/WelcomeWizard.svelte
- [ ] T018 [US2] Persist Step 1 preferences through update_settings path in src/lib/services/settings.ts
- [ ] T019 [P] [US2] Add localized Step 1 labels and helper text in messages/en.json and messages/it.json

**Checkpoint**: Step 1 captures and persists regional and appearance preferences.

---

## Phase 5: User Story 3 - Collector's Core Setup (Priority: P2)

**Goal**: Capture favorite scale and power method with keyboard-friendly controls and persist selections.

**Independent Test**: Complete Step 2 and verify scale + power method are saved and available in settings.

### Implementation for User Story 3

- [ ] T020 [US3] Implement Modeling step UI (scale grid, unit confirmation, power method group) in src/lib/features/onboarding/steps/ModelingStep.svelte
- [ ] T021 [US3] Add arrow-key navigation support for scale and power selections in src/lib/features/onboarding/steps/ModelingStep.svelte
- [ ] T022 [US3] Add Step 2 validation and continue guard in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T023 [US3] Persist Step 2 preferences through update_settings path in src/lib/services/settings.ts
- [ ] T024 [P] [US3] Add localized Step 2 labels, ratios, and hints in messages/en.json and messages/it.json

**Checkpoint**: Step 2 reliably captures collector configuration and persists it.

---

## Phase 6: User Story 4 - Archive/Sync Setup (Priority: P3)

**Goal**: Provide archive import, Google Drive restore entry point, and skip/start-fresh exit.

**Independent Test**: On Step 3, each action path (local import, drive restore, skip) exits onboarding safely and prevents double actions while busy.

### Implementation for User Story 4

- [ ] T025 [US4] Implement Import step action bays and busy lock UI in src/lib/features/onboarding/steps/ImportStep.svelte
- [ ] T026 [US4] Implement local archive picker integration using dialog plugin in src/lib/services/import/localImport.ts
- [ ] T027 [US4] Implement Google Drive restore orchestration stub in src/lib/services/import/googleDriveImport.ts
- [ ] T028 [US4] Wire Import step actions and disabled/back lock behavior in src/lib/features/onboarding/WelcomeWizard.svelte
- [ ] T029 [US4] Complete onboarding by setting firstRun=false after successful save in src/lib/features/onboarding/onboarding-state.svelte.ts
- [ ] T030 [P] [US4] Add localized Step 3 action copy and error messages in messages/en.json and messages/it.json

**Checkpoint**: Step 3 supports import/restore/skip paths and always allows safe entry into app shell.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, docs sync, and full workflow verification.

- [ ] T031 [P] Align onboarding contract notes with implemented commands in specs/042-welcome-page/contracts/onboarding.openapi.yaml
- [ ] T032 Reconcile onboarding UX copy consistency with mechanical precision tone in src/lib/features/onboarding/WelcomeWizard.svelte
- [ ] T033 Validate quickstart steps against final implementation in specs/042-welcome-page/quickstart.md
- [ ] T034 Run end-to-end verification checklist updates in specs/042-welcome-page/quickstart.md

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

- Build step component structure before wiring persistence.
- Add keyboard and validation behavior before completion flow wiring.
- Finalize localization keys after UI shape is stable.

## Parallel Opportunities

- Setup: T003 and T004 can run in parallel after T001/T002.
- Foundational: T009 and T010 can run in parallel while T008 is in progress.
- US2: T019 can run in parallel with T015-T018.
- US3: T024 can run in parallel with T020-T023.
- US4: T030 can run in parallel with T025-T029.

---

## Parallel Example: User Story 2

```bash
Task: "T015 [US2] Implement Basics step UI in src/lib/features/onboarding/steps/BasicsStep.svelte"
Task: "T019 [P] [US2] Add localized Step 1 labels in messages/en.json and messages/it.json"
```

## Parallel Example: User Story 3

```bash
Task: "T020 [US3] Implement Modeling step UI in src/lib/features/onboarding/steps/ModelingStep.svelte"
Task: "T024 [P] [US3] Add localized Step 2 labels in messages/en.json and messages/it.json"
```

## Parallel Example: User Story 4

```bash
Task: "T026 [US4] Implement local archive picker integration in src/lib/services/import/localImport.ts"
Task: "T027 [US4] Implement Google Drive restore orchestration stub in src/lib/services/import/googleDriveImport.ts"
Task: "T030 [P] [US4] Add localized Step 3 action copy in messages/en.json and messages/it.json"
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2.
2. Deliver Phase 3 (US1) first-run gating.
3. Validate startup interception and post-completion bypass.

### Incremental Delivery

1. Add US2 for foundational preferences.
2. Add US3 for collector-specific defaults.
3. Add US4 for import/restore/skip exits.
4. Finish with Phase 7 polish and docs reconciliation.

### Team Parallel Strategy

1. Developer A: Route guard and wizard frame (T005-T014).
2. Developer B: Step 1 and Step 2 UI/persistence (T015-T024).
3. Developer C: Step 3 import actions and completion flow (T025-T030).
