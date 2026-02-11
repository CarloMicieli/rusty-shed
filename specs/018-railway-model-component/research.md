# Research: Reusable Railway Model Component

**Feature**: 018-railway-model-component
**Date**: 2026-02-11
**Status**: Complete

## Research Areas

This document consolidates findings for technical unknowns identified during planning.

---

## 1. Svelte 5 Component Structure with Runes

**Decision**: Use `$state` runes for component-local reactive state and `$derived` for computed values.

**Rationale**:

- Svelte 5 runes provide fine-grained reactivity without explicit reactive declarations
- `$state` replaces `let` for reactive variables, automatically tracking dependencies
- `$derived` replaces `$:` reactive statements for computed values
- `$props()` provides type-safe props destructuring

**Alternatives Considered**:

- Legacy Svelte 3/4 reactive syntax (`$:`) - rejected because Svelte 5 recommends runes for new code
- External state management (Zustand, Redux) - rejected as overkill for a display component

**Implementation Pattern**:

```typescript
<script lang="ts">
  import type { RailwayModel, RollingStock } from '$lib/bindings';

  // Type-safe props
  let { model, editable = false }: {
    model: RailwayModel;
    editable?: boolean
  } = $props();

  // Component-local state
  let activeTab = $state<'details' | 'rolling-stock'>('details');
  let expandedRows = $state<Set<number>>(new Set());

  // Derived/computed values
  let isSingleUnit = $derived(model.rolling_stock?.length === 1);
  let showTabs = $derived(!isSingleUnit);
</script>
```

---

## 2. Image Upload in Tauri

**Decision**: Use Tauri's file dialog API for browse selection and native drag-drop events for drag-and-drop, with backend validation.

**Rationale**:

- Tauri `dialog::FileDialogBuilder` provides cross-platform file selection
- Native HTML5 drag-drop events work seamlessly with Tauri's webview
- Backend receives base64-encoded image data via IPC command
- Validation (file type, size) happens in Rust before persisting

**Alternatives Considered**:

- Direct file system access via Tauri's `fs` API - rejected because it bypasses security sandboxing and requires explicit path management
- Frontend-only validation - rejected because it violates "Domain Logic Location" constitutional principle

**Implementation Pattern**:

```typescript
// Frontend: File selection
import { open } from '@tauri-apps/plugin-dialog';

async function selectImage() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp', 'gif'] }]
  });

  if (selected) {
    await uploadImage(selected.path);
  }
}

// Backend: Tauri command
#[tauri::command]
async fn upload_model_image(
    args: UploadModelImageArgs,
    state: State<'_, AppState>
) -> Result<String, CommandError> {
    // Validate file type and size
    // Save to media storage
    // Return image path
}
```

---

## 3. Drag-and-Drop Implementation

**Decision**: Use native HTML5 drag-drop events (`dragover`, `drop`) with Svelte event handlers and visual feedback.

**Rationale**:

- HTML5 drag-drop works natively in Tauri's webview without additional dependencies
- Simple to implement with Svelte's event binding (`on:dragover`, `on:drop`)
- Visual feedback via CSS classes provides clear UX

**Alternatives Considered**:

- Third-party drag-drop libraries (svelte-dnd-action) - rejected as unnecessary overhead for simple file drop
- Tauri-specific drag-drop events - rejected because HTML5 API is simpler and well-supported

**Implementation Pattern**:

```svelte
<script lang="ts">
  let isDragging = $state(false);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files[0];
      // Validate file type client-side for immediate feedback
      if (!file.type.startsWith('image/')) {
        showError('Please drop an image file');
        return;
      }
      await uploadImage(file);
    }
  }
</script>

<div
  class="hero-section {isDragging ? 'drag-active' : ''}"
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:drop={handleDrop}
>
  <!-- Image display -->
</div>
```

---

## 4. Tabbed Interface Patterns

**Decision**: Use shadcn-svelte `Tabs` component with ARIA attributes for accessibility.

**Rationale**:

- shadcn-svelte is already a project dependency
- `Tabs` component provides accessible tab navigation out-of-the-box (ARIA roles, keyboard navigation)
- Consistent with existing UI patterns in the project (see Dashboard)

**Alternatives Considered**:

- Custom tab implementation - rejected to avoid reinventing accessible patterns
- Accordion component for mobile - deferred; tabs are mobile-friendly with proper styling

**Implementation Pattern**:

```svelte
<script lang="ts">
  import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/components/ui/tabs';

  let activeTab = $state('details');
</script>

{#if showTabs}
  <Tabs bind:value={activeTab}>
    <TabsList>
      <TabsTrigger value="details">{m.railway_model_details()}</TabsTrigger>
      <TabsTrigger value="rolling-stock">{m.rolling_stock_list()}</TabsTrigger>
    </TabsList>

    <TabsContent value="details">
      <!-- Global specifications -->
    </TabsContent>

    <TabsContent value="rolling-stock">
      <!-- Rolling stock list -->
    </TabsContent>
  </Tabs>
{:else}
  <!-- Direct display for single-unit models -->
{/if}
```

---

## 5. Responsive Card Layout

**Decision**: Use Tailwind CSS responsive utilities with mobile-first approach, adapting shadcn-svelte `Card` component.

**Rationale**:

- Tailwind CSS 4.1.18 is the project's styling framework
- Mobile-first approach ensures usability on small screens
- shadcn-svelte `Card` components provide consistent styling (see MEMORY.md conventions)

**Alternatives Considered**:

- CSS Grid with complex breakpoints - rejected in favor of simpler Tailwind responsive classes
- Separate mobile/desktop components - rejected to maintain single reusable component

**Implementation Pattern**:

```svelte
<!-- Header: stacks vertically on mobile, horizontal on desktop -->
<div class="flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
  <h2 class="text-lg md:text-xl">{model.manufacturer} {model.product_code}</h2>
  <StatusBadge status={model.status} />
</div>

<!-- Hero: full-width on mobile, constrained on desktop -->
<div class="relative aspect-video w-full md:max-w-2xl">
  <SmartImage src={model.image_path} alt={model.product_code} />
</div>

<!-- Tabs: full-width with proper touch targets -->
<TabsList class="w-full md:w-auto">
  <TabsTrigger value="details" class="flex-1 md:flex-none">...</TabsTrigger>
</TabsList>

<!-- Rolling stock rows: stack on mobile, grid on desktop -->
<div class="space-y-2 md:grid md:grid-cols-2 md:gap-4 md:space-y-0">
  {#each model.rolling_stock as unit}
    <RollingStockRow {unit} />
  {/each}
</div>
```

---

## 6. Image Validation (Server-Side)

**Decision**: Validate file type, size, and dimensions in Rust before persisting, returning detailed errors.

**Rationale**:

- Constitutional principle "Domain Logic Location" requires validation in backend
- Frontend validation provides immediate UX feedback but can be bypassed
- Backend validation ensures data integrity

**Validation Rules**:

- **File Types**: JPEG, PNG, WebP, GIF (magic number validation, not just extension)
- **Max File Size**: 10 MB (configurable via settings)
- **Max Dimensions**: 4096x4096 pixels (prevents memory issues)
- **Min Dimensions**: 100x100 pixels (ensures usability)

**Alternatives Considered**:

- Frontend-only validation - rejected (violates constitution)
- No dimension validation - rejected (risk of memory issues with huge images)

**Implementation Pattern**:

```rust
// domain/value_objects.rs
#[derive(Debug, Clone)]
pub struct ModelImage {
    path: PathBuf,
    mime_type: String,
    size_bytes: u64,
    width: u32,
    height: u32,
}

impl ModelImage {
    pub fn validate(file_path: &Path) -> Result<Self, ImageValidationError> {
        // Read file header to detect MIME type (magic numbers)
        let mime_type = detect_mime_type(file_path)?;
        if !["image/jpeg", "image/png", "image/webp", "image/gif"].contains(&mime_type.as_str()) {
            return Err(ImageValidationError::UnsupportedFormat(mime_type));
        }

        // Check file size
        let size_bytes = file_path.metadata()?.len();
        if size_bytes > MAX_FILE_SIZE {
            return Err(ImageValidationError::FileTooLarge {
                size: size_bytes,
                max: MAX_FILE_SIZE
            });
        }

        // Check dimensions using image crate
        let img = image::open(file_path)?;
        let (width, height) = (img.width(), img.height());

        if width > MAX_DIMENSION || height > MAX_DIMENSION {
            return Err(ImageValidationError::DimensionsTooLarge { width, height });
        }

        if width < MIN_DIMENSION || height < MIN_DIMENSION {
            return Err(ImageValidationError::DimensionsTooSmall { width, height });
        }

        Ok(Self {
            path: file_path.to_path_buf(),
            mime_type,
            size_bytes,
            width,
            height
        })
    }
}
```

---

## Additional Considerations

### Testing Strategy

**Component Tests (Vitest)**:

- Render with single-unit model → no tabs shown
- Render with multi-unit model → tabs shown
- Image upload: file selection dialog opens
- Drag-drop: visual feedback on drag over
- Responsive: verify mobile/desktop classes

**Backend Tests (cargo test)**:

- `upload_model_image`: valid image succeeds
- `upload_model_image`: invalid file type rejected
- `upload_model_image`: oversized file rejected
- `upload_model_image`: invalid dimensions rejected

**Integration Tests**:

- End-to-end image upload flow (frontend → backend → storage)

### Paraglide i18n Keys

All user-facing strings must use Paraglide. New keys needed:

- `railway_model_details` - "Railway Model Details"
- `rolling_stock_list` - "Rolling Stock List"
- `upload_image` - "Upload Image"
- `drag_drop_image_here` - "Drag and drop an image here"
- `series_code` - "Series Code"
- `road_number` - "Road Number"
- `depot` - "Depot"
- `livery` - "Livery"
- `control_type` - "Control Type"
- `dcc_interface` - "DCC Interface"
- `coupling_type` - "Coupling Type"
- `error_invalid_image_format` - "Invalid image format. Please upload a JPEG, PNG, WebP, or GIF file."
- `error_image_too_large` - "Image file is too large. Maximum size is {maxSize} MB."
- `error_image_dimensions_invalid` - "Image dimensions are invalid. Must be between {minDim}x{minDim} and {maxDim}x{maxDim} pixels."

---

## Summary

All technical unknowns have been resolved:

1. ✅ **Svelte 5 Runes**: Use `$state`, `$derived`, `$props()` for reactivity
2. ✅ **Image Upload**: Tauri dialog API + HTML5 drag-drop + backend validation
3. ✅ **Drag-and-Drop**: Native HTML5 events with visual feedback
4. ✅ **Tabbed Interface**: shadcn-svelte `Tabs` component
5. ✅ **Responsive Layout**: Tailwind CSS mobile-first utilities
6. ✅ **Image Validation**: Server-side validation in Rust (type, size, dimensions)

Ready to proceed to **Phase 1: Design & Contracts**.
