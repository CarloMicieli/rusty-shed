# Phase 0: Research & Technical Decisions

**Feature**: Migrate Tauri 2 Settings
**Date**: 2026-02-15
**Status**: Complete

## Overview

This document consolidates research findings and technical decisions for implementing Tauri 2 settings management using official Tauri plugins and Svelte 5 reactive state patterns.

## Decision 1: Settings Persistence - tauri-plugin-store

**Decision**: Use `tauri-plugin-store` for all user settings persistence

**Rationale**:

- Official Tauri 2 plugin designed specifically for application settings and preferences
- Provides JSON-based key-value store with automatic file system management
- Supports reactive updates and change notifications out of the box
- Platform-agnostic storage location (uses OS-appropriate app data directory)
- No schema migrations needed - simpler than SQLite for simple key-value data
- Atomic writes and thread-safe access built-in

**Alternatives Considered**:

1. **SQLite via sqlx** (current project standard)
   - Rejected: Overkill for simple settings (6 fields)
   - Rejected: Requires migration files and schema management
   - Rejected: Settings don't need relational database features
   - Rejected: Contradicts Tauri 2 best practices for app preferences

2. **File-based JSON (custom implementation)**
   - Rejected: Reinventing the wheel
   - Rejected: No atomic writes or concurrency guarantees
   - Rejected: Manual path management across platforms

**Implementation Notes**:

- Store path: Managed by Tauri (e.g., `~/.config/rusty-shed/settings.json` on Linux)
- API: `Store::new("settings.json")` in Rust, `load()` for initialization
- Thread safety: Plugin handles locking internally
- Change notifications: Use Tauri events to notify frontend of external changes

**References**:

- [Tauri Store Plugin v2 docs](https://v2.tauri.app/plugin/store/)
- [Store plugin GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store)

---

## Decision 2: Window State Management - tauri-plugin-window-state

**Decision**: Use `tauri-plugin-window-state` for window position and size persistence

**Rationale**:

- Official Tauri 2 plugin designed for window geometry management
- Automatically saves and restores window position, size, and maximized state
- Handles multi-monitor scenarios and display configuration changes
- Zero-code integration - works via plugin initialization
- Respects OS window management policies

**Alternatives Considered**:

1. **Manual window state tracking via tauri-plugin-store**
   - Rejected: Reinvents plugin functionality
   - Rejected: Requires manual event listeners for resize/move
   - Rejected: More complex multi-monitor handling

2. **Browser localStorage (frontend-only)**
   - Rejected: Cannot control native window from frontend before window creation
   - Rejected: Race conditions during initialization

**Implementation Notes**:

- Configuration: Add to `tauri.conf.json` plugins section
- Initialization: Call `.plugin(tauri_plugin_window_state::Builder::default().build())` in `main.rs`
- State file: Automatically managed by plugin (separate from settings.json)
- Fallback behavior: Plugin provides sensible defaults if saved position is off-screen

**References**:

- [Tauri Window State Plugin v2 docs](https://v2.tauri.app/plugin/window-state/)
- [Window State plugin GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/window-state)

---

## Decision 3: OS Language Detection - tauri-plugin-os

**Decision**: Use `tauri-plugin-os` to detect operating system locale on first run

**Rationale**:

- Official Tauri 2 plugin providing OS information APIs
- Provides `locale()` function returning OS language code (e.g., "en-US", "it-IT")
- Cross-platform support (Linux, Windows, macOS)
- Read-only, safe API with no permissions required
- Lightweight - no additional dependencies beyond Tauri core

**Alternatives Considered**:

1. **Browser navigator.language (frontend)**
   - Rejected: Not available at backend initialization time
   - Rejected: May not reflect true OS settings in Tauri webview

2. **Platform-specific APIs (sys-locale crate)**
   - Rejected: tauri-plugin-os already includes this functionality
   - Rejected: Adds redundant dependency

**Implementation Notes**:

- API: `tauri_plugin_os::locale()` returns `Option<String>` with locale code
- Parsing: Extract language prefix (e.g., "it" from "it-IT")
- Supported languages: Check against ["en", "it"], fallback to "en"
- First-run detection: Check `first_run` boolean in settings store

**References**:

- [Tauri OS Plugin v2 docs](https://v2.tauri.app/plugin/os/)
- [OS plugin GitHub](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/os)

---

## Decision 4: Reactive State Management - Svelte 5 Runes

**Decision**: Use Svelte 5 runes (`$state`, `$derived`, `$effect`) for reactive settings state

**Rationale**:

- Svelte 5 runes provide fine-grained reactivity without wrappers
- `$state` creates reactive settings object that updates UI automatically
- `$effect` can listen for IPC events and update state
- Simpler than stores for component-level state
- Aligns with Rusty Shed's existing Svelte 5 patterns

**Implementation Pattern**:

```typescript
// SettingsState.svelte.ts
export class SettingsState {
  settings = $state<UserSettings>({
    /* defaults */
  });

  async load() {
    this.settings = await invoke('get_settings');
  }

  async update(partial: Partial<UserSettings>) {
    this.settings = await invoke('update_settings', { settings: partial });
  }
}
```

**Alternatives Considered**:

1. **Svelte 4 stores (writable, derived)**
   - Rejected: Project uses Svelte 5, runes are the modern approach
   - Rejected: More boilerplate than runes

2. **Global state management library (e.g., Zustand, Pinia)**
   - Rejected: Overkill for single settings object
   - Rejected: Adds unnecessary dependency

**References**:

- [Svelte 5 Runes documentation](https://svelte.dev/docs/svelte/$state)
- Existing Rusty Shed patterns: `src/lib/features/*/FeatureState.svelte.ts`

---

## Decision 5: IPC Contract Design - Tauri specta Integration

**Decision**: Use `specta` for TypeScript type generation from Rust types

**Rationale**:

- Already used in Rusty Shed project (see constitution)
- Generates TypeScript types from Rust structs automatically
- Compile-time type safety across IPC boundary
- Reduces manual type synchronization errors

**IPC Command Pattern**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GetSettingsResult {
    pub currency: String,
    pub language: Language,
    pub measure_unit: MeasureUnit,
    pub favourite_scale: String,
    pub power_system: PowerSystem,
}

#[tauri::command]
#[specta::specta]
pub async fn get_settings(app: AppHandle) -> Result<GetSettingsResult, String> {
    // Implementation
}
```

**Validation Strategy**:

- Use `validator` crate for Rust-side validation
- Validate Args at IPC boundary before calling use cases
- Return validation errors as structured error responses

**References**:

- [Tauri specta integration](https://github.com/oscartbeaumont/tauri-specta)
- Project constitution: API Design & Transport Boundary section

---

## Decision 6: Testing Strategy

**Decision**: Multi-layer testing approach

**Test Layers**:

1. **Rust Unit Tests**:
   - Test domain value objects (UserSettings validation)
   - Test use case logic (initialize_settings, update_settings)
   - Mock tauri-plugin-store using traits

2. **Rust Integration Tests**:
   - Test IPC command handlers with real plugin instances
   - Use temporary store files for isolation
   - Test error handling and validation

3. **Frontend Unit Tests (Vitest)**:
   - Test SettingsState reactive behavior
   - Mock Tauri invoke calls
   - Test component props and events

4. **Frontend Integration Tests (Vitest)**:
   - Test end-to-end settings flow (load → update → persist)
   - Test reactive UI updates after settings change

**Coverage Target**: 80% for settings module (per SC-005)

**Mocking Approach**:

- Rust: Use trait abstraction for `SettingsRepository`
- Frontend: Mock `@tauri-apps/api/core::invoke` using Vitest mocks

**References**:

- Project constitution: Testing Standards section
- Existing test patterns: `src/__tests__/` and `src-tauri/src/*/tests/`

---

## Technical Risks & Mitigations

### Risk 1: Plugin Compatibility

**Risk**: Tauri plugins may have version conflicts or bugs
**Mitigation**: Use official v2 plugins from Tauri plugins-workspace, pin versions in Cargo.toml

### Risk 2: Window Off-Screen After Monitor Change

**Risk**: Saved window position becomes invalid if user disconnects monitor
**Mitigation**: tauri-plugin-window-state handles this automatically; test multi-monitor scenarios

### Risk 3: Settings Corruption

**Risk**: JSON file corruption due to improper shutdown or disk errors
**Mitigation**:

- tauri-plugin-store uses atomic writes
- Validate settings on load, fallback to defaults if invalid
- Log errors for debugging

### Risk 4: Reactive State Synchronization

**Risk**: Settings changes in one UI component not reflected in others
**Mitigation**:

- Use single SettingsState instance across app (singleton pattern)
- Svelte 5 runes automatically propagate changes
- Add integration test to verify multi-component reactivity

---

## Best Practices Summary

1. **Plugin Initialization**: Initialize all plugins in `main.rs` before building the app
2. **Error Handling**: All IPC commands return `Result<T, String>` with descriptive error messages
3. **Validation**: Validate all settings at Rust boundary, not just frontend
4. **Defaults**: Provide sensible defaults for all settings (English language, empty currency, etc.)
5. **Atomicity**: Use single transaction for related settings updates when possible
6. **Logging**: Log settings changes for debugging (redact sensitive values if any)
7. **Documentation**: Document all IPC commands with rustdoc, all settings fields with TSDoc

---

## Open Questions: NONE

All technical decisions have been finalized based on user requirements and Tauri 2 best practices.
