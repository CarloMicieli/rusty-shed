# Research: Model Image Upload System

**Feature**: 015-model-image-upload  
**Date**: February 8, 2026  
**Purpose**: Resolve technical unknowns and establish best practices for implementing image upload functionality in Tauri 2

---

## Research Tasks

### R1: Tauri 2 File Dialog API

### R2: Tauri 2 Filesystem Plugin & Permissions

### R3: Image Format Validation Strategies

### R4: File Naming Conventions & Sanitization

### R5: Drag & Drop Implementation in Svelte 5

### R6: Asset Protocol for Local Image Display

### R7: Error Handling Patterns for File Operations

---

## R1: Tauri 2 File Dialog API

### Research Question

How do we implement file selection using Tauri 2's dialog API? What are the configuration options for file type filtering?

### Findings

**Plugin**: `@tauri-apps/plugin-dialog`

**Frontend Usage**:

```typescript
import { open } from '@tauri-apps/plugin-dialog';

const selected = await open({
  multiple: false,
  filters: [
    {
      name: 'Images',
      extensions: ['jpg', 'jpeg', 'png', 'webp']
    }
  ]
});

if (selected) {
  // selected is a string path
  await commands.uploadModelImage({ modelId, filePath: selected });
}
```

**Key Features**:

- `multiple: false` - Single file selection only
- `filters` - Limit visible files to image formats
- Returns `string | null` (path or cancel)
- Cross-platform (uses native OS dialogs)

**Decision**: Use `@tauri-apps/plugin-dialog` with filter `['jpg', 'jpeg', 'png', 'webp']`

**Rationale**: Official Tauri 2 plugin, type-safe, cross-platform, allows format pre-filtering at OS level

**Alternatives Considered**:

- HTML `<input type="file">` - Rejected: Doesn't integrate well with Tauri security model, requires base64 encoding
- Custom file browser - Rejected: Over-engineering for simple file selection

---

## R2: Tauri 2 Filesystem Plugin & Permissions

### Research Question

How do we handle file copying and permissions in Tauri 2? What capabilities must be configured?

### Findings

**Approach**: Use standard Rust `tokio::fs` instead of Tauri fs plugin

**Rationale**:

- Tauri fs plugin is designed for frontend access to filesystem
- Backend Rust code has full filesystem access without restrictions
- No need for complex scope configurations
- Simpler error handling with standard library

**File Copy Operation** (Backend Rust):

```rust
use tokio::fs;

pub async fn copy_image_to_storage(
    source_path: &Path,
    dest_path: &Path,
) -> Result<(), std::io::Error> {
    fs::copy(source_path, dest_path).await?;
    Ok(())
}
```

**Capabilities Configuration**: No special fs capabilities needed for backend operations

**Storage Location**:

```rust
use tauri::Manager;

fn get_image_storage_dir(app_handle: &tauri::AppHandle) -> Result<PathBuf, Error> {
    let app_data = app_handle.path().app_data_dir()?;
    let models_dir = app_data.join("models");

    // Ensure directory exists
    std::fs::create_dir_all(&models_dir)?;

    Ok(models_dir)
}
```

**Decision**: Use `tokio::fs` for file operations, `app_data_dir()` for storage location

**Rationale**: Backend has full filesystem access, no capability configuration needed, simpler code

**Alternatives Considered**:

- `@tauri-apps/plugin-fs` from frontend - Rejected: More complex, requires scope configuration, unnecessary for backend operations

---

## R3: Image Format Validation Strategies

### Research Question

How do we validate that uploaded files are actual images in supported formats? Should we trust file extensions?

### Findings

**Never Trust File Extensions**: File extensions can be easily spoofed (renamed `.txt` to `.jpg`)

**Solution**: Use `image` crate for MIME type detection

**Implementation**:

```rust
use image::ImageFormat;
use std::path::Path;

pub fn validate_image_format(path: &Path) -> Result<ImageFormat, ValidationError> {
    // Reads file header (magic bytes) to determine format
    let format = image::ImageReader::open(path)?
        .with_guessed_format()?
        .format();

    match format {
        Some(ImageFormat::Jpeg) | Some(ImageFormat::Png) | Some(ImageFormat::WebP) => {
            Ok(format.unwrap())
        }
        _ => Err(ValidationError::UnsupportedFormat)
    }
}
```

**File Size Validation**:

```rust
pub fn validate_file_size(path: &Path, max_bytes: u64) -> Result<(), ValidationError> {
    let metadata = std::fs::metadata(path)?;
    if metadata.len() > max_bytes {
        return Err(ValidationError::FileTooLarge {
            size: metadata.len(),
            max: max_bytes
        });
    }
    Ok(())
}
```

**Decision**: Use `image` crate for format validation via magic bytes, validate size via `fs::metadata`

**Rationale**: Prevents malicious file uploads, reliable format detection, industry standard approach

**Alternatives Considered**:

- Extension-only validation - Rejected: Easily spoofed, security risk
- `mime` crate alone - Rejected: Doesn't validate actual image data
- `infer` crate - Alternative: Good option, but `image` crate already used in project

---

## R4: File Naming Conventions & Sanitization

### Research Question

How do we generate unique, filesystem-safe filenames from model IDs? How do we handle special characters?

### Findings

**Model ID Format**: Railway model IDs contain colons (e.g., `marklin:39216`)

**Problem**: Colons are reserved characters in Windows filesystems

**Solution**: Replace colons with underscores, preserve original extension

**Implementation**:

```rust
use std::path::{Path, PathBuf};

pub fn sanitize_filename_from_model_id(model_id: &str, extension: &str) -> String {
    // Replace colons with underscores for filesystem compatibility
    let sanitized_id = model_id.replace(':', "_");
    format!("{}.{}", sanitized_id, extension)
}

pub fn resolve_image_path(
    storage_dir: &Path,
    model_id: &str,
    extension: &str,
) -> PathBuf {
    let filename = sanitize_filename_from_model_id(model_id, extension);
    storage_dir.join(filename)
}
```

**Filename Examples**:

- Model ID: `marklin:39216` → Filename: `marklin_39216.jpg`
- Model ID: `roco:73241` → Filename: `roco_73241.png`
- Model ID: `fleischmann:4380` → Filename: `fleischmann_4380.webp`

**Uniqueness**: Model IDs are unique database primary keys, ensuring no filename collisions

**Decision**: Replace `:` with `_`, use model ID + extension for deterministic naming

**Rationale**: Filesystem-safe, deterministic (no UUIDs needed), maintains relationship to model

**Alternatives Considered**:

- UUID-based filenames - Rejected: Requires database column to store mapping, adds complexity
- Hash-based names - Rejected: Loses human readability, debugging harder
- URL encoding - Rejected: Ugly filenames, harder to manage manually

---

## R5: Drag & Drop Implementation in Svelte 5

### Research Question

How do we implement drag & drop file upload in Svelte 5 with proper visual feedback?

### Findings

**Svelte 5 Approach**: Use `$state` rune for reactive drag state

**Implementation**:

```svelte
<script lang="ts">
  import { commands } from '$lib/bindings';
  import type { RailwayModelId } from '$lib/bindings';

  let { modelId }: { modelId: RailwayModelId } = $props();

  let isDragging = $state(false);
  let isUploading = $state(false);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) return;

    const file = files[0]; // Take only first file

    // Get file path - NOTE: Not directly available in browser
    // Must use dialog approach instead
    await uploadFile(file);
  }
</script>

<div
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  class:dragging={isDragging}
>
  Drop image here
</div>
```

**Important Limitation**: Browser drag & drop doesn't provide file paths for security reasons!

**Solution**: Use HTML5 File API + Tauri command that accepts file bytes

**Alternative Approach**:

```rust
// Backend command that accepts file bytes instead of path
#[tauri::command]
pub async fn upload_model_image_bytes(
    model_id: String,
    file_name: String,
    file_data: Vec<u8>,
) -> Result<(), CommandError> {
    // Validate bytes, determine format, write to storage
}
```

**Decision**:

- File Explorer: Use `@tauri-apps/plugin-dialog` (returns path, backend reads file)
- Drag & Drop: Accept file bytes from frontend, backend writes directly

**Rationale**: Works within browser/Tauri security constraints, provides best UX

**Alternatives Considered**:

- Path-only approach - Rejected: Drag & drop doesn't expose file paths in browser
- Base64 encoding - Rejected: Inefficient for large files (33% overhead)

---

## R6: Asset Protocol for Local Image Display

### Research Question

How do we display locally stored images in the Tauri app? Can we use direct file paths?

### Findings

**Problem**: Cannot use `file://` protocol or direct paths like `C:/path/to/image.jpg` in `<img>` tags due to security restrictions

**Solution**: Use Tauri Asset Protocol

**Asset Protocol Format**:

- Local path: `/home/user/.local/share/rusty-shed/models/marklin_39216.jpg`
- Asset URL: `asset://localhost/models/marklin_39216.jpg`

**Note**: Feature 014 already implemented this pattern!

**Backend Command** (from Feature 014):

```rust
#[tauri::command]
pub async fn get_railway_model_image(
    model_id: String,
    app_handle: tauri::AppHandle,
) -> Result<RailwayModelImageResponse, CommandError> {
    // Returns path or placeholder flag
}
```

**Frontend Usage** (from Feature 014):

```svelte
<script>
  let imageResponse = await commands.getRailwayModelImage(modelId);

  let imageSrc = imageResponse.exists
    ? `asset://localhost/${imageResponse.relativePath}`
    : '/placeholder.svg';
</script>

<img src={imageSrc} alt="Model" />
```

**Decision**: Reuse existing asset protocol pattern from Feature 014

**Rationale**: Already implemented, proven to work, consistent with existing code

**No Additional Work Needed**: Display logic already exists, upload just needs to write files to the same location

---

## R7: Error Handling Patterns for File Operations

### Research Question

How should we handle and communicate file system errors (permissions, disk full, corrupted files) to users?

### Findings

**Error Categories**:

1. **Validation Errors** (user-fixable): Wrong format, file too large, corrupted image
2. **System Errors** (system-level): Permission denied, disk full, path not found
3. **Business Errors** (app logic): Model not found, already has image

**Implementation**:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageUploadError {
    #[error("Unsupported image format. Please use JPEG, PNG, or WEBP.")]
    UnsupportedFormat,

    #[error("File size ({size} MB) exceeds maximum allowed size ({max} MB)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Failed to read image file: {0}")]
    CorruptedImage(String),

    #[error("Insufficient permissions to write to storage directory")]
    PermissionDenied,

    #[error("Not enough disk space available")]
    DiskFull,

    #[error("Model with ID '{0}' not found")]
    ModelNotFound(String),

    #[error("System error: {0}")]
    SystemError(String),
}
```

**Frontend Error Display**:

```svelte
{#if error}
  <Alert variant="destructive">
    <AlertTitle>{m.upload_error_title()}</AlertTitle>
    <AlertDescription>{error}</AlertDescription>
  </Alert>
{/if}
```

**User-Friendly Messages** (Paraglide):

```json
{
  "upload_error_unsupported_format": "This file format is not supported. Please use JPEG, PNG, or WEBP images.",
  "upload_error_file_too_large": "This file is too large. Maximum size is 50 MB.",
  "upload_error_permission_denied": "Cannot save image. Please check application permissions.",
  "upload_error_disk_full": "Not enough disk space to save the image.",
  "upload_success": "Image uploaded successfully!"
}
```

**Decision**: Use `thiserror` for structured errors, map to user-friendly Paraglide messages in frontend

**Rationale**: Clear error messages help users fix issues, structured errors enable proper logging

**Alternatives Considered**:

- Generic error messages - Rejected: Poor UX, users can't fix issues
- Error codes only - Rejected: Not user-friendly
- `anyhow` - Rejected: Less structured than `thiserror` for public APIs

---

## Summary of Decisions

| Research Area         | Decision                                       | Rationale                                                |
| --------------------- | ---------------------------------------------- | -------------------------------------------------------- |
| **File Selection**    | `@tauri-apps/plugin-dialog` with image filters | Official plugin, cross-platform, pre-filters at OS level |
| **File Operations**   | `tokio::fs` in Rust backend                    | Full access, no capability config needed, simpler        |
| **Format Validation** | `image` crate for magic byte detection         | Secure, reliable, prevents spoofed extensions            |
| **Filename Strategy** | Model ID with `:` → `_`, deterministic naming  | Filesystem-safe, no DB column needed, unique by design   |
| **Drag & Drop**       | Accept file bytes, two-command approach        | Works within browser security constraints                |
| **Image Display**     | Reuse existing asset protocol from Feature 014 | Already implemented, proven pattern                      |
| **Error Handling**    | `thiserror` + Paraglide i18n messages          | Structured errors, user-friendly messages                |
| **Storage Location**  | `{app_data_dir}/models/`                       | Standard AppData location, cross-platform                |

---

## Dependencies to Add

### Backend (Cargo.toml)

```toml
[dependencies]
image = "0.25"          # Image format validation
thiserror = "2.0"       # Structured error handling
tokio = { version = "1.43", features = ["fs"] }  # Async file operations
```

### Frontend (package.json)

```json
{
  "dependencies": {
    "@tauri-apps/plugin-dialog": "^2.0.0"
  }
}
```

---

## Integration Points

**Reuses from Feature 014**:

- Media module structure (`src-tauri/src/media/`)
- Path resolution logic (`:` → `_` sanitization)
- Asset protocol for image display
- `getRailwayModelImage` command for retrieval

**New Components**:

- Upload use case
- Validation logic
- File storage infrastructure
- Upload UI components
- Drag & drop interaction

---

## Risk Mitigation

| Risk                                | Mitigation                                                         |
| ----------------------------------- | ------------------------------------------------------------------ |
| **Large file uploads slow down UI** | Async operations in Rust, loading indicators in UI                 |
| **Disk full during upload**         | Check available space before copying, clear error message          |
| **Malicious file uploads**          | Validate via magic bytes, not extensions; restrict formats         |
| **Filename collisions**             | Model IDs are unique PKs, deterministic naming prevents collisions |
| **Permission errors**               | Check directory writability on first upload, clear error messages  |
| **Corrupted images**                | Validate with `image` crate before saving, rollback on error       |

---

**Research Complete**: All technical unknowns resolved. Ready for Phase 1 (Design & Contracts).
