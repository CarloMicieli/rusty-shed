# Quickstart: Google Drive Cloud Backup

**Feature**: 008-google-drive-backup  
**Date**: 2026-01-30

## Prerequisites

Before implementing this feature, ensure:

1. **Google Cloud Console Setup**:
   - Create a new project in [Google Cloud Console](https://console.cloud.google.com/)
   - Enable the Google Drive API
   - Create OAuth 2.0 credentials (Desktop application type)
   - Add authorized redirect URI: `http://127.0.0.1` (loopback)
   - Add custom URI scheme for Android: `rusty-shed://oauth/callback`
   - Note your Client ID (Client Secret is NOT needed for PKCE)

2. **Development Environment**:
   - Rust 1.93.0+ with `cargo`
   - Node.js 20+ with `pnpm`
   - Tauri CLI v2

---

## Phase 1: Add Dependencies

### 1.1 Rust Dependencies

Add to `src-tauri/Cargo.toml`:

```toml
[dependencies]
# OAuth 2.0 with PKCE
oauth2 = "5"

# Google Drive API
google-drive3 = "7.0"

# Secret handling
secrecy = { version = "0.10", features = ["serde"] }
zeroize = "1"

# Network connectivity
online = { version = "4.0.2", default-features = false, features = ["tokio"] }

# Compression
flate2 = "1"

# Tauri plugins
tauri-plugin-stronghold = "2.3"

[target.'cfg(not(target_os = "android"))'.dependencies]
keyring = { version = "3", features = ["windows-native", "sync-secret-service", "crypto-rust"] }

# For dev performance with stronghold
[profile.dev.package.scrypt]
opt-level = 3
```

### 1.2 Tauri Plugins

```bash
cd /home/carlo/Projects/rusty-shed
pnpm tauri add oauth
pnpm tauri add deep-link
pnpm tauri add stronghold
```

### 1.3 Frontend Dependencies

```bash
pnpm add @tauri-apps/plugin-stronghold
```

---

## Phase 2: Create Feature Module Structure

### 2.1 Backend Structure

```bash
mkdir -p src-tauri/src/features/cloud_backup/{domain,application,infrastructure}
```

Create the module files:

```
src-tauri/src/features/cloud_backup/
├── mod.rs
├── domain/
│   ├── mod.rs
│   ├── backup.rs
│   ├── connection.rs
│   └── errors.rs
├── application/
│   ├── mod.rs
│   ├── connect_google.rs
│   ├── disconnect_google.rs
│   ├── sync_backup.rs
│   ├── restore_backup.rs
│   └── list_backups.rs
└── infrastructure/
    ├── mod.rs
    ├── google_drive.rs
    ├── oauth_service.rs
    ├── secure_storage.rs
    └── connectivity.rs
```

### 2.2 Frontend Structure

```bash
mkdir -p src/lib/features/cloud-backup/{components,controllers,stores}
```

Create the feature files:

```
src/lib/features/cloud-backup/
├── index.ts
├── components/
│   ├── GoogleConnectButton.svelte
│   ├── SyncButton.svelte
│   ├── BackupList.svelte
│   ├── RestoreConfirmModal.svelte
│   └── ConnectivityIndicator.svelte
├── controllers/
│   └── cloudBackup.svelte.ts
└── stores/
    └── connectivity.ts
```

---

## Phase 3: Implementation Order

Follow this order for incremental, testable development:

### Sprint 1: OAuth Foundation (P1)

1. `infrastructure/secure_storage.rs` - Token storage abstraction
2. `infrastructure/oauth_service.rs` - OAuth PKCE flow
3. `domain/connection.rs` - Connection value object
4. `application/connect_google.rs` - Connect use case
5. `application/disconnect_google.rs` - Disconnect use case
6. Register Tauri commands
7. `GoogleConnectButton.svelte` - UI component
8. **Test**: Complete OAuth flow, verify token storage

### Sprint 2: Backup Upload (P1)

1. `infrastructure/google_drive.rs` - Drive API client
2. `domain/backup.rs` - Backup entity
3. `application/sync_backup.rs` - Sync use case
4. `SyncButton.svelte` - UI component
5. `cloudBackup.svelte.ts` - Controller with sync state
6. **Test**: Upload backup, verify in Drive appDataFolder

### Sprint 3: Restore & List (P2)

1. `application/list_backups.rs` - List backups query
2. `application/restore_backup.rs` - Restore use case
3. `BackupList.svelte` - UI component
4. `RestoreConfirmModal.svelte` - Confirmation modal
5. **Test**: List backups, restore from backup

### Sprint 4: Polish (P3)

1. `infrastructure/connectivity.rs` - Network detection
2. `connectivity.ts` - Connectivity store
3. `ConnectivityIndicator.svelte` - UI indicator
4. Version limit enforcement (BR-02)
5. Import lock detection (BR-03)
6. Error message improvements
7. **Test**: Offline handling, version limit

---

## Phase 4: Key Implementation Patterns

### 4.1 OAuth PKCE Flow (Desktop)

```rust
// src-tauri/src/features/cloud_backup/infrastructure/oauth_service.rs
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const DRIVE_APPDATA_SCOPE: &str = "https://www.googleapis.com/auth/drive.appdata";

pub struct OAuthService {
    client: BasicClient,
}

impl OAuthService {
    pub fn new(client_id: &str) -> Self {
        let client = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_auth_uri(AuthUrl::new(GOOGLE_AUTH_URL.to_string()).unwrap())
            .set_token_uri(TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).unwrap());

        Self { client }
    }

    pub fn generate_auth_url(&self, redirect_uri: &str) -> (String, PkceCodeVerifier, CsrfToken) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(DRIVE_APPDATA_SCOPE.to_string()))
            .add_scope(Scope::new("email".to_string()))
            .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap())
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), pkce_verifier, csrf_token)
    }
}
```

### 4.2 Secure Storage Abstraction

```rust
// src-tauri/src/features/cloud_backup/infrastructure/secure_storage.rs
use secrecy::{ExposeSecret, SecretString};

pub trait SecureStorage: Send + Sync {
    fn store(&self, key: &str, value: SecretString) -> Result<(), StorageError>;
    fn retrieve(&self, key: &str) -> Result<Option<SecretString>, StorageError>;
    fn delete(&self, key: &str) -> Result<(), StorageError>;
}

#[cfg(not(target_os = "android"))]
pub struct KeyringStorage {
    service: String,
}

#[cfg(not(target_os = "android"))]
impl SecureStorage for KeyringStorage {
    fn store(&self, key: &str, value: SecretString) -> Result<(), StorageError> {
        let entry = keyring::Entry::new(&self.service, key)?;
        entry.set_password(value.expose_secret())?;
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Option<SecretString>, StorageError> {
        let entry = keyring::Entry::new(&self.service, key)?;
        match entry.get_password() {
            Ok(password) => Ok(Some(SecretString::new(password.into()))),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(StorageError::from(e)),
        }
    }

    fn delete(&self, key: &str) -> Result<(), StorageError> {
        let entry = keyring::Entry::new(&self.service, key)?;
        entry.delete_credential()?;
        Ok(())
    }
}
```

### 4.3 Backup Compression

```rust
// src-tauri/src/features/cloud_backup/application/sync_backup.rs
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

pub fn compress_database(db_path: &Path) -> Result<Vec<u8>, BackupError> {
    let db_content = std::fs::read(db_path)?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&db_content)?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}

pub fn decompress_backup(compressed: &[u8]) -> Result<Vec<u8>, BackupError> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}
```

### 4.4 Frontend Controller (Svelte 5)

```typescript
// src/lib/features/cloud-backup/controllers/cloudBackup.svelte.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  ConnectionStatusResponse,
  BackupListResponse,
  SyncStatusResponse
} from '$lib/bindings';

export class CloudBackupController {
  // Reactive state
  connectionStatus = $state<ConnectionStatusResponse | null>(null);
  backups = $state<BackupListResponse | null>(null);
  syncStatus = $state<SyncStatusResponse | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // Derived state
  isConnected = $derived(this.connectionStatus?.isConnected ?? false);
  isSyncing = $derived(this.syncStatus?.isSyncing ?? false);

  async init() {
    await this.refreshConnectionStatus();
    await this.setupEventListeners();
  }

  private async setupEventListeners() {
    await listen<{ progressPercent: number; stage: string }>(
      'cloud-backup://sync-progress',
      (event) => {
        this.syncStatus = {
          ...this.syncStatus!,
          progressPercent: event.payload.progressPercent,
          statusMessage: event.payload.stage
        };
      }
    );
  }

  async refreshConnectionStatus() {
    try {
      this.connectionStatus = await invoke('cloud_backup_get_connection_status');
    } catch (e) {
      this.error = String(e);
    }
  }

  async connect() {
    this.isLoading = true;
    this.error = null;
    try {
      this.connectionStatus = await invoke('cloud_backup_connect_google');
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async disconnect() {
    this.isLoading = true;
    try {
      await invoke('cloud_backup_disconnect_google');
      this.connectionStatus = null;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async syncNow() {
    this.isLoading = true;
    this.error = null;
    try {
      await invoke('cloud_backup_sync_now');
      await this.refreshBackups();
      await this.refreshConnectionStatus();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async refreshBackups() {
    if (!this.isConnected) return;
    try {
      this.backups = await invoke('cloud_backup_list_backups');
    } catch (e) {
      this.error = String(e);
    }
  }

  async restore(backupId: string, confirmation: string) {
    this.isLoading = true;
    this.error = null;
    try {
      await invoke('cloud_backup_restore', { args: { backupId, confirmation } });
      // App will reload after restore
    } catch (e) {
      this.error = String(e);
      this.isLoading = false;
    }
  }
}
```

---

## Phase 5: Paraglide Messages

Add to `messages/en.json`:

```json
{
  "cloudBackup.title": "Cloud Backup",
  "cloudBackup.connectGoogle": "Connect Google Drive",
  "cloudBackup.disconnect": "Disconnect",
  "cloudBackup.syncNow": "Sync Now",
  "cloudBackup.lastSync": "Last synced: {timestamp}",
  "cloudBackup.neverSynced": "Never synced",
  "cloudBackup.connectedAs": "Connected as {email}",
  "cloudBackup.restore": "Restore",
  "cloudBackup.restoreWarning": "This will replace your current collection with the backup. Type RESTORE to confirm.",
  "cloudBackup.confirmPlaceholder": "Type RESTORE to confirm",
  "cloudBackup.offline": "Sync unavailable - no internet connection",
  "cloudBackup.syncing": "Syncing...",
  "cloudBackup.backupCount": "{count} backups available",
  "cloudBackup.initialBackup": "Initial Backup",
  "cloudBackup.error.offline": "Cannot sync while offline",
  "cloudBackup.error.tokenExpired": "Session expired. Please reconnect.",
  "cloudBackup.error.importInProgress": "Cannot backup during data import"
}
```

---

## Phase 6: Testing Checklist

### Unit Tests (Rust)

- [ ] `secure_storage.rs` - Token store/retrieve/delete
- [ ] `oauth_service.rs` - Auth URL generation with PKCE
- [ ] `sync_backup.rs` - Compression/decompression
- [ ] `backup.rs` - Entity validation

### Integration Tests (Rust)

- [ ] OAuth flow with mock server
- [ ] Drive API with mock responses
- [ ] Version limit enforcement

### Frontend Tests (Vitest)

- [ ] Controller state management
- [ ] Component rendering
- [ ] Error handling

### E2E Tests (Manual)

- [ ] Complete OAuth flow on Windows
- [ ] Complete OAuth flow on Linux
- [ ] Backup upload and verify in Drive
- [ ] Restore and verify data
- [ ] Offline detection and UI state

---

## Quick Reference

| Resource           | Location                                                   |
| ------------------ | ---------------------------------------------------------- |
| Feature Spec       | [spec.md](spec.md)                                         |
| Research           | [research.md](research.md)                                 |
| Data Model         | [data-model.md](data-model.md)                             |
| API Contracts      | [contracts/tauri-commands.md](contracts/tauri-commands.md) |
| Google Drive Scope | `https://www.googleapis.com/auth/drive.appdata`            |
| OAuth Flow         | PKCE (S256)                                                |
| Backup Format      | Gzip-compressed SQLite                                     |
| Max Backups        | 5 (oldest deleted)                                         |
