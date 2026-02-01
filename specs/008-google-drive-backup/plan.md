# Implementation Plan: Google Drive Cloud Backup

**Branch**: `008-google-drive-backup` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/008-google-drive-backup/spec.md`

## Summary

Enable users to securely backup and restore their railway collection database to Google Drive. The system uses OAuth 2.0 with PKCE for authentication, stores tokens securely using platform-appropriate keystores, uploads compressed SQLite backups to a private app folder, and provides restore functionality with version management (max 5 backups).

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)  
**Primary Dependencies**:

- Backend: `oauth2` (PKCE), `google-drive3` (Drive API), `keyring` (desktop), `tauri-plugin-stronghold` (Android)
- Frontend: SvelteKit, `@tauri-apps/api`, Svelte 5 runes
- Tauri Plugins: `oauth`, `deep-link`, `stronghold`, `shell`

**Storage**:

- Local: SQLite via sqlx (existing collection database)
- Cloud: Google Drive `appDataFolder` (private, app-only access)
- Credentials: OS keyring (Windows/Linux) or Stronghold vault (Android)

**Testing**: `cargo test` (Rust), `vitest` (frontend)  
**Target Platform**: Windows, Linux, Android  
**Project Type**: Desktop/Mobile hybrid (Tauri 2)  
**Performance Goals**: Backup upload < 30s for typical collection, UI remains responsive  
**Constraints**: Offline-capable detection, < 200ms for connectivity checks  
**Scale/Scope**: Single-user, personal collection (typically < 50MB database)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Status  | Notes                                                                         |
| ---------------------------- | ------- | ----------------------------------------------------------------------------- |
| **Database (Persistence)**   | ✅ PASS | Uses existing SQLite via sqlx; no new migrations required for backup metadata |
| **State Management**         | ✅ PASS | OAuth state managed via secure storage, not domain aggregates                 |
| **API Design & Transport**   | ✅ PASS | New Tauri commands follow ADR 8 conventions (Args → Input → UseCase)          |
| **Domain Logic Location**    | ✅ PASS | All backup/restore logic in Rust backend; frontend only for UX                |
| **Type Generation**          | ✅ PASS | All commands use specta for TypeScript binding generation                     |
| **Safe Rust Practices**      | ✅ PASS | Result<T, E> error handling, no unwrap in production code                     |
| **Test-First Emphasis**      | ✅ PASS | Unit tests for backup service, integration tests for OAuth flow               |
| **Paraglide Strings**        | ✅ PASS | All UI strings via Paraglide messaging system                                 |
| **Performance Requirements** | ✅ PASS | Progress indicators for sync, background threading for uploads                |

**Re-check after Phase 1**: All gates pass. No violations requiring justification.

## Project Structure

### Documentation (this feature)

```text
specs/008-google-drive-backup/
├── plan.md              # This file
├── research.md          # Phase 0 output (complete)
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (Tauri command definitions)
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/src/
├── features/
│   └── cloud_backup/           # NEW: Feature module
│       ├── mod.rs              # Module exports
│       ├── domain/
│       │   ├── mod.rs
│       │   ├── backup.rs       # Backup entity
│       │   ├── connection.rs   # GoogleConnection value object
│       │   └── errors.rs       # Domain errors
│       ├── application/
│       │   ├── mod.rs
│       │   ├── connect_google.rs    # OAuth use case
│       │   ├── disconnect_google.rs # Disconnect use case
│       │   ├── sync_backup.rs       # Upload backup use case
│       │   ├── restore_backup.rs    # Restore use case
│       │   └── list_backups.rs      # Query use case
│       └── infrastructure/
│           ├── mod.rs
│           ├── google_drive.rs      # Drive API client
│           ├── oauth_service.rs     # OAuth flow handler
│           ├── secure_storage.rs    # Token storage abstraction
│           └── connectivity.rs      # Network status checker
├── commands/
│   └── cloud_backup.rs         # NEW: Tauri command handlers
└── lib.rs                      # Register new commands

src/lib/features/
└── cloud-backup/               # NEW: Frontend feature
    ├── components/
    │   ├── GoogleConnectButton.svelte
    │   ├── SyncButton.svelte
    │   ├── BackupList.svelte
    │   ├── RestoreConfirmModal.svelte
    │   └── ConnectivityIndicator.svelte
    ├── controllers/
    │   └── cloudBackup.svelte.ts   # Controller with $state/$derived
    ├── stores/
    │   └── connectivity.ts         # Connectivity store
    └── index.ts                    # Feature exports

src/routes/my-settings/
└── +page.svelte                # UPDATE: Add cloud backup section
```

**Structure Decision**: Follows existing feature-grouped DDD pattern. Backend creates new `cloud_backup` feature module with domain/application/infrastructure layers. Frontend creates matching feature folder with controller class pattern.

## Complexity Tracking

> No constitution violations. Table intentionally empty.

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| —         | —          | —                                    |

## Dependencies to Add

### Rust (src-tauri/Cargo.toml)

```toml
# OAuth 2.0
oauth2 = "5"

# Google Drive API
google-drive3 = "7.0"

# Secure storage (desktop)
[target.'cfg(not(target_os = "android"))'.dependencies]
keyring = { version = "3", features = ["windows-native", "sync-secret-service", "crypto-rust"] }

# Secret handling
secrecy = { version = "0.10", features = ["serde"] }
zeroize = "1"

# Network connectivity
online = { version = "4.0.2", default-features = false, features = ["tokio"] }

# Compression
flate2 = "1"
```

### Tauri Plugins

```bash
pnpm tauri add oauth
pnpm tauri add deep-link
pnpm tauri add stronghold
```

### Frontend (package.json)

```json
{
  "@tauri-apps/plugin-stronghold": "^2.0.0"
}
```
