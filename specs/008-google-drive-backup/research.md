# Research: Google Drive Cloud Backup

**Feature**: 008-google-drive-backup  
**Date**: 2026-01-30  
**Status**: Complete

## Executive Summary

This document consolidates research findings for implementing secure Google Drive cloud backup across Windows, Linux, and Android platforms in a Tauri 2 application.

---

## 1. OAuth 2.0 Authentication

### Decision: Authorization Code Grant with PKCE

**Rationale**: Google explicitly recommends PKCE (Proof Key for Code Exchange) for desktop and mobile applications. It protects against authorization code interception without requiring a client secret at runtime.

**Alternatives Considered**:

| Alternative        | Rejected Because                                                                   |
| ------------------ | ---------------------------------------------------------------------------------- |
| Implicit Grant     | Deprecated by Google, tokens exposed in URL                                        |
| Device Code Flow   | Poor UX (requires manual code entry), not recommended for apps with browser access |
| Client Credentials | For server-to-server, not user-facing apps                                         |

### Redirect Strategy by Platform

| Platform      | Strategy          | Implementation                                       |
| ------------- | ----------------- | ---------------------------------------------------- |
| Windows/Linux | Loopback redirect | Temporary local server on `http://127.0.0.1:<port>`  |
| Android       | Custom URI scheme | `rusty-shed://oauth/callback` registered in manifest |

### Google Cloud Console Configuration

1. Create OAuth 2.0 credentials for "Desktop application" type
2. Configure authorized redirect URIs:
   - `http://127.0.0.1` (loopback, port dynamically assigned)
   - `rusty-shed://oauth/callback` (for Android)
3. Request scope: `https://www.googleapis.com/auth/drive.appdata` (non-sensitive)

### Recommended Crates

| Crate                    | Version | Purpose                                         |
| ------------------------ | ------- | ----------------------------------------------- |
| `oauth2`                 | 5.x     | Core OAuth 2.0 with PKCE support                |
| `tauri-plugin-oauth`     | 2.x     | Localhost server for callback capture (desktop) |
| `tauri-plugin-deep-link` | 2.x     | Custom URI scheme handling (Android)            |

### Security Requirements

- Use **S256** challenge method for PKCE (not plain)
- Validate **state** parameter to prevent CSRF
- Bind localhost server to **127.0.0.1 only** (not 0.0.0.0)
- Disable HTTP redirect following in reqwest client
- Store tokens using secure storage (see Section 2)

---

## 2. Secure Token Storage

### Decision: Platform-Aware Hybrid Storage

**Primary (Android)**: `tauri-plugin-stronghold` - Official Tauri plugin with Argon2-encrypted vault  
**Primary (Desktop)**: `keyring-rs` - Cross-platform OS keyring abstraction

**Rationale**: No single crate covers Windows, Linux, AND Android. Stronghold provides full cross-platform support but requires a vault password. For desktop, OS keyrings are more seamless.

**Alternatives Considered**:

| Alternative           | Rejected Because                                                  |
| --------------------- | ----------------------------------------------------------------- |
| `tauri-plugin-store`  | No encryption, stores as plain JSON                               |
| `keyring-rs` only     | No Android support                                                |
| Custom encrypted file | Reinventing the wheel, key management complexity                  |
| Stronghold everywhere | Acceptable alternative, but desktop users expect seamless keyring |

### Platform Coverage

| Platform | Primary Store                          | Crate/Plugin                               |
| -------- | -------------------------------------- | ------------------------------------------ |
| Windows  | Credential Manager                     | `keyring-rs` (windows-native feature)      |
| Linux    | Secret Service (GNOME Keyring/KWallet) | `keyring-rs` (sync-secret-service feature) |
| Android  | Stronghold encrypted vault             | `tauri-plugin-stronghold`                  |

### Dependencies

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-stronghold = "2.3"
secrecy = { version = "0.10", features = ["serde"] }
zeroize = "1"

[target.'cfg(not(target_os = "android"))'.dependencies]
keyring = { version = "3", features = [
    "windows-native",
    "sync-secret-service",
    "crypto-rust"
] }
```

### Security Best Practices

- Use `secrecy::SecretString` for token handling (zeroizes on drop)
- Implement token rotation (refresh 5 minutes before expiry)
- Never log or derive Debug for token types
- Use structured key naming: `com.rusty-shed.oauth.google.{user_id}`

---

## 3. Google Drive API Integration

### Decision: `google-drive3` Crate with `appDataFolder`

**Rationale**: Official Google crate with full async/Tokio support. The `appDataFolder` scope provides app-private storage that's invisible to users and other apps.

**Alternatives Considered**:

| Alternative              | Rejected Because                                           |
| ------------------------ | ---------------------------------------------------------- |
| Direct REST with reqwest | Significant implementation effort, no advantage            |
| `drive.file` scope       | Files visible in user's Drive, not appropriate for backups |
| Third-party crates       | None actively maintained for Drive API v3                  |

### OAuth Scope

```
https://www.googleapis.com/auth/drive.appdata
```

**Properties**:

- **Non-sensitive scope**: Simpler OAuth verification with Google
- Files stored in hidden app data folder
- Only this app can access the files
- Data auto-deleted when user uninstalls app
- Cannot share files, cannot trash (must permanently delete)

### Dependencies

```toml
[dependencies]
google-drive3 = "7.0"
```

### File Format Decision

**Format**: Gzip-compressed SQLite (`.db.gz`)

| Format              | Compression | Chosen Because                                     |
| ------------------- | ----------- | -------------------------------------------------- |
| Raw SQLite          | 0%          | ❌ Large files, slow uploads                       |
| **Gzip compressed** | 60-80%      | ✅ Fast, low CPU, excellent compression for SQLite |
| Encrypted           | N/A         | ⚠️ Future consideration for sensitive data         |

### Naming Convention

```
rusty_shed_backup_{ISO8601}_{schema_version}.db.gz
```

Example: `rusty_shed_backup_20260130T143022Z_v5.db.gz`

### API Operations Required

| Operation     | Method                                   | Notes                                 |
| ------------- | ---------------------------------------- | ------------------------------------- |
| Create folder | `files().create()`                       | Check if exists first                 |
| Upload file   | `files().create().upload_resumable()`    | For files > 5MB                       |
| List files    | `files().list().spaces("appDataFolder")` | Sort by modifiedTime desc             |
| Download file | `files().get().param("alt", "media")`    | Returns file content                  |
| Delete file   | `files().delete()`                       | Permanent (no trash in appDataFolder) |

### Error Handling

- Exponential backoff for 5xx errors and rate limits
- Max 3 retries with 100ms × 2^attempt delay
- Token expiry triggers re-authentication flow
- User-friendly error messages for network failures

---

## 4. Network Connectivity Detection

### Decision: Backend-Primary Hybrid Approach

**Rationale**: Browser's `navigator.onLine` is unreliable (only detects network interface, not actual internet access). Backend verification provides authoritative state, while frontend provides instant UI feedback.

**Alternatives Considered**:

| Alternative                        | Rejected Because                          |
| ---------------------------------- | ----------------------------------------- |
| Frontend only (`navigator.onLine`) | Unreliable, false positives with VMs/VPNs |
| DNS lookup                         | Can be cached, doesn't verify HTTP layer  |
| ICMP ping                          | Blocked on many networks                  |

### Architecture

```
Frontend (Svelte)                      Backend (Rust)
┌────────────────────┐                ┌────────────────────┐
│ navigator.onLine   │◀───events──────│ online crate       │
│ (instant feedback) │                │ (authoritative)    │
│                    │───command──────▶│                    │
│ Svelte store       │                │ Periodic checks    │
└────────────────────┘                └────────────────────┘
```

### Dependencies

```toml
[dependencies]
online = { version = "4.0.2", default-features = false, features = ["tokio"] }
```

### Implementation Strategy

1. **Backend periodic check**: Every 30 seconds via `online` crate
2. **On-demand check**: Before sync operations
3. **Event emission**: `connectivity-changed` event on state change
4. **Debouncing**: 500ms-1s to prevent UI flicker on rapid changes
5. **Frontend fallback**: `navigator.onLine` for immediate feedback

### Platform-Specific Notes

| Platform | Notes                                                                  |
| -------- | ---------------------------------------------------------------------- |
| Windows  | `online` crate works, checks Google/Firefox captive portals            |
| Linux    | `online` crate works, no D-Bus dependency required                     |
| Android  | Consider Kotlin plugin for `ConnectivityManager` callbacks for best UX |

---

## 5. Tauri 2 Plugins Required

| Plugin                    | Purpose                         | Platform                              |
| ------------------------- | ------------------------------- | ------------------------------------- |
| `tauri-plugin-oauth`      | Localhost OAuth callback server | Desktop                               |
| `tauri-plugin-deep-link`  | Custom URI scheme handling      | Android                               |
| `tauri-plugin-stronghold` | Encrypted token storage         | Android (primary), Desktop (fallback) |
| `tauri-plugin-shell`      | Open system browser for OAuth   | All                                   |

---

## 6. Key Decisions Summary

| Area          | Decision                                    | Key Reason                           |
| ------------- | ------------------------------------------- | ------------------------------------ |
| OAuth Flow    | PKCE Authorization Code                     | Google recommendation, security      |
| Token Storage | keyring-rs (desktop) + Stronghold (Android) | Platform-appropriate secure storage  |
| Drive API     | `google-drive3` + `appDataFolder`           | Official crate, private storage      |
| File Format   | Gzip-compressed SQLite                      | Best compression/performance balance |
| Connectivity  | Hybrid (backend authoritative)              | Reliability over simplicity          |
| Backup Limit  | 5 versions                                  | Per spec BR-02                       |

---

## 7. Risk Mitigations

| Risk                  | Mitigation                                           |
| --------------------- | ---------------------------------------------------- |
| Token interception    | PKCE S256, localhost-only binding                    |
| Token exposure        | SecretString, no Debug, zeroize on drop              |
| Corrupted backup      | Validate SQLite integrity before upload              |
| Quota exceeded        | Compress files, enforce version limit                |
| Network flakiness     | Retry with exponential backoff, clear error messages |
| OAuth token expiry    | Automatic refresh, re-auth flow if refresh fails     |
| Multi-device conflict | Last-write-wins (per spec, no conflict resolution)   |

---

## 8. Open Questions for Planning

1. **Vault password for Stronghold on Android**: Should we derive from device credentials, or prompt user?
2. **Backup encryption**: Should backups be encrypted before upload (beyond HTTPS)?
3. **Progress reporting**: `google-drive3` doesn't expose upload progress—implement chunked upload?
4. **Android OAuth redirect**: App Links vs custom scheme—need to verify Tauri 2 support.
