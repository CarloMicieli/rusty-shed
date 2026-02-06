# Technology Stack: Railway Model Details Page (Feature 014)

**Last Updated**: February 6, 2026

## Frontend Stack

### Framework & Runtime
- **Framework**: Svelte 5 (with Runes: `$state`, `$derived`, `$props`)
- **Build Tool**: Vite
- **Language**: TypeScript (strict mode)
- **CSS Framework**: Tailwind CSS 4
- **UI Components**: shadcn-svelte 1.1.1 (headless component library)

### Key Dependencies
- `@floating-ui/dom` - Floating UI for popovers/menus
- `uuid` - UUID generation for client-side unique IDs
- `zod` - Runtime type validation
- `@lucide/svelte` - Icon library (563 icons)
- `d3-scale`, `d3-array` - Data visualization utilities
- `layerchart` - Chart rendering library

### Localization
- **Tool**: Paraglide-JS 2.7.1
- **Requirement**: All user-facing strings MUST use Paraglide-JS i18n (no hardcoded text)
- **Available Languages**: English (en.json), Italian (it.json)

### Testing
- **Unit Tests**: Vitest with happy-dom environment
- **E2E Tests**: Playwright (configured in vitest.config.ts)

### Routing
- **Router**: SvelteKit (file-based routing)
- **Route Pattern**: `/src/routes/+page.svelte`
- **Feature Route**: `/models/[modelId]/+page.svelte` (dynamic segment)

---

## Backend Stack

### Runtime & Build
- **Language**: Rust 1.93.0+
- **Edition**: 2024
- **Package Manager**: Cargo
- **Build System**: Tauri 2.9.5 (Desktop app framework)

### Database
- **DBMS**: SQLite (embedded)
- **ORM**: sqlx 0.8.6 with macros
- **Migrations**: Custom migration system via `Database::run_migrations()`
- **Pool**: Tokio async runtime with SQLite connection pool

### Async Runtime
- **Executor**: Tokio 1.49.0 (multi-threaded runtime)
- **Pattern**: async/await throughout

### Key Crates
- **Serialization**: serde 1.0.228 (with derive macros)
- **Error Handling**: thiserror 2.0.18
- **API Typing**: specta 2.0.0-rc.22 with specta-typescript 0.0.9 (automatic TypeScript bindings)
- **Validation**: garde 0.22.1 (derive-based validation)
- **Logging**: flexi_logger 0.31.8 with log 0.4.29
- **JSON**: serde_json 1.0.149
- **UUID**: uuid 1.20.0 (v4 generation, serde support)
- **Date/Time**: chrono 0.4.43 (serde support)
- **Decimal**: rust_decimal 1.40.0 (for monetary values)

### Architecture Pattern
- **Design**: Domain-Driven Design (DDD)
- **Module Structure**:
  - `application/` - Use cases and application services
  - `domain/` - Domain entities, value objects, business logic
  - `infrastructure/` - Database, file I/O, external services
  - `interface/` - Command handlers, API contracts, command arguments
- **Error Handling**: Custom `CommandError` type with field-level validation
- **Commands**: Tauri commands with `#[tauri::command]` and `#[specta::specta]` macros

### File System
- **Models Directory**: `{AppLocalData}/models/`
- **Image Storage**: Railway model images stored with filename = `{railway_model_id}` (with ":" replaced by "_")
- **Image Extensions**: .png or .jpg/.jpeg
- **API**: Tauri plugin `tauri-plugin-fs` 2.4.5 for filesystem operations
- **Base Directories**: Tauri provides platform-specific paths (Windows: AppData, macOS: Library, Linux: .config)

### Plugins
- `tauri-plugin-fs` - Filesystem operations
- `tauri-plugin-http` - HTTP client
- `tauri-plugin-log` - Logging with rotation
- `tauri-plugin-opener` - Open URLs/files in default apps
- `tauri-plugin-oauth` - OAuth 2.0 flows (Google Drive backup)
- `tauri-plugin-stronghold` - Secure credential storage
- `tauri-plugin-deep-link` - Deep link handling

---

## IPC Communication

### Command Pattern
- **Rust → TypeScript**: Tauri commands with automatic type generation
- **Type Generation**: specta-typescript creates `src/lib/bindings.ts`
- **Invocation**: Imported from generated bindings in frontend
- **Return Types**: Result-like pattern with error handling

### Example Command Signature
```rust
#[tauri::command]
#[specta::specta]
async fn command_name(
    state: tauri::State<'_, AppState>,
    param: String,
) -> Result<ResponseType, CommandError> {
    // Implementation
}
```

---

## Data Model (Existing)

### Key Entities (No Schema Changes for Feature 014)
- **RailwayModel (Box)**: Core product with manufacturer, product code, description, scale, era, power method, image
- **RollingStock (OwnedRollingStock)**: Individual units with type, road number, depot, series code, etc.
- **Manufacturer**: Master data for model producers
- **RailwayCompany**: Master data for railway operators

### Image Management
- **Table**: Not explicitly modeled; stored as files in filesystem
- **Index Field**: Railway model ID (used as filename)
- **Filename Pattern**: `{model_id_with_underscores}.{png|jpg|jpeg}`
- **Storage Location**: `{app_local_data}/models/`

---

## Development Workflow

### Frontend Commands
```bash
pnpm dev              # Start dev server (hot reload)
pnpm build            # Production build
pnpm check            # Type checking + svelte-check
pnpm lint             # ESLint + Prettier
pnpm format           # Auto-format code
pnpm test             # Run Vitest suite
pnpm prepare          # Compile Paraglide messages
```

### Rust Commands
```bash
pnpm rust:fmt         # Format with rustfmt
pnpm rust:build       # Compile
pnpm rust:test        # Run tests + cargo test
pnpm rust:clippy      # Lint (warnings as errors)
```

### TypeScript Bindings
- Generated automatically during development from Rust `#[specta::specta]` macros
- Output: `src/lib/bindings.ts`
- Regenerate: Run dev server or `cargo build`

---

## Quality & Compliance

### Code Standards
- **Rust**: RFC 430 naming conventions, idiomatic patterns
- **TypeScript**: ESLint config, Prettier formatting
- **Testing**: Unit tests with Vitest, integration tests with Playwright
- **Accessibility**: WCAG 2.1 AA compliance required

### Verification Steps
- `pnpm check` - Type safety
- `pnpm lint` - Code style
- `pnpm test` - Unit tests
- `pnpm rust:clippy` - Rust lints (zero warnings)
- `pnpm rust:test` - Rust tests

### Localization
- All UI strings must use Paraglide-JS
- No hardcoded strings in components
- Support for English and Italian by default

---

## Notes for Feature 014

### Image Handling in Media Module
1. **Retrieve Image**: Check filesystem for image file with extensions .png, .jpg, or .jpeg
2. **Fallback**: If image missing, generate placeholder using HTML/CSS (text "No picture yet" with styling)
3. **Filename Resolution**: Transform railway model ID by replacing ":" with "_"
4. **API**: Create Tauri command to retrieve image path or placeholder

### Module Organization
- **Media Module**: New feature module to handle all image/media operations
- **Structure**: Follow existing DDD pattern (application, domain, infrastructure, interface)
- **Move**: Relocate `get_image_path` from `lib.rs` to `media` module command handlers

---

## Related Documentation
- [Rust Standards](../.github/instructions/rust.instructions.md)
- [Svelte Standards](../.github/instructions/svelte.instructions.md)
- [Tauri Commands Blueprint](./docs/blueprints/use_case_command_blueprint.md)
