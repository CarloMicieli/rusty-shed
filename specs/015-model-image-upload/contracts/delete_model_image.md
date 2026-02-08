# API Contract: deleteModelImage

**Command**: `delete_model_image`  
**Method**: DELETE  
**Purpose**: Remove the image associated with a railway model

---

## Request

### TypeScript (Frontend)

```typescript
interface DeleteModelImageArgs {
  modelId: string; // Railway model identifier (e.g., "marklin:39216")
}

// Usage
const result = await commands.deleteModelImage({
  modelId: 'marklin:39216'
});
```

### Rust (Backend)

```rust
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelImageArgs {
    #[validate(length(min = 1))]
    pub model_id: String,
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

**Semantic**: Image successfully deleted (or no image existed)

### Error Response

```typescript
{
  status: "error",
  error: CommandError
}
```

**CommandError Types**:

```typescript
type CommandError = { NotFound: string } | { InfrastructureError: string };
```

---

## Error Scenarios

| Error Type              | Condition           | Message Example                           |
| ----------------------- | ------------------- | ----------------------------------------- |
| **NotFound**            | Model doesn't exist | "Model with ID 'marklin:39216' not found" |
| **InfrastructureError** | Permission denied   | "Permission denied: cannot delete file"   |
| **InfrastructureError** | Delete failed       | "Failed to delete image: {details}"       |

**Note**: If no image exists for the model, command returns success (idempotent).

---

## Validation Rules

### Request Validation

1. **model_id**: Non-empty string

### Domain Validation

1. **Model exists**: Model with given ID must exist in database
2. **Image exists**: Optional (no error if image doesn't exist)

---

## Behavior

### Happy Path

1. Validate request arguments
2. Check model exists in database
3. Resolve image path based on model ID
4. Check if image file exists
5. If exists, delete file from storage
6. Return success

### Idempotency

**Idempotent**: Multiple calls with same model ID have same effect (image deleted).

**No error if image doesn't exist**: This is by design for better UX.

---

## Performance

- **Expected**: <100ms (single file deletion)
- **Worst case**: ~1s on slow filesystems
- **Blocking**: No (runs in async Rust backend)

---

## Security

1. **Path Validation**: Only deletes from models directory
2. **No Arbitrary Deletes**: Path computed from model ID, not user input
3. **Model Ownership**: Implicitly validated (single-user app, all models belong to user)
4. **Tauri Security Model**: File operations run in privileged backend

---

## Example Usage

### Frontend (Svelte)

```svelte
<script lang="ts">
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages';
  import { AlertDialog } from '$lib/components/ui/alert-dialog';

  let showDeleteDialog = $state(false);
  let isDeleting = $state(false);

  function handleDeleteClick() {
    showDeleteDialog = true;
  }

  async function confirmDelete() {
    isDeleting = true;
    error = null;

    try {
      const result = await commands.deleteModelImage({
        modelId: model.id
      });

      if (result.status === 'ok') {
        success = m.image_deleted();
        // Clear image from UI
        imageResponse = null;
      } else {
        error = mapError(result.error);
      }
    } catch (e) {
      error = m.delete_error_unknown();
    } finally {
      isDeleting = false;
      showDeleteDialog = false;
    }
  }
</script>

{#if imageResponse?.exists}
  <Button variant="destructive" onclick={handleDeleteClick}>
    <Trash2 size={16} />
    {m.delete_image()}
  </Button>
{/if}

<AlertDialog bind:open={showDeleteDialog}>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{m.confirm_delete_image_title()}</AlertDialogTitle>
      <AlertDialogDescription>
        {m.confirm_delete_image_description()}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel>{m.cancel()}</AlertDialogCancel>
      <AlertDialogAction onclick={confirmDelete} disabled={isDeleting}>
        {isDeleting ? m.deleting() : m.delete()}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
```

---

## Design Decision: Idempotent Deletion

**Why no error when image doesn't exist?**

1. **Better UX**: User goal is "ensure no image exists" - whether it was already gone is irrelevant
2. **Simpler Frontend**: No need to check existence before calling delete
3. **Race Conditions**: If image deleted externally, command still succeeds
4. **RESTful Pattern**: DELETE is idempotent in REST APIs

**Alternative Rejected**: Return error if image doesn't exist

- **Downside**: Forces frontend to check existence first
- **Downside**: Complicates error handling
- **Downside**: No practical benefit to user

---

## Related Commands

- **uploadModelImage**: Upload image for model
- **uploadModelImageBytes**: Upload via drag & drop
- **getRailwayModelImage**: Check if image exists
