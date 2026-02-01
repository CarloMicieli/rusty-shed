# Tauri Commands Contract: Google Drive Cloud Backup

**Feature**: 008-google-drive-backup  
**Date**: 2026-01-30  
**ADR Reference**: ADR-008 (Standardize Tauri Command and Use Case)

## Overview

This document defines the Tauri IPC command contracts for the cloud backup feature. All commands follow ADR-008 conventions and use specta for TypeScript type generation.

---

## Command Summary

| Command                              | Type    | Description                          | Auth Required |
| ------------------------------------ | ------- | ------------------------------------ | ------------- |
| `cloud_backup_get_connection_status` | Query   | Get current Google connection status | No            |
| `cloud_backup_connect_google`        | Command | Initiate OAuth flow                  | No            |
| `cloud_backup_disconnect_google`     | Command | Disconnect Google account            | Yes           |
| `cloud_backup_sync_now`              | Command | Upload backup to Drive               | Yes           |
| `cloud_backup_list_backups`          | Query   | List available backups               | Yes           |
| `cloud_backup_restore`               | Command | Restore from backup                  | Yes           |
| `cloud_backup_get_sync_status`       | Query   | Get current sync operation status    | No            |
| `cloud_backup_check_connectivity`    | Query   | Check internet connectivity          | No            |

---

## Command Definitions

### 1. Get Connection Status

Returns the current Google account connection status.

```rust
/// Query: Get Google connection status
///
/// Returns the current connection state including email and last sync time.
/// This is a query operation with no side effects.
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_connection_status(
    state: State<'_, AppState>,
) -> Result<ConnectionStatusResponse, CommandError> {
    // ...
}
```

**Request**: None

**Response**:

```typescript
interface ConnectionStatusResponse {
  isConnected: boolean;
  email: string | null;
  connectedAt: string | null; // ISO 8601
  lastSyncAt: string | null; // ISO 8601
}
```

**Errors**:
| Code | Description |
|------|-------------|
| `STORAGE_ERROR` | Failed to read from secure storage |

---

### 2. Connect Google

Initiates the OAuth 2.0 PKCE flow for Google authentication.

```rust
/// Command: Initiate Google OAuth flow
///
/// Opens system browser for Google authentication. Returns when OAuth
/// callback is received or timeout occurs.
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_connect_google(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ConnectionStatusResponse, CommandError> {
    // 1. Generate PKCE verifier + challenge
    // 2. Start localhost callback server (desktop) or register deep link (android)
    // 3. Open browser to Google OAuth URL
    // 4. Wait for callback with auth code
    // 5. Exchange code for tokens
    // 6. Store tokens securely
    // 7. Fetch user email from Google
    // 8. Return connection status
}
```

**Request**: None

**Response**:

```typescript
interface ConnectionStatusResponse {
  isConnected: boolean;
  email: string | null;
  connectedAt: string | null;
  lastSyncAt: string | null;
}
```

**Errors**:
| Code | Description |
|------|-------------|
| `OAUTH_CANCELLED` | User closed browser or cancelled auth |
| `OAUTH_TIMEOUT` | No callback received within timeout (5 min) |
| `OAUTH_TOKEN_ERROR` | Failed to exchange auth code for tokens |
| `STORAGE_ERROR` | Failed to store tokens securely |
| `NETWORK_ERROR` | Network request failed |

---

### 3. Disconnect Google

Disconnects the Google account and removes stored credentials.

```rust
/// Command: Disconnect Google account
///
/// Revokes OAuth tokens and removes stored credentials.
/// Does NOT delete backups from Google Drive.
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_disconnect_google(
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    // 1. Revoke tokens with Google (best effort)
    // 2. Delete tokens from secure storage
    // 3. Clear connection state
}
```

**Request**: None

**Response**: `void` (success) or error

**Errors**:
| Code | Description |
|------|-------------|
| `NOT_CONNECTED` | No Google account connected |
| `STORAGE_ERROR` | Failed to delete from secure storage |

---

### 4. Sync Now (Backup)

Uploads the current collection database to Google Drive.

```rust
/// Command: Upload backup to Google Drive
///
/// Compresses and uploads the collection database. Enforces version limit.
/// Emits progress events during upload.
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_sync_now(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<BackupListItem, CommandError> {
    // 1. Check connectivity (BR-04)
    // 2. Check no import in progress (BR-03)
    // 3. Acquire database lock
    // 4. Compress database with gzip
    // 5. Calculate checksum
    // 6. Get/create backup folder in Drive
    // 7. Upload file with resumable upload
    // 8. Set app properties metadata
    // 9. Enforce version limit (delete oldest if > 5)
    // 10. Update last sync timestamp
    // 11. Return new backup info
}
```

**Request**: None

**Response**:

```typescript
interface BackupListItem {
  id: string;
  label: string;
  createdAt: string; // ISO 8601
  sizeBytes: number;
  sizeFormatted: string; // "2.4 MB"
  recordCount: number;
  isInitial: boolean;
}
```

**Events Emitted**:

```typescript
// Event: "cloud-backup://sync-progress"
interface SyncProgressEvent {
  operationId: string;
  progressPercent: number; // 0.0 - 1.0
  stage: 'compressing' | 'uploading' | 'finalizing';
}
```

**Errors**:
| Code | Description |
|------|-------------|
| `NOT_CONNECTED` | No Google account connected |
| `OFFLINE_ERROR` | No internet connection (BR-04) |
| `IMPORT_IN_PROGRESS` | Data import in progress (BR-03) |
| `TOKEN_EXPIRED` | OAuth token expired, re-auth required |
| `DRIVE_ERROR` | Google Drive API error |
| `COMPRESSION_ERROR` | Failed to compress database |
| `DATABASE_LOCKED` | Could not acquire database lock |

---

### 5. List Backups

Lists all available backups from Google Drive.

```rust
/// Query: List available cloud backups
///
/// Fetches backup list from Google Drive, sorted by date (newest first).
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_list_backups(
    state: State<'_, AppState>,
) -> Result<BackupListResponse, CommandError> {
    // 1. Check connectivity
    // 2. Refresh token if needed
    // 3. List files in backup folder
    // 4. Parse app properties metadata
    // 5. Sort by date descending
    // 6. Return list
}
```

**Request**: None

**Response**:

```typescript
interface BackupListResponse {
  backups: BackupListItem[];
  totalCount: number;
}

interface BackupListItem {
  id: string;
  label: string;
  createdAt: string;
  sizeBytes: number;
  sizeFormatted: string;
  recordCount: number;
  isInitial: boolean;
}
```

**Errors**:
| Code | Description |
|------|-------------|
| `NOT_CONNECTED` | No Google account connected |
| `OFFLINE_ERROR` | No internet connection |
| `TOKEN_EXPIRED` | OAuth token expired |
| `DRIVE_ERROR` | Google Drive API error |

---

### 6. Restore from Backup

Restores the local database from a cloud backup.

```rust
/// Command: Restore collection from cloud backup
///
/// Downloads and replaces local database with selected backup.
/// Requires explicit "RESTORE" confirmation (FR-013).
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_restore(
    app: AppHandle,
    state: State<'_, AppState>,
    args: RestoreBackupArgs,
) -> Result<(), CommandError> {
    // 1. Validate confirmation == "RESTORE"
    // 2. Check connectivity
    // 3. Download backup file from Drive
    // 4. Decompress gzip
    // 5. Validate SQLite integrity
    // 6. Close current database connection
    // 7. Backup current database (safety)
    // 8. Replace database file
    // 9. Reopen database connection
    // 10. Emit completion event
}
```

**Request**:

```typescript
interface RestoreBackupArgs {
  backupId: string;
  confirmation: string; // Must be "RESTORE"
}
```

**Response**: `void` (success) or error

**Events Emitted**:

```typescript
// Event: "cloud-backup://restore-progress"
interface RestoreProgressEvent {
  operationId: string;
  progressPercent: number;
  stage: 'downloading' | 'decompressing' | 'validating' | 'replacing';
}

// Event: "cloud-backup://restore-complete"
interface RestoreCompleteEvent {
  backupId: string;
  restoredAt: string; // ISO 8601
}
```

**Errors**:
| Code | Description |
|------|-------------|
| `NOT_CONNECTED` | No Google account connected |
| `OFFLINE_ERROR` | No internet connection |
| `INVALID_CONFIRMATION` | Confirmation not "RESTORE" |
| `BACKUP_NOT_FOUND` | Backup ID not found in Drive |
| `INTEGRITY_ERROR` | Backup file corrupted or invalid SQLite |
| `RESTORE_FAILED` | Failed to replace database |
| `TOKEN_EXPIRED` | OAuth token expired |

---

### 7. Get Sync Status

Returns the current sync operation status (for progress tracking).

```rust
/// Query: Get current sync operation status
///
/// Returns status of any in-progress sync or restore operation.
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_sync_status(
    state: State<'_, AppState>,
) -> Result<SyncStatusResponse, CommandError> {
    // Return current operation state from memory
}
```

**Request**: None

**Response**:

```typescript
interface SyncStatusResponse {
  operationId: string | null;
  isSyncing: boolean;
  progressPercent: number;
  statusMessage: string;
}
```

**Errors**: None (always succeeds)

---

### 8. Check Connectivity

Checks internet connectivity status.

```rust
/// Query: Check internet connectivity
///
/// Performs actual connectivity check (not just interface status).
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_check_connectivity() -> Result<ConnectivityStatus, CommandError> {
    // Use `online` crate to check actual internet access
}
```

**Request**: None

**Response**:

```typescript
interface ConnectivityStatus {
  isOnline: boolean;
  checkedAt: string; // ISO 8601
}
```

**Errors**: None (always returns status)

---

## Events

### Connectivity Changed

Emitted when network connectivity state changes.

```typescript
// Event: "cloud-backup://connectivity-changed"
interface ConnectivityChangedEvent {
  isOnline: boolean;
  checkedAt: string;
}
```

### Sync Progress

Emitted during backup or restore operations.

```typescript
// Event: "cloud-backup://sync-progress"
interface SyncProgressEvent {
  operationId: string;
  progressPercent: number;
  stage: string;
}
```

### Restore Complete

Emitted when restore operation completes successfully.

```typescript
// Event: "cloud-backup://restore-complete"
interface RestoreCompleteEvent {
  backupId: string;
  restoredAt: string;
}
```

---

## Error Codes

| Code                    | HTTP Equiv | Description                         |
| ----------------------- | ---------- | ----------------------------------- |
| `NOT_CONNECTED`         | 401        | Google account not connected        |
| `TOKEN_EXPIRED`         | 401        | OAuth token expired, re-auth needed |
| `OFFLINE_ERROR`         | 503        | No internet connection              |
| `IMPORT_IN_PROGRESS`    | 409        | Cannot backup during import         |
| `INVALID_CONFIRMATION`  | 400        | Restore confirmation invalid        |
| `BACKUP_NOT_FOUND`      | 404        | Backup ID not found                 |
| `BACKUP_LIMIT_EXCEEDED` | 409        | Max 5 backups (should not occur)    |
| `INTEGRITY_ERROR`       | 422        | Backup file corrupted               |
| `DRIVE_ERROR`           | 502        | Google Drive API error              |
| `STORAGE_ERROR`         | 500        | Secure storage error                |
| `COMPRESSION_ERROR`     | 500        | Compression/decompression failed    |
| `DATABASE_LOCKED`       | 423        | Database in use                     |
| `RESTORE_FAILED`        | 500        | Database replacement failed         |
| `OAUTH_CANCELLED`       | 499        | User cancelled OAuth                |
| `OAUTH_TIMEOUT`         | 408        | OAuth callback timeout              |
| `OAUTH_TOKEN_ERROR`     | 401        | Token exchange failed               |
| `NETWORK_ERROR`         | 503        | Network request failed              |

---

## TypeScript Generated Types

After running `pnpm tauri build` with specta, these types will be available in `src/lib/bindings.ts`:

```typescript
// Commands
export function cloudBackupGetConnectionStatus(): Promise<ConnectionStatusResponse>;
export function cloudBackupConnectGoogle(): Promise<ConnectionStatusResponse>;
export function cloudBackupDisconnectGoogle(): Promise<void>;
export function cloudBackupSyncNow(): Promise<BackupListItem>;
export function cloudBackupListBackups(): Promise<BackupListResponse>;
export function cloudBackupRestore(args: RestoreBackupArgs): Promise<void>;
export function cloudBackupGetSyncStatus(): Promise<SyncStatusResponse>;
export function cloudBackupCheckConnectivity(): Promise<ConnectivityStatus>;

// Types
export interface ConnectionStatusResponse {
  /* ... */
}
export interface BackupListItem {
  /* ... */
}
export interface BackupListResponse {
  /* ... */
}
export interface RestoreBackupArgs {
  /* ... */
}
export interface SyncStatusResponse {
  /* ... */
}
export interface ConnectivityStatus {
  /* ... */
}
```
