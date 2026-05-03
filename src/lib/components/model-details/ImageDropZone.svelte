<script lang="ts">
  import { Upload } from 'lucide-svelte';
  import { listen } from '@tauri-apps/api/event';
  import { readFile } from '@tauri-apps/plugin-fs';
  import * as m from '$lib/paraglide/messages.js';
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

  const validMimeTypes = ['image/jpeg', 'image/png', 'image/webp'];
  const extMimeMap: Record<string, string> = {
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    webp: 'image/webp'
  };

  // Tauri native drag-drop: on Linux/Windows the OS-level file drop is intercepted
  // by the native layer and fires tauri://drag-drop instead of HTML5 ondrop.
  $effect(() => {
    let unlisten: (() => void) | undefined;
    let isDestroyed = false;

    listen<{ paths: string[]; position: unknown }>('tauri://drag-drop', async (event) => {
      dragCounter = 0;
      const paths = event.payload.paths;

      if (paths.length === 0) return;
      if (paths.length > 1) {
        toaster.error(m.upload_error_multiple_files());
        return;
      }

      const filePath = paths[0];
      const ext = filePath.split('.').pop()?.toLowerCase() ?? '';
      const mime = extMimeMap[ext];

      if (!mime || !validMimeTypes.includes(mime)) {
        toaster.error(m.upload_error_unsupported_format({ format: ext }));
        return;
      }

      const bytes = await readFile(filePath);
      blobUrl = URL.createObjectURL(new Blob([bytes], { type: mime }));
      // Create a synthetic File so pendingFile is non-null (used to open the dialog)
      pendingFile = new File([bytes], filePath.split('/').pop() ?? 'image', { type: mime });
    }).then((fn) => {
      if (isDestroyed) {
        fn();
      } else {
        unlisten = fn;
      }
    });

    return () => {
      isDestroyed = true;
      unlisten?.();
    };
  });

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
      toaster.error(m.upload_error_multiple_files());
      return;
    }

    const file = files[0];

    if (file.type && !validMimeTypes.includes(file.type)) {
      toaster.error(m.upload_error_unsupported_format({ format: file.type }));
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
      <p class="text-lg font-semibold text-primary">{m.image_drop_zone_drag_message()}</p>
    {:else}
      <Upload class="mb-4 h-12 w-12 text-muted-foreground" />
      <p class="mb-2 text-sm font-semibold">{m.drag_and_drop_hint()}</p>
      <p class="text-xs text-muted-foreground">{m.upload_formats_hint()}</p>
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
