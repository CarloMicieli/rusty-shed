# Data Model: Modern Steampunk Theme System

**Feature**: 011-steampunk-theme  
**Created**: 2026-01-30  
**Status**: Complete

## Entities

### 1. ThemeValue (Enum)

Represents the user's theme preference choice.

| Value             | Description                     |
| ----------------- | ------------------------------- |
| `steampunk-light` | Light "Parchment & Brass" theme |
| `steampunk-dark`  | Dark "Iron & Copper" theme      |
| `system`          | Follow OS preference (default)  |

**TypeScript Definition**:

```typescript
type ThemeValue = 'steampunk-light' | 'steampunk-dark' | 'system';
```

**Rust Definition**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeValue {
    SteampunkLight,
    SteampunkDark,
    System,
}

impl Default for ThemeValue {
    fn default() -> Self {
        Self::System
    }
}
```

### 2. ResolvedTheme (Enum)

The actual theme applied after resolving system preference.

| Value   | Description           |
| ------- | --------------------- |
| `light` | Light theme is active |
| `dark`  | Dark theme is active  |

**TypeScript Definition**:

```typescript
type ResolvedTheme = 'light' | 'dark';
```

### 3. ThemeState (Store State)

Frontend state for theme management.

| Field       | Type            | Description                          |
| ----------- | --------------- | ------------------------------------ |
| `current`   | `ThemeValue`    | User's stored preference             |
| `resolved`  | `ResolvedTheme` | Actual theme after system resolution |
| `isLoading` | `boolean`       | Loading state during initialization  |

**TypeScript Definition**:

```typescript
interface ThemeState {
  current: ThemeValue;
  resolved: ResolvedTheme;
  isLoading: boolean;
}
```

### 4. SettingsDto (Extended)

Existing settings DTO with new theme field.

| Field                 | Type          | Required | Description                 |
| --------------------- | ------------- | -------- | --------------------------- |
| `id`                  | `number`      | Yes      | Always 1 (singleton)        |
| `currency`            | `Currency`    | Yes      | User's preferred currency   |
| `lengthUnit`          | `MeasureUnit` | Yes      | Measurement unit preference |
| `favoriteScale`       | `Scale`       | Yes      | Preferred model scale       |
| `favoritePowerMethod` | `PowerMethod` | Yes      | DC/AC/DCC preference        |
| `languageCode`        | `string`      | Yes      | UI language code            |
| `theme`               | `ThemeValue`  | Yes      | **NEW**: Theme preference   |

### 5. UpdateSettingsPayload (Extended)

Existing update payload with new theme field.

| Field                 | Type          | Required | Description                 |
| --------------------- | ------------- | -------- | --------------------------- |
| `currency`            | `Currency`    | Yes      | User's preferred currency   |
| `lengthUnit`          | `MeasureUnit` | Yes      | Measurement unit preference |
| `favoriteScale`       | `Scale`       | Yes      | Preferred model scale       |
| `favoritePowerMethod` | `PowerMethod` | Yes      | DC/AC/DCC preference        |
| `languageCode`        | `string`      | Yes      | UI language code            |
| `theme`               | `ThemeValue`  | Yes      | **NEW**: Theme preference   |

---

## Database Schema

### Migration: `0007_add_theme_setting.sql`

```sql
-- Add theme setting column with system as default
ALTER TABLE settings ADD COLUMN theme TEXT NOT NULL DEFAULT 'system';
```

### Updated Settings Table

| Column                  | Type    | Constraints                 | Description                                      |
| ----------------------- | ------- | --------------------------- | ------------------------------------------------ |
| `id`                    | INTEGER | PRIMARY KEY, CHECK (id = 1) | Singleton constraint                             |
| `currency`              | TEXT    | NOT NULL                    | Currency code (EUR, USD, etc.)                   |
| `length_unit`           | TEXT    | NOT NULL                    | MILLIMETERS, INCHES                              |
| `favorite_scale`        | TEXT    | NOT NULL                    | H0, N, etc.                                      |
| `favorite_power_method` | TEXT    | NOT NULL                    | DC, AC, DCC                                      |
| `language_code`         | TEXT    | NOT NULL                    | en, it, etc.                                     |
| `theme`                 | TEXT    | NOT NULL, DEFAULT 'system'  | **NEW**: steampunk-light, steampunk-dark, system |

---

## State Transitions

### Theme Resolution Flow

```
┌─────────────────┐
│  App Mounts     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Load from Tauri │
│ get_settings()  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────┐
│ themeStore.current = response   │
└────────┬────────────────────────┘
         │
         ▼
    ┌────┴────┐
    │ system? │
    └────┬────┘
    YES  │  NO
    ▼    ▼
┌───────────────┐  ┌─────────────────────┐
│ Check OS via  │  │ resolved = current  │
│ matchMedia()  │  │ (light or dark)     │
└───────┬───────┘  └──────────┬──────────┘
        │                     │
        ▼                     │
┌───────────────────┐         │
│ resolved = OS     │         │
│ preference        │◄────────┘
└───────┬───────────┘
        │
        ▼
┌────────────────────────────┐
│ document.body.dataset.theme│
│ = `steampunk-${resolved}`  │
└────────────────────────────┘
```

### Theme Change Flow

```
┌─────────────────────┐
│ User selects theme  │
│ in Settings page    │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Call update_settings│
│ with new theme      │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ themeStore.current  │
│ = new value         │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Re-resolve theme    │
│ (same as init flow) │
└─────────────────────┘
```

---

## Validation Rules

### ThemeValue Validation

| Rule         | Constraint                                                    |
| ------------ | ------------------------------------------------------------- |
| Valid values | Must be one of: `steampunk-light`, `steampunk-dark`, `system` |
| Default      | `system` when not specified                                   |
| Persistence  | Stored as lowercase kebab-case in SQLite                      |

### Rust Parsing

```rust
impl FromStr for ThemeValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "steampunk-light" => Ok(Self::SteampunkLight),
            "steampunk-dark" => Ok(Self::SteampunkDark),
            "system" => Ok(Self::System),
            _ => Err(format!("Invalid theme value: {}", s)),
        }
    }
}
```

---

## Relationships

```
┌──────────────────┐
│     Settings     │
│    (SQLite)      │
├──────────────────┤
│ theme: TEXT      │──────┐
└──────────────────┘      │
                          │
                          ▼
                   ┌──────────────┐
                   │  ThemeValue  │
                   │    (Enum)    │
                   └──────┬───────┘
                          │
              ┌───────────┼───────────┐
              │           │           │
              ▼           ▼           ▼
        ┌─────────┐ ┌─────────┐ ┌─────────┐
        │  light  │ │  dark   │ │ system  │
        └────┬────┘ └────┬────┘ └────┬────┘
             │           │           │
             └─────┬─────┘           │
                   │                 │
                   ▼                 ▼
            ┌──────────────┐  ┌──────────────┐
            │ResolvedTheme │  │ OS Preference│
            │ (light/dark) │◄─│  Detection   │
            └──────────────┘  └──────────────┘
                   │
                   ▼
            ┌──────────────────────┐
            │ document.body        │
            │ data-theme attribute │
            └──────────────────────┘
```
