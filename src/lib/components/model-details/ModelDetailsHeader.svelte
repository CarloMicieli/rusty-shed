<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type { RailwayModelView, RailwayModelImageResponse } from '$lib/bindings';
  import ImageUpload from './ImageUpload.svelte';
  import ImageDropZone from './ImageDropZone.svelte';
  import LanguageFallbackBadge from '$lib/components/LanguageFallbackBadge.svelte';

  interface Props {
    model: RailwayModelView;
    imageResponse?: RailwayModelImageResponse | null;
    onImageChange?: () => void;
  }

  let { model, imageResponse, onImageChange }: Props = $props();

  let uploadVersion = $state(0);

  // Convert filesystem path to Tauri asset protocol; append version to bust WebView cache after uploads
  const imageSrc = $derived(
    imageResponse?.imagePath
      ? `${convertFileSrc(imageResponse.imagePath)}?v=${uploadVersion}`
      : null
  );
</script>

<div class="mb-8">
  <!-- Image Container -->
  <div class="relative mb-6 aspect-[3/1] overflow-hidden rounded-lg bg-zinc-900">
    {#if imageResponse?.hasImage && imageSrc}
      <!-- Blurred backdrop: fills letterbox bars with a softened version of the image -->
      <img
        src={imageSrc}
        alt=""
        aria-hidden="true"
        class="absolute inset-0 h-full w-full scale-110 object-cover opacity-60 blur-2xl"
      />
      <!-- Primary image: always fully visible, centered -->
      <img
        src={imageSrc}
        alt={model.description}
        class="absolute inset-0 z-10 h-full w-full object-contain"
      />
    {:else if imageResponse?.placeholderHtml}
      <div class="absolute inset-0">
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html imageResponse.placeholderHtml}
      </div>
    {:else}
      <!-- Fallback placeholder -->
      <div
        class="absolute inset-0 flex items-center justify-center bg-muted"
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
        onUploadSuccess={() => {
          uploadVersion++;
          onImageChange?.();
        }}
      />
    </div>

    <!-- Drag & Drop Upload -->
    <div>
      <ImageDropZone
        modelId={model.id}
        onUploadSuccess={() => {
          uploadVersion++;
          onImageChange?.();
        }}
      />
    </div>
  </div>

  <!-- Header Text -->
  <div class="mb-4">
    <h1 class="mb-2 text-3xl font-bold tracking-tight sm:text-4xl">
      {model.description}
      {#if model.descriptionLang !== getLocale()}
        <LanguageFallbackBadge lang={model.descriptionLang} />
      {/if}
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
