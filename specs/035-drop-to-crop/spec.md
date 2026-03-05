# Feature Specification: Drop-to-Crop Railway Model Image Workflow

**Feature Branch**: `035-drop-to-crop`
**Created**: 2026-03-05
**Status**: Draft
**Input**: User description: "Drop-to-Crop Railway Model Image Workflow — frictionless drag-and-drop into a crop dialog before saving a standardized model photo."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Drag Image Onto Model Photo Zone (Priority: P1)

A collector is browsing their catalog and wants to add a photo for a model. They drag an image file from their OS file manager and drop it directly onto the model's photo area. The system immediately opens a crop dialog pre-loaded with that image. The collector adjusts the crop, confirms, and the image is saved as the model's photo.

**Why this priority**: This is the core value proposition of the feature — eliminating the browse-click-select flow with a single drag gesture.

**Independent Test**: Can be fully tested by dragging a JPEG onto the model photo zone, completing the crop, and verifying the saved image appears in the catalog entry. Delivers end-to-end image import value without any other story.

**Acceptance Scenarios**:

1. **Given** a model detail view with no photo, **When** the user drags a `.jpg` file onto the photo zone, **Then** the zone shows a "Drop here to update photo" overlay and the crop dialog opens automatically when the file is released.
2. **Given** the crop dialog is open with the dropped image, **When** the user adjusts the crop area and confirms, **Then** the cropped image is saved to the model's catalog entry and the updated photo appears in the header.
3. **Given** a model already has a photo, **When** the user drops a new image onto the photo zone, **Then** the same crop-and-save flow runs, replacing the existing photo.

---

### User Story 2 - Browse-and-Crop via File Dialog (Priority: P2)

A collector prefers using the existing "Upload Image" button to browse their filesystem. After selecting a file, the same crop dialog opens so they can standardize the framing before saving — consistent with the drag-and-drop flow.

**Why this priority**: Preserves existing functionality while aligning it with the new crop-first experience. Users who prefer explicit file selection should get the same quality result.

**Independent Test**: Can be fully tested by clicking the upload button, selecting a valid image, completing the crop dialog, and verifying the saved result. Does not depend on drag-and-drop mechanics.

**Acceptance Scenarios**:

1. **Given** the model detail view, **When** the user clicks the browse/upload button and selects a `.png` file, **Then** the crop dialog opens with the selected image.
2. **Given** the crop dialog is open, **When** the user confirms the crop, **Then** the image is saved using the same storage path as the drag-and-drop flow.
3. **Given** the crop dialog is open, **When** the user cancels, **Then** no changes are saved and the existing photo (or placeholder) remains.

---

### User Story 3 - Invalid File Type Rejection (Priority: P3)

A collector accidentally drops a `.pdf` or `.tiff` file onto the photo zone. The system rejects it with a clear notification and returns to the idle state without opening the crop dialog.

**Why this priority**: Prevents user confusion and preserves system stability. Lower priority because the primary paths already handle valid files.

**Independent Test**: Can be fully tested by dropping an unsupported file type and verifying a rejection toast appears and the zone returns to idle.

**Acceptance Scenarios**:

1. **Given** a model detail view, **When** the user drops a `.pdf` file onto the photo zone, **Then** a toast notification appears explaining only JPEG, PNG, and WebP are accepted, and the crop dialog does not open.
2. **Given** a model detail view, **When** the user drops a `.gif` file, **Then** the same rejection toast is displayed.

---

### Edge Cases

- What happens when a user drops multiple files at once? The first valid image is used; excess files are ignored with a notification.
- What happens if the user drags a file from outside the OS (e.g., from a browser)? If the dragged item resolves to a valid image blob, the flow proceeds; otherwise it is rejected gracefully.
- What happens when the user dismisses the crop dialog without confirming? No image is saved; the model photo state is unchanged.
- What happens if the crop area produces an image below a minimum usable size? The system warns the user before saving.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The model photo zone MUST display a visually distinct "Drop here to update photo" overlay when a file is dragged over it, using an animated transition.
- **FR-002**: The photo zone MUST accept only `.jpg`, `.jpeg`, `.png`, and `.webp` files; all other types MUST be rejected with a toast notification.
- **FR-003**: On a valid file drop, the system MUST automatically open a crop dialog pre-loaded with the dropped image, without requiring additional user action.
- **FR-004**: The crop dialog MUST allow the user to adjust the crop area, zoom, and confirm or cancel before any data is persisted.
- **FR-005**: On crop confirmation, the system MUST save the cropped image using the same storage pathway as the existing upload mechanism.
- **FR-006**: The browse/upload button MUST continue to function and, when a file is selected, MUST route through the same crop dialog before saving.
- **FR-007**: The photo zone MUST have three distinct visual states: Idle (standard UI), Dragging (highlighted border + overlay), and Processing (loading indicator while saving).
- **FR-008**: All user-visible strings (overlay text, error messages, button labels) MUST use the project's internationalization system; no hardcoded strings are permitted.
- **FR-009**: The drop zone drag state MUST be managed as reactive state, avoiding imperative class manipulation.

### Key Entities

- **Model Photo**: The stored image asset associated with a railway model catalog entry. Has an associated model ID, file path in app storage, and dimensions. Replaced atomically on update.
- **Crop Selection**: A transient, in-memory value representing the user's chosen rectangular region and zoom level within a loaded image. Never persisted directly; used only to produce the final saved photo.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: A user can go from dragging an image file to a saved, cropped model photo in 5 interactions or fewer (drag, drop, adjust crop, confirm — counting the save as automatic).
- **SC-002**: Invalid file types are rejected within 500 ms of the drop event, with a visible notification, in 100% of cases.
- **SC-003**: The crop dialog opens within 1 second of a valid file being dropped, even for image files up to 20 MB.
- **SC-004**: The existing browse-to-upload path continues to function correctly after this change, with no regression in save success rate.
- **SC-005**: The photo zone visual transition between Idle and Dragging states is perceptible (visible change) and completes within 200 ms.

## Assumptions

- The application already has a Tauri command to persist an image given raw bytes and a model ID (`uploadModelImageBytes`); the new crop flow will produce bytes to pass to this command.
- Cropping produces a single rectangular output; advanced shapes (circular, freehand) are out of scope.
- Only one image can be active per model; the new image replaces the old one entirely.
- The crop dialog is a modal overlay; users cannot interact with the rest of the application while it is open.
- File size validation (e.g., max 50 MB) is already enforced by the backend and does not need to be duplicated in the drop zone, except as a user-facing hint.
