# Feature Specification: Data Archive Export

**Feature Branch**: `016-data-archive-export`  
**Created**: February 8, 2026  
**Status**: Draft  
**Input**: User description: "Export app data to a single archive file using the same format as import"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Export Complete Collection (Priority: P1)

A user wants to create a backup of their entire railway model collection data and images for safekeeping. They want a single archive file that can be restored if their local data is lost or corrupted, or used to migrate to another device.

**Why this priority**: This is the core functionality - enabling users to export their collection provides critical data portability and disaster recovery capabilities. Without this, users have no way to safeguard their investment in curating their digital collection.

**Independent Test**: Can be fully tested by triggering an export from the app, receiving a single archive file, and verifying it contains a valid manifest.json and all referenced images.

**Acceptance Scenarios**:

1. **Given** a collection with 25 railway models and 15 collection items, **When** the user initiates a full export, **Then** the system generates a single archive file containing manifest.json with all 40 records.
2. **Given** 12 images stored in the app's media directory, **When** the export completes, **Then** all 12 images are included in the archive's `/images/` folder.
3. **Given** the user confirms export destination, **When** the process completes, **Then** a success notification displays with the archive location and file size.

---

### User Story 2 - Choose Export Location (Priority: P1)

A user wants to control where the exported archive is saved (e.g., external drive, cloud sync folder, desktop) to facilitate their backup or migration workflow.

**Why this priority**: Users have different backup strategies - some use external drives, others sync to cloud services. Without the ability to choose the location, the feature becomes less useful for many workflows.

**Independent Test**: Can be tested by initiating an export and verifying a file picker dialog allows selecting any writable location accessible to the user.

**Acceptance Scenarios**:

1. **Given** the user triggers an export, **When** the export dialog appears, **Then** a file picker allows browsing and selecting any writable folder.
2. **Given** the user selects an external USB drive as the destination, **When** the export completes, **Then** the archive is written to the selected drive location.
3. **Given** the user cancels the file picker dialog, **When** the dialog closes, **Then** no export is performed and no temporary files remain.

---

### User Story 3 - Export Preview Summary (Priority: P2)

Before executing the export, a user wants to see what will be included (counts of models, collection items, images) to verify the scope is correct and ensure nothing unexpected is being exported.

**Why this priority**: Users benefit from transparency about what data is being exported, especially before potentially long operations. This builds trust and helps catch issues early (e.g., accidentally exporting an empty database).

**Independent Test**: Can be tested by opening the export dialog and verifying the preview accurately reflects database counts without executing the export.

**Acceptance Scenarios**:

1. **Given** a collection with 50 railway models and 30 collection items, **When** the export preview displays, **Then** the user sees "50 railway models, 30 collection items" in the summary.
2. **Given** 18 images are stored in the media directory, **When** the preview displays, **Then** the estimated archive size includes "18 images" in the breakdown.
3. **Given** the database is empty, **When** the export preview displays, **Then** a warning message indicates "No data to export" and the confirm button is disabled.

---

### User Story 4 - Selective Entity Export (Priority: P2)

A user wants to export only specific types of data (e.g., just railway models catalog without collection items, or just seller directory) rather than always exporting everything.

**Why this priority**: Different use cases exist - a user might want to share their model catalog with a friend without sharing purchase prices, or export just their maintenance logs for analysis in another tool.

**Independent Test**: Can be tested by selecting specific entity types in the export dialog and verifying the resulting archive contains only those entities in the manifest.

**Acceptance Scenarios**:

1. **Given** the export dialog with checkboxes for each entity type, **When** the user unchecks "Collection Items" and "Sellers", **Then** the resulting archive contains only railway model catalog entries.
2. **Given** the user selects only "Maintenance Logs", **When** the export completes, **Then** the manifest.json includes maintenance records but excludes other entity types.
3. **Given** all entity checkboxes are unchecked, **When** the user attempts to confirm, **Then** the confirm button is disabled with a message "Select at least one entity type".

---

### User Story 5 - Progress Feedback for Large Exports (Priority: P2)

When exporting a large collection with hundreds of records and many images, the user wants to see progress updates to understand the operation is working and estimate completion time.

**Why this priority**: Large exports can take significant time. Without progress feedback, users may think the app has frozen, leading to frustration or premature cancellation of the export.

**Independent Test**: Can be tested by exporting a large dataset (500+ records, 100+ images) and verifying progress indicators update at reasonable intervals throughout the process.

**Acceptance Scenarios**:

1. **Given** an export of 500 records with 200 images begins, **When** the export is in progress, **Then** a progress bar shows percentage completion updated at least every 500ms.
2. **Given** the export is processing images, **When** progress updates, **Then** the user sees "Processing image 45 of 200" or similar status message.
3. **Given** the export takes longer than 5 seconds, **When** progress displays, **Then** an estimated time remaining is shown (e.g., "About 15 seconds remaining").

---

### User Story 6 - Include Orphaned Images Warning (Priority: P3)

A user has images in their media directory that are not referenced by any records. The system should warn the user about these orphaned images and optionally include them in the export.

**Why this priority**: Users may have images they uploaded but haven't yet linked to records. While less critical, providing visibility prevents data loss and confusion when these images are missing after a migration.

**Independent Test**: Can be tested by adding images to the media directory without linking them to records, initiating an export, and verifying a warning appears with options to include or exclude orphaned images.

**Acceptance Scenarios**:

1. **Given** 5 images exist in media directory with no corresponding records, **When** the export preview displays, **Then** a warning shows "5 orphaned images found" with details.
2. **Given** orphaned images are detected, **When** the user enables "Include orphaned images", **Then** these images are added to the `/images/` folder in the archive.
3. **Given** orphaned images are detected and the user chooses to exclude them, **When** the export completes, **Then** only images referenced in the manifest are included in the archive.

---

### Edge Cases

- What happens when the selected export location becomes unavailable during export (e.g., USB drive removed)? → Export fails with clear error message, temporary files are cleaned up, user can retry.
- What happens when the destination folder lacks sufficient disk space for the export? → System checks available space before starting, displays error if insufficient, prevents partial write.
- What happens when the user's collection has no images? → Export succeeds with manifest.json only, no `/images/` folder is created in the archive.
- What happens if a referenced image file is missing from the media directory? → Warning is generated, export continues, missing images are listed in export log, manifest references remain intact.
- What happens when the export is cancelled mid-process? → Partial archive is deleted, no incomplete file remains at destination, user receives cancellation confirmation.
- How does the system handle very large collections (1000+ records, 500+ images)? → Progress updates continuously, archive creation streams to disk without loading everything in memory, completes without crash or timeout.
- What happens when filename encoding issues exist (special characters, non-ASCII)? → Filenames are sanitized using UTF-8 encoding, problematic characters are replaced with safe equivalents, mapping is logged.

---

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST generate export archives in `.zip` format compatible with the import feature (spec 010).
- **FR-002**: System MUST create a `manifest.json` file at the root level of the archive containing all exported entity data.
- **FR-003**: System MUST structure the manifest.json with the same schema format required by the import feature.
- **FR-004**: System MUST include an `/images/` folder within the archive containing all referenced image files.
- **FR-005**: System MUST allow users to select the destination location for the exported archive via a file picker dialog.
- **FR-006**: System MUST allow users to specify a custom filename for the export archive.
- **FR-007**: System MUST provide a default filename following the pattern: `rusty-shed-export-YYYY-MM-DD.zip` based on current date.
- **FR-008**: System MUST display a preview summary before executing export showing counts of each entity type to be exported.
- **FR-009**: System MUST allow users to selectively export specific entity types (railway models, collection items, sellers, maintenance logs, digital roster entries).
- **FR-010**: System MUST prevent export when the database is empty or no entity types are selected.
- **FR-011**: System MUST validate that all image files referenced in exported records exist in the media directory.
- **FR-012**: System MUST generate warnings for missing images but allow export to proceed with remaining valid data.
- **FR-013**: System MUST detect orphaned images (files in media directory not referenced by any record).
- **FR-014**: System MUST provide an option to include or exclude orphaned images from the export.
- **FR-015**: System MUST display progress updates during export operations showing percentage completion and current action.
- **FR-016**: System MUST provide estimated time remaining for exports taking longer than 5 seconds.
- **FR-017**: System MUST check available disk space at the destination before starting the export.
- **FR-018**: System MUST prevent export if insufficient disk space is available at the destination.
- **FR-019**: System MUST support cancellation of in-progress exports at any point.
- **FR-020**: System MUST clean up any partial or temporary files if export is cancelled or fails.
- **FR-021**: System MUST display a completion notification showing export success, archive location, and file size.
- **FR-022**: System MUST generate an export log listing any warnings (missing images, orphaned files) encountered during the process.
- **FR-023**: System MUST sanitize filenames for cross-platform compatibility using UTF-8 encoding.
- **FR-024**: System MUST preserve all record relationships (e.g., collection items linked to railway models) in the manifest structure.

### Key Entities

- **Export Package**: The complete archive file (`.zip`) containing the manifest and images. Represents the output of the export operation.
- **Manifest**: A JSON file (`manifest.json`) containing all exported relational data conforming to the same schema required by the import feature (spec 010).
- **Railway Model**: Catalog product records with manufacturer, product code, scale, and technical specifications to be included in exports.
- **Collection Item**: Records of owned railway models with purchase details and acquisition information to be included in exports.
- **Digital Roster Entry**: DCC-enabled rolling stock records with decoder information and addresses to be included in exports.
- **Maintenance Log**: Maintenance events and service history records for collection items to be included in exports.
- **Seller**: Business or individual seller directory entries to be included in exports.
- **Media Asset**: Image files stored in the app's media directory, referenced by records in the manifest.
- **Export Session**: The transient state during an export operation, including user selections, validation results, and progress tracking.

---

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can complete a standard export (50 records, 20 images) in under 15 seconds from dialog open to file saved.
- **SC-002**: 100% of exported archives can be successfully imported back into the application without data loss.
- **SC-003**: Users can identify what will be exported before confirming the operation by reviewing the preview summary.
- **SC-004**: Export operations handle interruptions gracefully with zero partial files left on disk after cancellation or failure.
- **SC-005**: Large exports (1000+ records, 500+ images) complete without application freezing, memory issues, or crashes.
- **SC-006**: Users can export and save archives to any writable location accessible to their system (local drives, network shares, cloud sync folders).
- **SC-007**: Users understand export results within 5 seconds of viewing the completion notification (success/warnings clearly communicated).
- **SC-008**: Exported archives use less than 110% of the theoretical minimum size (efficient compression without excessive overhead).

---

## Business Rules Reference

| Rule ID     | Rule Name                 | Description                                                                                                       |
| ----------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **BR-EX01** | **Format Compatibility**  | Export archives must use the exact same structure and schema as required by the import feature for full roundtrip |
| **BR-EX02** | **Image Completeness**    | Missing images generate warnings but do not block export; manifest references remain intact for future recovery   |
| **BR-EX03** | **Clean Cancellation**    | Cancelled or failed exports must delete all partial/temporary files to avoid disk clutter                         |
| **BR-EX04** | **Filename Sanitization** | All filenames in the archive are sanitized for cross-platform compatibility (Windows, macOS, Linux)               |

---

## Assumptions

- The application has read access to its internal database and media directory.
- The user has write permissions to the selected export destination.
- Archive compression libraries are available for creating `.zip` files.
- The manifest.json schema definition from the import feature (spec 010) is accessible and stable.
- Exported archives will be used with the same version of the application or a compatible future version.
- Users understand that selective exports may result in incomplete datasets (e.g., exporting collection items without corresponding railway models requires the models to already exist in the target system).
