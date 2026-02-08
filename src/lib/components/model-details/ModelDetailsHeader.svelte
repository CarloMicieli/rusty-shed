<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages.js';
  import type { RailwayModelView, RailwayModelImageResponse } from '$lib/bindings';
  import ImageUpload from './ImageUpload.svelte';
  import ImageDropZone from './ImageDropZone.svelte';

  interface Props {
    model: RailwayModelView;
    imageResponse?: RailwayModelImageResponse | null;
    onImageChange?: () => void;
  }

  let { model, imageResponse, onImageChange }: Props = $props();

  // Convert filesystem path to Tauri asset protocol
  const imageSrc = $derived(
    imageResponse?.imagePath ? convertFileSrc(imageResponse.imagePath) : null
  );
</script>

<div class="mb-8">
  <!-- Image Container -->
  <div class="mb-6 overflow-hidden rounded-lg">
    {#if imageResponse?.hasImage && imageSrc}
      <img
        src={imageSrc}
        alt={model.description}
        class="h-auto w-full object-cover"
        style="max-height: 400px;"
      />
    {:else if imageResponse?.placeholderHtml}
      <div class="h-64">
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html imageResponse.placeholderHtml}
      </div>
    {:else}
      <!-- Fallback placeholder -->
      <div
        class="flex h-64 items-center justify-center bg-muted"
        role="img"
        aria-label={m.model_image_alt_no_image()}
      >
        <p class="text-muted-foreground">{m.model_image_placeholder()}</p>
      </div>
    {/if}
  </div>

  <!-- Image Upload Component -->
  <div class="mb-6 grid gap-4 md:grid-cols-2">
    <!-- File Explorer Upload -->
    <div>
      <ImageUpload
        modelId={model.id}
        hasExistingImage={imageResponse?.hasImage ?? false}
        onUploadSuccess={onImageChange}
      />
    </div>

    <!-- Drag & Drop Upload -->
    <div>
      <ImageDropZone modelId={model.id} onUploadSuccess={onImageChange} />
    </div>
  </div>

  <!-- Header Text -->
  <div class="mb-4">
    <h1 class="mb-2 text-3xl font-bold tracking-tight sm:text-4xl">
      {model.description}
    </h1>
    <p class="text-lg text-muted-foreground">
      {model.manufacturer.display} | {model.productCode}
    </p>
  </div>

  <!-- Quick Badges -->
  <div class="flex flex-wrap gap-2">
    <span
      class="inline-flex items-center rounded-full bg-primary/10 px-3 py-1 text-sm font-medium text-primary"
    >
      {model.scale}
    </span>
    <span
      class="inline-flex items-center rounded-full bg-primary/10 px-3 py-1 text-sm font-medium text-primary"
    >
      {model.epoch}
    </span>
    <span
      class="inline-flex items-center rounded-full bg-primary/10 px-3 py-1 text-sm font-medium text-primary"
    >
      {model.powerMethod}
    </span>
  </div>
</div>
