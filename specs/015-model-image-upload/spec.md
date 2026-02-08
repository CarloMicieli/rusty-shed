# Feature Specification: Model Image Upload System

**Feature Branch**: `015-model-image-upload`  
**Created**: February 8, 2026  
**Status**: Draft  
**Input**: User description: "Implement an image upload system for the Model Details page that supports Drag & Drop and File Explorer selection. Selected images must be validated as web-friendly formats, copied to the application's designated AppData folder with a unique filename, and the resulting local path must be saved to the model record to allow persistent rendering via the Tauri asset protocol."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Add Primary Model Image via File Explorer (Priority: P1)

A user viewing a model's details wants to add a primary photograph by browsing their computer's file system and selecting an image file. The system accepts the file, stores it securely, and displays it immediately on the model details page.

**Why this priority**: This is the most fundamental interaction - users expect to click a button and select an image from their computer. This represents the core MVP functionality that delivers immediate value.

**Independent Test**: Can be fully tested by opening a model details page, clicking an "Upload Image" or "Add Photo" button, selecting a valid image file (JPEG, PNG, WEBP), and verifying the image displays on the page. The image should persist after closing and reopening the page.

**Acceptance Scenarios**:

1. **Given** a user is viewing a model details page with no images, **When** they click the upload button and select a JPEG image from their file system, **Then** the image displays on the page and is saved to the model record.
2. **Given** a user is viewing a model details page, **When** they select a PNG image with transparency, **Then** the image is properly stored and displays with its transparency preserved.
3. **Given** a user is viewing a model details page, **When** they select a WEBP image, **Then** the image is properly stored and displays correctly.
4. **Given** a user navigates away from the model details page and returns, **When** they view the same model, **Then** the previously uploaded image is still displayed.

---

### User Story 2 - Add Model Image via Drag & Drop (Priority: P2)

A user viewing a model's details wants to quickly add an image by dragging a file from their desktop or file explorer directly onto the model details page. The system accepts the dropped file and handles it identically to file-selected images.

**Why this priority**: Drag & drop is a modern UX enhancement that improves workflow efficiency for power users who manage many images. It builds on the core upload functionality from P1.

**Independent Test**: Can be fully tested by opening a model details page, dragging an image file from the desktop or file explorer onto the designated drop zone, and verifying the image is stored and displayed. Should work exactly like file selection method.

**Acceptance Scenarios**:

1. **Given** a user is viewing a model details page, **When** they drag a JPEG image file from their desktop onto the drop zone, **Then** the image is uploaded, stored, and displayed on the page.
2. **Given** a user is viewing a model details page with a drag zone, **When** they hover a draggable image file over the drop zone, **Then** visual feedback indicates the zone is ready to accept the file.
3. **Given** a user starts dragging an image file over the page, **When** they release it outside the designated drop zone, **Then** the file is not uploaded and no error occurs.

---

### User Story 3 - Reject Invalid File Formats (Priority: P2)

A user attempts to upload a file that is not a web-friendly image format (e.g., TIFF, BMP, RAW, or non-image file like PDF or DOC). The system validates the file before processing using magic byte detection (via Rust `image` crate) and provides clear feedback about why the file was rejected.

**Why this priority**: File validation prevents errors, corrupted data, and poor user experience. It's essential for system reliability and must be implemented early to avoid technical debt.

**Independent Test**: Can be fully tested by attempting to upload various non-supported file formats (TIFF, BMP, PDF, TXT, RAW camera files) and verifying that each is rejected with a clear error message explaining which formats are supported.

**Acceptance Scenarios**:

1. **Given** a user attempts to select a PDF file via file explorer, **When** the file dialog opens, **Then** only image files (JPEG, PNG, WEBP) are selectable by default.
2. **Given** a user drags a TIFF file onto the drop zone, **When** the file is dropped, **Then** an error message displays indicating the file format is not supported and lists acceptable formats.
3. **Given** a user attempts to upload a corrupted or incomplete image file, **When** validation occurs, **Then** an error message indicates the file could not be processed.
4. **Given** a user drags multiple files at once onto the drop zone, **When** the files are dropped, **Then** the system rejects the action and displays a message indicating only one image can be uploaded at a time.

---

### User Story 4 - Replace Existing Model Image (Priority: P3)

A user viewing a model that already has an image wants to replace it with a better photograph. The system allows them to upload a new image, which replaces the existing one while properly cleaning up the old file.

**Why this priority**: Image replacement is a secondary workflow that improves content management but isn't essential for initial launch. Users can work around this by manually deleting and re-adding images if needed.

**Independent Test**: Can be fully tested by uploading an initial image to a model, then uploading a second image, and verifying: (1) the new image replaces the old one on the page, (2) the old image file is removed from storage, (3) the model record is updated with the new image path.

**Acceptance Scenarios**:

1. **Given** a model has an existing image, **When** a user uploads a new image via file selection, **Then** the new image replaces the old one on the page and in storage.
2. **Given** a model has an existing image file stored in AppData, **When** it is replaced with a new image, **Then** the old image file is deleted from the file system to avoid orphaned files.
3. **Given** a user is in the process of replacing an image, **When** they cancel the operation, **Then** the original image remains unchanged.

---

### User Story 5 - Delete Model Image (Priority: P3)

A user wants to remove an image from a model's details page entirely. The system provides a clear way to delete the image, removes it from display, deletes the file from storage, and updates the model record.

**Why this priority**: Deletion is a cleanup operation that's important for long-term content management but not critical for MVP. Early users can live with images they can't remove temporarily.

**Independent Test**: Can be fully tested by uploading an image to a model, clicking a delete/remove button, confirming the deletion if prompted, and verifying: (1) the image disappears from the page, (2) the file is removed from AppData, (3) the model record no longer references the image path.

**Acceptance Scenarios**:

1. **Given** a model has an uploaded image, **When** a user clicks a delete/remove icon, **Then** the image is removed from the page, deleted from storage, and the model record is updated.
2. **Given** a user clicks delete on an image, **When** a confirmation dialog appears, **Then** the user can choose to confirm or cancel the deletion.
3. **Given** a user deletes an image, **When** they navigate away and return to the model details page, **Then** the image remains deleted and does not reappear.

---

### Edge Cases

- What happens when a user uploads an extremely large image file (>50MB)?
- What happens when a user uploads an image with an unusual aspect ratio (e.g., 10000x100 pixels)?
- What happens when the AppData folder is full or write-protected?
- What happens when a user uploads an image while offline?
- What happens when a user uploads an image with special characters or very long filenames?
- What happens when two users (or two app instances) try to upload images to the same model simultaneously?
- What happens when a user uploads an image with the same filename as an existing image for a different model?
- What happens when the image file is deleted or moved from AppData outside the application?
- What happens if the operating system denies permission to write to the AppData folder?
- What happens when a user tries to upload an image that appears valid but is corrupted or truncated?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a file selection interface (button/control) that opens the operating system's file picker dialog
- **FR-002**: System MUST provide a visual drop zone on the model details page that accepts dragged image files
- **FR-003**: System MUST validate uploaded files to ensure they are in a supported web-friendly format (JPEG, PNG, WEBP)
- **FR-004**: System MUST reject files that are not in supported formats with a clear error message listing acceptable formats
- **FR-005**: System MUST validate that uploaded files are actual image files and not corrupted or renamed non-image files
- **FR-006**: System MUST generate a unique filename for each uploaded image to prevent naming conflicts
- **FR-007**: System MUST copy uploaded image files to the application's designated AppData folder structure
- **FR-008**: System MUST save the local file path reference to the model record in the database
- **FR-009**: System MUST display uploaded images on the model details page using the saved file path
- **FR-010**: System MUST persist image references so they remain visible after page reload or application restart
- **FR-011**: System MUST provide visual feedback during file upload (e.g., loading indicator or progress state)
- **FR-012**: System MUST provide visual feedback when a file is being dragged over the drop zone (hover state)
- **FR-013**: System MUST handle image replacement by updating the model record and removing the old image file from storage
- **FR-014**: System MUST provide a way to delete/remove an uploaded image from a model
- **FR-015**: System MUST delete orphaned image files from storage when images are removed or replaced
- **FR-016**: System MUST prevent multiple simultaneous file uploads for the same model
- **FR-017**: System MUST enforce a maximum file size limit of 50MB per image
- **FR-018**: System MUST handle file system errors gracefully (e.g., insufficient permissions, disk full) with user-friendly error messages
- **FR-019**: System MUST sanitize filenames to remove or replace special characters that could cause file system issues
- **FR-020**: System MUST verify that the AppData folder exists and is writable before attempting to save files
- **FR-021**: Users MUST be able to upload only one image at a time (no batch uploads)

### Key Entities

- **Model**: Railway model record that can have an associated image. Key attributes include model identifier and other model metadata (name, manufacturer, scale, etc.). Note: Image file path is computed deterministically from model ID at runtime, not stored in database.
- **Image File**: Physical image file stored in AppData with attributes including unique filename, file extension, file size, and original upload date
- **Image Reference**: The computed file path (derived from model ID using naming convention `{model_id_sanitized}.{extension}`) that links the model to its image file, enabling retrieval and display

## Success Criteria _(mandatory)_

### Measurable Outcomes

**Hardware Baseline**: Standard desktop hardware is defined as: Intel i5 (8th gen) or AMD Ryzen 5 (2nd gen) equivalent, 8GB RAM, SSD storage, running on supported OS (Linux/Windows/macOS). Performance targets represent p95 (95th percentile) measurements.

- **SC-001**: Users can successfully upload and view a model image in under 30 seconds from the time they click the upload button
- **SC-002**: System successfully validates and rejects 100% of non-supported file formats with clear error messages
- **SC-003**: Uploaded images persist correctly and remain visible after application restart in 100% of test cases
- **SC-004**: Users can complete the drag & drop workflow in under 15 seconds with 90% success rate on first attempt
- **SC-005**: System handles file system errors gracefully with user-friendly messages in 100% of error scenarios (disk full, permission denied, etc.)
- **SC-006**: No orphaned image files remain in storage after image replacement or deletion operations
- **SC-007**: System processes images up to 50MB without performance degradation or timeout
- **SC-008**: Upload operations complete within 5 seconds for files under 10MB on standard desktop hardware (p95)
- **SC-009**: Zero file naming conflicts occur when multiple images are uploaded across different models
- **SC-010**: Visual feedback for drag & drop states (hover, uploading) appears within 100ms of user interaction
