# Feature Specification: Welcome Page

**Feature Branch**: `042-welcome-page`
**Created**: May 20, 2026
**Status**: Draft
**Input**: User description for first-run onboarding wizard with 3-step flow and Tauri/Svelte integration.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - First-Run Detection (Priority: P1)

As a new user, when I launch the app for the first time, I want to be greeted with a Welcome Wizard so that I can set up my preferences and start using the app effectively.

**Why this priority**: Ensures users have a personalized experience from the start and avoids dashboard flash before onboarding state is known.

**Independent Test**: Launch the app with `has_completed_onboarding` set to `false` or `null` and verify that the Welcome Wizard is displayed.

**Acceptance Scenarios**:

1. **Given** app launch, **When** `has_completed_onboarding` is `false` or `null`, **Then** the Welcome Wizard is rendered instead of the main shell.
2. **Given** app launch, **When** `has_completed_onboarding` is `true`, **Then** onboarding is skipped and the main shell is rendered.

---

### User Story 2 - Regional & Appearance Setup (Priority: P1)

As a user, I want to select my preferred theme and language so that the app feels tailored to my preferences.

**Why this priority**: These are foundational settings that shape immediate usability and localization.

**Independent Test**: Complete Step 1 and verify selected theme and language are persisted and applied.

**Acceptance Scenarios**:

1. **Given** Step 1, **When** I select theme and language, **Then** those values are saved in onboarding state.
2. **Given** Step 1 completion, **When** onboarding persists settings, **Then** theme and language are updated through settings persistence.

---

### User Story 3 - Collector's Core Setup (Priority: P2)

As a railway model collector, I want to specify my favorite scale, measurement unit, and power method so that the app can provide relevant defaults.

**Why this priority**: This defines domain-specific operational defaults used across inventory workflows.

**Independent Test**: Complete Step 2 and verify scale, measurement unit, and power method are persisted.

**Acceptance Scenarios**:

1. **Given** Step 2, **When** I choose scale, measurement unit, and power method, **Then** all values are saved.
2. **Given** Step 2, **When** I use keyboard controls (Enter/arrow keys), **Then** I can select options and advance without mouse-only interaction.

---

### User Story 4 - Archive/Sync Setup (Priority: P3)

As a user, I want to import or restore data from archive/cloud so I can quickly start with existing collection data, or skip and start fresh.

**Why this priority**: Supports migration while guaranteeing a non-blocking path into the app.

**Independent Test**: On Step 3, verify local import, Google Drive restore, and skip each complete without corrupting onboarding state.

**Acceptance Scenarios**:

1. **Given** Step 3, **When** I import a `.json` or `.db` archive, **Then** data import is attempted and onboarding can complete on success.
2. **Given** Step 3, **When** I choose Google Drive restore, **Then** OAuth, backup selection, and restore are executed with inline error feedback on failure.
3. **Given** Step 3, **When** I choose Skip and Start Fresh, **Then** `has_completed_onboarding` is set to `true` and the main shell loads.

## Technical Requirements & Logic

### First-Run Detection

- Canonical key: `has_completed_onboarding`.
- Semantic rule: `true` means onboarding complete, `false`/`null` means onboarding required.
- On app launch, resolve onboarding status before dashboard mount.
- If onboarding is required, intercept default shell rendering and mount Welcome Wizard layout.

## Step-by-Step UI Flow Design

### Step 1: Regional & Appearance (The Basics)

- Theme Selection (Light vs Dark) with large selectable cards.
- Language selection (visual labels/grid or selector).

### Step 2: The Collector's Core (Modeling)

- Favorite scale selection (HO, N, O, OO, Z, G, etc.).
- Measurement unit selection (Metric vs Imperial).
- Power method selection (DC, DCC, AC).

### Step 3: Archive / Sync (The Technical Stuff)

- Import from JSON/DB Archive.
- Connect and restore from Google Drive.
- Skip and Start Fresh action.

## Design & UX Best Practices for Tailwind 4

- Visual scale examples with ratio badges (for example `HO 1:87`).
- Smooth step transitions using transform-based transitions (`transition-transform duration-150 ease-out`).
- Keyboard-friendly interaction (`Enter` to proceed, arrow keys for grouped selectors).
- Busy-state locking during import/restore to avoid duplicate triggers.

## Non-Functional Requirements

- Startup gating must avoid dashboard flash.
- Initialization surface should render within 100ms from webview-ready in typical environment.
- All user-facing strings must be Paraglide-backed in both English and Italian.
- Wizard transitions should remain smooth under normal desktop rendering workload.
