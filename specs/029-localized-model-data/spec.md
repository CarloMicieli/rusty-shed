# Feature Specification: Localized Railway Model Data

**Feature Branch**: `029-localized-model-data`
**Created**: 2026-02-25
**Status**: Draft
**Input**: User description: "Implement localized data management for railway models per ADR-0009. Railway model description and details fields must support English and Italian translations using a relational translation table (railway_model_translations) with FTS5 search index. English is always the fallback language. All other text fields (notes, rich text editor content) remain monolingual and save in the current user language. Only two languages are in scope: English (en) and Italian (it)."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Model in My Language (Priority: P1)

A collector opens a railway model in the catalogue. The model's description and details are displayed in their configured language (Italian). For any model where an Italian translation does not exist, the English text is shown automatically so the collector always sees meaningful content.

**Why this priority**: This is the core user-visible outcome of the feature. Without correct language display, every other story has no observable value.

**Independent Test**: Can be fully tested by switching the app language to Italian, viewing a model that has both Italian and English translations, and verifying Italian text appears. Then viewing a model with only English and verifying English text appears without errors.

**Acceptance Scenarios**:

1. **Given** a railway model with both English and Italian translations and the app set to Italian, **When** the user opens that model, **Then** the Italian description and details are displayed.
2. **Given** a railway model with only an English translation and the app set to Italian, **When** the user opens that model, **Then** the English description and details are displayed (fallback), and the interface indicates the content is in English.
3. **Given** the app is set to English, **When** the user opens any model that has an English translation, **Then** the English description and details are displayed without any fallback indicator.

---

### User Story 2 - Add Translations When Creating a Model (Priority: P2)

A collector creates a new railway model in the catalogue. The creation form provides separate input fields for the English description and details, and optional fields for the Italian equivalents. The English description is required; all other fields are optional.

**Why this priority**: Data entry is the primary way translations enter the system. Without this story, the system has no localized content to display.

**Independent Test**: Can be fully tested by creating a new railway model, filling in the English description, leaving Italian fields blank, saving, and verifying the model is saved and displayed correctly in both English and Italian language modes.

**Acceptance Scenarios**:

1. **Given** the model creation form, **When** the user fills in only the English description and saves, **Then** the model is saved successfully with the English translation and no Italian translation is stored.
2. **Given** the model creation form, **When** the user fills in both English and Italian descriptions and saves, **Then** the model is saved with both translations, each independently retrievable.
3. **Given** the model creation form, **When** the user attempts to save without any English description, **Then** the save is rejected with a clear validation message asking for the English description.
4. **Given** the model creation form, **When** the user fills in the Italian description but leaves English description blank, **Then** the save is rejected — English description is required.

---

### User Story 3 - Edit Existing Translations (Priority: P3)

A collector opens an existing railway model and updates its description or details. Existing translations for English and Italian are displayed in their respective input fields. The collector can update either language independently without affecting the other.

**Why this priority**: Collection data evolves over time; existing records need to be correctable and enrichable with new language content.

**Independent Test**: Can be fully tested by editing a model that has only an English translation, adding an Italian description, saving, and verifying both translations are stored and displayed correctly.

**Acceptance Scenarios**:

1. **Given** a model with only an English translation, **When** the user opens the edit form, **Then** the English fields are pre-filled and the Italian fields are empty.
2. **Given** a model with only an English translation, **When** the user adds an Italian description and saves, **Then** the model now has both translations, and the Italian content is shown when the app is set to Italian.
3. **Given** a model with both translations, **When** the user updates the English description and saves, **Then** only the English translation is changed; the Italian translation is unaffected.
4. **Given** a model with both translations, **When** the user clears the Italian description and saves, **Then** the Italian translation is removed, and the English fallback is shown in Italian mode.

---

### User Story 4 - Search Models Across Languages (Priority: P4)

A collector uses the search feature to find a railway model by entering a word or phrase. The search returns matching models regardless of which language the search term is in — a search in Italian finds Italian-translated models, and a search in English finds English-translated models.

**Why this priority**: Search is a primary navigation tool in a large collection, and its correctness across languages validates the integrity of the underlying data storage.

**Independent Test**: Can be fully tested by adding a model with unique English and Italian descriptions, then searching for a word unique to each language and confirming the model appears in both result sets.

**Acceptance Scenarios**:

1. **Given** a model with an Italian description containing the word "locomotiva", **When** the user searches for "locomotiva", **Then** that model appears in the search results.
2. **Given** a model with an English description containing "steam locomotive", **When** the user searches for "steam", **Then** that model appears in the results regardless of the current app language.
3. **Given** a search query with no matches in any language, **When** the user submits the search, **Then** an empty result set is returned with no error.

---

### User Story 5 - Non-Localized Fields Are Unaffected (Priority: P5)

A collector adds or edits notes and rich-text content (e.g., maintenance history, personal remarks) for a railway model. These fields are not subject to any language selection — they are saved exactly as typed, in whatever language the user writes them.

**Why this priority**: Confirming non-localized fields are untouched is a scope-boundary validation story. It prevents regression and clarifies what the feature does not change.

**Independent Test**: Can be fully tested by entering notes in Italian while the app is set to Italian, then switching to English and verifying the same Italian text still appears in the notes field unchanged.

**Acceptance Scenarios**:

1. **Given** the app is set to Italian and a model has notes written in Italian, **When** the user switches the app to English, **Then** the notes still display the original Italian text unchanged.
2. **Given** the model edit form, **When** the user types notes in any language and saves, **Then** the notes are stored and retrieved verbatim with no language transformation.

---

### Edge Cases

- What happens when a model has an Italian translation for description but no Italian translation for details? → Each field falls back independently: Italian description is shown; English details are shown as fallback.
- What happens if a user switches the app language while a model detail view is open? → The displayed description and details refresh to reflect the new language (or fallback), without requiring a manual reload.
- What happens if both the Italian and English translations are absent for a model's description? → The description field displays as empty; the model remains accessible through other fields (product code, manufacturer).
- What happens if a very long description is provided in one language but not the other? → No length constraint difference between languages; the fallback shows the content regardless of length.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST store description and details for each railway model independently per supported language (English and Italian).
- **FR-002**: System MUST display description and details in the user's currently configured language when a translation for that language exists.
- **FR-003**: System MUST automatically display the English translation when no translation exists in the user's configured language for a given field.
- **FR-004**: System MUST visually indicate to the user when displayed content is shown as an English fallback rather than in the user's configured language.
- **FR-005**: System MUST require an English description when creating a new railway model; all other localized fields (Italian description, English/Italian details) are optional.
- **FR-006**: System MUST allow users to add, update, or remove Italian translations for description and details on any existing railway model.
- **FR-007**: System MUST support text search across railway model descriptions and details in both English and Italian simultaneously.
- **FR-008**: System MUST NOT apply language selection or fallback logic to notes or rich-text content fields; those fields are stored and retrieved verbatim.
- **FR-009**: System MUST associate each stored translation with its language code so the interface can determine display language and fallback state at retrieval time.
- **FR-010**: System MUST handle each localized field (description, details) independently — fallback for one field does not imply fallback for the other on the same model.

### Key Entities

- **Railway Model**: The core catalogue entry representing a physical model item. Has structured fields (product code, manufacturer, scale, etc.) and two localized text fields: description and details.
- **Railway Model Translation**: A language-specific version of a model's description and details. Belongs to exactly one Railway Model and one language. A model may have zero to two translations (one per supported language). A translation with no text for a field is treated as absent for that field.
- **Supported Language**: One of the two permitted language codes — English (`en`) and Italian (`it`). English is designated as the mandatory fallback language.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: 100% of railway model views show description and details in the user's configured language when a translation for that language exists — no missing-translation errors are visible to the user.
- **SC-002**: When a model has no Italian translation, English content is shown automatically in all cases — users in Italian mode never see a blank description for a model that has an English translation.
- **SC-003**: Users can create a model with only an English description and later add or remove the Italian translation in a single editing session, with changes persisted on save.
- **SC-004**: Search results for terms entered in either English or Italian are returned in under 2 seconds for a collection of up to 10,000 railway models.
- **SC-005**: Notes and rich-text fields are unaffected — 100% of existing note content is preserved verbatim and displayed identically regardless of the active app language.
- **SC-006**: A fallback indicator is visible to the user in every case where English content is being shown in place of a missing Italian translation.

## Assumptions

- The two supported languages are English (`en`) and Italian (`it`); no other language codes are in scope for this feature.
- English is the mandatory fallback. The system assumes an English translation will always be available for any model visible in the catalogue; enforcement at creation time is sufficient to maintain this invariant.
- Fallback applies at the individual field level: a model may have an Italian description but fall back to English for details, or vice versa.
- Notes and rich-text content (e.g., maintenance history) are personal, free-form entries with no language selection requirement.
- The user's configured language is a single app-level setting (not per-model); switching it affects all models simultaneously.
- Italian is optional at all times — no validation error occurs if the Italian fields are left blank.
- Deleting an Italian translation returns the display for that field to the English fallback; it does not affect the English translation.
- The feature does not introduce any changes to non-catalogue domains (wishlist, budget, DCC inventory, etc.).
