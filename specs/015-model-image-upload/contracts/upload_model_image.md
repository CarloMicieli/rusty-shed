# API Contract: uploadModelImage

**Command**: `upload_model_image`  
**Method**: File Explorer Selection → Path-based Upload  
**Purpose**: Upload an image for a railway model by copying from a user-selected file path

---

## Request

### TypeScript (Frontend)

```typescript
interface UploadModelImageArgs {
  modelId: string; // Railway model identifier (e.g., "marklin:39216")
  filePath: string; // Absolute path to selected image file
}

// Usage
const result = await commands.uploadModelImage({
  modelId: 'marklin:39216',
  filePath: '/home/user/Pictures/train.jpg'
});
```

### Rust (Backend)

```rust
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageArgs {
    #[validate(length(min = 1))]
    pub model_id: String,

    #[validate(length(min = 1))]
    pub file_path: String,
}
```

---

## Response

### Success Response

```typescript
{
  status: "ok",
  data: null
}
```

**Semantic**: Image successfully uploaded and stored

### Error Response

```typescript
{
  status: "error",
  error: CommandError
}
```

**CommandError Types**:

```typescript
type CommandError =
  | { ValidationError: string }
  | { NotFound: string }
  | { InfrastructureError: string };
```

---

## Error Scenarios

| Error Type              | Condition           | Message Example                                                |
| ----------------------- | ------------------- | -------------------------------------------------------------- |
| **ValidationError**     | Unsupported format  | "Unsupported image format. Supported formats: JPEG, PNG, WEBP" |
| **ValidationError**     | File too large      | "File size (75 MB) exceeds maximum allowed size (50 MB)"       |
| **ValidationError**     | Corrupted image     | "Image file is corrupted or invalid"                           |
| **ValidationError**     | File not found      | "File not found"                                               |
| **NotFound**            | Model doesn't exist | "Model with ID 'marklin:39216' not found"                      |
| **InfrastructureError** | Permission denied   | "Permission denied: cannot write to storage directory"         |
| **InfrastructureError** | Disk full           | "Not enough disk space available"                              |
| **InfrastructureError** | Copy failed         | "Failed to copy file: {details}"                               |

---

## Validation Rules

### Request Validation

1. **model_id**: Non-empty string
2. **file_path**: Non-empty string, must be absolute path

### Domain Validation

1. **File exists**: Source file must exist at provided path
2. **File size**: Must be ≤ 50 MB
3. **Format**: Must be JPEG, PNG, or WEBP (validated via magic bytes, not extension)
4. **Image integrity**: File must be readable by image decoder
5. **Model exists**: Model with given ID must exist in database

---

## Behavior

### Happy Path

1. Validate request arguments
2. Check model exists in database
3. Validate source file (format, size, integrity)
4. Determine destination path: `{app_data_dir}/models/{model_id_sanitized}.{ext}`
5. Delete existing image if present (replacement)
6. Copy source file to destination
7. Return success

### Idempotency

**Not strictly idempotent**: Multiple calls with different source files will replace the image.

**Acceptable**: This is expected behavior (image replacement).

---

## Performance

- **Expected**: <5s for files <10MB
- **Worst case**: ~30s for 50MB files on slow storage
- **Blocking**: No (runs in async Rust backend)

---

## Security

1. **Path Validation**: Prevent directory traversal attacks
2. **Format Validation**: Magic byte detection prevents malicious file uploads
3. **Size Limit**: Prevents disk exhaustion attacks
4. **No Arbitrary Paths**: Frontend provides path via dialog, backend validates
5. **Tauri Security Model**: File operations run in privileged backend, not exposed to web context

---

## Example Usage

### Frontend (Svelte)

```svelte
<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';

  async function handleSelectImage() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'webp']
        }
      ]
    });

    if (!selected) return; // User cancelled

    isUploading = true;
    error = null;

    try {
      const result = await commands.uploadModelImage({
        modelId: model.id,
        filePath: selected
      });

      if (result.status === 'ok') {
        success = m.upload_success();
        // Refresh image display
        await loadImage();
      } else {
        error = mapError(result.error);
      }
    } catch (e) {
      error = m.upload_error_unknown();
    } finally {
      isUploading = false;
    }
  }
</script>

<Button onclick={handleSelectImage} disabled={isUploading}>
  {isUploading ? m.uploading() : m.upload_image()}
</Button>
```

---

## Related Commands

- **getRailwayModelImage**: Retrieve image path for display (Feature 014)
- **deleteModelImage**: Remove model's image (this feature)
- **uploadModelImageBytes**: Alternative upload via file bytes (drag & drop)
