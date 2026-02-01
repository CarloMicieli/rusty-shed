# Feature Specification: Google Drive Cloud Backup

**Feature Branch**: `008-google-drive-backup`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Google Drive Cloud Backup - allows users to link Google account, sync collection data to cloud, manage backup versions, and restore from cloud backups"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Connect Google Account (Priority: P1)

The user wants to link their Google account to the application so they can begin using cloud backup functionality. From the settings page, they click "Connect Google Drive," which opens the system browser for Google authentication and permission granting.

**Why this priority**: Without account linking, no cloud backup functionality is possible. This is the foundational capability that unlocks all other features.

**Independent Test**: Can be fully tested by clicking "Connect Google Drive," completing OAuth flow, and verifying the connected email is displayed in settings.

**Acceptance Scenarios**:

1. **Given** the user is on the settings page with no Google account connected, **When** they click "Connect Google Drive," **Then** the system browser opens to Google's authentication page.
2. **Given** the user completes Google authentication and grants permissions, **When** the OAuth callback is received, **Then** the settings page displays the user's Google email address and a "Disconnect" option.
3. **Given** the user has a connected Google account, **When** they click "Disconnect," **Then** the account is unlinked and the "Connect Google Drive" button reappears.

---

### User Story 2 - Manual Backup Sync (Priority: P1)

The user wants to manually sync their collection data to the cloud to ensure it's safely stored. They click "Sync Now" and see a progress indicator while the upload occurs, followed by an updated "Last Successful Sync" timestamp.

**Why this priority**: This is the core value proposition—actually backing up data. Without this, the feature provides no practical benefit to users.

**Independent Test**: Can be fully tested by clicking "Sync Now" with a connected account, verifying progress indicator appears, and confirming "Last Successful Sync" timestamp updates upon completion.

**Acceptance Scenarios**:

1. **Given** the user has a connected Google account and is online, **When** they click "Sync Now," **Then** a progress indicator appears showing sync is in progress.
2. **Given** a sync operation completes successfully, **When** the upload finishes, **Then** the "Last Successful Sync" timestamp updates to the current date and time.
3. **Given** no backup folder exists in the user's Drive, **When** the user initiates their first sync, **Then** the app creates a `RustyShedBackups` folder and labels the entry as "Initial Backup."
4. **Given** a sync operation fails due to network interruption, **When** the error occurs, **Then** the user sees a clear error message explaining what went wrong.

---

### User Story 3 - Restore from Backup (Priority: P2)

The user wants to restore their collection from a previous cloud backup. They view a list of available backups sorted by date, select one, and confirm the restore action to replace their local data.

**Why this priority**: Data recovery is critical for the feature's promise of "never lose your data," but it's used less frequently than backup operations.

**Independent Test**: Can be fully tested by viewing backup list, selecting a backup, confirming restore action, and verifying local data matches the restored version.

**Acceptance Scenarios**:

1. **Given** the user has cloud backups available, **When** they navigate to the restore section, **Then** they see a list of available backups sorted by date and time (newest first).
2. **Given** the user selects a backup to restore, **When** they click "Restore," **Then** a warning appears stating that current local data will be overwritten.
3. **Given** the warning is displayed, **When** the user types "RESTORE" as confirmation, **Then** the local collection is replaced with the selected cloud backup.
4. **Given** the warning is displayed, **When** the user dismisses the warning or types incorrect confirmation, **Then** no data is changed and the user remains on the restore screen.

---

### User Story 4 - Version Management (Priority: P3)

The system automatically manages backup versions to prevent unlimited storage consumption. The user benefits from having multiple recovery points without manual cleanup.

**Why this priority**: This is an automated background feature that enhances the experience but isn't directly user-initiated.

**Independent Test**: Can be tested by creating 6 backups and verifying only 5 remain (oldest deleted), or by viewing backup list and confirming versions are properly labeled.

**Acceptance Scenarios**:

1. **Given** the user has 5 existing backups, **When** they create a 6th backup, **Then** the oldest backup is automatically removed.
2. **Given** backups exist in the cloud, **When** the user views the backup list, **Then** each backup shows its creation date/time and a descriptive label.

---

### User Story 5 - Offline Handling (Priority: P3)

The user understands why backup operations are unavailable when offline. The interface clearly communicates the connectivity requirement.

**Why this priority**: This is a defensive feature that prevents user confusion but doesn't add primary functionality.

**Independent Test**: Can be tested by disabling network, verifying "Sync Now" is disabled with appropriate message, then re-enabling network and verifying button becomes active.

**Acceptance Scenarios**:

1. **Given** the user is offline, **When** they view the backup section, **Then** the "Sync Now" button is disabled and a message indicates an internet connection is required.
2. **Given** the user was offline and regains connectivity, **When** the connection is restored, **Then** the "Sync Now" button becomes enabled.

---

### Edge Cases

- What happens when the user revokes Google permissions from their Google account settings outside the app?
- How does the system handle if the "Rusty Shed Backups" folder is manually deleted from Google Drive?
- What happens if two devices try to sync simultaneously with the same Google account?
- How does the system behave if a backup file is corrupted or incomplete in Google Drive?
- What happens if the user's Google Drive storage is full?

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a "Connect Google Drive" button that initiates OAuth 2.0 authentication flow via the system browser.
- **FR-002**: System MUST request only the minimum required Google Drive permissions (access to app-created files only).
- **FR-003**: System MUST securely store OAuth tokens locally for persistent authentication.
- **FR-004**: System MUST display the connected Google account email address when authenticated.
- **FR-005**: System MUST provide a "Disconnect" option that revokes tokens and removes stored credentials.
- **FR-006**: System MUST provide a "Sync Now" button that uploads the current collection database to Google Drive.
- **FR-007**: System MUST display a progress indicator during sync operations.
- **FR-008**: System MUST display and persist the "Last Successful Sync" timestamp.
- **FR-009**: System MUST create a `RustyShedBackups` folder in Google Drive on first sync if it doesn't exist.
- **FR-010**: System MUST label the first backup as "Initial Backup" with subsequent backups labeled by date/time.
- **FR-011**: System MUST maintain a maximum of 5 backup versions, removing the oldest when creating a 6th.
- **FR-012**: System MUST provide a list view of available cloud backups sorted by date/time (newest first).
- **FR-013**: System MUST require the user to type "RESTORE" as confirmation before overwriting local data.
- **FR-014**: System MUST completely replace local collection data when restore is confirmed.
- **FR-015**: System MUST disable sync functionality and display an appropriate message when offline.
- **FR-016**: System MUST prevent backup operations while a large data import is in progress.
- **FR-017**: System MUST display clear error messages when sync or restore operations fail.
- **FR-018**: System MUST only access files created by the application (cannot see or modify other Drive files).

### Business Rules

| Rule ID | Rule Name        | Description                                                                                                |
| ------- | ---------------- | ---------------------------------------------------------------------------------------------------------- |
| BR-01   | Ownership        | The app only has access to the files it creates. It cannot see or delete other files in the user's Drive.  |
| BR-02   | Version Limit    | The app maintains a maximum of 5 historical backup versions. When a 6th is created, the oldest is removed. |
| BR-03   | Data Integrity   | A backup cannot be performed if the application is currently performing a large data import.               |
| BR-04   | Offline Handling | If the user is offline, sync functionality is disabled with an appropriate message.                        |

### Key Entities

- **Google Connection**: Represents the user's authenticated Google account link, including OAuth tokens, connected email address, and connection status.
- **Cloud Backup**: Represents a single backup instance stored in Google Drive, including creation timestamp, version label, and file reference.
- **Backup Folder**: The dedicated `RustyShedBackups` folder in Google Drive that contains all backup files.
- **Sync Operation**: Represents an in-progress or completed sync attempt, including status, progress, and result.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can successfully move their collection from one computer to another using the "Sync" and "Restore" functionality.
- **SC-002**: Users can connect their Google account and complete their first backup within 3 minutes.
- **SC-003**: Users can see exactly when their data was last secured via the "Last Successful Sync" timestamp.
- **SC-004**: If a backup fails, users receive a clear error message explaining what went wrong within 5 seconds of failure detection.
- **SC-005**: Users can view and select from up to 5 available backup versions when restoring.
- **SC-006**: The restore confirmation flow prevents accidental data overwrites 100% of the time (requires typing "RESTORE").
- **SC-007**: Offline users immediately understand that sync is unavailable through disabled controls and messaging.

## Assumptions

- Users have a Google account and are willing to use Google Drive for cloud storage.
- The application has been registered with Google Cloud Platform for OAuth 2.0 access.
- The user's device has a web browser installed for OAuth authentication flow.
- The backup file size is reasonable for typical internet connections (collection database is not gigabytes in size).
- The application uses Tauri's capability to detect online/offline status reliably.
