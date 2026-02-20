# Feature Specification: Rich Text Editor for RailwayModelCard

**Feature Branch**: `025-rich-text-editor`
**Created**: 2026-02-20
**Status**: Draft
**Input**: User description: "Feature 25: rich text editor for the RailwayModelCard — seamless display/editor toggle, bold/italic/lists, Markdown persistence, plain-text paste, placeholder, typography consistency, and a floating toolbar."

---

## User Scenarios & Testing _(mandatory)_

### User Story 1 — View and Edit Model Details in Rich Text (Priority: P1)

A collector opens a railway model card that already has some notes in the Details tab. The content is rendered as formatted text (bold headings, bullet points). The collector clicks anywhere in the text area to enter edit mode; a lightweight toolbar appears and the content becomes editable. The collector makes a change, clicks outside the editor (blur), and the changes are saved automatically. The editor returns to Display Mode and the updated content is rendered.

**Why this priority**: This is the core interaction loop. Without a functioning Display → Edit → Save cycle, no other capability can be exercised.

**Independent Test**: Can be fully tested by loading an existing model card with details text, clicking to activate the editor, typing new content, clicking outside, and verifying the persisted content re-renders correctly in Display Mode. Delivers the complete viewing and editing value immediately.

**Acceptance Scenarios**:

1. **Given** a model card with existing details text, **When** the user hovers over the text area, **Then** a subtle hover indicator (border glow or background shift) signals the area is interactive.
2. **Given** hover state is active, **When** the user clicks anywhere in the text area, **Then** the Display Mode transitions instantly to Editor Mode with a formatting toolbar visible.
3. **Given** Editor Mode is active and the user has made changes, **When** the user clicks outside the editor boundary, **Then** changes are saved automatically without a confirmation prompt.
4. **Given** Editor Mode is active, **When** the user has made no changes and clicks outside, **Then** the editor returns to Display Mode silently with no save operation triggered.
5. **Given** changes have been saved, **When** the editor returns to Display Mode, **Then** the formatted content is correctly rendered (bold, italic, lists) without a visible layout shift.

---

### User Story 2 — Apply Rich Formatting (Bold, Italic, Lists) (Priority: P2)

A collector enters Editor Mode and uses the toolbar to apply bold formatting to a locomotive name, italic to a note about a limited edition run, and creates a bulleted list of maintenance tasks. The underlying data stored in the database is valid Markdown, so it can be rendered or exported in the future.

**Why this priority**: Formatting is the primary differentiator between a plain textarea and a rich text editor. It unlocks the structured note-taking use cases described in the feature request.

**Independent Test**: Can be tested by entering Editor Mode, selecting text and clicking Bold, clicking Italic, and inserting a bulleted list. After saving, verify that the Display Mode renders bold, italic, and list formatting correctly. Verify the database stores well-formed Markdown.

**Acceptance Scenarios**:

1. **Given** Editor Mode is active, **When** the user selects text and activates Bold, **Then** the selected text is displayed in bold and the stored value contains the Markdown bold syntax (`**text**`).
2. **Given** Editor Mode is active, **When** the user selects text and activates Italic, **Then** the selected text is italicised and the stored value contains the Markdown italic syntax (`*text*`).
3. **Given** Editor Mode is active, **When** the user inserts an unordered list, **Then** bullet points are rendered and the stored value contains Markdown list syntax (`- item`).
4. **Given** Editor Mode is active, **When** the user inserts an ordered list, **Then** numbered items are rendered and the stored value contains Markdown ordered list syntax (`1. item`).
5. **Given** formatted content has been saved, **When** the model card reloads from the database, **Then** all formatting is preserved and rendered correctly.

---

### User Story 3 — Paste Content from External Source (Priority: P3)

A collector copies a locomotive description from a manufacturer's website (e.g. Hornby or Bachmann) and pastes it into the editor. Instead of importing messy HTML tags, inline styles, or non-standard characters, the editor strips web-specific styling and retains only the readable text with its basic structure (paragraph breaks and simple formatting).

**Why this priority**: Collectors commonly source content from manufacturer pages. Dirty paste would corrupt the Markdown store and break rendering.

**Independent Test**: Can be tested by copying styled HTML text from a web page, pasting into the editor, and verifying that the stored Markdown contains no HTML tags or inline styles. Basic paragraph structure should be preserved.

**Acceptance Scenarios**:

1. **Given** Editor Mode is active, **When** the user pastes HTML-rich content from a web page, **Then** the pasted result contains plain text and basic structure only — no HTML tags, inline styles, or non-standard whitespace.
2. **Given** Editor Mode is active, **When** the user pastes plain text, **Then** the text is inserted as-is without modification.
3. **Given** a paste operation produces a very large block of text, **When** it is inserted, **Then** the editor expands vertically to accommodate the content without adding a scrollbar within the card.

---

### User Story 4 — Empty Model Card Shows Placeholder Text (Priority: P4)

A collector adds a new railway model with no details yet. When they open the Details tab, they see a helpful placeholder message guiding them to enter content. The placeholder disappears as soon as they click to enter Edit Mode.

**Why this priority**: Placeholder text reduces friction for new entries by providing in-context guidance, but it does not block any other feature from working.

**Independent Test**: Can be tested by creating a model with an empty details field, navigating to the Details tab, and verifying placeholder text is visible. Clicking the area should show the editor with an empty state (no placeholder visible), and saving empty content should return to Display Mode with the placeholder visible again.

**Acceptance Scenarios**:

1. **Given** a model card with no details content, **When** the Details tab is displayed, **Then** a placeholder message (e.g., "Add maintenance notes, DCC addresses, or other details…") is visible in Display Mode.
2. **Given** the placeholder is showing, **When** the user clicks the text area, **Then** Editor Mode opens with an empty editable area (the placeholder is not present in the editor itself).
3. **Given** Editor Mode is open and the user saves without entering content, **When** the editor returns to Display Mode, **Then** the placeholder is shown again.

---

### Edge Cases

- What happens when saving fails (e.g., IPC error to the Tauri backend)? The editor must not discard the user's unsaved content and must surface a visible error notification.
- What happens if the details content is extremely long (e.g., 5,000+ characters)? The editor must expand vertically and remain performant without truncating content or showing an internal scrollbar.
- What happens when the user switches tabs mid-edit (e.g., from Details to Rolling Stock)? The editor should auto-save before the tab switches, or stage the content so it is not lost.
- What happens with content that contains Markdown control characters (e.g., `*`, `_`, `#`) entered as literal text? The editor must handle escaping so the rendered Display Mode matches the user's intent.
- What happens when the model card's `editable` prop is false? The rich text area must render in Display Mode only — no hover affordance, no ability to enter Editor Mode.

---

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST display the model's `details` field as rendered rich text (bold, italic, lists presented as styled content) when in Display Mode.
- **FR-002**: The system MUST transition from Display Mode to Editor Mode when the user clicks anywhere within the text area, with no perceptible delay.
- **FR-003**: The system MUST display a subtle visual affordance (hover border highlight or background shift) when the cursor is positioned over an editable text area in Display Mode.
- **FR-004**: The system MUST auto-save the content and return to Display Mode when the editor loses focus, provided the `editable` prop is true.
- **FR-005**: The system MUST NOT trigger a save operation when the user exits the editor without making any changes.
- **FR-006**: The system MUST preserve unsaved content in the editor if an auto-save operation fails, and MUST display a visible error notification.
- **FR-007**: The system MUST support applying **Bold** formatting to selected text, with the stored value containing Markdown bold syntax.
- **FR-008**: The system MUST support applying **Italic** formatting to selected text, with the stored value containing Markdown italic syntax.
- **FR-009**: The system MUST support inserting and editing **unordered (bulleted) lists**, stored as Markdown list syntax.
- **FR-010**: The system MUST support inserting and editing **ordered (numbered) lists**, stored as Markdown ordered list syntax.
- **FR-011**: The system MUST store all content as valid Markdown in the database, regardless of how it was entered or formatted visually.
- **FR-012**: When content is pasted from an external source containing HTML or web styling, the system MUST strip HTML tags and inline styles, retaining only readable text and basic paragraph structure.
- **FR-013**: The system MUST display a configurable placeholder message in Display Mode when the `details` field is empty or null.
- **FR-014**: The editor area MUST expand vertically to accommodate content rather than displaying a scrollbar within the card boundary.
- **FR-015**: The editor MUST inherit the font family and line-height of the surrounding card, preventing layout shifts when switching between Display Mode and Editor Mode.
- **FR-016**: When the `editable` prop is false, the system MUST render the text area in Display Mode only, with no hover affordance and no ability to enter Editor Mode.
- **FR-017**: The formatting toolbar MUST remain accessible while editing — either as a persistent bar at the top of the editor area, or as a contextual bar that appears when text is selected.

### Key Entities

- **ModelDetails**: The multi-line text content for a railway model (`details` field on `RailwayModel`). Stored as Markdown. May be null or empty for new models. Editable in the Details tab of `RailwayModelCard`.
- **RichTextEditor**: The component that manages the Display/Editor Mode toggle, the formatting toolbar, paste normalisation, and persistence of `ModelDetails`.
- **FormattingToolbar**: The set of formatting controls (Bold, Italic, Unordered List, Ordered List) accessible during Editor Mode.

---

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A collector can click into the Details area and begin typing within 200 milliseconds of the click, with no noticeable lag in the Display → Editor Mode transition.
- **SC-002**: A collector can apply bold, italic, or list formatting using the toolbar in 2 interactions or fewer (select text → activate format).
- **SC-003**: When the editor loses focus, content is auto-saved and Display Mode is restored in under 500 milliseconds, with no visible layout shift.
- **SC-004**: Pasting richly styled content from a manufacturer's web page produces a clean result — no HTML tags or inline styles are visible in Display Mode or detectable in the stored Markdown.
- **SC-005**: All stored content round-trips correctly: content written in Editor Mode is rendered identically in Display Mode after a full application reload.
- **SC-006**: The editor area handles at least 5,000 characters of content without displaying a scrollbar within the card — the editor grows vertically to fit.
- **SC-007**: When `editable` is false, the rendered text area is visually indistinguishable from surrounding card content — no hover effects or interactive cues are shown.
- **SC-008**: When a save error occurs, no existing formatting or content is lost — the user can retry or continue editing from the same state.

---

## Assumptions

- The feature targets the `details` field in the Details tab of `RailwayModelCard`. The `description` field in the card header remains a single-line plain-text field handled by the existing `InPlaceEdit` component and is out of scope.
- The underlying database schema requires no changes — `details` is already a nullable text column, and Markdown is valid text content.
- Auto-save on blur is the correct save strategy; no explicit Save button is required. This is consistent with the existing `InPlaceEdit` pattern in the codebase.
- The initial formatting set is: Bold, Italic, Unordered List, and Ordered List. Heading levels (H1/H2/H3) are out of scope to keep the editor focused on short-form notes rather than long-form documents.
- Plain-text paste normalisation applies when pasting from external sources (websites, word processors). Pasting within the same editor or between Markdown-aware tools should not alter the content.
- The feature is scoped to `RailwayModelCard`. Other text fields in the application are not in scope.
