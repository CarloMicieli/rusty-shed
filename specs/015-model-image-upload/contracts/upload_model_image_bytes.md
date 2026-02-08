# API Contract: uploadModelImageBytes

**Command**: `upload_model_image_bytes`  
**Method**: Drag & Drop → Bytes-based Upload  
**Purpose**: Upload an image for a railway model by accepting file bytes from drag & drop operation

---

## Request

### TypeScript (Frontend)

```typescript
interface UploadModelImageBytesArgs {
  modelId: string; // Railway model identifier (e.g., "marklin:39216")
  fileName: string; // Original filename (e.g., "train.jpg")
  fileData: number[]; // File bytes as array of unsigned 8-bit integers
}

// Usage
const result = await commands.uploadModelImageBytes({
  modelId: 'marklin:39216',
  fileName: 'train.jpg',
  fileData: Array.from(new Uint8Array(await file.arrayBuffer()))
});
```

### Rust (Backend)

```rust
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageBytesArgs {
    #[validate(length(min = 1))]
    pub model_id: String,

    #[validate(length(min = 1))]
    pub file_name: String,

    pub file_data: Vec<u8>,
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

**CommandError Types**: Same as `uploadModelImage`

---

## Error Scenarios

| Error Type              | Condition           | Message Example                                                |
| ----------------------- | ------------------- | -------------------------------------------------------------- |
| **ValidationError**     | Empty file data     | "File data is empty"                                           |
| **ValidationError**     | File too large      | "File size (75 MB) exceeds maximum allowed size (50 MB)"       |
| **ValidationError**     | Unsupported format  | "Unsupported image format. Supported formats: JPEG, PNG, WEBP" |
| **ValidationError**     | Corrupted image     | "Image file is corrupted or invalid"                           |
| **NotFound**            | Model doesn't exist | "Model with ID 'marklin:39216' not found"                      |
| **InfrastructureError** | Permission denied   | "Permission denied: cannot write to storage directory"         |
| **InfrastructureError** | Disk full           | "Not enough disk space available"                              |
| **InfrastructureError** | Write failed        | "Failed to write file: {details}"                              |

---

## Validation Rules

### Request Validation

1. **model_id**: Non-empty string
2. **file_name**: Non-empty string (used only for extension detection)
3. **file_data**: Non-empty byte array

### Domain Validation

1. **File size**: Must be ≤ 50 MB
2. **Format**: Detected from file bytes (magic bytes), not filename extension
3. **Image integrity**: Bytes must form a valid image
4. **Model exists**: Model with given ID must exist in database

---

## Behavior

### Happy Path

1. Validate request arguments
2. Check model exists in database
3. Write bytes to temporary file
4. Validate image file (format, size, integrity)
5. Determine destination path: `{app_data_dir}/models/{model_id_sanitized}.{ext}`
6. Delete existing image if present (replacement)
7. Move validated file to destination
8. Clean up temporary file
9. Return success

### Temporary File Handling

- Temporary file created in OS temp directory
- Validated before moving to permanent storage
- Always cleaned up (even on error)
- No risk of orphaned temp files

### Idempotency

**Not strictly idempotent**: Multiple calls with different file data will replace the image.

**Acceptable**: This is expected behavior (image replacement).

---

## Performance

- **Expected**: <5s for files <10MB
- **Worst case**: ~30s for 50MB files on slow storage
- **Blocking**: No (runs in async Rust backend)
- **Note**: Byte transfer overhead ~10-20% slower than path-based upload

---

## Security

1. **Byte Validation**: Validate bytes as actual image data (magic bytes)
2. **Size Limit**: Enforced before writing to disk
3. **Temporary File**: Validated before permanent storage
4. **No Arbitrary Writes**: Backend controls destination path
5. **Tauri Security Model**: File operations run in privileged backend

---

## Example Usage

### Frontend (Svelte)

```svelte
<script lang="ts">
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';

  let isDragging = $state(false);
  let isUploading = $state(false);
  let error = $state<string | null>(null);

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

    // Take only first file
    const file = files[0];

    // Validate file type (hint to user)
    if (!['image/jpeg', 'image/png', 'image/webp'].includes(file.type)) {
      error = m.upload_error_unsupported_format();
      return;
    }

    isUploading = true;
    error = null;

    try {
      // Read file as array buffer
      const arrayBuffer = await file.arrayBuffer();
      const fileData = Array.from(new Uint8Array(arrayBuffer));

      const result = await commands.uploadModelImageBytes({
        modelId: model.id,
        fileName: file.name,
        fileData
      });

      if (result.status === 'ok') {
        success = m.upload_success();
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

<div
  class="drop-zone"
  class:dragging={isDragging}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  {#if isDragging}
    <p>{m.drop_image_here()}</p>
  {:else if isUploading}
    <p>{m.uploading()}</p>
  {:else}
    <p>{m.drag_and_drop_hint()}</p>
  {/if}
</div>
```

---

## Why Two Upload Commands?

### uploadModelImage (Path-based)

- **Use case**: File Explorer selection
- **Pros**: More efficient (no byte transfer), simpler
- **Cons**: Requires file path (dialog API)

### uploadModelImageBytes (Bytes-based)

- **Use case**: Drag & Drop
- **Pros**: Works with browser File API
- **Cons**: Requires byte transfer (10-20% overhead), temporary file

**Justification**: Browser drag & drop doesn't expose file paths for security reasons. Two commands provide optimal UX for both interaction patterns.

---

## Related Commands

- **uploadModelImage**: Path-based upload (file explorer)
- **getRailwayModelImage**: Retrieve image path for display
- **deleteModelImage**: Remove model's image
