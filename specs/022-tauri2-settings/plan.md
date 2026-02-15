# Implementation Plan: Migrate Tauri 2 Settings

**Branch**: `022-tauri2-settings` | **Date**: 2026-02-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/022-tauri2-settings/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Migrate application settings management to use Tauri 2's official plugin ecosystem (tauri-plugin-store for settings persistence, tauri-plugin-window-state for window geometry, tauri-plugin-os for language detection). Implement reactive settings updates across the frontend using Svelte 5 runes. Support five user-configurable settings (currency, language, measure unit, favourite scale, power system) plus hidden first-run tracking. Ensure settings persist across application restarts and update all UI components immediately when changed, without requiring restart.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)
**Primary Dependencies**:

- Backend: Tauri 2.9.x, tauri-plugin-store, tauri-plugin-window-state, tauri-plugin-os, serde, specta, validator
- Frontend: SvelteKit (Svelte 5.48.2), Paraglide-JS, Tailwind CSS 4.1.18, shadcn-svelte

**Storage**: tauri-plugin-store (JSON file-based key-value store managed by Tauri)
**Testing**: cargo test (Rust backend), vitest (frontend with happy-dom)
**Target Platform**: Desktop (Linux, Windows, macOS via Tauri 2)
**Project Type**: Desktop application (Tauri 2 architecture with Rust backend + SvelteKit frontend)
**Performance Goals**:

- Settings read operations: <50ms (local file read)
- Settings write operations: <200ms (file write + reactive updates)
- UI reactive updates: <500ms after settings change
- Window state restoration: <100ms on app startup

**Constraints**:

- Offline-capable (all settings stored locally)
- Must persist settings across application restarts
- Reactive state: settings changes must propagate to all UI components without restart
- Window position must handle multi-monitor and display configuration changes gracefully
- All user-facing strings must use Paraglide-JS

**Scale/Scope**:

- Single-user desktop application
- ~6 settings fields (5 user-visible + 1 hidden)
- ~4 window state fields (x, y, width, height)
- Estimated ~5-8 IPC commands for settings CRUD operations

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Code Quality ✅

- **Status**: COMPLIANT
- **Evidence**: Plan includes linting (cargo clippy, cargo fmt, pnpm lint) and formatting requirements. IPC commands will be documented with rustdoc.

### Testing Standards ✅

- **Status**: COMPLIANT
- **Evidence**: FR-018 and FR-019 require unit and integration tests. Plan includes 80% test coverage target for settings module (SC-005).

### User Experience Consistency ✅

- **Status**: COMPLIANT
- **Evidence**: FR-017 mandates Paraglide-JS for all user-facing text. Settings page will follow existing shadcn-svelte patterns.

### Performance Requirements ✅

- **Status**: COMPLIANT
- **Evidence**: SC-002 specifies 500ms reactive update latency. Technical context defines <200ms for read queries, meeting the <200ms UI-critical operation requirement.

### Safe Rust Practices ✅

- **Status**: COMPLIANT
- **Evidence**: All settings operations will use Result<T, E> error handling. No panics in production flows. Validation at IPC boundary.

### Database (Persistence) — REQUIRED ⚠️ JUSTIFIED DEVIATION

- **Status**: DEVIATION (using tauri-plugin-store instead of SQLite)
- **Justification**: Settings are simple key-value pairs that do not require relational database features. Tauri 2's official recommendation for application settings is tauri-plugin-store (see [Tauri Store Plugin docs](https://v2.tauri.app/plugin/store/)). SQLite would add unnecessary complexity for this use case. The constitution's Database law targets domain data and aggregates, not application configuration.
- **Migration Note**: Spec explicitly states "no migration is required from the settings sqlite database table" (FR-001), indicating intentional move away from SQLite for settings.

### State Management / Persistence Strategy — REQUIRED ✅

- **Status**: NOT APPLICABLE (settings are not domain aggregates)
- **Reasoning**: The Domain Event Tracking pattern applies to domain aggregates (e.g., RailwayModel, RollingStock). Application settings are configuration data managed through a dedicated plugin, not business domain entities requiring event sourcing.

### API Design & Transport Boundary — REQUIRED ✅

- **Status**: COMPLIANT
- **Evidence**: All IPC commands will follow ADR 8 conventions with Args/Input types, derive specta::Type for TypeScript generation, and validate at boundary using validator::Validate.

### Domain Logic Location — REQUIRED ✅

- **Status**: COMPLIANT
- **Evidence**: Settings validation logic (e.g., supported languages, valid window positions) will live in Rust backend. Frontend only provides UI for settings modification.

## Project Structure

### Documentation (this feature)

```text
specs/022-tauri2-settings/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── settings-ipc.md  # IPC command contracts
├── checklists/
│   └── requirements.md  # Spec quality checklist (already exists)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── settings/                    # Settings module (NEW)
│   │   ├── mod.rs                   # Module declaration
│   │   ├── domain/                  # Domain types and validation
│   │   │   ├── mod.rs
│   │   │   ├── user_settings.rs     # UserSettings value object
│   │   │   └── window_state.rs      # WindowState value object
│   │   ├── application/             # Use cases
│   │   │   ├── mod.rs
│   │   │   ├── get_settings.rs      # Read settings use case
│   │   │   ├── update_settings.rs   # Update settings use case
│   │   │   └── initialize_settings.rs # First-run initialization
│   │   ├── infrastructure/          # Plugin integration
│   │   │   ├── mod.rs
│   │   │   ├── store_repository.rs  # tauri-plugin-store integration
│   │   │   └── os_language.rs       # tauri-plugin-os integration
│   │   └── interface/               # Tauri commands
│   │       ├── mod.rs
│   │       └── commands.rs          # IPC command handlers
│   ├── lib.rs                       # Register settings module and commands
│   └── main.rs                      # Initialize plugins (store, window-state, os)
└── Cargo.toml                       # Add plugin dependencies

src/
├── lib/
│   ├── features/
│   │   └── settings/                # Settings feature module (NEW)
│   │       ├── components/
│   │       │   ├── SettingsForm.svelte       # Settings form UI
│   │       │   ├── LanguageSelector.svelte   # Language dropdown
│   │       │   ├── CurrencySelector.svelte   # Currency dropdown
│   │       │   ├── MeasureUnitSelector.svelte
│   │       │   ├── ScaleSelector.svelte
│   │       │   └── PowerSystemSelector.svelte
│   │       ├── SettingsController.svelte.ts  # Business logic controller
│   │       └── SettingsState.svelte.ts       # Reactive state management (runes)
│   └── paraglide/                   # i18n messages (UPDATED)
│       └── messages/
│           └── en.json              # Add settings-related strings
│           └── it.json              # Add settings-related strings
└── routes/
    └── settings/                    # Settings page route (NEW or UPDATE)
        └── +page.svelte             # Settings page component

tests/ (or src/__tests__/)
├── unit/
│   └── settings/                    # Settings unit tests (NEW)
│       ├── user_settings.test.ts    # Frontend state tests
│       └── settings_controller.test.ts
└── integration/
    └── settings/                    # Settings integration tests (NEW)
        └── settings_ipc.test.ts     # IPC command integration tests

src-tauri/src/settings/tests/        # Rust unit tests (inline or submodule)
├── domain_tests.rs                  # Test UserSettings validation
└── use_case_tests.rs                # Test use cases
```

**Structure Decision**: Following Rusty Shed's clean architecture pattern with domain-driven design. Settings module organized into domain (value objects), application (use cases), infrastructure (plugin integration), and interface (IPC commands) layers. Frontend follows existing feature module pattern with components, controller, and state management using Svelte 5 runes. No SQLite migrations required per spec; using tauri-plugin-store for persistence.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                                  | Why Needed                                                                                                                                                    | Simpler Alternative Rejected Because                                                                                                                               |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Using tauri-plugin-store instead of SQLite | Settings are simple key-value configuration data that don't require relational database features. Tauri 2's official recommendation for app settings storage. | SQLite would be overkill for 6 simple settings fields. Adds migration complexity, schema management overhead, and contradicts Tauri 2 best practices for settings. |

---

## Post-Design Constitution Check (Phase 1 Complete)

**Re-evaluation Date**: 2026-02-15

All constitutional requirements remain satisfied after Phase 1 design:

- ✅ **Code Quality**: IPC contracts documented in [contracts/settings-ipc.md](contracts/settings-ipc.md), data model in [data-model.md](data-model.md)
- ✅ **Testing Standards**: Testing strategy defined in [research.md](research.md) with unit/integration test plans
- ✅ **User Experience Consistency**: Paraglide integration documented in [quickstart.md](quickstart.md)
- ✅ **Performance Requirements**: SLAs defined in [contracts/settings-ipc.md](contracts/settings-ipc.md) (all <200ms)
- ✅ **Safe Rust Practices**: All IPC commands return `Result<T, String>` per contract design
- ⚠️ **Database (Persistence)**: Justified deviation remains valid (tauri-plugin-store for simple settings)
- ✅ **State Management**: Not applicable (settings are not domain aggregates)
- ✅ **API Design & Transport Boundary**: All IPC commands follow ADR 8 conventions with specta types
- ✅ **Domain Logic Location**: Validation logic in Rust per [data-model.md](data-model.md)

**Conclusion**: No new violations introduced. Plan ready for task generation with `/speckit.tasks`.

---

## Artifacts Generated

### Phase 0: Research

- [research.md](research.md) - Technical decisions and plugin selection rationale

### Phase 1: Design & Contracts

- [data-model.md](data-model.md) - UserSettings and WindowState entity definitions
- [contracts/settings-ipc.md](contracts/settings-ipc.md) - Complete IPC command specifications
- [quickstart.md](quickstart.md) - Developer guide for using settings feature
- Updated: [CLAUDE.md](../../CLAUDE.md) - Added Tauri plugin dependencies to agent context
