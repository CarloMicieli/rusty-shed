---
name: tauri2
description: Use this skill when working with Tauri 2.0 features, IPC communication, Tauri commands, plugins, window management, file system operations, or when the user asks about Tauri-specific functionality, app configuration, or desktop/mobile app features.
version: 1.0.0
---

# Tauri 2.0 Integration Standards

This skill provides comprehensive guidelines for building desktop and mobile applications with Tauri 2.0, focusing on IPC, plugins, security, and cross-platform development.

## When This Skill Applies

Use this skill when:

- Implementing Tauri commands or IPC communication
- Working with Tauri plugins (fs, dialog, http, oauth, etc.)
- Configuring Tauri app settings (`tauri.conf.json`)
- Managing application state with `tauri::State`
- Implementing window management or native features
- Working with file system, HTTP, or system APIs
- User mentions: Tauri, IPC, commands, plugins, desktop app, mobile app

## Project Context

- **Framework**: Tauri 2.0 (latest stable)
- **Backend**: Rust in `src-tauri/`
- **Frontend**: Svelte 5 + SvelteKit in `src/`
- **Type Safety**: `tauri-specta` + `specta-typescript` for type generation
- **Platforms**: Desktop (Windows, macOS, Linux) + Mobile (iOS, Android)

## Core Principles

### Security First

- Minimize IPC surface area (principle of least privilege)
- Use strict allowlist in `tauri.conf.json`
- Validate all inputs from frontend
- Never trust data from the UI layer
- Use Content Security Policy (CSP)
- Scope file system access appropriately

### Type Safety

- Use `tauri-specta` to generate TypeScript types from Rust
- Keep frontend and backend types in sync automatically
- Never use `any` types in TypeScript for Tauri commands
- Define clear command interfaces

### Clean Architecture

- Treat Tauri commands as "adapters" (infrastructure layer)
- Keep business logic in domain layer
- Commands should orchestrate use cases, not contain logic
- Commands convert between Tauri's world and domain types

## Tauri Commands

### Command Structure

```rust
use tauri::State;
use specta::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct User {
    id: String,
    email: String,
    name: String,
}

/// Authenticates a user with email and password.
///
/// # Errors
/// Returns error if credentials are invalid or service is unavailable.
#[tauri::command]
#[specta::specta]
pub async fn login(
    request: LoginRequest,
    auth_service: State<'_, AuthService>,
) -> Result<User, String> {
    auth_service
        .login(&request.email, &request.password)
        .await
        .map_err(|e| e.to_string())
}
```

### Command Registration

```rust
use tauri_specta::Builder as SpecBuilder;

fn main() {
    // Generate TypeScript bindings
    let builder = SpecBuilder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            login,
            logout,
            get_dashboard_summary,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(specta_typescript::Typescript::default(), "../src/lib/bindings.ts")
        .expect("Failed to export TypeScript bindings");

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            // App setup
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend Usage

```typescript
// src/lib/bindings.ts (auto-generated)
import { invoke } from '@tauri-apps/api/core';

export type LoginRequest = { email: string; password: string };
export type User = { id: string; email: string; name: string };

export async function login(request: LoginRequest): Promise<User> {
  return invoke('login', { request });
}
```

```svelte
<script lang="ts">
  import { login } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';

  let email = $state('');
  let password = $state('');

  async function handleLogin() {
    try {
      const user = await login({ email, password });
      // Handle successful login
    } catch (error) {
      // Handle error
    }
  }
</script>
```

## State Management

### Managed State

```rust
use tauri::{State, Manager};

pub struct AppState {
    db_pool: sqlx::SqlitePool,
    config: AppConfig,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db_pool = create_db_pool()?;
            app.manage(AppState {
                db_pool,
                config: load_config()?,
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
async fn get_items(state: State<'_, AppState>) -> Result<Vec<Item>, String> {
    let pool = &state.db_pool;
    // Use pool
    Ok(items)
}
```

### Application Handle

```rust
use tauri::{AppHandle, Manager};

#[tauri::command]
#[specta::specta]
async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("settings")
        .ok_or("Settings window not found")?;

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    Ok(())
}
```

## Tauri Plugins

### Common Plugins

- **fs**: File system operations with proper scoping
- **dialog**: Native file/folder/message dialogs
- **http**: HTTP client for external API calls
- **oauth**: OAuth authentication flows
- **opener**: Open URLs/files with default system apps
- **log**: Application logging
- **stronghold**: Secure storage for secrets

### File System Plugin

```rust
// In tauri.conf.json
{
  "plugins": {
    "fs": {
      "scope": [
        "$APPDATA/*",
        "$DOCUMENT/*"
      ]
    }
  }
}
```

```typescript
// Frontend usage
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

const content = await readTextFile('data.json', {
  baseDir: BaseDirectory.AppData
});

await writeTextFile('data.json', JSON.stringify(data), {
  baseDir: BaseDirectory.AppData
});
```

### Dialog Plugin

```typescript
import { open, save, message } from '@tauri-apps/plugin-dialog';

// Open file dialog
const file = await open({
  multiple: false,
  filters: [
    {
      name: 'JSON',
      extensions: ['json']
    }
  ]
});

// Save file dialog
const path = await save({
  defaultPath: 'export.csv'
});

// Message dialog
await message('Operation completed successfully', {
  title: 'Success',
  kind: 'info'
});
```

### HTTP Plugin

```typescript
import { fetch } from '@tauri-apps/plugin-http';

const response = await fetch('https://api.example.com/data', {
  method: 'GET',
  headers: {
    Authorization: `Bearer ${token}`
  }
});

const data = await response.json();
```

## Window Management

### Creating Windows

```rust
use tauri::{Manager, WebviewWindowBuilder};

#[tauri::command]
#[specta::specta]
async fn open_detail_window(app: AppHandle, item_id: String) -> Result<(), String> {
    WebviewWindowBuilder::new(
        &app,
        format!("item-{}", item_id),
        tauri::WebviewUrl::App(format!("/item/{}", item_id).into()),
    )
    .title("Item Details")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

### Frontend Window API

```typescript
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const window = getCurrentWebviewWindow();

// Window operations
await window.minimize();
await window.maximize();
await window.close();
await window.setTitle('New Title');
await window.setSize({ width: 800, height: 600 });

// Window events
window.listen('window-focus', () => {
  console.log('Window focused');
});
```

## Events System

### Emitting Events from Rust

```rust
use tauri::{Emitter, Manager};

#[tauri::command]
#[specta::specta]
async fn start_sync(app: AppHandle) -> Result<(), String> {
    // Start background sync
    tokio::spawn(async move {
        // Do sync work
        app.emit("sync-progress", SyncProgress { percent: 50 })
            .expect("Failed to emit event");

        app.emit("sync-complete", ())
            .expect("Failed to emit event");
    });

    Ok(())
}
```

### Listening to Events in Frontend

```svelte
<script lang="ts">
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { onMount } from 'svelte';

  let progress = $state(0);

  onMount(() => {
    const window = getCurrentWebviewWindow();

    const unlisten = window.listen<{ percent: number }>('sync-progress', (event) => {
      progress = event.payload.percent;
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

<div>Progress: {progress}%</div>
```

## Configuration

### tauri.conf.json Structure

```json
{
  "productName": "RustyShed",
  "version": "0.1.0",
  "identifier": "com.example.rustyshed",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  },
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
    },
    "windows": [
      {
        "title": "RustyShed",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ]
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.icns", "icons/icon.ico"]
  },
  "plugins": {
    "fs": {
      "scope": ["$APPDATA/*", "$DOCUMENT/*"]
    },
    "http": {
      "scope": ["https://api.example.com/*"]
    }
  }
}
```

## Error Handling

### Backend Error Mapping

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

#[tauri::command]
#[specta::specta]
async fn get_item(id: String) -> Result<Item, String> {
    find_item(&id)
        .await
        .map_err(|e: AppError| e.into())
}
```

### Frontend Error Handling

```typescript
try {
  const item = await getItem(id);
  // Handle success
} catch (error) {
  if (typeof error === 'string') {
    // Show user-friendly error message
    console.error('Error:', error);
  }
}
```

## Mobile Support

### Platform-Specific Code

```rust
#[cfg(target_os = "android")]
use tauri_plugin_android::*;

#[cfg(target_os = "ios")]
use tauri_plugin_ios::*;

#[tauri::command]
#[specta::specta]
async fn platform_specific_action() -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        Ok("Android".to_string())
    }

    #[cfg(target_os = "ios")]
    {
        Ok("iOS".to_string())
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Ok("Desktop".to_string())
    }
}
```

## Best Practices

### Do's

- Use `tauri-specta` for type-safe IPC
- Validate all inputs from frontend
- Use managed state for shared resources
- Implement proper error types and handling
- Scope file system and HTTP access strictly
- Use events for background operations
- Keep commands thin (orchestration only)
- Document all public commands
- Test commands with unit tests

### Don'ts

- **Don't** put business logic in commands
- **Don't** use `unwrap()` in commands (return Results)
- **Don't** trust frontend data without validation
- **Don't** expose unnecessary commands to frontend
- **Don't** use global mutable state
- **Don't** block async runtime with sync operations
- **Don't** hardcode file paths (use Tauri's path APIs)
- **Don't** skip CSP configuration

## Security Checklist

- [ ] CSP configured in `tauri.conf.json`
- [ ] File system scope properly restricted
- [ ] HTTP plugin scope limited to necessary domains
- [ ] All command inputs validated
- [ ] Sensitive data stored securely (Stronghold)
- [ ] Commands follow principle of least privilege
- [ ] No secrets in frontend code
- [ ] Proper error messages (no sensitive info leakage)

## Testing

### Command Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_success() {
        let service = create_test_auth_service();
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login(request, State::from(&service)).await;
        assert!(result.is_ok());
    }
}
```

## Performance

- Use async commands for I/O operations
- Avoid blocking the main thread
- Stream large data sets instead of loading all at once
- Use background tasks for long-running operations
- Cache frequently accessed data in managed state
- Use efficient serialization (bincode for binary, serde_json for JSON)

## Resources

- [Tauri 2.0 Documentation](https://v2.tauri.app/)
- [Tauri Plugins](https://v2.tauri.app/plugin/)
- [tauri-specta Documentation](https://github.com/oscartbeaumont/tauri-specta)
- [Security Best Practices](https://v2.tauri.app/security/)
- [Mobile Development Guide](https://v2.tauri.app/develop/mobile/)
