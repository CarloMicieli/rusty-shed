<script lang="ts">
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Button } from '$lib/components/ui/button';
  import { Upload, Box } from 'lucide-svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages';

  interface _Props {
    model: RailwayModel;
    editable?: boolean;
    onBrowseImage?: () => void;
    onImageDropped?: (file: File, blobUrl: string) => void;
    onError?: (error: string) => void;
  }

  const { model, editable = false, onBrowseImage, onImageDropped, onError }: _Props = $props();

  let dragState = $state(false);

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

    const blobUrl = URL.createObjectURL(file);
    onImageDropped?.(file, blobUrl);
  }
</script>

<div
  class="hero-section relative aspect-video overflow-hidden rounded-xl bg-zinc-900 {dragState
    ? 'ring-2 ring-[#E2994F]'
    : ''}"
  style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
  role="img"
  aria-label={m.railway_model_image_alt()}
  ondragover={editable ? handleDragOver : undefined}
  ondragleave={editable ? handleDragLeave : undefined}
  ondrop={editable ? handleDrop : undefined}
>
  {#if model.image_path}
    <img
      src={convertFileSrc(model.image_path)}
      alt="{model.manufacturer} {model.product_code}"
      class="block h-full w-full object-cover object-center"
    />
    {#if editable}
      <div class="absolute top-2 right-2">
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
