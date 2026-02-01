# Research: Data Import Utility

**Feature**: 010-data-import-utility  
**Created**: January 30, 2026  
**Status**: Complete

---

## Research Tasks

This document resolves all technical unknowns identified during the Technical Context phase.

---

## 1. Archive Handling in Rust

### Decision: Use `zip` crate for ZIP files, `flate2` + `tar` for `.tar.gz` archives

### Rationale

- **`zip` crate** (v2.x): Mature, well-maintained, supports reading ZIP archives without extracting to disk (streaming). Handles ZIP64 for large files.
- **`flate2`**: Standard gzip decompression crate, pairs with `tar` for `.tar.gz` format.
- The spec mentions `.zip` and `.gz` - we interpret `.gz` as `.tar.gz` since a standalone gzip'd file isn't suitable for manifest + images structure.

### Implementation Pattern

```rust
// Pseudo-code for archive extraction
pub enum ArchiveFormat {
    Zip,
    TarGz,
}

pub fn detect_format(path: &Path) -> Result<ArchiveFormat, ArchiveError> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("zip") => Ok(ArchiveFormat::Zip),
        Some("gz") | Some("tgz") => Ok(ArchiveFormat::TarGz),
        _ => Err(ArchiveError::UnsupportedFormat),
    }
}
```

### Alternatives Considered

| Alternative            | Rejected Because                                                   |
| ---------------------- | ------------------------------------------------------------------ |
| `unzip` CLI subprocess | Security risk (shell injection), not cross-platform reliable       |
| `async-compression`    | Added complexity for no real benefit (file I/O is blocking anyway) |
| `archive-rs`           | Less mature, smaller community                                     |

### Dependencies to Add

```toml
# src-tauri/Cargo.toml
zip = "2.6"
flate2 = "1.1"
tar = "0.4"
```

---

## 2. JSON Schema Validation in Rust

### Decision: Use `jsonschema` crate with embedded schema

### Rationale

- **`jsonschema` crate** (v0.29+): Full JSON Schema Draft 2020-12 support, fast validation, good error messages.
- Schema is embedded in the binary at compile time using `include_str!()` macro.
- Errors are structured and can be mapped to user-friendly messages.

### Implementation Pattern

```rust
use jsonschema::{JSONSchema, ValidationError};
use serde_json::Value;

const MANIFEST_SCHEMA: &str = include_str!("manifest_schema.json");

pub fn validate_manifest(manifest: &Value) -> Result<(), Vec<ValidationError>> {
    let schema: Value = serde_json::from_str(MANIFEST_SCHEMA)?;
    let compiled = JSONSchema::compile(&schema)?;

    let result = compiled.validate(manifest);
    if let Err(errors) = result {
        return Err(errors.collect());
    }
    Ok(())
}
```

### Alternatives Considered

| Alternative                       | Rejected Because                                          |
| --------------------------------- | --------------------------------------------------------- |
| `valico`                          | Outdated, no JSON Schema 2020-12 support                  |
| `schemars` for runtime validation | schemars generates schemas, doesn't validate against them |
| Manual validation                 | Error-prone, unmaintainable                               |

### Dependencies to Add

```toml
jsonschema = { version = "0.29", default-features = false }
```

---

## 3. File System Patterns for Media Storage

### Decision: Use Tauri's app data directory with UUID-prefixed filenames

### Rationale

- **Location**: `tauri::api::path::app_data_dir()` / `media/` subfolder
- **Collision Avoidance**: Prepend UUID to original filename: `{uuid}_{original_name}.ext`
- **Atomic Write**: Write to temp file, then rename (avoids partial files on crash)

### Implementation Pattern

```rust
use tauri::api::path::app_data_dir;
use uuid::Uuid;

pub fn store_media(
    app_handle: &tauri::AppHandle,
    original_name: &str,
    data: &[u8],
) -> Result<String, MediaError> {
    let media_dir = app_data_dir(app_handle)?.join("media");
    std::fs::create_dir_all(&media_dir)?;

    let uuid = Uuid::new_v4();
    let safe_name = sanitize_filename(original_name);
    let stored_name = format!("{}_{}", uuid, safe_name);
    let target_path = media_dir.join(&stored_name);

    // Atomic write pattern
    let temp_path = target_path.with_extension("tmp");
    std::fs::write(&temp_path, data)?;
    std::fs::rename(&temp_path, &target_path)?;

    Ok(stored_name)
}
```

### Alternatives Considered

| Alternative                           | Rejected Because                                          |
| ------------------------------------- | --------------------------------------------------------- |
| Hash-based naming (content-addressed) | Harder to trace back to original; no deduplication needed |
| Overwrite existing files              | Violates BR-IM02 (image isolation)                        |
| Database BLOB storage                 | Images can be large; file system is more appropriate      |

---

## 4. Duplicate Detection Strategy

### Decision: Query database during preview phase, cache results in memory

### Rationale

- **Railway Models**: Query by `(manufacturer_id, product_code)` unique constraint
- **Collection Items**: Query by `(railway_model_id, purchase_date)`
- **Performance**: Batch queries using `IN` clauses to minimize round trips
- **Memory**: Store HashSet of existing keys for O(1) lookup during import

### Implementation Pattern

```rust
pub struct DuplicateChecker {
    existing_models: HashSet<(ManufacturerId, ProductCode)>,
    existing_items: HashSet<(RailwayModelId, NaiveDate)>,
}

impl DuplicateChecker {
    pub async fn load(pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        // Batch load existing keys
        let models = sqlx::query_as!(...)
            .fetch_all(pool)
            .await?;
        // ... build HashSets
    }

    pub fn is_duplicate_model(&self, mfr: &ManufacturerId, code: &ProductCode) -> bool {
        self.existing_models.contains(&(mfr.clone(), code.clone()))
    }
}
```

### Alternatives Considered

| Alternative                       | Rejected Because                                                           |
| --------------------------------- | -------------------------------------------------------------------------- |
| Check each record individually    | O(n) database queries, too slow for large imports                          |
| Database UNIQUE constraint errors | Would require attempting writes, violates preview-before-write requirement |
| Bloom filter                      | Overkill for expected scale; false positives unacceptable                  |

---

## 5. Scale String Normalization

### Decision: Normalize during manifest parsing, before validation

### Rationale

- The existing `Scale` enum already defines canonical values (H0, N, TT, etc.)
- Create a normalization map for common variations
- Apply normalization as part of manifest deserialization

### Implementation Pattern

```rust
pub fn normalize_scale(input: &str) -> Scale {
    match input.to_uppercase().as_str() {
        "HO" | "H0" | "H-0" => Scale::H0,
        "N" => Scale::N,
        "TT" => Scale::TT,
        "Z" => Scale::Z,
        "G" | "GARDEN" => Scale::G,
        "0" | "O" | "SCALE0" => Scale::Scale0,
        "00" | "OO" => Scale::Scale00,
        "1" | "I" | "SCALE1" => Scale::Scale1,
        other => {
            // Log warning, default or error
            tracing::warn!(input = other, "Unknown scale, defaulting to H0");
            Scale::H0
        }
    }
}
```

### User Preference Integration

The spec mentions normalizing to "user's preferred format". This can be achieved by:

1. During import: normalize all scales to canonical enum values
2. During display: format according to user's locale/preference (handled by frontend)

---

## 6. Atomic Transaction Pattern

### Decision: Use sqlx transactions with explicit rollback on any error

### Rationale

- **BR-IM01**: If any record fails validation, entire import aborts
- All database writes wrapped in a single transaction
- On error, transaction rollback is automatic when `tx` is dropped

### Implementation Pattern

```rust
pub async fn execute_import(
    pool: &SqlitePool,
    session: &ImportSession,
) -> Result<ImportResult, ImportError> {
    let mut tx = pool.begin().await?;

    // Write all records within transaction
    for model in &session.models_to_import {
        insert_railway_model(&mut tx, model).await?;
    }

    for item in &session.items_to_import {
        insert_collection_item(&mut tx, item).await?;
    }

    // Only commit if everything succeeded
    tx.commit().await?;

    Ok(ImportResult { ... })
}
```

---

## 7. Frontend File Handling

### Decision: Use Tauri's `tauri-plugin-fs` for file access, HTML5 drag-and-drop for UX

### Rationale

- `tauri-plugin-fs` is already in dependencies
- HTML5 File API provides drag-and-drop with `File` objects
- Send file path to backend via IPC; backend handles extraction

### Implementation Pattern

```svelte
<script lang="ts">
  function handleDrop(event: DragEvent) {
    event.preventDefault();
    const files = event.dataTransfer?.files;
    if (files?.[0]) {
      const path = (files[0] as any).path; // Tauri extends File with path
      controller.analyzePackage(path);
    }
  }
</script>

<div ondragover={(e) => e.preventDefault()} ondrop={handleDrop} class="drop-zone">
  Drop import file here
</div>
```

---

## 8. Progress Reporting for Large Imports

### Decision: Use Tauri events for real-time progress updates

### Rationale

- For imports with 1000+ records, UI should not freeze
- Backend emits progress events; frontend subscribes and updates UI
- Progress reported as percentage with current phase label

### Implementation Pattern

```rust
// Backend
app_handle.emit("import-progress", ImportProgress {
    phase: "Validating records",
    current: 500,
    total: 1000,
    percentage: 50,
})?;

// Frontend
import { listen } from '@tauri-apps/api/event';

onMount(() => {
  const unlisten = listen<ImportProgress>('import-progress', (event) => {
    progress = event.payload;
  });
  return () => unlisten.then(f => f());
});
```

---

## Summary of Dependencies to Add

```toml
# src-tauri/Cargo.toml [dependencies]
zip = "2.6"
flate2 = "1.1"
tar = "0.4"
jsonschema = { version = "0.29", default-features = false }
```

No new frontend dependencies required - existing Tauri APIs and SvelteKit patterns suffice.

---

## Resolved Unknowns Summary

| Unknown                | Resolution                                         |
| ---------------------- | -------------------------------------------------- |
| Archive extraction     | `zip` + `flate2`/`tar` crates                      |
| JSON Schema validation | `jsonschema` crate with embedded schema            |
| Media storage location | `app_data_dir()/media/` with UUID prefix           |
| Duplicate detection    | Batch-load keys into HashSet for O(1) lookup       |
| Scale normalization    | Normalize during parsing to canonical `Scale` enum |
| Atomic transactions    | sqlx transaction wrapper with auto-rollback        |
| File drag-and-drop     | HTML5 File API + Tauri path extension              |
| Progress reporting     | Tauri event system for real-time updates           |
