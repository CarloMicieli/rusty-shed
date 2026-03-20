<script lang="ts">
  import { onMount } from 'svelte';
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Button } from '$lib/components/ui/button';
  import { Upload, Box } from 'lucide-svelte';
  import { listen } from '@tauri-apps/api/event';
  import { readFile } from '@tauri-apps/plugin-fs';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';

  interface _Props {
    model: RailwayModel;
    editable?: boolean;
    imageVersion?: number;
    onBrowseImage?: () => void;
    onImageDropped?: (file: File, blobUrl: string) => void;
    onError?: (error: string) => void;
  }

  const {
    model,
    editable = false,
    imageVersion = 0,
    onBrowseImage,
    onImageDropped,
    onError
  }: _Props = $props();

  let dragState = $state(false);
  let blobUrl = $state<string | null>(null);

  // Read the image directly from disk into a blob URL so the WebView asset
  // cache (which ignores query parameters) is bypassed completely.
  // Re-runs whenever image_path or imageVersion changes (upload trigger).
  $effect(() => {
    const path = model.image_path;
    void imageVersion; // register as dependency so upload bumps re-read

    if (!path) {
      if (blobUrl) URL.revokeObjectURL(blobUrl);
      blobUrl = null;
      return;
    }

    let stale = false;
    const ext = path.split('.').pop()?.toLowerCase() ?? 'jpg';
    const mime = extMimeMap[ext] ?? 'image/jpeg';

    void readFile(path)
      .then((bytes) => {
        if (stale) return;
        const prev = blobUrl;
        blobUrl = URL.createObjectURL(new Blob([bytes], { type: mime }));
        if (prev) URL.revokeObjectURL(prev);
      })
      .catch((err: unknown) => {
        if (!stale) console.error('[RailwayModelImagePanel] readFile failed:', err);
      });

    return () => {
      stale = true;
    };
  });

  const extMimeMap: Record<string, string> = {
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    webp: 'image/webp'
  };
  const validMimeTypes = ['image/jpeg', 'image/png', 'image/webp'];

  onMount(() => {
    let unlistenEnter: (() => void) | undefined;
    let unlistenLeave: (() => void) | undefined;
    let unlistenDrop: (() => void) | undefined;

    listen<{ paths: string[]; position: unknown }>('tauri://drag-enter', () => {
      if (!editable) return;
      dragState = true;
    }).then((fn) => {
      unlistenEnter = fn;
    });

    listen<{ paths: string[]; position: unknown }>('tauri://drag-leave', () => {
      dragState = false;
    }).then((fn) => {
      unlistenLeave = fn;
    });

    listen<{ paths: string[]; position: unknown }>('tauri://drag-drop', async (event) => {
      dragState = false;
      if (!editable) return;

      const paths = event.payload.paths;
      if (paths.length === 0) return;

      const filePath = paths[0];
      const ext = filePath.split('.').pop()?.toLowerCase() ?? '';
      const mime = extMimeMap[ext];

      if (!mime || !validMimeTypes.includes(mime)) {
        toaster.error(m.upload_error_unsupported_format());
        return;
      }

      try {
        const bytes = await readFile(filePath);
        const blobUrl = URL.createObjectURL(new Blob([bytes], { type: mime }));
        const file = new File([bytes], filePath.split('/').pop() ?? 'image', { type: mime });
        onImageDropped?.(file, blobUrl);
      } catch {
        toaster.error(m.upload_error_unknown());
      }
    }).then((fn) => {
      unlistenDrop = fn;
    });

    return () => {
      unlistenEnter?.();
      unlistenLeave?.();
      unlistenDrop?.();
    };
  });

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dragState = true;
  }

  function handleDragLeave() {
    dragState = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragState = false;

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) return;

    const file = files[0];

    if (!file.type.startsWith('image/')) {
      onError?.('Invalid image format');
      return;
    }

    if (file.size > 50 * 1024 * 1024) {
      onError?.('File too large. Maximum size is 50 MB.');
      return;
    }

    const dropBlobUrl = URL.createObjectURL(file);
    onImageDropped?.(file, dropBlobUrl);
  }
</script>

<div
  class="hero-section relative aspect-[3/1] overflow-hidden rounded-xl bg-zinc-900 {dragState
    ? 'ring-2 ring-[#E2994F]'
    : ''}"
  style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
  role="img"
  aria-label={m.railway_model_image_alt()}
  ondragover={editable ? handleDragOver : undefined}
  ondragleave={editable ? handleDragLeave : undefined}
  ondrop={editable ? handleDrop : undefined}
>
  {#if model.image_path && blobUrl}
    <!-- Blurred backdrop: fills letterbox bars with a softened version of the image -->
    <img
      src={blobUrl}
      alt=""
      aria-hidden="true"
      class="absolute inset-0 h-full w-full scale-110 object-cover opacity-60 blur-2xl"
    />
    <!-- Primary image: always fully visible, centered -->
    <img
      src={blobUrl}
      alt="{model.manufacturer} {model.product_code}"
      class="absolute inset-0 z-10 h-full w-full object-contain"
    />
    {#if editable}
      <div class="absolute top-2 right-2 z-20">
        <Button
          variant="secondary"
          size="sm"
          onclick={onBrowseImage}
          aria-label={m.replace_image()}
        >
          <Upload class="h-4 w-4" />
          <span class="ml-2">{m.replace_image()}</span>
        </Button>
      </div>
    {/if}
  {:else}
    <div class="flex h-full w-full items-center justify-center">
      {#if editable}
        <div class="flex flex-col items-center gap-3 text-zinc-600">
          <Box class="size-12" />
          <p class="text-xs text-zinc-500">
            {dragState ? m.drag_drop_image_here() : m.railway_model_no_image()}
          </p>
          <Button variant="secondary" size="sm" onclick={onBrowseImage}>
            <Upload class="mr-2 h-4 w-4" />
            {m.upload_image()}
          </Button>
        </div>
      {:else}
        <div class="flex flex-col items-center gap-2 text-zinc-600/70">
          <Box class="size-12" />
          <p class="text-xs text-zinc-600">{m.railway_model_no_image()}</p>
        </div>
      {/if}
    </div>
  {/if}
</div>
