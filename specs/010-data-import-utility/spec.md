# Feature Specification: Data Import Utility

**Feature Branch**: `010-data-import-utility`  
**Created**: January 30, 2026  
**Status**: Draft  
**Input**: User description: "Package-based import system for migrating data and images into Rusty Shed with validation, conflict handling, and data integrity preservation"

---

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Import Valid Package (Priority: P1)

A user who has exported their model railway collection data from another system wants to bring that data into Rusty Shed. They have a properly formatted archive containing a manifest file and associated images.

**Why this priority**: This is the core functionality - without successful import of valid packages, the entire feature has no value. Users migrating to Rusty Shed need this to onboard their existing data.

**Independent Test**: Can be fully tested by providing a valid `.zip` package with manifest and images, completing the import workflow, and verifying all data appears in the collection.

**Acceptance Scenarios**:

1. **Given** a valid `.zip` archive with correct manifest structure, **When** the user drops the file into the app, **Then** the system extracts and validates the manifest without errors.
2. **Given** a validated package with 10 railway models and 5 collection items, **When** the user confirms the import, **Then** all 15 records are added to the local database.
3. **Given** a package with 8 images referenced in the manifest, **When** the import completes, **Then** all 8 images are stored in the app's internal media directory.

---

### User Story 2 - Preview Import Before Execution (Priority: P1)

A user wants to understand what will happen before committing to the import. They need visibility into the data that will be added, any duplicates that will be skipped, and any validation issues.

**Why this priority**: Users need confidence before modifying their collection. Without preview, users cannot make informed decisions about proceeding with potentially large data changes.

**Independent Test**: Can be tested by providing packages with various data states (valid, duplicates, errors) and verifying the preview accurately reports counts and issues before any data is written.

**Acceptance Scenarios**:

1. **Given** a package is loaded for analysis, **When** the preview screen displays, **Then** the user sees total records found, valid records count, and identified duplicates count.
2. **Given** a package with validation errors, **When** the preview displays, **Then** specific validation errors are listed with affected records identified.
3. **Given** the preview is displayed, **When** the user has not clicked "Confirm", **Then** no data has been written to the local database.

---

### User Story 3 - Handle Duplicate Records Gracefully (Priority: P2)

A user imports a package that contains some records already present in their local collection. The system should protect the user's existing data and clearly communicate what will be skipped.

**Why this priority**: Data integrity is critical. Users who have manually curated their collection must not have their work overwritten by an import.

**Independent Test**: Can be tested by first adding records manually, then importing a package containing those same records, and verifying local records remain unchanged.

**Acceptance Scenarios**:

1. **Given** a local collection contains a railway model with manufacturer "Märklin" and product code "39010", **When** importing a package with the same manufacturer/product code, **Then** the incoming record is skipped entirely.
2. **Given** a local collection has a collection item for a specific model purchased on "2024-06-15", **When** importing a package with the same model and purchase date, **Then** the incoming collection item is skipped.
3. **Given** duplicates are skipped during import, **When** the final report displays, **Then** the count of skipped duplicates is clearly shown (e.g., "12 skipped - already in collection").

---

### User Story 4 - Receive Clear Completion Report (Priority: P2)

After an import completes, the user needs a clear summary of what happened so they can verify the operation succeeded as expected.

**Why this priority**: Users need closure and confirmation. A clear report builds trust and helps users identify if something unexpected occurred.

**Independent Test**: Can be tested by completing various import scenarios and verifying the report accurately reflects the actions taken.

**Acceptance Scenarios**:

1. **Given** an import completes successfully, **When** the report displays, **Then** it shows counts for: records added, records skipped (duplicates), and any warnings.
2. **Given** an import with missing image warnings, **When** the report displays, **Then** warnings about missing images are included without blocking the success message.

---

### User Story 5 - Abort on Critical Validation Failure (Priority: P2)

When a package contains critically malformed data that would corrupt the database, the system must prevent any partial import and clearly explain why.

**Why this priority**: Preventing data corruption is essential. Users must trust that the system won't leave their database in an inconsistent state.

**Independent Test**: Can be tested by providing packages with schema violations and verifying zero records are written when validation fails.

**Acceptance Scenarios**:

1. **Given** a package with a manifest that fails JSON schema validation, **When** validation completes, **Then** the import is aborted before any records are written.
2. **Given** validation fails due to a malformed record, **When** the error is displayed, **Then** the user sees which record caused the failure and what the issue was.
3. **Given** the import is aborted, **When** the user checks their collection, **Then** no new records were added (atomic behavior).

---

### User Story 6 - Handle Missing Images as Warnings (Priority: P3)

A user imports a package where some images referenced in the manifest are missing from the `/images/` folder. The system should warn the user but still allow the data import to proceed.

**Why this priority**: Data is more valuable than images. Users should not be blocked from importing their data just because some images are missing.

**Independent Test**: Can be tested by providing a package with incomplete images and verifying data imports successfully with appropriate warnings.

**Acceptance Scenarios**:

1. **Given** a manifest references "br01_loco.jpg" but the image is missing from `/images/`, **When** validation runs, **Then** a warning is generated but validation passes.
2. **Given** missing images are detected, **When** the preview displays, **Then** the count and names of missing images are shown to the user.
3. **Given** missing images exist, **When** the user confirms import, **Then** data records are imported successfully and warnings are included in the final report.

---

### Edge Cases

- What happens when the archive is corrupted or cannot be extracted? → Display error message, abort import.
- What happens when the manifest.json is missing from the archive? → Validation fails, import aborted with clear message.
- What happens when image filenames in the archive collide with existing media files? → Images are renamed to avoid collision (per BR-IM02).
- What happens when the archive is empty? → Validation fails with "No data found" message.
- What happens when a `.gz` file contains an unsupported internal format? → Validation fails with format error.
- How does the system handle very large archives (thousands of records)? → Progress indicator updates at least every 100ms; no UI freeze >200ms; import completes without crash.
- What happens when strings require normalization (e.g., "HO" vs "H0")? → Normalized to user's preferred format per settings (BR-IM04).

---

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST accept archives with `.zip` or `.tar.gz` extension as valid import containers.
- **FR-002**: System MUST require a `manifest.json` file at the root level of the archive.
- **FR-003**: System MUST support an `/images/` folder within the archive containing `.png`, `.jpg`, and `.jpeg` files.
- **FR-004**: System MUST validate the `manifest.json` against a formal JSON schema before any data is written.
- **FR-005**: System MUST verify that all mandatory fields defined by domain models are present in each record.
- **FR-006**: System MUST validate relationship integrity (e.g., maintenance logs reference valid locomotive IDs within the manifest).
- **FR-007**: System MUST check that every image filename referenced in the manifest exists in the `/images/` folder.
- **FR-008**: System MUST generate warnings for missing images but allow import to proceed.
- **FR-009**: System MUST detect duplicate railway models by matching manufacturer AND product code.
- **FR-010**: System MUST detect duplicate collection items by matching railway model AND purchase date.
- **FR-011**: System MUST skip duplicate records entirely without merging any fields.
- **FR-012**: System MUST display a preview summary before executing the import (total records, valid records, duplicates, errors).
- **FR-013**: System MUST require explicit user confirmation before writing any data.
- **FR-014**: System MUST abort the entire import if JSON schema validation fails for any record (atomic import).
- **FR-015**: System MUST move imported images to the app's internal media directory.
- **FR-016**: System MUST rename imported images if filename collisions occur with existing media.
- **FR-017**: System MUST treat the user's archive as read-only and never modify it.
- **FR-018**: System MUST normalize scale strings (e.g., "HO", "ho", "H0") to the user's preferred format during import.
- **FR-019**: System MUST display a completion report showing counts of added records, skipped duplicates, and any warnings.
- **FR-020**: System MUST support drag-and-drop file selection for the import package.

### Key Entities

- **Import Package**: A compressed archive (`.zip` or `.gz`) containing a manifest and optional images. Represents the unit of import.
- **Manifest**: A JSON file (`manifest.json`) containing all relational data for models, collection items, roster entries, and sellers. Must conform to a defined schema.
- **Railway Model**: A product record with manufacturer, product code, scale, and other catalog information. Uniqueness determined by manufacturer + product code.
- **Collection Item**: A record of a specific railway model owned by the user, with purchase date and acquisition details. Uniqueness determined by railway model + purchase date.
- **Media Asset**: An image file associated with records in the manifest. Referenced by filename within the manifest.
- **Import Session**: The transient state during an import operation, including validation results, preview data, and execution status.

---

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can complete a standard import (50 records, 20 images) in under 30 seconds from file selection to completion report.
- **SC-002**: 100% of valid packages import successfully with all data and images correctly stored.
- **SC-003**: Zero data corruption occurs when importing packages with validation errors (atomic abort verified).
- **SC-004**: Users can identify import issues (duplicates, missing images, validation errors) from the preview without proceeding to import.
- **SC-005**: Existing local data remains completely unchanged when duplicate records are detected.
- **SC-006**: Users understand the import outcome within 5 seconds of viewing the completion report.
- **SC-007**: Large imports (1000+ records) complete without application freezing or crashing.

---

## Business Rules Reference

| Rule ID     | Rule Name            | Description                                                                                                  |
| ----------- | -------------------- | ------------------------------------------------------------------------------------------------------------ |
| **BR-IM01** | **Atomic Import**    | If JSON schema validation fails for any record, the entire import is aborted to prevent partial/broken state |
| **BR-IM02** | **Image Isolation**  | Imported images are moved to internal media directory and renamed if necessary to avoid filename collisions  |
| **BR-IM03** | **Read-Only Source** | The app treats the archive as read-only and never modifies the user's original file                          |
| **BR-IM04** | **Normalization**    | During import, scale strings are normalized to the user's preferred format defined in settings               |

---

## Assumptions

- The user has configured their preferred scale format in settings before importing (for normalization).
- The manifest.json schema definition will be maintained as part of this feature.
- The app has write access to its internal media directory.
- Archive extraction libraries are available to handle both `.zip` and `.gz` formats.
- The database supports transactions for atomic import operations.
