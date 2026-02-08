# Quickstart: Model Image Upload System

**Feature**: 015-model-image-upload  
**Target Audience**: Developers implementing this feature  
**Purpose**: Get up and running quickly with image upload functionality

---

## 🎯 What You're Building

Add image upload capability to the Railway Model Details page:

- **File Explorer**: Click button → select image → upload
- **Drag & Drop**: Drag image from desktop → drop → upload
- **Validation**: Only JPEG, PNG, WEBP; max 50MB
- **Storage**: Files saved to `{app_data}/models/` with deterministic names
- **Display**: Uses existing asset protocol from Feature 014

---

## 🚀 Quick Setup (5 Minutes)

### 1. Add Dependencies

**Backend** (`src-tauri/Cargo.toml`):

```toml
[dependencies]
image = "0.25"
thiserror = "2.0"
tokio = { version = "1.43", features = ["fs"] }
```

**Frontend** (`package.json`):

```json
{
  "dependencies": {
    "@tauri-apps/plugin-dialog": "^2.0.0"
  }
}
```

### 2. Install Frontend Dependency

```bash
pnpm install
```

### 3. Module Structure

Create these files:

**Backend**:

```
src-tauri/src/media/
├── application/
│   └── upload_model_image.rs    # NEW
├── domain/
│   └── image_validation.rs      # NEW
├── infrastructure/
│   └── file_storage.rs          # NEW
└── interface/
    └── commands.rs              # MODIFY (add upload commands)
```

**Frontend**:

```
src/lib/components/model-details/
├── ImageUpload.svelte           # NEW
└── ImageDropZone.svelte         # NEW
```

---

## 📝 Implementation Checklist

### Phase 1: Backend Foundation

- [ ] Create `image_validation.rs` with `ImageValidator`
- [ ] Create `file_storage.rs` with `FileStorage`
- [ ] Add error types: `ValidationError`, `StorageError`
- [ ] Write unit tests for validation logic

### Phase 2: Use Cases

- [ ] Implement `UploadModelImage` use case (path-based)
- [ ] Implement `UploadModelImageBytes` use case (bytes-based)
- [ ] Implement `DeleteModelImage` use case
- [ ] Write unit tests for use cases

### Phase 3: Commands

- [ ] Add `upload_model_image` Tauri command
- [ ] Add `upload_model_image_bytes` Tauri command
- [ ] Add `delete_model_image` Tauri command
- [ ] Register commands in `lib.rs`
- [ ] Generate TypeScript bindings

### Phase 4: Frontend Components

- [ ] Create `ImageUpload.svelte` with file explorer button
- [ ] Create `ImageDropZone.svelte` with drag & drop
- [ ] Add loading states and error handling
- [ ] Add Paraglide messages

### Phase 5: Integration

- [ ] Integrate upload component into Model Details page
- [ ] Add delete button with confirmation dialog
- [ ] Test file explorer flow
- [ ] Test drag & drop flow
- [ ] Test error scenarios

### Phase 6: Polish

- [ ] Add all Paraglide messages (en.json, it.json)
- [ ] Format code (`cargo fmt`, `pnpm format`)
- [ ] Run linters (`cargo clippy`, `pnpm lint`)
- [ ] Update documentation

---

## 🧪 Quick Test

### Manual Testing Script

```bash
# 1. Start dev server
pnpm dev

# 2. Navigate to a model details page
# URL: http://localhost:1420/models/marklin:39216

# 3. Test File Explorer Upload
# - Click "Upload Image" button
# - Select a JPEG file
# - Verify image displays

# 4. Test Drag & Drop
# - Drag a PNG from desktop
# - Drop on drop zone
# - Verify image replaces previous

# 5. Test Validation
# - Try to upload a PDF file
# - Verify error message displays

# 6. Test Delete
# - Click delete button
# - Confirm deletion
# - Verify image disappears

# 7. Test Persistence
# - Close app
# - Reopen app
# - Verify image still displays
```

---

## 🎨 Frontend Component Example

### Minimal Upload Button

```svelte
<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { commands } from '$lib/bindings';
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages';

  let { modelId } = $props();
  let isUploading = $state(false);

  async function handleUpload() {
    const file = await open({
      multiple: false,
      filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp'] }]
    });

    if (!file) return;

    isUploading = true;

    try {
      await commands.uploadModelImage({ modelId, filePath: file });
      // Success - image will auto-refresh
    } catch (error) {
      console.error('Upload failed:', error);
    } finally {
      isUploading = false;
    }
  }
</script>

<Button onclick={handleUpload} disabled={isUploading}>
  {isUploading ? m.uploading() : m.upload_image()}
</Button>
```

---

## 🔧 Backend Command Example

### Minimal Upload Command

```rust
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn upload_model_image(
    args: UploadModelImageArgs,
    app_handle: tauri::AppHandle,
) -> Result<(), CommandError> {
    // 1. Validate args
    args.validate()?;

    // 2. Create use case
    let storage = FileStorage::new(&app_handle)?;
    let use_case = UploadModelImage::new(storage);

    // 3. Execute
    let input = UploadImageInput::from_args(args)?;
    use_case.execute(input).await?;

    Ok(())
}
```

---

## 🗂️ File Naming Logic

### Path Resolution

```rust
// Model ID: "marklin:39216"
// Extension: "jpg"
// Result: "marklin_39216.jpg"

fn sanitize_filename(model_id: &str, extension: &str) -> String {
    let sanitized = model_id.replace(':', "_");
    format!("{}.{}", sanitized, extension)
}

// Storage path: {app_data}/models/marklin_39216.jpg
```

**Key Points**:

- Colons replaced with underscores (Windows compatibility)
- Extension from validated image format (not user input)
- Deterministic: Same model ID always produces same filename
- Unique: Model IDs are PKs, so no collisions

---

## 🌐 Localization Messages

Add to `messages/en.json`:

```json
{
  "upload_image": "Upload Image",
  "uploading": "Uploading...",
  "upload_success": "Image uploaded successfully!",
  "upload_error_unsupported_format": "This file format is not supported. Please use JPEG, PNG, or WEBP.",
  "upload_error_file_too_large": "File is too large. Maximum size is 50 MB.",
  "delete_image": "Delete Image",
  "deleting": "Deleting...",
  "confirm_delete_image_title": "Delete Image?",
  "confirm_delete_image_description": "This action cannot be undone.",
  "drag_and_drop_hint": "Drag & drop an image here",
  "drop_image_here": "Drop image to upload"
}
```

Mirror in `messages/it.json` with Italian translations.

---

## ⚡ Performance Tips

1. **Use async/await**: All file operations are async
2. **Show loading states**: File I/O can take seconds
3. **Validate early**: Check format before copying
4. **Clean up on error**: Delete temporary files
5. **Batch operations**: Don't support multiple uploads (out of scope)

---

## 🔒 Security Checklist

- [x] Validate file format via magic bytes (not extensions)
- [x] Enforce 50MB file size limit
- [x] Sanitize filenames (`:` → `_`)
- [x] Prevent directory traversal
- [x] No direct file:// protocol usage
- [x] Backend controls all file paths
- [x] Tauri IPC for all commands

---

## 📚 Related Documentation

- [spec.md](./spec.md) - Feature specification
- [data-model.md](./data-model.md) - Domain entities and value objects
- [contracts/](./contracts/) - API specifications
- [research.md](./research.md) - Technical research and decisions

---

## 🐛 Common Issues

### Issue: Image doesn't display after upload

**Solution**: Ensure you're using asset protocol:

```svelte
<img src={`asset://localhost/${relativePath}`} />
```

### Issue: Drag & drop doesn't work

**Solution**: Ensure you're reading file as bytes:

```typescript
const arrayBuffer = await file.arrayBuffer();
const fileData = Array.from(new Uint8Array(arrayBuffer));
```

### Issue: "Permission denied" error

**Solution**: Ensure storage directory is writable:

```rust
std::fs::create_dir_all(&storage_dir)?;
```

### Issue: TypeScript types not found

**Solution**: Regenerate bindings:

```bash
pnpm run generate:types
```

---

## ✅ Definition of Done

- [ ] All three commands implemented and registered
- [ ] Frontend components created and integrated
- [ ] Unit tests passing
- [ ] Manual testing complete (all scenarios)
- [ ] Paraglide messages added
- [ ] Code formatted and linted
- [ ] No TypeScript errors
- [ ] No Clippy warnings
- [ ] Feature branch merged to main

---

## 🎓 Next Steps

1. Read [data-model.md](./data-model.md) to understand entities
2. Read [contracts/](./contracts/) to understand API contracts
3. Start with backend validation logic (easiest)
4. Move to use cases (core business logic)
5. Add commands (thin layer over use cases)
6. Build frontend components (UI)
7. Integrate into Model Details page
8. Test everything manually
9. Polish and merge

**Estimated Time**: 8-12 hours for experienced developer

---

**Ready to start? Begin with Phase 1: Backend Foundation**
