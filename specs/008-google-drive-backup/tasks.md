# Tasks: Google Drive Cloud Backup

**Input**: Design documents from `/specs/008-google-drive-backup/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Not explicitly requested in specification. Tests omitted per template guidelines.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1, US2, etc.) - only for story phases

## Path Conventions

- **Backend**: `src-tauri/src/` (Rust)
- **Frontend**: `src/lib/` (Svelte/TypeScript)
- **Commands**: `src-tauri/src/commands/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependencies, and module structure

- [x] T001 Add Rust dependencies to src-tauri/Cargo.toml (oauth2, google-drive3, keyring, secrecy, zeroize, online, flate2)
- [x] T002 Add Tauri plugins via `pnpm tauri add oauth deep-link stronghold`
- [x] T003 [P] Add frontend dependency `@tauri-apps/plugin-stronghold` to package.json
- [x] T004 [P] Create backend feature module structure at src-tauri/src/features/cloud_backup/mod.rs
- [x] T005 [P] Create frontend feature folder structure at src/lib/features/cloud-backup/index.ts
- [x] T006 [P] Add Paraglide messages for cloud backup feature to messages/en.json
- [x] T007 Register cloud_backup feature module in src-tauri/src/lib.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 Create domain error types in src-tauri/src/features/cloud_backup/domain/errors.rs
- [x] T009 Create GoogleConnection value object in src-tauri/src/features/cloud_backup/domain/connection.rs
- [x] T010 Create CloudBackup entity in src-tauri/src/features/cloud_backup/domain/backup.rs
- [x] T011 [P] Create domain module exports in src-tauri/src/features/cloud_backup/domain/mod.rs
- [x] T012 Implement SecureStorage trait abstraction in src-tauri/src/features/cloud_backup/infrastructure/secure_storage.rs
- [x] T013 Implement KeyringStorage (desktop) in src-tauri/src/features/cloud_backup/infrastructure/secure_storage.rs
- [x] T013b [P] Implement StrongholdStorage (Android) in src-tauri/src/features/cloud_backup/infrastructure/secure_storage.rs
- [x] T014 [P] Create infrastructure module exports in src-tauri/src/features/cloud_backup/infrastructure/mod.rs
- [x] T015 Create application module exports in src-tauri/src/features/cloud_backup/application/mod.rs
- [x] T016 Create Tauri command handlers file at src-tauri/src/commands/cloud_backup.rs
- [x] T017 Register cloud backup commands in src-tauri/src/lib.rs invoke_handler

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Connect Google Account (Priority: P1) 🎯 MVP

**Goal**: User can link their Google account via OAuth and see connected status

**Independent Test**: Click "Connect Google Drive," complete OAuth, verify email displayed

### Backend Implementation

- [x] T018 [US1] Implement OAuthService with PKCE flow in src-tauri/src/features/cloud_backup/infrastructure/oauth_service.rs
- [x] T019 [US1] Implement connect_google use case in src-tauri/src/features/cloud_backup/application/connect_google.rs
- [x] T020 [US1] Implement disconnect_google use case in src-tauri/src/features/cloud_backup/application/disconnect_google.rs
- [x] T021 [US1] Implement cloud_backup_connect_google command in src-tauri/src/commands/cloud_backup.rs
- [x] T022 [US1] Implement cloud_backup_disconnect_google command in src-tauri/src/commands/cloud_backup.rs
- [x] T023 [US1] Implement cloud_backup_get_connection_status command in src-tauri/src/commands/cloud_backup.rs

### Frontend Implementation

- [x] T024 [P] [US1] Create CloudBackupController class in src/lib/features/cloud-backup/controllers/cloudBackup.svelte.ts
- [x] T025 [US1] Create GoogleConnectButton component in src/lib/features/cloud-backup/components/GoogleConnectButton.svelte
- [x] T026 [US1] Add cloud backup section to src/routes/my-settings/+page.svelte
- [x] T027 [US1] Add connection status display showing email and disconnect option

**Checkpoint**: User Story 1 complete - OAuth flow works independently

---

## Phase 4: User Story 2 - Manual Backup Sync (Priority: P1) 🎯 MVP

**Goal**: User can upload collection database to Google Drive with progress feedback

**Independent Test**: Click "Sync Now," see progress, verify timestamp updates

### Backend Implementation

- [x] T028 [US2] Implement GoogleDriveClient in src-tauri/src/features/cloud_backup/infrastructure/google_drive.rs
- [x] T029 [US2] Implement create/get backup folder logic in google_drive.rs
- [x] T030 [US2] Implement file upload with resumable upload in google_drive.rs
- [x] T031 [US2] Implement database compression in src-tauri/src/features/cloud_backup/application/sync_backup.rs
- [x] T032 [US2] Implement sync_backup use case with progress events in sync_backup.rs
- [x] T033 [US2] Implement cloud_backup_sync_now command in src-tauri/src/commands/cloud_backup.rs
- [x] T034 [US2] Implement cloud_backup_get_sync_status command in src-tauri/src/commands/cloud_backup.rs

### Frontend Implementation

- [x] T035 [P] [US2] Create SyncButton component with progress indicator in src/lib/features/cloud-backup/components/SyncButton.svelte
- [x] T036 [US2] Add sync progress event listener to CloudBackupController
- [x] T037 [US2] Display "Last Successful Sync" timestamp in settings UI
- [x] T038 [US2] Add error handling and toast notifications for sync failures

**Checkpoint**: User Stories 1 & 2 complete - Core backup functionality works

---

## Phase 5: User Story 3 - Restore from Backup (Priority: P2)

**Goal**: User can view backup list, select a backup, and restore with confirmation

**Independent Test**: View backup list, select backup, type RESTORE, verify data replaced

### Backend Implementation

- [x] T039 [US3] Implement file listing in src-tauri/src/features/cloud_backup/infrastructure/google_drive.rs
- [x] T040 [US3] Implement file download in google_drive.rs
- [x] T041 [US3] Implement list_backups use case in src-tauri/src/features/cloud_backup/application/list_backups.rs
- [x] T042 [US3] Implement restore_backup use case with decompression in src-tauri/src/features/cloud_backup/application/restore_backup.rs
- [x] T043 [US3] Implement database replacement logic with safety backup in restore_backup.rs
- [x] T044 [US3] Implement cloud_backup_list_backups command in src-tauri/src/commands/cloud_backup.rs
- [x] T045 [US3] Implement cloud_backup_restore command with RESTORE validation in src-tauri/src/commands/cloud_backup.rs

### Frontend Implementation

- [x] T046 [P] [US3] Create BackupList component in src/lib/features/cloud-backup/components/BackupList.svelte
- [x] T047 [P] [US3] Create RestoreConfirmModal component in src/lib/features/cloud-backup/components/RestoreConfirmModal.svelte
- [x] T048 [US3] Add backup list fetching to CloudBackupController
- [x] T049 [US3] Implement restore flow with RESTORE confirmation in controller
- [x] T050 [US3] Add restore progress events and app reload after restore

**Checkpoint**: User Stories 1, 2 & 3 complete - Full backup/restore cycle works

---

## Phase 6: User Story 4 - Version Management (Priority: P3)

**Goal**: System auto-manages backup versions (max 5), with proper labeling

**Independent Test**: Create 6 backups, verify only 5 remain (oldest deleted)

### Backend Implementation

- [x] T051 [US4] Implement version limit enforcement (BR-02) in sync_backup.rs
- [x] T052 [US4] Implement file deletion for oldest backup in google_drive.rs
- [x] T053 [US4] Implement backup labeling (Initial vs timestamped) in sync_backup.rs
- [x] T054 [US4] Add app_properties metadata to uploaded files

### Frontend Implementation

- [x] T055 [US4] Display backup labels (Initial, date/time) in BackupList component
- [x] T056 [US4] Show backup count in settings UI

**Checkpoint**: Version management automated

---

## Phase 7: User Story 5 - Offline Handling (Priority: P3)

**Goal**: User sees clear messaging when offline, sync button disabled

**Independent Test**: Disable network, verify button disabled with message

### Backend Implementation

- [x] T057 [US5] Implement connectivity checker in src-tauri/src/features/cloud_backup/infrastructure/connectivity.rs
- [x] T058 [US5] Implement cloud_backup_check_connectivity command in src-tauri/src/commands/cloud_backup.rs
- [x] T059 [US5] Implement connectivity-changed event emission with periodic checks
- [x] T060 [US5] Add offline guard to sync_backup and restore_backup use cases

### Frontend Implementation

- [x] T061 [P] [US5] Create connectivity store in src/lib/features/cloud-backup/stores/connectivity.ts
- [x] T062 [P] [US5] Create ConnectivityIndicator component in src/lib/features/cloud-backup/components/ConnectivityIndicator.svelte
- [x] T063 [US5] Add connectivity event listener to CloudBackupController
- [x] T064 [US5] Disable SyncButton when offline with appropriate message

**Checkpoint**: All user stories complete

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Error handling, validation, and production readiness

- [ ] T065 Add import-in-progress guard (BR-03) to sync_backup use case
- [ ] T066 [P] Add comprehensive error messages for all failure scenarios (FR-017)
- [ ] T075 [P] Handle revoked Google permissions gracefully (re-prompt OAuth on TOKEN_EXPIRED)
- [ ] T076 [P] Handle missing backup folder (recreate on sync if deleted from Drive)
- [ ] T077 [P] Handle Drive storage quota exceeded with user-friendly error message
- [ ] T078 Add operation locking to prevent concurrent sync from multiple devices
- [ ] T067 [P] Add JSDoc documentation to frontend controller and components
- [ ] T068 [P] Add Rust doc comments to all public APIs
- [ ] T069 Run `pnpm rust:format` and `pnpm rust:clippy` to verify Rust code
- [ ] T070 Run `pnpm format` and `pnpm lint` to verify frontend code
- [ ] T071 Run `pnpm check` to verify TypeScript types
- [ ] T072 Verify specta type generation produces correct bindings.ts
- [ ] T073 Manual E2E test: Complete OAuth → Sync → Restore cycle
- [ ] T074 Run quickstart.md validation checklist

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
     │
     ▼
Phase 2 (Foundational) ──── BLOCKS all user stories
     │
     ├──────────────────────────────────────────┐
     ▼                                          ▼
Phase 3 (US1: Connect)                    Phase 7 (US5: Offline)*
     │                                          │
     ▼                                          │
Phase 4 (US2: Sync) ◄───────────────────────────┘
     │                                    *Can start in parallel
     ▼                                     after Phase 2
Phase 5 (US3: Restore)
     │
     ▼
Phase 6 (US4: Versions)
     │
     ▼
Phase 8 (Polish)
```

### User Story Dependencies

| Story          | Depends On               | Can Parallel With |
| -------------- | ------------------------ | ----------------- |
| US1 (Connect)  | Phase 2 only             | US5 (Offline)     |
| US2 (Sync)     | US1 (needs auth)         | —                 |
| US3 (Restore)  | US2 (needs Drive client) | —                 |
| US4 (Versions) | US2 (sync logic)         | —                 |
| US5 (Offline)  | Phase 2 only             | US1               |

### MVP Delivery

**Minimum Viable Product**: Phases 1-4 (Setup + Foundation + US1 + US2)

After Phase 4, users can:

- ✅ Connect Google account
- ✅ Upload backups to cloud
- ✅ See last sync timestamp

This delivers core value before implementing restore/version management.

---

## Parallel Execution Examples

### Phase 1 (All parallelizable after T001-T002)

```bash
# After dependencies installed:
T003 & T004 & T005 & T006 can run in parallel
```

### Phase 3 - User Story 1

```bash
# Backend can proceed while frontend structure created:
T024 (controller) can run in parallel with T018-T023 (backend)
```

### Phase 7 - User Story 5 (Independent)

```bash
# Can start immediately after Phase 2:
T057-T064 can run in parallel with Phase 3 work
```

---

## Task Summary

| Phase           | Tasks          | Parallel Opportunities             |
| --------------- | -------------- | ---------------------------------- |
| 1. Setup        | T001-T007 (7)  | T003, T004, T005, T006             |
| 2. Foundational | T008-T017 (11) | T011, T013b, T014                  |
| 3. US1 Connect  | T018-T027 (10) | T024                               |
| 4. US2 Sync     | T028-T038 (11) | T035                               |
| 5. US3 Restore  | T039-T050 (12) | T046, T047                         |
| 6. US4 Versions | T051-T056 (6)  | —                                  |
| 7. US5 Offline  | T057-T064 (8)  | T061, T062                         |
| 8. Polish       | T065-T078 (14) | T066, T067, T068, T075, T076, T077 |
| **Total**       | **79 tasks**   | **19 parallelizable**              |
