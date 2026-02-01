# Quickstart: Data Import Utility Development

**Feature**: 010-data-import-utility  
**Created**: January 30, 2026

---

## Prerequisites

- Rust 1.93.0+ (`rustup update`)
- Node.js 20+ with pnpm 10.27+
- SQLite 3.x (bundled with sqlx)

---

## 1. Add Dependencies

### Rust (src-tauri/Cargo.toml)

Add to `[dependencies]`:

```toml
# Archive handling
zip = "2.6"
flate2 = "1.1"
tar = "0.4"

# JSON Schema validation
jsonschema = { version = "0.29", default-features = false }
```

Run:

```bash
cd src-tauri && cargo check
```

---

## 2. Create Feature Module Structure

```bash
# Backend
mkdir -p src-tauri/src/import/{domain,application,infrastructure,interface}

# Frontend
mkdir -p src/lib/features/import/components
mkdir -p src/routes/my-settings/import
```

---

## 3. Create Module Files

### Backend Entry Point

**src-tauri/src/import/mod.rs**:

```rust
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interface;
```

**src-tauri/src/import/domain/mod.rs**:

```rust
mod import_session;
mod manifest;
mod validation_error;

pub use import_session::*;
pub use manifest::*;
pub use validation_error::*;
```

### Register Module in lib.rs

Add to `src-tauri/src/lib.rs`:

```rust
pub mod import;
```

---

## 4. Copy JSON Schema

Copy the manifest schema to the source tree:

```bash
cp specs/010-data-import-utility/contracts/manifest.schema.json \
   src-tauri/src/import/domain/
```

This will be embedded at compile time using `include_str!()`.

---

## 5. Create Test Fixtures

Create a test import package for development:

```bash
mkdir -p src-tauri/fixtures/import-test/{images}

# Create minimal manifest
cat > src-tauri/fixtures/import-test/manifest.json << 'EOF'
{
  "version": "1.0",
  "exportedAt": "2026-01-30T10:00:00Z",
  "source": "Test Export",
  "data": {
    "manufacturers": [
      {"id": "mfr-1", "name": "Märklin", "countryCode": "DE"}
    ],
    "railwayCompanies": [
      {"id": "rc-1", "name": "Deutsche Bahn", "abbreviation": "DB"}
    ],
    "railwayModels": [
      {
        "id": "rm-1",
        "manufacturerId": "mfr-1",
        "productCode": "39010",
        "description": "BR 01 Steam Locomotive",
        "scale": "H0",
        "epoch": "III",
        "category": {"type": "locomotive", "subType": "steam"},
        "powerMethod": "ac"
      }
    ],
    "collectionItems": [],
    "sellers": [],
    "maintenanceCards": []
  }
}
EOF

# Create test archive
cd src-tauri/fixtures/import-test
zip -r ../test_import.zip manifest.json images/
cd ../../..
```

---

## 6. Add Paraglide Messages

Add i18n keys to `messages/en.json`:

```json
{
  "import.title": "Import Data",
  "import.dropzone.label": "Drop import file here or click to browse",
  "import.dropzone.formats": "Supported formats: .zip, .tar.gz",
  "import.preview.title": "Import Preview",
  "import.preview.totalRecords": "{count} total records found",
  "import.preview.newRecords": "{count} new records to import",
  "import.preview.duplicates": "{count} duplicates will be skipped",
  "import.preview.errors": "{count} validation errors",
  "import.preview.warnings": "{count} warnings",
  "import.confirm.button": "Confirm Import",
  "import.cancel.button": "Cancel",
  "import.progress.extracting": "Extracting archive...",
  "import.progress.validating": "Validating data...",
  "import.progress.importing": "Importing records...",
  "import.result.success": "{added} records added, {skipped} skipped",
  "import.result.failed": "Import failed: {reason}",
  "import.error.fileNotFound": "File not found",
  "import.error.unsupportedFormat": "Unsupported file format",
  "import.error.archiveCorrupted": "Archive is corrupted",
  "import.error.manifestMissing": "manifest.json not found in archive",
  "import.warning.missingImage": "Image not found: {filename}"
}
```

---

## 7. Development Workflow

### Run Backend Tests

```bash
# From repo root
pnpm rust:test

# Or with specific test
cd src-tauri && cargo test import::
```

### Run Frontend Dev Server

```bash
pnpm dev
```

### Verify Types After Adding Commands

After adding new Tauri commands:

```bash
# Regenerate TypeScript bindings
pnpm tauri dev  # This triggers specta generation
```

Check `src/lib/bindings.ts` for new command types.

---

## 8. Key Files to Implement

### Phase 1: Core Domain

1. `src-tauri/src/import/domain/manifest.rs` - Manifest DTOs
2. `src-tauri/src/import/domain/import_session.rs` - Session aggregate
3. `src-tauri/src/import/domain/validation_error.rs` - Error types

### Phase 2: Infrastructure

4. `src-tauri/src/import/infrastructure/archive_extractor.rs` - ZIP/GZ handling
5. `src-tauri/src/import/infrastructure/schema_validator.rs` - JSON Schema
6. `src-tauri/src/import/infrastructure/duplicate_checker.rs` - DB lookups
7. `src-tauri/src/import/infrastructure/media_storage.rs` - Image handling

### Phase 3: Application Layer

8. `src-tauri/src/import/application/validate_package.rs` - Use case
9. `src-tauri/src/import/application/preview_import.rs` - Use case
10. `src-tauri/src/import/application/execute_import.rs` - Use case

### Phase 4: Interface Layer

11. `src-tauri/src/import/interface/commands.rs` - Tauri commands

### Phase 5: Frontend

12. `src/lib/features/import/import.controller.svelte.ts` - Controller
13. `src/lib/features/import/components/*.svelte` - UI components
14. `src/routes/my-settings/import/+page.svelte` - Route

---

## 9. Verification Checklist

Before marking implementation complete:

- [ ] `pnpm rust:format` passes
- [ ] `pnpm rust:clippy` passes (no warnings)
- [ ] `pnpm rust:test` passes
- [ ] `pnpm lint` passes
- [ ] `pnpm check` passes
- [ ] `pnpm test` passes
- [ ] Manual test with fixture archive
- [ ] Manual test with duplicate detection
- [ ] Manual test with invalid manifest (should abort)

---

## 10. Related Documentation

- [Specification](./spec.md)
- [Implementation Plan](./plan.md)
- [Research](./research.md)
- [Data Model](./data-model.md)
- [Manifest JSON Schema](./contracts/manifest.schema.json)
- [Tauri API Contract](./contracts/tauri-api.md)
