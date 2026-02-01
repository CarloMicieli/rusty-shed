# Data Model: Google Drive Cloud Backup

**Feature**: 008-google-drive-backup  
**Date**: 2026-01-30

## Overview

This document defines the domain entities, value objects, and their relationships for the Google Drive Cloud Backup feature. The model follows DDD principles with clear separation between domain logic and infrastructure concerns.

---

## Entity Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Domain Layer                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────┐       ┌─────────────────────────────┐  │
│  │  GoogleConnection   │       │      CloudBackup            │  │
│  │  (Value Object)     │       │      (Entity)               │  │
│  ├─────────────────────┤       ├─────────────────────────────┤  │
│  │ - email: String     │       │ - id: BackupId              │  │
│  │ - connected_at: DT  │       │ - file_id: String           │  │
│  │ - status: ConnStatus│       │ - label: BackupLabel        │  │
│  └─────────────────────┘       │ - created_at: DateTime      │  │
│                                │ - size_bytes: u64           │  │
│                                │ - schema_version: i32       │  │
│  ┌─────────────────────┐       │ - status: BackupStatus      │  │
│  │   SyncOperation     │       └─────────────────────────────┘  │
│  │   (Entity)          │                                        │
│  ├─────────────────────┤       ┌─────────────────────────────┐  │
│  │ - id: OperationId   │       │    BackupMetadata           │  │
│  │ - started_at: DT    │       │    (Value Object)           │  │
│  │ - completed_at: DT? │       ├─────────────────────────────┤  │
│  │ - status: OpStatus  │       │ - app_version: String       │  │
│  │ - progress: f32     │       │ - record_count: u64         │  │
│  │ - error: String?    │       │ - platform: String          │  │
│  └─────────────────────┘       │ - checksum: String          │  │
│                                └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                     Infrastructure Layer                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────┐       ┌─────────────────────────────┐  │
│  │   OAuthTokens       │       │   DriveFile                 │  │
│  │   (Secure Storage)  │       │   (API Response)            │  │
│  ├─────────────────────┤       ├─────────────────────────────┤  │
│  │ - access_token      │       │ - id: String                │  │
│  │ - refresh_token     │       │ - name: String              │  │
│  │ - expires_at: DT    │       │ - size: i64                 │  │
│  │ - token_type: String│       │ - modified_time: String     │  │
│  └─────────────────────┘       │ - app_properties: Map       │  │
│                                └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Domain Entities

### CloudBackup

Represents a single backup instance stored in Google Drive.

| Field            | Type             | Description              | Validation               |
| ---------------- | ---------------- | ------------------------ | ------------------------ |
| `id`             | `BackupId`       | Unique identifier (UUID) | Required, format: UUIDv4 |
| `file_id`        | `String`         | Google Drive file ID     | Required, non-empty      |
| `label`          | `BackupLabel`    | Human-readable label     | Required, enum           |
| `created_at`     | `DateTime<Utc>`  | Creation timestamp       | Required                 |
| `size_bytes`     | `u64`            | Compressed file size     | Required, > 0            |
| `schema_version` | `i32`            | Database schema version  | Required, > 0            |
| `status`         | `BackupStatus`   | Current backup status    | Required, enum           |
| `metadata`       | `BackupMetadata` | Additional metadata      | Required                 |

**Business Rules**:

- Maximum 5 backups allowed (BR-02)
- First backup must have `label = Initial`
- Backups are immutable once created

### SyncOperation

Represents an in-progress or completed sync attempt.

| Field              | Type                    | Description              | Validation     |
| ------------------ | ----------------------- | ------------------------ | -------------- |
| `id`               | `OperationId`           | Unique identifier (UUID) | Required       |
| `operation_type`   | `OperationType`         | Backup or Restore        | Required, enum |
| `started_at`       | `DateTime<Utc>`         | Operation start time     | Required       |
| `completed_at`     | `Option<DateTime<Utc>>` | Completion time          | Optional       |
| `status`           | `OperationStatus`       | Current status           | Required, enum |
| `progress_percent` | `f32`                   | Progress (0.0-1.0)       | 0.0 ≤ x ≤ 1.0  |
| `error_message`    | `Option<String>`        | Error details if failed  | Optional       |

**State Transitions**:

```
InProgress → Completed | Failed | Cancelled
```

---

## Value Objects

### GoogleConnection

Represents the user's Google account connection state.

| Field          | Type               | Description            |
| -------------- | ------------------ | ---------------------- |
| `email`        | `String`           | Connected Google email |
| `connected_at` | `DateTime<Utc>`    | Connection timestamp   |
| `status`       | `ConnectionStatus` | Current status         |

### BackupMetadata

Additional information stored with each backup.

| Field          | Type     | Description                         |
| -------------- | -------- | ----------------------------------- |
| `app_version`  | `String` | App version at backup time          |
| `record_count` | `u64`    | Number of collection items          |
| `platform`     | `String` | OS platform (windows/linux/android) |
| `checksum`     | `String` | SHA-256 of uncompressed DB          |

### BackupLabel

```rust
pub enum BackupLabel {
    Initial,                    // First backup ever
    Manual(DateTime<Utc>),      // User-initiated backup with timestamp
}
```

---

## Enumerations

### ConnectionStatus

```rust
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    TokenExpired,
    Error(String),
}
```

### BackupStatus

```rust
pub enum BackupStatus {
    Available,      // Ready for restore
    Uploading,      // Currently being uploaded
    Corrupted,      // Integrity check failed
}
```

### OperationStatus

```rust
pub enum OperationStatus {
    InProgress,
    Completed,
    Failed,
    Cancelled,
}
```

### OperationType

```rust
pub enum OperationType {
    Backup,
    Restore,
}
```

---

## DTOs (Transport Layer)

### Command Args (Frontend → Backend)

```rust
/// Arguments for initiating OAuth flow
#[derive(Debug, Clone, Validate, specta::Type, serde::Deserialize)]
pub struct ConnectGoogleArgs {
    // No arguments needed - OAuth flow is stateless
}

/// Arguments for disconnecting Google account
#[derive(Debug, Clone, Validate, specta::Type, serde::Deserialize)]
pub struct DisconnectGoogleArgs {
    // No arguments needed
}

/// Arguments for initiating a backup
#[derive(Debug, Clone, Validate, specta::Type, serde::Deserialize)]
pub struct SyncBackupArgs {
    // No arguments needed - backs up current state
}

/// Arguments for restoring from backup
#[derive(Debug, Clone, Validate, specta::Type, serde::Deserialize)]
pub struct RestoreBackupArgs {
    /// The backup ID to restore from
    #[validate(length(min = 1))]
    pub backup_id: String,

    /// User confirmation (must be "RESTORE")
    #[validate(custom = "validate_restore_confirmation")]
    pub confirmation: String,
}

/// Arguments for listing backups
#[derive(Debug, Clone, Validate, specta::Type, serde::Deserialize)]
pub struct ListBackupsArgs {
    // No arguments needed - lists all available
}
```

### Query Responses (Backend → Frontend)

```rust
/// Connection status response
#[derive(Debug, Clone, specta::Type, serde::Serialize)]
pub struct ConnectionStatusResponse {
    pub is_connected: bool,
    pub email: Option<String>,
    pub connected_at: Option<String>,  // ISO 8601
    pub last_sync_at: Option<String>,  // ISO 8601
}

/// Single backup item in list
#[derive(Debug, Clone, specta::Type, serde::Serialize)]
pub struct BackupListItem {
    pub id: String,
    pub label: String,
    pub created_at: String,     // ISO 8601
    pub size_bytes: u64,
    pub size_formatted: String, // "2.4 MB"
    pub record_count: u64,
    pub is_initial: bool,
}

/// Backup list response
#[derive(Debug, Clone, specta::Type, serde::Serialize)]
pub struct BackupListResponse {
    pub backups: Vec<BackupListItem>,
    pub total_count: usize,
}

/// Sync operation status (for progress tracking)
#[derive(Debug, Clone, specta::Type, serde::Serialize)]
pub struct SyncStatusResponse {
    pub operation_id: Option<String>,
    pub is_syncing: bool,
    pub progress_percent: f32,
    pub status_message: String,
}

/// Network connectivity status
#[derive(Debug, Clone, specta::Type, serde::Serialize)]
pub struct ConnectivityStatus {
    pub is_online: bool,
    pub checked_at: String,  // ISO 8601
}
```

---

## Persistence Notes

### What IS Persisted Locally

| Data                | Storage        | Location                |
| ------------------- | -------------- | ----------------------- |
| OAuth tokens        | Secure storage | OS Keyring / Stronghold |
| Last sync timestamp | App settings   | Existing settings store |
| Connected email     | App settings   | Existing settings store |

### What is NOT Persisted Locally

| Data                   | Reason                                    |
| ---------------------- | ----------------------------------------- |
| Backup list            | Fetched from Google Drive on demand       |
| Backup files           | Stored only in Google Drive               |
| Sync operation history | Ephemeral, only current operation tracked |

### Google Drive Storage Structure

```
appDataFolder/
└── RustyShedBackups/           # Folder (created on first sync)
    ├── rusty_shed_backup_20260130T143022Z_v5.db.gz
    ├── rusty_shed_backup_20260129T091500Z_v5.db.gz
    ├── rusty_shed_backup_20260128T180000Z_v5.db.gz
    ├── rusty_shed_backup_20260127T120000Z_v5.db.gz
    └── rusty_shed_backup_20260126T090000Z_v5.db.gz  # Oldest (deleted when 6th added)
```

Each file includes `appProperties` metadata:

```json
{
  "appVersion": "1.2.0",
  "dbSchemaVersion": "5",
  "recordCount": "1523",
  "backupTimestamp": "2026-01-30T14:30:22Z",
  "platform": "linux",
  "checksum": "sha256:abc123...",
  "isInitial": "true"
}
```

---

## Validation Rules

| Entity/Field                     | Rule                      | Error Code              |
| -------------------------------- | ------------------------- | ----------------------- |
| `RestoreBackupArgs.confirmation` | Must equal "RESTORE"      | `INVALID_CONFIRMATION`  |
| `RestoreBackupArgs.backup_id`    | Must be valid UUID        | `INVALID_BACKUP_ID`     |
| Backup creation                  | Max 5 backups (BR-02)     | `BACKUP_LIMIT_EXCEEDED` |
| Sync operation                   | Not during import (BR-03) | `IMPORT_IN_PROGRESS`    |
| All sync ops                     | Must be online (BR-04)    | `OFFLINE_ERROR`         |
