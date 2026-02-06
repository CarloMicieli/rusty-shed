# Repository Tech Stack Summary

**Date**: February 6, 2026  
**Project**: Rusty Shed (Scale Model Collection Management)

---

## Executive Summary

**Rusty Shed** is a desktop application built with **Tauri 2** (Rust backend) + **Svelte 5 frontend** for managing railway model collections. The app provides budget tracking, digital inventory management, DCC decoder control, Google Drive backup, and detailed model documentation.

---

## Technology Stack

### 🖥️ Frontend

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| **Framework** | Svelte | 5.x | Reactive UI with Runes |
| **Build Tool** | Vite | Latest | Fast HMR dev server |
| **Language** | TypeScript | 5.x | Type-safe frontend |
| **CSS** | Tailwind CSS | 4.x | Utility-first styling |
| **UI Components** | shadcn-svelte | 1.1.1 | Headless component library |
| **Router** | SvelteKit | 2.x | File-based routing |
| **Localization** | Paraglide-JS | 2.7.1 | Message i18n (EN, IT) |
| **Icons** | Lucide Svelte | 0.563.1 | 563+ SVG icons |
| **Testing** | Vitest | Latest | Unit testing |
| **E2E Testing** | Playwright | Latest | Integration testing |
| **Data Viz** | D3 + Layerchart | Latest | Charts/graphs |
| **Validation** | Zod | 4.3.6 | Runtime type validation |
| **State** | Svelte $state Rune | 5.x | Reactive state management |

### 🦀 Backend

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| **Language** | Rust | 1.93.0+ | Memory-safe systems language |
| **Edition** | 2024 | - | Modern Rust syntax |
| **App Framework** | Tauri | 2.9.5 | Desktop app + IPC bridge |
| **Runtime** | Tokio | 1.49.0 | Async multi-threaded executor |
| **Database** | SQLite | Latest | Embedded relational DB |
| **ORM** | sqlx | 0.8.6 | Type-safe query builder |
| **Serialization** | serde | 1.0.228 | Data serialization framework |
| **API Typing** | specta | 2.0.0-rc.22 | Auto TypeScript bindings |
| **Error Handling** | thiserror | 2.0.18 | Custom error derivation |
| **Validation** | garde | 0.22.1 | Derive-based validation |
| **Logging** | flexi_logger | 0.31.8 | Rotating log files |
| **JSON** | serde_json | 1.0.149 | JSON parsing |
| **Decimals** | rust_decimal | 1.40.0 | Precise monetary values |
| **Date/Time** | chrono | 0.4.43 | Date/time handling |
| **UUID** | uuid | 1.20.0 | UUID generation (v4) |
| **HTTP** | reqwest | 0.12 | Async HTTP client |
| **OAuth2** | oauth2 | 5.x | Google Drive authentication |
| **Compression** | flate2 + tar + zip | Latest | Archive handling |

### 📱 Desktop Framework

| Component | Tech | Version | Purpose |
|-----------|------|---------|---------|
| **App Container** | Tauri | 2.9.5 | Native window + IPC |
| **Filesystem** | tauri-plugin-fs | 2.4.5 | File I/O operations |
| **HTTP** | tauri-plugin-http | 2.5.6 | HTTP requests |
| **Logging** | tauri-plugin-log | 2.8.0 | App logging |
| **OAuth** | tauri-plugin-oauth | 2 | OAuth flows |
| **Keychain** | tauri-plugin-stronghold | 2 | Secure storage |
| **URL Opener** | tauri-plugin-opener | 2.5.3 | Open external links |
| **Deep Links** | tauri-plugin-deep-link | 2 | URL scheme handling |

### 🔄 IPC Communication

- **Protocol**: Tauri Command System (RPC-like)
- **Type Safety**: specta-typescript auto-generates `src/lib/bindings.ts`
- **Pattern**: 
  - Backend: `#[tauri::command] #[specta::specta] async fn name(...)`
  - Frontend: `import { invoke } from '@tauri-apps/api/core'; invoke('name', args)`

---

## Architecture

### Backend (Rust)

**Pattern**: Domain-Driven Design (DDD) + Layered Architecture

```
src-tauri/src/
├── budget/                    # Budget tracking feature
│   ├── application/          # Use cases
│   ├── domain/              # Business logic
│   ├── infrastructure/       # Database, external services
│   └── interface/           # Tauri commands
├── catalog/                 # Railway model catalog
├── collecting/              # Collection management
├── dashboard/               # Dashboard summaries
├── dcc_inventory/           # DCC decoder management
├── cloud_backup/            # Google Drive sync
├── import/                  # Data import utilities
├── maintenance/             # Maintenance tracking
├── media/                   # Image management (NEW for Feature 014)
├── sellers/                 # Seller management
├── tracks_inventory/        # Track inventory
├── wishlist/                # Wish list management
├── core/                    # Shared infrastructure
│   ├── infrastructure/      # DB, error handling, logging
│   └── interface/           # Common commands
├── state.rs                 # AppState (managed state)
├── settings.rs              # User settings
├── lib.rs                   # Module root + command collector
└── main.rs                  # Desktop app entry point
```

**Key Principles**:
- No panics in production code (return Result)
- Strong typing with newtype patterns
- Comprehensive error handling with custom error types
- Async-first with Tokio
- No unsafe code unless documented
- Extensive rustdoc comments

### Frontend (Svelte)

**Pattern**: Component-based with file-based routing

```
src/
├── routes/                  # File-based routing
│   ├── +page.svelte        # Home page
│   ├── models/
│   │   ├── [modelId]/
│   │   │   └── +page.svelte # NEW: Model details page (Feature 014)
│   ├── collection/
│   ├── wishlist/
│   └── ...
├── lib/
│   ├── bindings.ts          # AUTO-GENERATED: Backend commands
│   ├── components/          # Reusable Svelte components
│   ├── utils/               # Helper functions
│   ├── stores/              # Svelte stores (if needed)
│   └── paraglide/           # Localization messages (compiled)
├── hooks.client.ts          # Global hooks
├── app.html                 # HTML template
└── app.d.ts                 # Type definitions
```

**Key Principles**:
- Svelte 5 Runes: $state, $derived, $props
- TypeScript strict mode
- Tailwind CSS first (no custom CSS unless necessary)
- shadcn-svelte for complex components
- Paraglide-JS for ALL user-facing strings
- One component per file (exception: small helpers in .svelte.ts)

---

## Data Persistence

### Database (SQLite)

- **Location**: `{AppLocalData}/database.sqlite`
- **Migration System**: Custom SQL migrations in `migrations/`
- **Seeding**: Initial data via `seed/` directory
- **Connection Pooling**: sqlx with Tokio runtime

### File System

- **Models Directory**: `{AppLocalData}/models/`
- **Image Storage**: Railway model images
  - Filename: `{model_id_with_underscores}.{png|jpg|jpeg}`
  - Example: `trn_railway-model_abc123.png`
- **Backup Location**: Google Drive (via oauth2 + google-drive3 crate)

### Base Directories (Tauri)

| OS | AppData | AppLocalData |
|----|---------|--------------|
| Windows | `%APPDATA%` | `%APPDATA%` |
| macOS | `~/Library/Application Support` | `~/Library/Application Support` |
| Linux | `~/.config` | `~/.cache` |

---

## Development Environment

### Package Management

- **Frontend**: pnpm (monorepo-friendly)
- **Backend**: Cargo (Rust package manager)

### Build & Run

```bash
# Frontend
pnpm dev              # Start dev server with HMR
pnpm build            # Production build
pnpm check            # Type check
pnpm lint             # Code style

# Backend
pnpm rust:fmt         # Format Rust code
pnpm rust:build       # Compile backend
pnpm rust:test        # Run Rust tests
pnpm rust:clippy      # Lint (warnings = errors)

# Full app
pnpm tauri dev        # Run desktop app in dev mode
pnpm tauri build      # Build production installer
```

### Code Quality

- **Frontend Linting**: ESLint + Prettier
- **Frontend Type Check**: svelte-check + TypeScript
- **Rust Formatting**: rustfmt (enforce via CI)
- **Rust Linting**: Clippy (warnings treated as errors)
- **Test Coverage**: Vitest (frontend) + cargo test (backend)

### TypeScript Bindings

- **Generation**: Automatic from Rust `#[specta::specta]` macros
- **Output File**: `src/lib/bindings.ts`
- **Regeneration**: Happens on dev server start or cargo build
- **Types**: Full type safety for IPC calls

---

## Internationalization (i18n)

### Paraglide-JS Integration

- **Config**: `project.inlang/settings.json`
- **Message Files**: 
  - `messages/en.json` - English
  - `messages/it.json` - Italian
- **Compilation**: `pnpm prepare` compiles to `src/lib/paraglide/`
- **Usage**: `import { t } from '$lib/paraglide/runtime'`
- **Requirement**: NO hardcoded user-facing strings

### Language Support

- English (en)
- Italian (it)
- Extensible for future languages

---

## Security Considerations

### Filesystem Access

- Path validation to prevent traversal attacks
- Reject paths with `..` or suspicious components
- Verify files exist before returning paths

### Database Access

- Prepared statements via sqlx (SQL injection prevention)
- Type-safe queries with compile-time validation
- Connection pooling with Tokio

### OAuth2 / Google Drive

- Secure credential storage via tauri-plugin-stronghold
- OAuth2 token management
- Connectivity monitoring

### Type Safety

- Strong typing prevents entire classes of bugs
- TypeScript on frontend, Rust on backend
- Compile-time validation where possible

---

## Notable Patterns

### Error Handling

```rust
// Backend: Custom error types with context
#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {field} - {message}")]
    ValidationError { field: String, message: String },
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

// Frontend: Zod validation
const schema = z.object({
    name: z.string().min(1),
    price: z.string().regex(/^\d+\.\d{2}$/),
});
```

### State Management

```rust
// Backend: AppState managed by Tauri
pub struct AppState {
    db_pool: SqlitePool,
    models_dir: PathBuf,
    initialized: AtomicBool,
}

// Frontend: Svelte $state rune
let count = $state(0);
let doubled = $derived(count * 2);
```

### Async/Await

```rust
// Backend: Tokio runtime
#[tauri::command]
async fn long_operation(state: tauri::State<'_, AppState>) -> Result<String, CommandError> {
    // Non-blocking I/O operations
    let data = query_database(&state).await?;
    Ok(data)
}

// Frontend: Top-level await (SvelteKit)
import { invoke } from '@tauri-apps/api/core';
let data = await invoke('command_name', { arg: value });
```

---

## Performance Optimizations

### Frontend

- Tree-shaking via ES modules
- Code splitting at route boundaries
- Vite's fast rebuild on HMR
- Image optimization (WebP if supported)
- CSS utility classes (no unused CSS)

### Backend

- Connection pooling for database
- Async I/O with Tokio (no blocking calls)
- Lazy evaluation with iterators
- Compiled query validation (sqlx macros)

---

## Testing Strategy

### Frontend (Vitest)

```bash
pnpm test             # Run all tests
pnpm test --coverage  # Coverage report
```

- Unit tests: Component behavior
- Integration tests: User flows (Playwright)

### Backend (cargo test)

```bash
pnpm rust:test        # Run all Rust tests
```

- Unit tests: Each module
- Test utilities: `src-tauri/src/test_utils.rs`
- Fixtures: `src-tauri/fixtures/`

---

## Deployment

### Desktop Installers

- **Windows**: .msi (MSI installer)
- **macOS**: .dmg (Disk image)
- **Linux**: .AppImage (universal executable)

### Versioning

- Semantic versioning (MAJOR.MINOR.PATCH)
- Tauri auto-updates supported
- Version: `src-tauri/Cargo.toml` + `src-tauri/tauri.conf.json`

---

## Key Stats

| Metric | Value |
|--------|-------|
| **Frontend Files** | ~40+ Svelte components |
| **Backend Modules** | 13 feature modules |
| **Database Tables** | 20+ tables (catalog, collection, etc.) |
| **Tauri Commands** | 50+ exposed commands |
| **Languages** | Rust, TypeScript, SQL |
| **Rust Dependencies** | 40+ crates |
| **Frontend Dependencies** | 20+ packages |
| **Minimum Rust Version** | 1.93.0 |
| **Target Platforms** | Windows, macOS, Linux |

---

## Important Links

- **Tauri Docs**: https://tauri.app/
- **Svelte 5 Docs**: https://svelte.dev/docs/svelte/what-is-svelte
- **Tailwind CSS**: https://tailwindcss.com/
- **sqlx Docs**: https://sqlx.rs/
- **Paraglide-JS**: https://inlang.com/

---

## Notes for Contributors

1. **Rust Standards**: Follow RFC 430 naming conventions
2. **Type Safety**: Always prefer strong typing over flexibility
3. **Error Handling**: Return `Result<T, E>` instead of panicking
4. **Testing**: Write tests alongside implementation
5. **Documentation**: Add rustdoc for public APIs, JSDoc for TypeScript
6. **Localization**: Use Paraglide-JS for all user-facing text
7. **Code Review**: Clippy must pass with zero warnings
8. **Commits**: Use Conventional Commits (feat:, fix:, docs:, etc.)

