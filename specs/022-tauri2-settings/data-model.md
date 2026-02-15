# Data Model: Tauri 2 Settings

**Feature**: Migrate Tauri 2 Settings
**Date**: 2026-02-15
**Status**: Design Complete

## Overview

This document defines the data structures for application settings and window state management. Settings are persisted using `tauri-plugin-store` as JSON key-value pairs.

---

## Entity: UserSettings

**Description**: Represents user-configurable application preferences

**Storage**: tauri-plugin-store (`settings.json`)

### Fields

| Field             | Type          | Required | Default             | Validation Rules                     | Description                                            |
| ----------------- | ------------- | -------- | ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `currency`        | `String`      | Yes      | `"EUR"`             | Non-empty string, max 10 chars       | User's preferred currency for displaying prices        |
| `language`        | `Language`    | Yes      | OS locale or `"en"` | Enum: `"en"` \| `"it"`               | Application display language                           |
| `measure_unit`    | `MeasureUnit` | Yes      | `"Metric"`          | Enum: `"Metric"` \| `"Imperial"`     | Measurement system for dimensions                      |
| `favourite_scale` | `String`      | Yes      | `""`                | Max 20 chars (e.g., "HO", "N", "OO") | User's preferred model railway scale                   |
| `power_system`    | `PowerSystem` | Yes      | `"DC"`              | Enum: `"DC"` \| `"AC"` \| `"DCC"`    | Preferred electrical system for model railways         |
| `first_run`       | `bool`        | Yes      | `true`              | Boolean                              | Flag indicating if this is the user's first app launch |

### Enums

#### Language

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "it")]
    Italian,
}
```

**Serialization**: Lowercase strings (`"en"`, `"it"`)
**Default**: Determined by OS locale, fallback to `English`

#### MeasureUnit

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum MeasureUnit {
    Metric,
    Imperial,
}
```

**Serialization**: Capitalized strings (`"Metric"`, `"Imperial"`)
**Default**: `Metric`

#### PowerSystem

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum PowerSystem {
    DC,
    AC,
    DCC,
}
```

**Serialization**: Uppercase strings (`"DC"`, `"AC"`, `"DCC"`)
**Default**: `DC`

### Validation Rules

**Invariants**:

1. `currency` must not be empty and must be ≤ 10 characters
2. `language` must be either `"en"` or `"it"`
3. `measure_unit` must be either `"Metric"` or `"Imperial"`
4. `favourite_scale` must be ≤ 20 characters (empty allowed)
5. `power_system` must be one of `"DC"`, `"AC"`, or `"DCC"`

**Validation Location**: Rust domain layer (`user_settings.rs` value object)

**Error Handling**: Return `Result<UserSettings, ValidationError>` on construction/update

### State Transitions

```
[First Launch]
  → first_run = true, language = detect_os_locale() || "en", others = defaults

[User Updates Setting]
  → Validate new value → Update field → Persist to store → Emit change event

[Subsequent Launches]
  → first_run = false (set after first successful initialization)
```

### JSON Storage Example

```json
{
  "currency": "EUR",
  "language": "en",
  "measure_unit": "Metric",
  "favourite_scale": "HO",
  "power_system": "DCC",
  "first_run": false
}
```

---

## Entity: WindowState

**Description**: Represents the main application window's display geometry

**Storage**: Managed automatically by `tauri-plugin-window-state` (separate state file)

**Note**: This entity is not directly managed by application code. The plugin handles persistence and restoration automatically.

### Fields (for reference only)

| Field          | Type   | Required | Default  | Validation Rules                     | Description                            |
| -------------- | ------ | -------- | -------- | ------------------------------------ | -------------------------------------- |
| `x`            | `i32`  | Yes      | Centered | Must be within visible screen bounds | Window X coordinate (pixels from left) |
| `y`            | `i32`  | Yes      | Centered | Must be within visible screen bounds | Window Y coordinate (pixels from top)  |
| `width`        | `u32`  | Yes      | `1280`   | Min: 800, Max: screen width          | Window width in pixels                 |
| `height`       | `u32`  | Yes      | `720`    | Min: 600, Max: screen height         | Window height in pixels                |
| `is_maximized` | `bool` | Yes      | `false`  | Boolean                              | Whether window is maximized            |

### Validation Rules

**Plugin-managed validation**:

1. If saved position is off-screen (e.g., monitor disconnected), plugin repositions to primary display
2. If saved size exceeds screen bounds, plugin resizes to fit
3. Plugin respects OS window management policies (minimum sizes, title bar visibility)

**No application code required** - fully handled by `tauri-plugin-window-state`

---

## Relationships

### UserSettings ↔ UI Components

**One-to-Many**: One `UserSettings` instance → Multiple UI components display/modify settings

**Reactivity**: Changes to `UserSettings` automatically propagate to all subscribed UI components via Svelte 5 runes

**Access Pattern**:

```typescript
// SettingsState.svelte.ts (singleton)
export class SettingsState {
  settings = $state<UserSettings>({
    /* loaded from backend */
  });

  // Any component reading this.settings will reactively update
}
```

### UserSettings ↔ Paraglide Locale

**Relationship**: `language` field determines active Paraglide locale

**Synchronization**:

1. On app startup: Load `language` from settings → Call `setLanguageTag(language)`
2. On language change: Update settings → Call `setLanguageTag(newLanguage)` → Trigger UI re-render

**Implementation**: SettingsController handles synchronization in `updateLanguage()` method

---

## Persistence Strategy

### Store Location

**Platform-specific paths** (managed by tauri-plugin-store):

- Linux: `~/.config/rusty-shed/settings.json`
- macOS: `~/Library/Application Support/rusty-shed/settings.json`
- Windows: `%APPDATA%\rusty-shed\settings.json`

### Write Strategy

**Atomic writes**: tauri-plugin-store uses atomic file writes (write to temp → rename) to prevent corruption

**Concurrency**: Plugin handles locking internally; safe for concurrent reads/writes

**Error handling**:

- If store file is corrupted → Log error → Initialize with defaults
- If write fails → Return error to caller → Retry or notify user

### Read Strategy

**On app startup**:

1. Initialize `tauri-plugin-store` with `settings.json` path
2. Attempt to load settings: `store.get("user_settings")`
3. If not found (first run) → Initialize defaults with OS language detection
4. If found → Deserialize JSON to `UserSettings` struct
5. If deserialization fails → Log error → Use defaults

**On demand** (after startup):

- Frontend calls `get_settings` IPC command
- Backend reads from in-memory cache (plugin caches loaded store)
- Return current settings to frontend

### Update Strategy

**Partial updates supported**:

```rust
// Update only language field
store.set("user_settings", updated_settings)?;
store.save()?; // Flush to disk
```

**Full update flow**:

1. Frontend calls `update_settings` with partial settings object
2. Backend loads current settings from store
3. Backend merges partial update into current settings
4. Backend validates merged settings
5. Backend saves to store if validation passes
6. Backend emits change event (if using event-driven architecture)
7. Frontend receives updated settings and updates UI reactively

---

## Migration Notes

**From SQLite settings table** (if it exists):

Per spec FR-001: "no migration is required from the settings sqlite database table"

**Implication**: Previous settings in SQLite are NOT migrated. Users will see default settings on first run of the new version.

**Justification**: Settings are low-value data (easily reconfigured by user). Automatic migration adds complexity without significant user benefit.

**User Experience**: On first launch after migration, users will need to re-configure their preferences. This is acceptable for a desktop application with a small number of settings.

---

## Testing Considerations

### Unit Tests

**UserSettings validation**:

- Test valid settings pass validation
- Test invalid currency (empty string) fails
- Test invalid language (unsupported code) fails
- Test favourite_scale max length validation

**Default initialization**:

- Test OS locale detection (mock `tauri-plugin-os`)
- Test fallback to English for unsupported locales

### Integration Tests

**Store persistence**:

- Create settings → Save → Load → Assert equal
- Update settings → Save → Load → Assert updated
- Corrupt JSON file → Load → Assert defaults

**IPC commands**:

- Call `get_settings` → Assert returns current settings
- Call `update_settings` with valid data → Assert persisted
- Call `update_settings` with invalid data → Assert error returned

---

## Performance Considerations

**Read performance**:

- tauri-plugin-store caches loaded store in memory
- Subsequent reads are instant (no disk I/O)
- Expected: <5ms for in-memory reads

**Write performance**:

- Atomic file write: ~50-100ms (depends on disk speed)
- Acceptable for user-initiated actions (not performance-critical)

**Memory footprint**:

- UserSettings struct: ~100 bytes
- JSON file: ~200-300 bytes
- Negligible memory impact

---

## Security Considerations

**Data sensitivity**: Settings contain no sensitive information (no passwords, tokens, PII)

**File permissions**: tauri-plugin-store uses OS-default file permissions (user read/write only)

**Validation**: All settings validated at Rust boundary to prevent invalid data from persisting

**No external access**: Settings file is local-only; no network transmission
