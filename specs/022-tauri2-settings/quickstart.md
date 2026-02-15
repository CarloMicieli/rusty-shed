# Quickstart Guide: Tauri 2 Settings

**Feature**: Migrate Tauri 2 Settings
**Date**: 2026-02-15
**Audience**: Developers working on Rusty Shed

---

## Overview

This guide shows how to use the Tauri 2 settings system in Rusty Shed. Settings are managed using official Tauri plugins and reactive Svelte 5 state.

**What you'll learn**:

- How to read user settings from any component
- How to update settings reactively
- How settings persist across app restarts
- How to add new settings fields

---

## Quick Start (Frontend)

### 1. Access Settings State

Settings are managed by a singleton `SettingsState` instance:

```typescript
// In any Svelte component
import { settingsState } from '$lib/features/settings/SettingsState.svelte';

// Read current settings
const { currency, language, measure_unit } = settingsState.settings;

// Settings are reactive - UI updates automatically when they change
```

### 2. Display Settings in UI

```svelte
<script lang="ts">
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
</script>

<div>
  <p>Current language: {settingsState.settings.language}</p>
  <p>Currency: {settingsState.settings.currency}</p>
  <p>Measure unit: {settingsState.settings.measure_unit}</p>
</div>
```

**Reactivity**: When settings change (from any source), this component re-renders automatically.

### 3. Update Settings

```typescript
import { settingsState } from '$lib/features/settings/SettingsState.svelte';

// Update a single setting
await settingsState.update({ language: 'it' });

// Update multiple settings at once
await settingsState.update({
  currency: 'USD',
  measure_unit: 'Imperial'
});

// UI updates automatically after successful update
```

### 4. Handle Errors

```typescript
try {
  await settingsState.update({ language: 'it' });
} catch (error) {
  console.error('Failed to update settings:', error);
  // Show error to user (toast notification)
  showToast({ type: 'error', message: m.settings_update_failed() });
}
```

---

## Quick Start (Backend)

### 1. Add Settings Plugin Dependencies

**File**: `src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-store = "2.0"
tauri-plugin-window-state = "2.0"
tauri-plugin-os = "2.0"
```

### 2. Initialize Plugins in main.rs

**File**: `src-tauri/src/main.rs`

```rust
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_window_state;
use tauri_plugin_os;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            settings::interface::commands::initialize_settings,
            settings::interface::commands::get_settings,
            settings::interface::commands::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Implement Settings Use Case

**File**: `src-tauri/src/settings/application/update_settings.rs`

```rust
use crate::settings::domain::UserSettings;
use crate::settings::infrastructure::SettingsRepository;
use tauri::AppHandle;

pub async fn update_settings(
    app: &AppHandle,
    updates: UpdateSettingsInput,
) -> Result<UserSettings, String> {
    // 1. Load current settings
    let mut settings = SettingsRepository::load(app)
        .map_err(|e| format!("Failed to load settings: {}", e))?;

    // 2. Apply updates
    if let Some(currency) = updates.currency {
        settings.set_currency(currency)?;
    }
    if let Some(language) = updates.language {
        settings.set_language(language);
    }
    // ... other fields

    // 3. Validate updated settings
    settings.validate()?;

    // 4. Save to store
    SettingsRepository::save(app, &settings)
        .map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(settings)
}
```

### 4. Add IPC Command Handler

**File**: `src-tauri/src/settings/interface/commands.rs`

```rust
use crate::settings::application;
use crate::settings::domain::UserSettings;
use tauri::{command, AppHandle};
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Type)]
pub struct UpdateSettingsArgs {
    pub currency: Option<String>,
    pub language: Option<Language>,
    // ... other fields
}

#[command]
#[specta::specta]
pub async fn update_settings(
    app: AppHandle,
    args: UpdateSettingsArgs,
) -> Result<UserSettings, String> {
    // Validate args
    args.validate().map_err(|e| e.to_string())?;

    // Map to use case input
    let input = UpdateSettingsInput {
        currency: args.currency,
        language: args.language,
        // ... other fields
    };

    // Call use case
    application::update_settings(&app, input).await
}
```

---

## Common Patterns

### Pattern 1: Check if First Run

```typescript
import { settingsState } from '$lib/features/settings/SettingsState.svelte';

if (settingsState.settings.first_run) {
  // Show onboarding wizard
  showOnboarding();
}
```

### Pattern 2: Sync Language with Paraglide

```typescript
import { settingsState } from '$lib/features/settings/SettingsState.svelte';
import { setLanguageTag } from '$lib/paraglide/runtime';

// Sync language on settings change
$effect(() => {
  setLanguageTag(settingsState.settings.language);
});
```

### Pattern 3: Settings Form Component

```svelte
<script lang="ts">
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { Button } from '$lib/components/ui/button';
  import { Select } from '$lib/components/ui/select';

  let currency = $state(settingsState.settings.currency);
  let language = $state(settingsState.settings.language);

  async function saveSettings() {
    await settingsState.update({ currency, language });
    showToast({ message: m.settings_saved() });
  }
</script>

<form onsubmit={saveSettings}>
  <Select bind:value={currency} options={['EUR', 'USD', 'GBP']} />
  <Select bind:value={language} options={['en', 'it']} />
  <Button type="submit">Save</Button>
</form>
```

### Pattern 4: Load Settings on App Startup

**File**: `src/routes/+layout.svelte`

```svelte
<script lang="ts">
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { onMount } from 'svelte';

  onMount(async () => {
    // Load settings when app starts
    await settingsState.load();
  });
</script>

<slot />
```

---

## Adding a New Setting Field

### Step 1: Update Rust Types

**File**: `src-tauri/src/settings/domain/user_settings.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UserSettings {
    pub currency: String,
    pub language: Language,
    pub measure_unit: MeasureUnit,
    pub favourite_scale: String,
    pub power_system: PowerSystem,
    pub first_run: bool,
    // NEW FIELD:
    pub theme: Theme,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}
```

### Step 2: Update UpdateSettingsArgs

**File**: `src-tauri/src/settings/interface/commands.rs`

```rust
#[derive(Debug, Clone, Deserialize, Type)]
pub struct UpdateSettingsArgs {
    pub currency: Option<String>,
    pub language: Option<Language>,
    // ... existing fields
    pub theme: Option<Theme>,  // NEW
}
```

### Step 3: Regenerate TypeScript Types

Run the specta type generation (if automated) or manually update:

```typescript
export interface UserSettings {
  currency: string;
  language: Language;
  measure_unit: MeasureUnit;
  favourite_scale: string;
  power_system: PowerSystem;
  first_run: boolean;
  theme: Theme; // NEW
}

export enum Theme {
  Light = 'Light',
  Dark = 'Dark',
  Auto = 'Auto'
}
```

### Step 4: Add UI Component

**File**: `src/lib/features/settings/components/ThemeSelector.svelte`

```svelte
<script lang="ts">
  import { Select } from '$lib/components/ui/select';
  import type { Theme } from '$lib/bindings';

  interface Props {
    value: Theme;
    onchange: (theme: Theme) => void;
  }

  let { value, onchange }: Props = $props();
</script>

<Select
  {value}
  onchange={(e) => onchange(e.target.value as Theme)}
  options={[
    { value: 'Light', label: m.theme_light() },
    { value: 'Dark', label: m.theme_dark() },
    { value: 'Auto', label: m.theme_auto() }
  ]}
/>
```

### Step 5: Update Settings Form

Add the new field to the settings form:

```svelte
<ThemeSelector
  value={settingsState.settings.theme}
  onchange={(theme) => settingsState.update({ theme })}
/>
```

---

## Testing Checklist

### Frontend Tests

```typescript
// src/__tests__/unit/settings/settings_state.test.ts
import { describe, it, expect, vi } from 'vitest';
import { SettingsState } from '$lib/features/settings/SettingsState.svelte';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core');

describe('SettingsState', () => {
  it('should load settings on initialization', async () => {
    const mockSettings = { language: 'en', currency: 'EUR' /* ... */ };
    vi.mocked(invoke).mockResolvedValue(mockSettings);

    const state = new SettingsState();
    await state.load();

    expect(state.settings).toEqual(mockSettings);
    expect(invoke).toHaveBeenCalledWith('get_settings');
  });

  it('should update settings and persist', async () => {
    const updatedSettings = { language: 'it', currency: 'EUR' /* ... */ };
    vi.mocked(invoke).mockResolvedValue(updatedSettings);

    const state = new SettingsState();
    await state.update({ language: 'it' });

    expect(state.settings.language).toBe('it');
    expect(invoke).toHaveBeenCalledWith('update_settings', {
      args: { language: 'it' }
    });
  });
});
```

### Backend Tests

```rust
// src-tauri/src/settings/application/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_settings_validates_currency() {
        let result = update_settings_sync(UpdateSettingsInput {
            currency: Some("".to_string()), // Invalid: empty
            ..Default::default()
        });

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("currency"));
    }

    #[test]
    fn test_update_settings_merges_partial_update() {
        let current = UserSettings {
            currency: "EUR".to_string(),
            language: Language::English,
            // ... other fields
        };

        let input = UpdateSettingsInput {
            language: Some(Language::Italian),
            ..Default::default()
        };

        let updated = apply_updates(current, input);

        assert_eq!(updated.currency, "EUR"); // Unchanged
        assert_eq!(updated.language, Language::Italian); // Updated
    }
}
```

---

## Performance Tips

1. **Batch updates**: Update multiple settings in one `update()` call instead of multiple calls

   ```typescript
   // Good: Single update
   await settingsState.update({ currency: 'USD', language: 'it' });

   // Bad: Multiple updates
   await settingsState.update({ currency: 'USD' });
   await settingsState.update({ language: 'it' });
   ```

2. **Avoid unnecessary reads**: Settings are cached in memory, but avoid reading in tight loops

   ```typescript
   // Good: Read once
   const { currency } = settingsState.settings;
   for (const item of items) {
     formatPrice(item.price, currency);
   }

   // Bad: Read in loop
   for (const item of items) {
     formatPrice(item.price, settingsState.settings.currency);
   }
   ```

3. **Use reactive effects sparingly**: Only use `$effect` for side effects, not for derived values

   ```typescript
   // Good: Derived value
   const formattedPrice = $derived(formatPrice(price, settingsState.settings.currency));

   // Bad: Effect for derived value
   let formattedPrice = $state('');
   $effect(() => {
     formattedPrice = formatPrice(price, settingsState.settings.currency);
   });
   ```

---

## Troubleshooting

### Settings not persisting

**Symptom**: Settings reset to defaults after app restart

**Causes**:

1. Store file path is incorrect → Check plugin initialization
2. Permissions issue → Check file system permissions for app data directory
3. Save operation failing silently → Add error logging to save calls

**Solution**: Check browser console and Rust logs for errors

### Window position not restored

**Symptom**: Window always opens at default position

**Causes**:

1. `tauri-plugin-window-state` not initialized → Check `main.rs`
2. State file corrupted → Delete state file and restart
3. Multi-monitor setup changed → Plugin should handle this automatically

**Solution**: Verify plugin initialization order in `main.rs`

### Language changes not reflected in UI

**Symptom**: Language setting updates but UI text doesn't change

**Causes**:

1. Paraglide `setLanguageTag()` not called → Add effect to sync language
2. Some components not using Paraglide messages → Audit for hardcoded strings

**Solution**: Add language sync effect in `SettingsController`

---

## Further Reading

- [Tauri Store Plugin Documentation](https://v2.tauri.app/plugin/store/)
- [Tauri Window State Plugin Documentation](https://v2.tauri.app/plugin/window-state/)
- [Tauri OS Plugin Documentation](https://v2.tauri.app/plugin/os/)
- [Svelte 5 Runes Guide](https://svelte.dev/docs/svelte/$state)
- [Rusty Shed Architecture Guide](../../ARCHITECTURE.md) (if exists)
