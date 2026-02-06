<script lang="ts">
  import type { RailwayModelView, RailwayModelImageResponse } from '$lib/bindings';

  interface Props {
    model: RailwayModelView;
    imageResponse?: RailwayModelImageResponse | null;
  }

  let { model, imageResponse }: Props = $props();
</script>

<div class="mb-8">
  <!-- Image Container -->
  <div class="mb-6 overflow-hidden rounded-lg">
    {#if imageResponse?.hasImage && imageResponse.imagePath}
      <img
        src={imageResponse.imagePath}
        alt={model.description}
        class="h-auto w-full object-cover"
        style="max-height: 400px;"
      />
    {:else if imageResponse?.placeholderHtml}
      <div class="h-64">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html imageResponse.placeholderHtml}
      </div>
    {:else}
      <!-- Fallback placeholder -->
      <div
        class="flex h-64 items-center justify-center bg-muted"
        role="img"
        aria-label="No image available"
      >
        <p class="text-muted-foreground">No image yet</p>
      </div>
    {/if}
  </div>

  <!-- Header Text -->
  <div class="mb-4">
    <h1 class="mb-2 text-3xl font-bold tracking-tight sm:text-4xl">
      {model.description}
    </h1>
    <p class="text-muted-foreground text-lg">
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
