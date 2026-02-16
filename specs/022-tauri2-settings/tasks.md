# Tasks: Migrate Tauri 2 Settings

**Input**: Design documents from `/specs/022-tauri2-settings/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/settings-ipc.md

**Tests**: Included (FR-018 and FR-019 require unit and integration tests)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

This is a Tauri 2 desktop application with:

- **Backend**: `src-tauri/src/` (Rust)
- **Frontend**: `src/lib/` and `src/routes/` (SvelteKit)
- **Tests**: `src/__tests__/` (frontend), `src-tauri/src/settings/tests/` (backend)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and plugin configuration

- [x] T001 Add tauri-plugin-store v2.0 dependency to src-tauri/Cargo.toml
- [x] T002 [P] Add tauri-plugin-window-state v2.0 dependency to src-tauri/Cargo.toml
- [x] T003 [P] Add tauri-plugin-os v2.0 dependency to src-tauri/Cargo.toml
- [x] T004 [P] Add validator crate dependency to src-tauri/Cargo.toml for settings validation
- [x] T005 Initialize tauri-plugin-store in src-tauri/src/main.rs using StoreBuilder
- [x] T006 [P] Initialize tauri-plugin-window-state in src-tauri/src/main.rs
- [x] T007 [P] Initialize tauri-plugin-os in src-tauri/src/main.rs
- [x] T008 Create settings module structure: src-tauri/src/settings/mod.rs
- [x] T009 [P] Create domain layer structure: src-tauri/src/settings/domain/mod.rs
- [x] T010 [P] Create application layer structure: src-tauri/src/settings/application/mod.rs
- [x] T011 [P] Create infrastructure layer structure: src-tauri/src/settings/infrastructure/mod.rs
- [x] T012 [P] Create interface layer structure: src-tauri/src/settings/interface/mod.rs
- [x] T013 Register settings module in src-tauri/src/lib.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T014 [P] Create Language enum in src-tauri/src/settings/domain/user_settings.rs with English/Italian variants, derive Serialize, Deserialize, Type
- [x] T015 [P] Create MeasureUnit enum in src-tauri/src/settings/domain/user_settings.rs with Metric/Imperial variants
- [x] T016 [P] Create PowerSystem enum in src-tauri/src/settings/domain/user_settings.rs with DC/AC/DCC variants
- [x] T017 Create UserSettings struct in src-tauri/src/settings/domain/user_settings.rs with all 6 fields (currency, language, measure_unit, favourite_scale, power_system, first_run)
- [x] T018 Add Default implementation for UserSettings in src-tauri/src/settings/domain/user_settings.rs (EUR currency, English language, Metric, empty scale, DC, first_run=true)
- [x] T019 [P] Add settings-related Paraglide message keys to messages/en.json (settings page labels, field names, save button, error messages)
- [x] T020 [P] Add settings-related Paraglide message keys to messages/it.json (Italian translations)
- [x] T021 Run pnpm prepare to compile Paraglide messages
- [x] T022 Configure specta type generation for settings types (if not already configured in project)
- [x] T023 Create frontend settings feature directory: src/lib/features/settings/

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Configure Application Preferences (Priority: P1) 🎯 MVP

**Goal**: Users can view and modify all 5 settings (currency, language, measure unit, favourite scale, power system) with immediate reactive updates and persistence across restarts

**Independent Test**: Open Settings page → Change each setting → Verify immediate UI update → Restart app → Verify all settings persisted

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T024 [P] [US1] Create unit test file src-tauri/src/settings/domain/tests.rs for UserSettings validation
- [x] T025 [P] [US1] Write test for UserSettings default values in src-tauri/src/settings/domain/tests.rs
- [x] T026 [P] [US1] Write test for currency validation (empty string fails, 1-10 chars passes) in src-tauri/src/settings/domain/tests.rs
- [x] T027 [P] [US1] Write test for favourite_scale validation (max 20 chars) in src-tauri/src/settings/domain/tests.rs
- [x] T028 [P] [US1] Create integration test file src-tauri/src/settings/tests/integration_tests.rs for IPC commands
- [x] T029 [P] [US1] Write integration test for get_settings command in src-tauri/src/settings/tests/integration_tests.rs
- [x] T030 [P] [US1] Write integration test for update_settings command in src-tauri/src/settings/tests/integration_tests.rs
- [x] T031 [P] [US1] Create frontend unit test file src/**tests**/unit/settings/settings_state.test.ts
- [x] T032 [P] [US1] Write test for SettingsState.load() in src/**tests**/unit/settings/settings_state.test.ts with mocked invoke
- [x] T033 [P] [US1] Write test for SettingsState.update() in src/**tests**/unit/settings/settings_state.test.ts with mocked invoke

### Backend Implementation for User Story 1

- [x] T034 [US1] Add validation methods to UserSettings in src-tauri/src/settings/domain/user_settings.rs (validate_currency, validate_favourite_scale)
- [x] T035 [US1] Implement validate() method on UserSettings in src-tauri/src/settings/domain/user_settings.rs that checks all invariants
- [x] T036 [US1] Create SettingsRepository trait in src-tauri/src/settings/infrastructure/mod.rs with load() and save() methods
- [x] T037 [US1] Implement StoreSettingsRepository struct in src-tauri/src/settings/infrastructure/store_repository.rs using tauri-plugin-store
- [x] T038 [US1] Implement SettingsRepository::load() in src-tauri/src/settings/infrastructure/store_repository.rs to read from store.get("user_settings")
- [x] T039 [US1] Implement SettingsRepository::save() in src-tauri/src/settings/infrastructure/store_repository.rs to write with store.set() and store.save()
- [x] T040 [US1] Add error handling for corrupted settings in SettingsRepository::load() (fallback to defaults)
- [x] T041 [US1] Create get_settings use case in src-tauri/src/settings/application/get_settings.rs
- [x] T042 [US1] Implement get_settings logic: load from repository, return UserSettings or error
- [x] T043 [US1] Create UpdateSettingsInput struct in src-tauri/src/settings/application/update_settings.rs with Option fields
- [x] T044 [US1] Create update_settings use case in src-tauri/src/settings/application/update_settings.rs
- [x] T045 [US1] Implement update_settings logic: load current → merge updates → validate → save → return updated settings
- [x] T046 [US1] Create UpdateSettingsArgs DTO in src-tauri/src/settings/interface/commands.rs with validator derives
- [x] T047 [US1] Implement get_settings IPC command handler in src-tauri/src/settings/interface/commands.rs with #[tauri::command] and #[specta::specta]
- [x] T048 [US1] Implement update_settings IPC command handler in src-tauri/src/settings/interface/commands.rs with args validation
- [x] T049 [US1] Register get_settings and update_settings commands in src-tauri/src/main.rs invoke_handler
- [x] T050 [US1] Generate TypeScript types for UserSettings and UpdateSettingsArgs using specta

### Frontend Implementation for User Story 1

- [x] T051 [P] [US1] Create SettingsState class in src/lib/features/settings/SettingsState.svelte.ts with $state rune for settings
- [x] T052 [US1] Implement SettingsState.load() method in src/lib/features/settings/SettingsState.svelte.ts to call invoke('get_settings')
- [x] T053 [US1] Implement SettingsState.update() method in src/lib/features/settings/SettingsState.svelte.ts to call invoke('update_settings')
- [x] T054 [US1] Export singleton settingsState instance in src/lib/features/settings/SettingsState.svelte.ts
- [x] T055 [P] [US1] Create LanguageSelector component in src/lib/features/settings/components/LanguageSelector.svelte
- [x] T056 [P] [US1] Create CurrencySelector component in src/lib/features/settings/components/CurrencySelector.svelte (text input)
- [x] T057 [P] [US1] Create MeasureUnitSelector component in src/lib/features/settings/components/MeasureUnitSelector.svelte
- [x] T058 [P] [US1] Create ScaleSelector component in src/lib/features/settings/components/ScaleSelector.svelte (text input)
- [x] T059 [P] [US1] Create PowerSystemSelector component in src/lib/features/settings/components/PowerSystemSelector.svelte
- [x] T060 [US1] Create SettingsForm component in src/lib/features/settings/components/SettingsForm.svelte that uses all 5 selectors
- [x] T061 [US1] Add form submission logic to SettingsForm.svelte that calls settingsState.update()
- [x] T062 [US1] Add error handling and toast notifications to SettingsForm.svelte
- [x] T063 [US1] Create Settings page route: src/routes/settings/+page.svelte
- [x] T064 [US1] Import and render SettingsForm in src/routes/settings/+page.svelte
- [x] T065 [US1] Add settings loading on mount in src/routes/settings/+page.svelte
- [x] T066 [US1] Add $effect in SettingsController to sync language changes with Paraglide setLanguageTag() for reactive UI updates
- [x] T067 [US1] Verify all settings components use Paraglide messages (no hardcoded strings)

### Validation for User Story 1

- [x] T068 [US1] Run cargo test in src-tauri to verify all backend unit tests pass
- [x] T069 [US1] Run pnpm test to verify all frontend unit tests pass
- [x] T070 [US1] Manual test: Open Settings page, change all 5 settings, verify immediate UI update
- [x] T071 [US1] Manual test: Restart application, verify all settings persisted
- [x] T072 [US1] Manual test: Change language setting, verify UI text updates without restart

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently - this is the MVP!

---

## Phase 4: User Story 2 - Automatic Language Detection on First Run (Priority: P2)

**Goal**: On first launch, automatically detect OS language and set to Italian/English (with English fallback), improving onboarding experience

**Independent Test**: Clear app data → Set OS to Italian → Launch app → Verify UI in Italian. Repeat with unsupported locale → Verify English fallback.

### Tests for User Story 2

- [x] T073 [P] [US2] Write unit test for OS locale parsing (e.g., "it-IT" → "it") in src-tauri/src/settings/infrastructure/tests.rs
- [x] T074 [P] [US2] Write unit test for language fallback logic (unsupported locale → English) in src-tauri/src/settings/infrastructure/tests.rs
- [x] T075 [P] [US2] Write integration test for initialize_settings with mocked OS locale in src-tauri/src/settings/tests/integration_tests.rs
- [x] T076 [P] [US2] Write integration test for first_run flag transition (true → false) in src-tauri/src/settings/tests/integration_tests.rs

### Backend Implementation for User Story 2

- [x] T077 [US2] Create os_language module in src-tauri/src/settings/infrastructure/os_language.rs
- [x] T078 [US2] Implement detect_os_language() function in src-tauri/src/settings/infrastructure/os_language.rs using tauri_plugin_os::locale()
- [x] T079 [US2] Implement parse_language_code() helper in src-tauri/src/settings/infrastructure/os_language.rs to extract language from locale (e.g., "it-IT" → Language::Italian)
- [x] T080 [US2] Add fallback logic to parse_language_code() for unsupported languages (default to Language::English)
- [x] T081 [US2] Create initialize_settings use case in src-tauri/src/settings/application/initialize_settings.rs
- [x] T082 [US2] Implement initialize_settings logic: check if settings exist → if not, detect OS language → create defaults with detected language → save → set first_run=true
- [x] T083 [US2] Add idempotency to initialize_settings (if settings already exist, return them unchanged)
- [x] T084 [US2] Implement initialize_settings IPC command handler in src-tauri/src/settings/interface/commands.rs
- [x] T085 [US2] Register initialize_settings command in src-tauri/src/main.rs invoke_handler

### Frontend Implementation for User Story 2

- [x] T086 [US2] Add initialization logic to app root layout: src/routes/+layout.svelte
- [x] T087 [US2] Call invoke('initialize_settings') in onMount of src/routes/+layout.svelte before loading other data
- [x] T088 [US2] Set initial Paraglide language based on initialized settings in src/routes/+layout.svelte
- [x] T089 [US2] Add error handling for initialization failures in src/routes/+layout.svelte (log and continue with defaults)

### Validation for User Story 2

- [x] T090 [US2] Run cargo test to verify OS language detection tests pass
- [x] T091 [US2] Manual test: Clear settings file → Set OS to Italian → Launch app → Verify UI in Italian
- [x] T092 [US2] Manual test: Clear settings file → Set OS to Spanish → Launch app → Verify UI in English (fallback)
- [x] T093 [US2] Manual test: Launch app second time → Verify language preference persisted (not re-detected from OS)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Window Position and Size Restoration (Priority: P3)

**Goal**: Application remembers window position and size across sessions, providing seamless continuation

**Independent Test**: Resize and move window → Restart app → Verify window appears at same position and size

### Implementation for User Story 3

**Note**: Most functionality is automatic via tauri-plugin-window-state, minimal code required

- [x] T094 [US3] Verify tauri-plugin-window-state is initialized in src-tauri/src/main.rs (already done in T006)
- [x] T095 [US3] Add window-state plugin configuration to src-tauri/tauri.conf.json if needed (check plugin docs)
- [x] T096 [US3] Test window state file location and verify it's created on first run

### Validation for User Story 3

- [x] T097 [US3] Manual test: Launch app → Move window to top-right corner → Close app → Relaunch → Verify window at top-right
- [x] T098 [US3] Manual test: Launch app → Resize window to small size → Close → Relaunch → Verify small size preserved
- [x] T099 [US3] Manual test: Launch app → Maximize window → Close → Relaunch → Verify window maximized
- [x] T100 [US3] Manual test: Move window to secondary monitor → Disconnect monitor → Relaunch → Verify window appears on primary monitor (edge case)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final quality checks

- [x] T101 [P] Run cargo fmt on src-tauri/ to format Rust code
- [x] T102 [P] Run cargo clippy on src-tauri/ and fix all warnings
- [x] T103 [P] Run pnpm format to format frontend code
- [x] T104 [P] Run pnpm lint and fix all linting issues
- [x] T105 Run pnpm check to verify TypeScript types across all settings components
- [x] T106 Run cargo test --all to verify all Rust tests pass
- [x] T107 Run pnpm test to verify all frontend tests pass
- [x] T108 Verify test coverage: Run cargo tarpaulin or coverage tool and ensure settings module has ≥80% coverage (SC-005)
- [x] T109 [P] Add rustdoc comments to all public types and functions in src-tauri/src/settings/
- [x] T110 [P] Add TSDoc comments to SettingsState and SettingsController in frontend
- [x] T111 Create optional reset_settings IPC command in src-tauri/src/settings/interface/commands.rs (factory reset feature)
- [x] T112 Add "Reset to Defaults" button to Settings page UI if reset_settings command implemented
- [x] T113 Run quickstart.md validation: Verify all code examples in specs/022-tauri2-settings/quickstart.md are accurate
- [x] T114 Update CLAUDE.md if new patterns emerged during implementation (optional)
- [x] T115 Final manual testing: Run through all 3 user stories end-to-end
- [x] T116 Verify FR-017: Audit all UI components to ensure zero hardcoded English strings (all use Paraglide)
- [x] T117 Performance check: Measure settings read/write latency (should be <200ms per technical context)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion (T001-T013) - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (T014-T023) - Core MVP
- **User Story 2 (Phase 4)**: Depends on Foundational (T014-T023) - Can run in parallel with US1 if staffed
- **User Story 3 (Phase 5)**: Depends on Setup (T006) - Mostly independent, can run in parallel with US1/US2
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: No dependencies on other stories - fully independent
- **User Story 2 (P2)**: Uses UserSettings from US1 but can be developed in parallel (shares foundational types)
- **User Story 3 (P3)**: Completely independent - plugin-based, no code dependencies

### Within Each User Story

**User Story 1**:

- Tests (T024-T033) can all run in parallel
- Domain types (T034-T035) before use cases
- Repository (T036-T040) in parallel with domain types
- Use cases (T041-T045) after domain + repository
- IPC commands (T046-T050) after use cases
- Frontend components (T055-T059) can all run in parallel
- Frontend integration (T060-T067) after components

**User Story 2**:

- Tests (T073-T076) can all run in parallel
- Backend implementation (T077-T085) sequential (OS detection → use case → IPC)
- Frontend implementation (T086-T089) sequential (layout integration)

**User Story 3**:

- Mostly validation, minimal implementation (already configured in Phase 1)

### Parallel Opportunities

**Setup Phase (T001-T013)**:

- All dependency additions (T001-T004) can run in parallel
- All plugin initializations (T005-T007) can run in parallel
- All module structure creation (T008-T012) can run in parallel

**Foundational Phase (T014-T023)**:

- Enums (T014-T016) can run in parallel
- Paraglide messages (T019-T020) can run in parallel

**User Story 1**:

- All tests (T024-T033): 10 parallel tasks
- Models (T034-T035) with Repository (T036-T040): 2 parallel tracks
- All frontend components (T055-T059): 5 parallel tasks

**Different User Stories**:

- US1 (T024-T072), US2 (T073-T093), US3 (T094-T100) can be worked on in parallel by different developers after Foundational phase completes

**Polish Phase**:

- Formatting/linting tasks (T101-T104) can run in parallel
- Documentation tasks (T109-T110) can run in parallel

---

## Parallel Example: User Story 1

```bash
# After Foundational phase completes, launch all US1 tests together:
Task T024: "Create unit test file src-tauri/src/settings/domain/tests.rs"
Task T025: "Write test for UserSettings default values"
Task T026: "Write test for currency validation"
Task T027: "Write test for favourite_scale validation"
Task T028: "Create integration test file src-tauri/src/settings/tests/integration_tests.rs"
Task T029: "Write integration test for get_settings"
Task T030: "Write integration test for update_settings"
Task T031: "Create frontend unit test file src/__tests__/unit/settings/settings_state.test.ts"
Task T032: "Write test for SettingsState.load()"
Task T033: "Write test for SettingsState.update()"

# Then launch all US1 frontend components together:
Task T055: "Create LanguageSelector component"
Task T056: "Create CurrencySelector component"
Task T057: "Create MeasureUnitSelector component"
Task T058: "Create ScaleSelector component"
Task T059: "Create PowerSystemSelector component"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T013)
2. Complete Phase 2: Foundational (T014-T023) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T024-T072)
4. **STOP and VALIDATE**: Test User Story 1 independently using acceptance scenarios from spec.md
5. Deploy/demo if ready - users can now configure all settings

**MVP Delivers**: Complete settings management with persistence and reactive updates

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP! - Core settings work)
3. Add User Story 2 → Test independently → Deploy/Demo (Enhanced onboarding with auto language)
4. Add User Story 3 → Test independently → Deploy/Demo (Full UX polish with window memory)
5. Polish phase → Final quality checks → Production ready

### Parallel Team Strategy

With multiple developers after Foundational phase:

**2 Developers**:

- Developer A: User Story 1 (T024-T072) - Core functionality
- Developer B: User Story 2 (T073-T093) + User Story 3 (T094-T100) - Enhancements

**3+ Developers**:

- Developer A: User Story 1 (T024-T072)
- Developer B: User Story 2 (T073-T093)
- Developer C: User Story 3 (T094-T100) + Polish setup (T101-T104)

Stories can complete independently and merge without conflicts (different files, different features).

---

## Task Execution Checklist

Before starting implementation:

- [ ] Review spec.md to understand all acceptance scenarios
- [ ] Review plan.md for architectural decisions and plugin usage
- [ ] Review data-model.md for entity definitions and validation rules
- [ ] Review contracts/settings-ipc.md for IPC command specifications
- [ ] Ensure all Tauri plugins are correctly versioned (v2.0)

During implementation:

- [ ] Write tests FIRST for each user story, ensure they FAIL
- [ ] Implement code to make tests PASS
- [ ] Run linting and formatting after each task group
- [ ] Commit after completing each user story phase
- [x] Test each user story independently before moving to next

After implementation:

- [ ] Verify all acceptance scenarios from spec.md
- [ ] Verify all success criteria (SC-001 through SC-007) are met
- [ ] Run full test suite and ensure ≥80% coverage
- [ ] Manual testing of all 3 user stories
- [ ] Performance validation (all operations <200ms)

---

## Notes

- **[P] tasks** = Different files, no dependencies, can run in parallel
- **[Story] label** = Maps task to specific user story for traceability
- Each user story should be independently completable and testable per spec.md
- **Tests are required** (FR-018, FR-019) - written before implementation (TDD)
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- **MVP = User Story 1 only** - delivers core value
- User Stories 2 and 3 are incremental enhancements
- Avoid: vague tasks, file conflicts, cross-story dependencies

---

## Summary

- **Total Tasks**: 117
- **Setup Tasks**: 13 (T001-T013)
- **Foundational Tasks**: 10 (T014-T023)
- **User Story 1 Tasks**: 49 (T024-T072) - MVP
- **User Story 2 Tasks**: 21 (T073-T093) - Enhanced onboarding
- **User Story 3 Tasks**: 7 (T094-T100) - Window state (mostly automatic)
- **Polish Tasks**: 17 (T101-T117)
- **Parallel Opportunities**: ~40 tasks marked [P] can run concurrently
- **Independent Stories**: All 3 user stories can be developed and tested independently
- **Test Coverage**: 24 test tasks (unit + integration) to achieve ≥80% coverage (SC-005)
