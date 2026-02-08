# Tasks Breakdown Summary: Data Archive Export (016)

**Date**: February 8, 2026 | **Status**: Ready for Implementation | **Total Tasks**: 70

---

## Overview

The feature is broken down into **70 actionable tasks** organized across 9 phases:

| Phase | Name                   | Tasks | Purpose                                      | Duration |
| ----- | ---------------------- | ----- | -------------------------------------------- | -------- |
| 1     | Setup & Infrastructure | 7     | Module structure, dependencies, i18n         | 1-2 days |
| 2     | Foundational           | 10    | Domain models, infrastructure (BLOCKING)     | 3-4 days |
| 3     | US1: Full Export       | 11    | Complete collection export + round-trip test | 3-4 days |
| 4     | US2: Choose Location   | 4     | File picker + destination selection          | 1-2 days |
| 5     | US3: Preview Summary   | 5     | Preview with record counts and warnings      | 2-3 days |
| 6     | US4: Selective Export  | 5     | Entity type checkboxes and validation        | 2-3 days |
| 7     | US5: Progress Feedback | 4     | Real-time progress events and ETA            | 2-3 days |
| 8     | US6: Orphaned Images   | 5     | Detect and include unlinked images           | 2-3 days |
| 9     | Polish                 | 14    | Error handling, docs, testing, security      | 2-3 days |

**Total Effort**:

- Solo developer: 10-14 days
- 4-person team (parallel): 4-6 days

---

## Task Organization

### By Phase

```
Phase 1: Setup (7 tasks)
├─ Module structure [P]
├─ Entry points [P]
├─ Module registration
├─ Manifest schema [P]
├─ Frontend structure [P]
├─ i18n keys [P]
└─ Test fixtures

Phase 2: Foundational (10 tasks - BLOCKS ALL STORIES)
├─ Domain: Error types [P]
├─ Domain: Entity selection [P]
├─ Domain: Export session
├─ Domain: Config [P]
├─ Domain: Progress [P]
├─ Domain: Result
├─ Infrastructure: Disk space checker
├─ Infrastructure: Media collector [P]
├─ Infrastructure: File picker [P]
├─ Infrastructure: Archive writer (manifest & ZIP)

US1: Full Export (11 tasks)
├─ Backend: Preview use case [P]
├─ Backend: Data collection [P]
├─ Backend: Execute export
├─ Backend: Tauri commands
├─ Frontend: Dialog component [P]
├─ Frontend: Preview display [P]
├─ Frontend: Progress bar [P]
├─ Frontend: Report component
├─ Frontend: Controller
├─ Integration: Route/UI
└─ Testing: Round-trip test

[US2 through US6 follow similar patterns...]

Phase 9: Polish (14 tasks)
├─ Error handling & recovery [P]
├─ Documentation [P]
├─ Performance optimization [P]
├─ Cross-platform testing [P]
├─ Additional unit tests
├─ Security hardening
└─ Integration verification
```

### By Technology

**Backend (Rust)**:

- Domain models: 6 types
- Use cases: 3 (preview, collect, execute)
- Infrastructure: 4 modules (disk, media, picker, archive)
- Commands: 3 Tauri commands
- Tests: 7 integration test suites
- **Total backend tasks**: 28

**Frontend (Svelte)**:

- Components: 5 (Dialog, Preview, Progress, Report, Selector)
- Controller: 1
- Types: 1
- Routes: 1 (integration)
- Event listeners: 1 (progress)
- **Total frontend tasks**: 11

**Cross-Cutting**:

- Testing: 7 integration test suites
- Documentation: 2 docs
- Performance: 1 benchmark/profile
- Security: 1 hardening
- Error handling: 1 recovery
- **Total cross-cutting**: 12

---

## Task Granularity

Each task is:

- ✅ **Specific**: Exact file path included
- ✅ **Actionable**: Clear "what to do" statement
- ✅ **Testable**: Success criteria implicit in description
- ✅ **Independent**: Marked [P] for parallel execution capability
- ✅ **Sized**: Typically 1-3 hours of work each

### Example Tasks

**T019 [P] [US1]** Create `preview_export` use case

- File: `src-tauri/src/export/application/preview_export.rs`
- Function signature and logic defined
- Calls documented

**T025 [P] [US1]** Create `ExportProgress.svelte`

- File: `src/lib/features/export/components/ExportProgress.svelte`
- Props and event listeners specified
- Display requirements clear

**T048 [P] [US6]** Implement orphaned image detection

- File: `src-tauri/src/export/infrastructure/media_collector.rs`
- Algorithm: scan files, query DB, find differences
- Return type: `Vec<OrphanedImage>`

---

## Parallelization Opportunities

### Phase 1 (Setup)

All 6 tasks marked [P] can run in parallel:

- 2 developers can complete in 1 day vs. 1-2 solo

### Phase 2 (Foundational)

6 tasks marked [P] can run in parallel:

- Domain types [P]: T008, T009, T012 in parallel
- Infrastructure [P]: T014, T015, T016 in parallel
- Sequential dependencies minimal

### User Story Phases (3-8)

Each story has multiple [P] tasks:

- **US1**: 4 frontend [P] tasks (T023-T025, T027) can run in parallel
- **US1**: 2 backend [P] tasks (T019-T020) can run in parallel
- **US2**: 2 tasks [P] (T030, T032) can run in parallel
- Similar patterns in US3-6

**Example Parallel Timeline**:

```
Day 1-2:  Phase 1 Setup (7 tasks, team of 2 = 1-2 days)
Day 3-6:  Phase 2 Foundational (10 tasks, team of 2-3 = 3-4 days)
Day 7-10: US1+US2 Parallel (15 tasks, team of 3-4 = 3-4 days)
  ├─ Dev A: US1 backend (T019-T022)
  ├─ Dev B: US1 frontend (T023-T027)
  ├─ Dev C: US2 + US1 testing (T029-T033)
  └─ All validate MVP together
Day 11+:  US3-6 + Polish (28 tasks, team of 4 = 7-10 days)
```

---

## MVP Scope (User Stories 1 & 2)

**Minimum viable product**: 11 tasks over 7 days (1 developer) or 3-4 days (3 developers)

```
Phase 1: Setup (7 tasks)
Phase 2: Foundational (10 tasks)
US1 (Phase 3): Full export (11 tasks)
US2 (Phase 4): Choose location (4 tasks)
─────────────
MVP Total: 32 tasks → 5-7 days (1 dev) or 2-3 days (3-4 devs)
```

**Features in MVP**:

- ✅ Full collection backup to ZIP archive
- ✅ Choose any writable destination
- ✅ Manifest structure matches import format
- ✅ All referenced images included
- ✅ Success notification with file location
- ✅ Round-trip export/import works

**Not in MVP (Phase 5+)**:

- Preview summary (P2 feature)
- Selective entity export (P2 feature)
- Progress feedback (P2 feature)
- Orphaned image handling (P3 feature)

---

## Task Checklist Format

All tasks follow standard checkbox format:

```
- [ ] T001 [P?] [Story?] Description with file path
```

**Example**:

- `- [ ] T019 [P] [US1]` Create `preview_export` use case in `src-tauri/src/export/application/preview_export.rs`

**Status tracking**:

- Unchecked `[ ]` = not started
- Checked `[x]` = complete
- Can be marked as you progress

---

## Testing Strategy

### Test-First Approach

- T029, T033, T038, T043, T047, T052 are integration tests
- Write failing tests BEFORE implementation
- Implementation makes tests pass

### Test Coverage

- 7 integration test suites (one per story + foundational tests)
- Domain unit tests (entity selection, progress calculation)
- Round-trip validation (export → import → compare)
- Cross-platform testing (Windows, macOS, Linux)
- Performance benchmarks (large exports)

### Test Locations

```
src-tauri/tests/integration/
├── export_import_roundtrip.rs    (T029 - US1)
├── export_destination.rs          (T033 - US2)
├── export_preview.rs              (T038 - US3)
├── export_selective.rs            (T043 - US4)
├── export_progress.rs             (T047 - US5)
└── export_orphaned_images.rs      (T052 - US6)
```

---

## Dependencies Summary

### Phase Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundational) ← MUST COMPLETE BEFORE ANY USER STORY
    ↓
US1 + US2 (MVP)
    ↓
US3, US4, US5, US6 (can proceed in parallel or sequentially)
    ↓
Phase 9 (Polish)
```

### Within-User-Story Dependencies

```
US1 (Full Export)
├─ T019 (preview) can run with T020, T021, T022
├─ T020 (collect data) must complete before T021
├─ T021 (execute) depends on T020
└─ T023-T027 (frontend) can run independently in parallel

US2 (Choose Location)
├─ T030 (file picker) improves T021
├─ T031 (execute updates) depends on T030 complete
└─ T032-T033 (frontend+tests) depend on T030, T031
```

---

## Sign-Off Checkpoints

| Checkpoint        | Criteria                                 | Impact                     |
| ----------------- | ---------------------------------------- | -------------------------- |
| **After Phase 1** | Module structure ready                   | Can begin Phase 2          |
| **After Phase 2** | All domain models + infrastructure ready | Can begin ALL user stories |
| **After US1+US2** | Full export + location selection working | MVP ready, can release     |
| **After US3+US4** | Preview + selective export               | Enhanced UX, user control  |
| **After US5+US6** | Progress + orphaned images               | Complete feature           |
| **After Phase 9** | Tests, docs, security, performance       | Production ready           |

---

## Quick Start for Developers

1. **Read**: This summary (you are here!)
2. **Open**: [tasks.md](./tasks.md) for full task list with checkboxes
3. **Start Phase 1**: Tasks T001-T007 (setup)
4. **Then Phase 2**: Tasks T008-T018 (foundational - REQUIRED for all stories)
5. **Pick Story**: Start with US1 (T019-T029) for MVP
6. **Mark Progress**: Check off `[x]` as you complete each task
7. **Follow Ordering**: Respect task dependencies, use [P] for parallel work

---

## File Reference

| Document                             | Purpose                                                 |
| ------------------------------------ | ------------------------------------------------------- |
| [tasks.md](./tasks.md)               | Full task list with 70 checkboxes (this file in detail) |
| [spec.md](./spec.md)                 | User stories and acceptance criteria                    |
| [plan.md](./plan.md)                 | Architecture and technical decisions                    |
| [research.md](./research.md)         | Technical research findings                             |
| [PLAN_SUMMARY.md](./PLAN_SUMMARY.md) | Executive summary                                       |
| [INDEX.md](./INDEX.md)               | Navigation by role                                      |

---

**Status**: ✅ **Tasks Generated - Ready for Implementation**

All 70 tasks are:

- ✅ Specific and actionable
- ✅ Properly sequenced with dependencies
- ✅ Marked for parallel execution where applicable
- ✅ Organized by user story
- ✅ Estimated for solo and team timelines
- ✅ Testable at each checkpoint

**Next Step**: Open [tasks.md](./tasks.md) and start checking off T001-T007 (Phase 1 Setup)
