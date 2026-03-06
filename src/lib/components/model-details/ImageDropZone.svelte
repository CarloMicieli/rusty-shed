<script lang="ts">
  import { Upload } from 'lucide-svelte';
  import {
    drag_and_drop_hint,
    drop_here_to_update_photo,
    upload_error_multiple_files
  } from '$lib/paraglide/messages.js';
  import { toaster } from '$lib/toaster';
  import type { RailwayModelId } from '$lib/bindings';
  import ImageCropDialog from './ImageCropDialog.svelte';

  interface Props {
    modelId: RailwayModelId;
    onUploadSuccess?: () => void;
  }

  let { modelId, onUploadSuccess }: Props = $props();

  // State
  let dragCounter = $state(0);
  const isDragging = $derived(dragCounter > 0);
  let pendingFile = $state<File | null>(null);
  let blobUrl = $state<string | null>(null);

  function handleDragEnter(event: DragEvent) {
    event.preventDefault();
    dragCounter++;
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    dragCounter--;
  }

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragCounter = 0;

    const files = event.dataTransfer?.files;
    if (!files || files.length === 0) {
      return;
    }

    if (files.length > 1) {
      toaster.error(upload_error_multiple_files());
      return;
    }

    const file = files[0];

    const validMimeTypes = ['image/jpeg', 'image/png', 'image/webp'];
    if (file.type && !validMimeTypes.includes(file.type)) {
      toaster.error(`Unsupported format: ${file.type}. Supported formats: JPEG, PNG, WebP`);
      return;
    }

    blobUrl = URL.createObjectURL(file);
    pendingFile = file;
  }
</script>

<div class="space-y-4">
  <!-- Drop Zone -->
  <div
    role="button"
    tabindex="0"
    class={[
      'relative flex min-h-[200px] cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed p-6 text-center transition-colors duration-200 ease-in-out',
      isDragging
        ? 'cursor-copy border-primary bg-primary/10 ring-2 ring-primary ring-inset'
        : 'border-muted-foreground/25 hover:border-muted-foreground/50'
    ]}
    ondragenter={handleDragEnter}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
  >
    {#if isDragging}
      <Upload class="mb-4 h-12 w-12 text-primary" />
      <p class="text-lg font-semibold text-primary">{drop_here_to_update_photo()}</p>
    {:else}
      <Upload class="mb-4 h-12 w-12 text-muted-foreground" />
      <p class="mb-2 text-sm font-semibold">{drag_and_drop_hint()}</p>
      <p class="text-xs text-muted-foreground">JPEG, PNG, or WebP (max 50MB)</p>
    {/if}
  </div>

  <!-- Crop Dialog -->
  <ImageCropDialog
    open={pendingFile !== null}
    imageSrc={blobUrl ?? ''}
    fileName={pendingFile?.name ?? 'image.jpg'}
    {modelId}
    onSaveSuccess={() => {
      pendingFile = null;
      blobUrl = null;
      onUploadSuccess?.();
    }}
    onCancel={() => {
      pendingFile = null;
      blobUrl = null;
    }}
  />
</div>
