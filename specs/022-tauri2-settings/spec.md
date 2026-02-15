# Feature Specification: Migrate Tauri 2 Settings

**Feature Branch**: `022-tauri2-settings`
**Created**: 2026-02-15
**Status**: Draft
**Input**: User description: "Migrate application settings management to Tauri 2 APIs with window state persistence and reactive settings updates"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Configure Application Preferences (Priority: P1)

As a user, I want to customize the application's display and behavior settings (currency, language, measurement units, favourite scale, power system) so that the application matches my regional preferences and collecting focus.

**Why this priority**: This is the core functionality of settings management. Without the ability to configure and persist preferences, users cannot personalize their experience or work with their preferred units and formats.

**Independent Test**: Can be fully tested by opening the Settings page, changing each setting, closing and reopening the application, and verifying all preferences are preserved.

**Acceptance Scenarios**:

1. **Given** the user is on the Settings page, **When** they change the currency setting, **Then** the new currency is immediately applied throughout the application and persists after restart
2. **Given** the user is on the Settings page, **When** they change the language setting, **Then** all UI text updates to the selected language without requiring a restart
3. **Given** the user changes the measure unit setting, **When** they view measurements anywhere in the app, **Then** all measurements display in the selected unit system
4. **Given** the user sets their favourite scale, **When** they browse rolling stock, **Then** the application highlights or filters items matching that scale preference
5. **Given** the user selects a power system preference, **When** they work with railway models, **Then** the application respects this preference in relevant contexts

---

### User Story 2 - Automatic Language Detection on First Run (Priority: P2)

As a new user launching the application for the first time, I want the application to automatically detect my operating system's language and use it if supported (English or Italian), defaulting to English otherwise, so I don't have to manually configure the language.

**Why this priority**: This enhances the first-run experience by providing immediate localization. However, it's secondary to the core settings functionality since users can always change the language manually.

**Independent Test**: Can be tested by clearing all application data, setting the OS to Italian, launching the app, and verifying it starts in Italian. Repeat with an unsupported language (e.g., French) and verify it defaults to English.

**Acceptance Scenarios**:

1. **Given** this is the user's first time launching the application and their OS language is Italian, **When** the application starts, **Then** the UI displays in Italian
2. **Given** this is the user's first time launching the application and their OS language is English, **When** the application starts, **Then** the UI displays in English
3. **Given** this is the user's first time launching the application and their OS language is unsupported (e.g., Spanish), **When** the application starts, **Then** the UI displays in English as the fallback
4. **Given** the user has previously run the application, **When** they launch it again, **Then** the application uses their saved language preference (not the OS language)

---

### User Story 3 - Window Position and Size Restoration (Priority: P3)

As a user, I want the application to remember where I positioned the window and its size, so that when I reopen the application, it appears exactly where I left it.

**Why this priority**: This is a quality-of-life improvement that enhances the user experience but is not critical to the core functionality. Users can manually resize and reposition the window if needed.

**Independent Test**: Can be tested by launching the app, resizing and moving the window to a specific position, closing the app, relaunching it, and verifying the window appears in the same position and size.

**Acceptance Scenarios**:

1. **Given** the user has resized the application window, **When** they close and reopen the application, **Then** the window opens with the same dimensions
2. **Given** the user has moved the application window to a different screen position, **When** they close and reopen the application, **Then** the window opens at the same screen coordinates
3. **Given** the user has maximized the application window, **When** they close and reopen the application, **Then** the window opens in maximized state
4. **Given** the user's display configuration has changed (e.g., disconnected external monitor), **When** they open the application, **Then** the window opens at a valid position on the available display

---

### Edge Cases

- What happens when the stored window position is off-screen (e.g., user disconnected a monitor)? The application should detect this and reposition the window to a visible location on the primary display.
- What happens when settings file is corrupted or missing? The application should create new settings with sensible defaults (OS language or English, default currency, etc.).
- What happens when a user manually edits the settings file to invalid values? The application should validate settings on load and reset invalid values to defaults.
- What happens when settings are changed simultaneously from multiple sources (if applicable)? The most recent change should win, and all UI should update reactively.
- What happens on first run when OS language detection fails? The application should default to English and log the issue.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST persist all user settings using Tauri 2's official settings API or Store plugin (no SQLite table)
- **FR-002**: System MUST provide IPC commands for reading user settings from the frontend
- **FR-003**: System MUST provide IPC commands for writing/updating user settings from the frontend
- **FR-004**: Settings changes MUST be reactive: when a setting is updated, all parts of the application displaying or using that setting MUST update immediately without requiring an application restart
- **FR-005**: System MUST support a currency setting (user-selectable currency preference)
- **FR-006**: System MUST support a language setting with options for English and Italian
- **FR-007**: On first application run, system MUST detect the operating system's default language
- **FR-008**: If the OS language is supported (English or Italian), system MUST set the application language to match; otherwise, system MUST default to English
- **FR-009**: System MUST support a measure unit setting (imperial/metric or similar units relevant to model railways)
- **FR-010**: System MUST support a favourite scale setting (preferred model railway scale)
- **FR-011**: System MUST support a power system setting (DC/AC/DCC or relevant power systems for model railways)
- **FR-012**: System MUST track whether this is the user's first run using a boolean flag stored in settings (not exposed in the Settings UI)
- **FR-013**: System MUST persist the main application window's size (width and height)
- **FR-014**: System MUST persist the main application window's position (x and y coordinates)
- **FR-015**: On application startup, system MUST restore the window to its previously saved size and position
- **FR-016**: If saved window position is off-screen or invalid, system MUST reposition the window to a visible location
- **FR-017**: All user-facing text in the Settings page and related components MUST use Paraglide-JS for localization
- **FR-018**: Settings implementation MUST be covered by unit tests
- **FR-019**: Settings implementation MUST be covered by integration tests verifying IPC commands and persistence
- **FR-020**: Code MUST follow the project's clean architecture patterns (domain, application, infrastructure, interface layers)

### Key Entities

- **UserSettings**: Represents the user's application preferences
  - Contains: currency (string or enum), language (enum: English/Italian), measureUnit (enum or string), favouriteScale (string or enum), powerSystem (enum or string), firstRun (boolean)
  - Persisted using Tauri 2 settings mechanism
  - Accessible via IPC commands from frontend
  - Changes trigger reactive updates across the application

- **WindowState**: Represents the main application window's display state
  - Contains: width (number), height (number), x (number), y (number), isMaximized (boolean, if applicable)
  - Persisted using Tauri 2 settings or window state API
  - Restored automatically on application startup
  - Validated to ensure window appears on a visible display

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: When a user changes any setting on the Settings page, the change is persisted and survives an application restart (100% persistence across restarts)
- **SC-002**: When a user changes a setting, all UI components reflecting that setting update within 500ms without requiring an application restart (reactive updates)
- **SC-003**: When a user resizes or repositions the window and relaunches the application, the window appears within 10 pixels of the saved position and size (95% accuracy accounting for OS window manager variations)
- **SC-004**: On first run with an Italian OS locale, the application starts in Italian; with an English OS locale, it starts in English; with any other locale, it defaults to English (100% correct language detection)
- **SC-005**: All settings functionality is covered by automated tests with at least 80% code coverage for the settings module
- **SC-006**: Settings page displays all translatable text in the user's selected language using Paraglide-JS (zero hardcoded English strings in UI)
- **SC-007**: When window position is invalid (off-screen), the application successfully repositions the window to a visible location on launch (100% recovery from invalid positions)
