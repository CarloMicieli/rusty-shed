# Data Model: Model Image Upload System

**Feature**: 015-model-image-upload  
**Date**: February 8, 2026  
**Purpose**: Define entities, value objects, and data structures for image upload functionality

---

## Overview

This feature does NOT introduce new database tables. Instead, it uses **deterministic file naming** based on model IDs to establish a convention-based relationship between railway models and their images.

**Key Principle**: File path = `{app_data_dir}/models/{model_id_sanitized}.{extension}`

---

## Entities

### RailwayModel (Existing, No Changes)

**Location**: `src-tauri/src/catalog/domain/railway_model.rs`

The existing `RailwayModel` entity is not modified. Images are associated through filename convention, not database foreign keys.

**Key Attribute**:

- `id`: `RailwayModelId` (String, format: `{manufacturer}:{product_code}`)

**Example**:

- ID: `"marklin:39216"`
- Corresponding image filename: `marklin_39216.jpg`

---

## Value Objects

### ImageFormat

**Purpose**: Represent supported image formats

**Definition**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
}

impl ImageFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::WebP),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }
}
```

---

### ModelImagePath

**Purpose**: Value object representing the path to a model's image file

**Definition**:

```rust
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelImagePath {
    full_path: PathBuf,
    relative_path: PathBuf,
}

impl ModelImagePath {
    pub fn new(storage_dir: &Path, model_id: &str, format: ImageFormat) -> Self {
        let filename = sanitize_filename(model_id, format);
        let full_path = storage_dir.join(&filename);
        let relative_path = PathBuf::from("models").join(&filename);

        Self {
            full_path,
            relative_path,
        }
    }

    pub fn full_path(&self) -> &Path {
        &self.full_path
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub fn exists(&self) -> bool {
        self.full_path.exists()
    }
}

fn sanitize_filename(model_id: &str, format: ImageFormat) -> String {
    let sanitized_id = model_id.replace(':', "_");
    format!("{}.{}", sanitized_id, format.extension())
}
```

**Example**:

- Model ID: `"marklin:39216"`
- Format: `ImageFormat::Jpeg`
- Full path: `/home/user/.local/share/rusty-shed/models/marklin_39216.jpg`
- Relative path: `models/marklin_39216.jpg`

---

### FileSize

**Purpose**: Value object for file size validation

**Definition**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileSize(u64); // bytes

impl FileSize {
    pub const MAX_SIZE: u64 = 50 * 1024 * 1024; // 50 MB

    pub fn new(bytes: u64) -> Result<Self, ValidationError> {
        if bytes > Self::MAX_SIZE {
            return Err(ValidationError::FileTooLarge {
                size_mb: bytes / (1024 * 1024),
                max_mb: Self::MAX_SIZE / (1024 * 1024),
            });
        }
        Ok(Self(bytes))
    }

    pub fn bytes(&self) -> u64 {
        self.0
    }

    pub fn megabytes(&self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0)
    }
}
```

---

## Domain Services

### ImageValidator

**Purpose**: Validate uploaded images

**Definition**:

```rust
use std::path::Path;

pub struct ImageValidator;

impl ImageValidator {
    pub fn validate(path: &Path) -> Result<ImageFormat, ValidationError> {
        // 1. Check file exists
        if !path.exists() {
            return Err(ValidationError::FileNotFound);
        }

        // 2. Check file size
        let metadata = std::fs::metadata(path)
            .map_err(|e| ValidationError::IoError(e.to_string()))?;

        FileSize::new(metadata.len())?;

        // 3. Validate image format via magic bytes
        let format = Self::detect_format(path)?;

        Ok(format)
    }

    fn detect_format(path: &Path) -> Result<ImageFormat, ValidationError> {
        use image::ImageReader;

        let reader = ImageReader::open(path)
            .map_err(|_| ValidationError::CorruptedImage)?;

        let format = reader
            .with_guessed_format()
            .map_err(|_| ValidationError::CorruptedImage)?
            .format();

        match format {
            Some(image::ImageFormat::Jpeg) => Ok(ImageFormat::Jpeg),
            Some(image::ImageFormat::Png) => Ok(ImageFormat::Png),
            Some(image::ImageFormat::WebP) => Ok(ImageFormat::WebP),
            _ => Err(ValidationError::UnsupportedFormat),
        }
    }
}
```

---

## Domain Errors

### ValidationError

**Purpose**: Errors that occur during image validation

**Definition**:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("File not found")]
    FileNotFound,

    #[error("Unsupported image format. Supported formats: JPEG, PNG, WEBP")]
    UnsupportedFormat,

    #[error("File size ({size_mb} MB) exceeds maximum allowed size ({max_mb} MB)")]
    FileTooLarge { size_mb: u64, max_mb: u64 },

    #[error("Image file is corrupted or invalid")]
    CorruptedImage,

    #[error("I/O error: {0}")]
    IoError(String),
}
```

### StorageError

**Purpose**: Errors that occur during file storage operations

**Definition**:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Failed to create storage directory: {0}")]
    DirectoryCreation(String),

    #[error("Permission denied: cannot write to storage directory")]
    PermissionDenied,

    #[error("Not enough disk space available")]
    DiskFull,

    #[error("Failed to copy file: {0}")]
    CopyFailed(String),

    #[error("Failed to delete old image: {0}")]
    DeleteFailed(String),

    #[error("I/O error: {0}")]
    IoError(String),
}
```

---

## Application Layer DTOs

### UploadModelImageArgs

**Purpose**: Transport DTO for upload command (File Explorer path)

**Definition**:

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageArgs {
    #[validate(length(min = 1))]
    pub model_id: String,

    #[validate(length(min = 1))]
    pub file_path: String,
}
```

### UploadModelImageBytesArgs

**Purpose**: Transport DTO for upload command (Drag & Drop bytes)

**Definition**:

```rust
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageBytesArgs {
    #[validate(length(min = 1))]
    pub model_id: String,

    #[validate(length(min = 1))]
    pub file_name: String,

    // Note: Vec<u8> doesn't validate, but we validate in use case
    pub file_data: Vec<u8>,
}
```

### DeleteModelImageArgs

**Purpose**: Transport DTO for delete command

**Definition**:

```rust
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelImageArgs {
    #[validate(length(min = 1))]
    pub model_id: String,
}
```

---

## Use Case Inputs

### UploadImageInput

**Purpose**: Validated input for upload use case

**Definition**:

```rust
use std::path::PathBuf;

pub struct UploadImageInput {
    pub model_id: String,
    pub source_path: PathBuf,
}

impl UploadImageInput {
    pub fn from_args(args: UploadModelImageArgs) -> Result<Self, ValidationError> {
        args.validate()?;

        Ok(Self {
            model_id: args.model_id,
            source_path: PathBuf::from(args.file_path),
        })
    }
}
```

### UploadImageBytesInput

**Purpose**: Validated input for upload-from-bytes use case

**Definition**:

```rust
pub struct UploadImageBytesInput {
    pub model_id: String,
    pub file_name: String,
    pub file_data: Vec<u8>,
}

impl UploadImageBytesInput {
    pub fn from_args(args: UploadModelImageBytesArgs) -> Result<Self, ValidationError> {
        args.validate()?;

        // Validate file size
        FileSize::new(args.file_data.len() as u64)?;

        Ok(Self {
            model_id: args.model_id,
            file_name: args.file_name,
            file_data: args.file_data,
        })
    }
}
```

---

## Infrastructure Layer

### FileStorage

**Purpose**: Handle low-level file operations

**Interface**:

```rust
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct FileStorage {
    storage_dir: PathBuf,
}

impl FileStorage {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, StorageError> {
        let storage_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| StorageError::DirectoryCreation(e.to_string()))?
            .join("models");

        // Ensure directory exists
        std::fs::create_dir_all(&storage_dir)
            .map_err(|e| StorageError::DirectoryCreation(e.to_string()))?;

        Ok(Self { storage_dir })
    }

    pub async fn copy_image(
        &self,
        source: &Path,
        dest: &Path,
    ) -> Result<(), StorageError> {
        fs::copy(source, dest)
            .await
            .map_err(|e| StorageError::CopyFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn write_image(
        &self,
        dest: &Path,
        data: &[u8],
    ) -> Result<(), StorageError> {
        fs::write(dest, data)
            .await
            .map_err(|e| StorageError::CopyFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_image(&self, path: &Path) -> Result<(), StorageError> {
        if path.exists() {
            fs::remove_file(path)
                .await
                .map_err(|e| StorageError::DeleteFailed(e.to_string()))?;
        }

        Ok(())
    }

    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }
}
```

---

## State Transitions

### Upload Flow

```
┌─────────────────┐
│ User Action     │
│ (Select/Drop)   │
└────────┬────────┘
         │
         v
┌─────────────────┐
│ Frontend        │
│ Triggers Upload │
└────────┬────────┘
         │
         v
┌─────────────────────────┐
│ Backend: Validate Args  │
│ (model_id, file_path)   │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Domain: Validate File   │
│ - Format (magic bytes)  │
│ - Size (<50MB)          │
│ - Readability           │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Check Model Exists      │
│ (Query DB)              │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Generate Destination    │
│ {model_id}.{ext}        │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Delete Old Image        │
│ (if exists)             │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Copy/Write to Storage   │
│ {app_data}/models/      │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Return Success          │
│ Frontend refreshes      │
└─────────────────────────┘
```

### Delete Flow

```
┌─────────────────┐
│ User Action     │
│ (Click Delete)  │
└────────┬────────┘
         │
         v
┌─────────────────────────┐
│ Frontend Confirms       │
│ (Optional Dialog)       │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Backend: Validate Args  │
│ (model_id)              │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Resolve Image Path      │
│ Check if exists         │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Delete File             │
│ (if exists)             │
└────────┬────────────────┘
         │
         v
┌─────────────────────────┐
│ Return Success          │
│ Frontend updates UI     │
└─────────────────────────┘
```

---

## File System Structure

```
{app_data_dir}/
└── models/
    ├── marklin_39216.jpg
    ├── roco_73241.png
    ├── fleischmann_4380.webp
    ├── brawa_40123.jpg
    └── ... (one file per model with image)
```

**Characteristics**:

- One directory for all model images
- Flat structure (no subdirectories)
- Deterministic filenames based on model ID
- No orphaned files (deletion handled by app)
- No index/manifest file needed

---

## Integration with Feature 014

**Feature 014** (Railway Model Details Page) introduced:

- `getRailwayModelImage` command
- Path resolution with `:` → `_` sanitization
- Asset protocol for display

**Feature 015** adds:

- `uploadModelImage` command
- `uploadModelImageBytes` command (for drag & drop)
- `deleteModelImage` command
- Validation and storage infrastructure

**Shared Logic**:

- Path sanitization function (`:` → `_`)
- Storage directory location (`{app_data_dir}/models/`)
- Asset protocol usage

---

## No Database Changes

**Why No Database Column?**

Instead of adding an `image_path` column to `railway_models` table:

**Advantages of Deterministic Naming**:

1. **Zero Schema Changes**: No migrations, no version bump
2. **Atomic Operations**: File existence check = image exists
3. **Simpler Logic**: Path is computed, not queried
4. **Easier Cleanup**: Delete by model ID, no orphan tracking
5. **Consistent Convention**: All features follow same pattern

**Trade-offs**:

- Cannot store multiple images per model (acceptable for MVP)
- Cannot store original filename (acceptable, not needed)
- Must sanitize model IDs for filesystem (already done in Feature 014)

---

## Summary

- **No new tables or columns**
- **Convention-based**: Model ID → Filename mapping
- **Value objects**: ImageFormat, ModelImagePath, FileSize
- **Domain services**: ImageValidator
- **Errors**: ValidationError, StorageError
- **DTOs**: Upload/Delete Args + validated Inputs
- **Infrastructure**: FileStorage for file operations
- **Integration**: Reuses Feature 014 path resolution and display logic

**Ready for Contract Definition (API Specifications)**
