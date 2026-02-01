# OAuth 2.0 Research: Google Drive Integration for Tauri 2

**Research Date**: 2026-01-30  
**Target Platforms**: Windows, Linux, Android  
**Framework**: Tauri 2 (Rust backend) + SvelteKit frontend

---

## Executive Summary

For implementing Google OAuth 2.0 in a Tauri 2 desktop/mobile application targeting Windows, Linux, and Android, the **recommended approach** is:

1. **OAuth Flow**: Authorization Code Grant with **PKCE** (Proof Key for Code Exchange)
2. **Redirect Capture**: Loopback IP address (`127.0.0.1`) for desktop, Deep Links for Android
3. **Rust Crates**: `oauth2` crate + `tauri-plugin-oauth` for local server handling
4. **Google Drive Scope**: `https://www.googleapis.com/auth/drive.file` (non-sensitive, app-created files only)

---

## 1. OAuth Flow Recommendation

### Primary Flow: Authorization Code Grant with PKCE

**Why PKCE?**

- PKCE (RFC 7636) is the standard for native/desktop applications
- Google explicitly recommends and supports PKCE for installed apps
- Protects against authorization code interception attacks
- No client secret required at runtime (important for desktop apps that cannot keep secrets)

**How PKCE Works:**

1. Generate a cryptographic `code_verifier` (43-128 character random string)
2. Create `code_challenge` = Base64URL(SHA256(code_verifier))
3. Include `code_challenge` in authorization URL
4. Include `code_verifier` when exchanging code for tokens
5. Google verifies the challenge matches the verifier

### Alternative: Device Authorization Flow

For scenarios where opening a browser is problematic:

- User receives a code to enter on a separate device
- Useful for limited-input devices
- **Not recommended** for Tauri apps with full browser access

---

## 2. Redirect URI Strategies by Platform

### Desktop (Windows & Linux): Loopback IP Address

Google recommends **loopback IP address** (`http://127.0.0.1:<port>`) for desktop apps:

```
http://127.0.0.1:8000
http://127.0.0.1:8001  (fallback ports)
```

**Why Loopback?**

- Google explicitly supports it for desktop OAuth clients
- Custom URI schemes are **deprecated** by Google for security reasons
- Works without external server or domain registration
- Port can be dynamic (any available port)

**Implementation Strategy:**

1. Start a temporary local HTTP server on an available port
2. Open system browser with authorization URL
3. Listen for redirect on loopback address
4. Capture authorization code from callback URL
5. Exchange code for tokens
6. Display success message and close server

### Android: Deep Links (Universal/App Links)

For Android, use **custom URI schemes** or **App Links**:

**Option A: Custom URI Scheme (simpler)**

```
rusty-shed://oauth/callback
```

- Registered in `tauri.conf.json` under `plugins.deep-link.mobile`
- No server verification required
- Risk of app impersonation

**Option B: App Links (more secure)**

```
https://your-domain.com/oauth/callback
```

- Requires `.well-known/assetlinks.json` on your server
- Verified by Android OS
- Recommended for production

**Tauri 2 Deep Link Configuration:**

```json
{
  "plugins": {
    "deep-link": {
      "mobile": [
        {
          "scheme": ["rusty-shed"],
          "appLink": false
        }
      ]
    }
  }
}
```

---

## 3. Recommended Rust Crates & Tauri Plugins

### Primary: `oauth2` crate

**Version**: 5.x (latest)  
**Features**: `reqwest` (async HTTP client)

```toml
[dependencies]
oauth2 = { version = "5", features = ["reqwest"] }
```

**Why `oauth2`?**

- Strongly-typed, comprehensive OAuth 2.0 implementation
- Built-in PKCE support
- Supports all OAuth flows
- Has a dedicated [Google example](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/google.rs)
- Async support with tokio

**Example PKCE Flow:**

```rust
use oauth2::{
    AuthUrl, ClientId, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl
};
use oauth2::basic::BasicClient;

let client = BasicClient::new(ClientId::new(client_id))
    .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth")?)
    .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token")?)
    .set_redirect_uri(RedirectUrl::new(format!("http://127.0.0.1:{}", port))?);

// Generate PKCE challenge
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate authorization URL
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new("https://www.googleapis.com/auth/drive.file".to_string()))
    .set_pkce_challenge(pkce_challenge)
    .url();
```

### Secondary: `tauri-plugin-oauth`

**Version**: 2.0.0  
**Purpose**: Spawns temporary localhost server for OAuth redirects

```toml
[dependencies]
tauri-plugin-oauth = "2"
```

**Why use it?**

- Purpose-built for Tauri OAuth flows
- Handles port management automatically
- Provides callback URL capture
- Works on desktop platforms

**Usage:**

```rust
use tauri_plugin_oauth::{start, OauthConfig};

let config = OauthConfig {
    ports: Some(vec![8000, 8001, 8002]),
    response: Some("Authentication successful! You can close this window.".into()),
};

let port = start_with_config(config, |url| {
    // Parse authorization code from URL
    // Exchange for tokens
}).await?;
```

### Deep Linking: `tauri-plugin-deep-link`

**For Android callback handling:**

```toml
[target."cfg(any(target_os = \"linux\", windows))".dependencies]
tauri-plugin-single-instance = { version = "2", features = ["deep-link"] }

[dependencies]
tauri-plugin-deep-link = "2"
```

### Token Storage: `tauri-plugin-store`

**For secure token persistence:**

```toml
[dependencies]
tauri-plugin-store = "2"
```

**Security Note**: Store should be combined with OS keychain for sensitive tokens:

- Windows: Windows Credential Manager
- Linux: libsecret/GNOME Keyring
- Android: Android Keystore

---

## 4. Google Cloud Console Setup

### Step 1: Create OAuth 2.0 Client

1. Go to [Google Cloud Console > APIs & Services > Credentials](https://console.cloud.google.com/apis/credentials)
2. Click "Create Credentials" → "OAuth client ID"
3. Select **"Desktop app"** application type
4. Name it "Rusty Shed Desktop"
5. Save the **Client ID** (no client secret needed for PKCE)

### Step 2: Configure OAuth Consent Screen

1. Go to [OAuth consent screen](https://console.cloud.google.com/apis/credentials/consent)
2. Choose "External" user type (or "Internal" for Workspace domains)
3. Fill in app information:
   - App name: "Rusty Shed"
   - User support email
   - Developer contact email
4. Add scopes:
   - `https://www.googleapis.com/auth/drive.file`
5. Add test users (during development)

### Step 3: Enable Google Drive API

1. Go to [API Library](https://console.cloud.google.com/apis/library)
2. Search for "Google Drive API"
3. Click "Enable"

### Redirect URIs to Configure

For **Desktop app** OAuth client:

```
http://127.0.0.1
http://localhost
```

Note: Google allows any port for loopback addresses, so you don't need to specify ports.

For **Android** (if using separate client):

- Create an "Android" application type
- Configure with your app's package name and SHA-1 fingerprint

---

## 5. Google Drive API Scopes

### Recommended Scope: `drive.file`

```
https://www.googleapis.com/auth/drive.file
```

**What it allows:**

- Create new files in user's Drive
- Read/modify files created by your app
- Read/modify files explicitly shared with your app by user

**What it restricts:**

- Cannot see or access other Drive files
- Cannot browse Drive file tree
- Cannot access files from other apps

**Benefits:**

- **Non-sensitive scope** - simpler OAuth verification process
- Follows principle of least privilege
- Matches FR-002: "minimum required Google Drive permissions"
- Matches BR-01: "app only has access to files it creates"

### Alternative Scopes (if needed)

| Scope            | Use Case                   | Sensitivity   |
| ---------------- | -------------------------- | ------------- |
| `drive.appdata`  | App-specific hidden folder | Non-sensitive |
| `drive.readonly` | Browse all Drive files     | Restricted    |
| `drive`          | Full Drive access          | Restricted    |

**Recommendation**: Stick with `drive.file` - it perfectly matches the backup use case.

---

## 6. Security Best Practices

### 6.1 PKCE Implementation

✅ **DO:**

- Use S256 challenge method (SHA-256), not `plain`
- Generate new `code_verifier` for each authorization request
- Use cryptographically secure random generator
- Verify `state` parameter on callback

```rust
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
```

### 6.2 State Parameter (CSRF Protection)

✅ **DO:**

- Generate random, unguessable `state` value
- Store `state` before redirect
- Verify `state` matches on callback
- Reject requests with mismatched state

```rust
let csrf_token = CsrfToken::new_random();
// Store csrf_token
let (auth_url, _) = client
    .authorize_url(|| csrf_token.clone())
    .url();

// On callback:
if received_state != expected_state {
    return Err("CSRF attack detected");
}
```

### 6.3 Localhost Server Security

✅ **DO:**

- Bind only to `127.0.0.1` (not `0.0.0.0`)
- Use short-lived server (close after receiving callback)
- Validate received URL structure
- Prefer random high-numbered ports

⚠️ **CAUTION:**

- Any local process can send requests to the loopback server
- Always validate the authorization code with Google before trusting
- The callback URL is just a delivery mechanism, not authentication

### 6.4 Token Storage

✅ **DO:**

- Store tokens encrypted at rest
- Use OS-provided secure storage when available
- Refresh tokens have no expiration but can be revoked
- Access tokens expire (typically 1 hour)
- Store refresh token, request new access token as needed

### 6.5 HTTP Client Configuration

✅ **DO:**

- Disable HTTP redirect following (SSRF prevention)
- Use HTTPS for all token endpoints
- Verify TLS certificates

```rust
let http_client = reqwest::ClientBuilder::new()
    .redirect(reqwest::redirect::Policy::none())
    .build()?;
```

### 6.6 Token Revocation

On "Disconnect" action:

1. Call Google's revocation endpoint
2. Clear locally stored tokens
3. Forget refresh token

```
POST https://oauth2.googleapis.com/revoke
Content-Type: application/x-www-form-urlencoded

token=<refresh_token_or_access_token>
```

---

## 7. Platform-Specific Considerations

### Windows

| Aspect        | Implementation                                         |
| ------------- | ------------------------------------------------------ |
| Redirect      | Loopback IP (127.0.0.1)                                |
| Browser       | Opens system default browser via `tauri-plugin-opener` |
| Token Storage | Use `tauri-plugin-store` + Windows Credential Manager  |
| Deep Links    | Supported but not recommended for OAuth                |

### Linux

| Aspect        | Implementation                                     |
| ------------- | -------------------------------------------------- |
| Redirect      | Loopback IP (127.0.0.1)                            |
| Browser       | Opens via `xdg-open`                               |
| Token Storage | Use `tauri-plugin-store` + libsecret/GNOME Keyring |
| Deep Links    | Requires `register_all()` at runtime               |
| Firewall      | Most firewalls allow loopback by default           |

### Android

| Aspect        | Implementation                              |
| ------------- | ------------------------------------------- |
| Redirect      | Custom URI scheme or App Links              |
| Browser       | Opens Chrome Custom Tabs or default browser |
| Token Storage | Android Keystore                            |
| Deep Links    | Configured via Tauri deep-link plugin       |
| Config        | Requires `tauri.conf.json` mobile section   |

**Android-specific code:**

```rust
#[cfg(target_os = "android")]
{
    // Use deep-link plugin for callback
    app.deep_link().on_open_url(|urls| {
        // Handle OAuth callback
    });
}
```

---

## 8. Implementation Architecture

### Recommended Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (Svelte)                        │
├─────────────────────────────────────────────────────────────────┤
│  1. User clicks "Connect Google Drive"                          │
│  2. invoke("start_oauth") → receives auth_url                   │
│  3. Opens auth_url in system browser                            │
│  4. Listens for oauth_callback event                            │
│  5. Updates UI with connection status                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Backend (Rust/Tauri)                         │
├─────────────────────────────────────────────────────────────────┤
│  start_oauth command:                                            │
│    1. Generate PKCE verifier + challenge                        │
│    2. Generate CSRF state token                                  │
│    3. Store state & verifier in memory                          │
│    4. Start localhost server (desktop) or register deep link    │
│    5. Build Google OAuth URL with params                         │
│    6. Return URL to frontend                                     │
│                                                                  │
│  On callback received:                                           │
│    1. Validate state parameter                                   │
│    2. Extract authorization code                                 │
│    3. Exchange code + verifier for tokens                        │
│    4. Fetch user email from Google                               │
│    5. Store tokens securely                                      │
│    6. Emit oauth_callback event to frontend                      │
│    7. Shut down localhost server                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Google OAuth Server                          │
├─────────────────────────────────────────────────────────────────┤
│  - Authorization endpoint: accounts.google.com/o/oauth2/v2/auth │
│  - Token endpoint: oauth2.googleapis.com/token                   │
│  - Revocation: oauth2.googleapis.com/revoke                      │
│  - Userinfo: www.googleapis.com/oauth2/v2/userinfo               │
└─────────────────────────────────────────────────────────────────┘
```

### Module Structure

```
src-tauri/src/
├── oauth/
│   ├── mod.rs              # OAuth module
│   ├── google.rs           # Google-specific OAuth client
│   ├── tokens.rs           # Token storage & refresh
│   └── callback_server.rs  # Localhost callback server (desktop)
└── google_drive/
    ├── mod.rs              # Drive API module
    ├── client.rs           # Drive API client
    ├── backup.rs           # Backup operations
    └── restore.rs          # Restore operations
```

---

## 9. Key Dependencies Summary

### Cargo.toml additions:

```toml
[dependencies]
oauth2 = { version = "5", features = ["reqwest"] }
tauri-plugin-oauth = "2"
tauri-plugin-deep-link = "2"
tauri-plugin-store = "2"
reqwest = { version = "0.12", features = ["json"] }

[target."cfg(desktop)".dependencies]
tauri-plugin-single-instance = { version = "2", features = ["deep-link"] }
```

### tauri.conf.json additions:

```json
{
  "plugins": {
    "deep-link": {
      "desktop": {
        "schemes": ["rusty-shed"]
      },
      "mobile": [
        {
          "scheme": ["rusty-shed"],
          "appLink": false
        }
      ]
    }
  }
}
```

---

## 10. References

1. **Google OAuth for Native Apps**: https://developers.google.com/identity/protocols/oauth2/native-app
2. **PKCE RFC 7636**: https://tools.ietf.org/html/rfc7636
3. **oauth2-rs crate**: https://docs.rs/oauth2/latest/oauth2/
4. **tauri-plugin-oauth**: https://github.com/FabianLars/tauri-plugin-oauth
5. **Tauri Deep Linking**: https://v2.tauri.app/plugin/deep-linking/
6. **Google Drive API Scopes**: https://developers.google.com/identity/protocols/oauth2/scopes#drive
7. **OAuth Best Practices RFC 8252**: https://tools.ietf.org/html/rfc8252

---

## Appendix: Google OAuth Endpoints

| Endpoint         | URL                                             |
| ---------------- | ----------------------------------------------- |
| Authorization    | `https://accounts.google.com/o/oauth2/v2/auth`  |
| Token Exchange   | `https://oauth2.googleapis.com/token`           |
| Token Revocation | `https://oauth2.googleapis.com/revoke`          |
| User Info        | `https://www.googleapis.com/oauth2/v2/userinfo` |

### Authorization URL Parameters

| Parameter               | Required    | Description                               |
| ----------------------- | ----------- | ----------------------------------------- |
| `client_id`             | Yes         | OAuth client ID from Google Cloud Console |
| `redirect_uri`          | Yes         | Loopback URL or deep link                 |
| `response_type`         | Yes         | Always `code`                             |
| `scope`                 | Yes         | Space-separated list of scopes            |
| `code_challenge`        | Recommended | PKCE challenge                            |
| `code_challenge_method` | Recommended | `S256`                                    |
| `state`                 | Recommended | CSRF protection token                     |
| `login_hint`            | Optional    | Pre-fill email if known                   |
