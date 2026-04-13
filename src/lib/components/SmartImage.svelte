<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { imageService } from '$lib/services/image.service.svelte';

  type ImageCategory = 'static' | 'railway_model';

  interface Props {
    id: string;
    category: ImageCategory;
    alt?: string;
    class?: string;
    placeholder?: import('svelte').Snippet;
    error?: import('svelte').Snippet;
  }

  let {
    id,
    category,
    alt = '',
    class: className = '',
    placeholder,
    error: errorSlot
  }: Props = $props();

  let resolvedSrc = $state<string | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function resolveImageSource() {
    if (!id) {
      error = 'Missing image identifier';
      resolvedSrc = null;
      loading = false;
      return;
    }

    if (category === 'static') {
      resolvedSrc = `/logos/${id}`;
      error = null;
      loading = false;
      return;
    }

    loading = true;
    error = null;
    resolvedSrc = null;

    try {
      const path = await imageService.resolveImagePath(id, category);
      if (path) {
        resolvedSrc = convertFileSrc(path);
      } else {
        error = 'Unable to load image';
      }
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unable to load image';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void resolveImageSource();
  });
</script>

<div class="smart-image">
  {#if loading}
    <div class="smart-image__placeholder" aria-busy="true">
      {#if placeholder}
        {@render placeholder()}
      {:else}
        Loading image...
      {/if}
    </div>
  {:else if error}
    <div class="smart-image__fallback" role="img" aria-label={alt || 'Image unavailable'}>
      {#if errorSlot}
        {@render errorSlot()}
      {:else}
        Image unavailable
      {/if}
    </div>
  {:else if resolvedSrc}
    <img
      src={resolvedSrc}
      alt={alt || id}
      class={`smart-image__img${className ? ` ${className}` : ''}`}
    />
  {:else}
    <div class="smart-image__fallback" role="img" aria-label={alt || id}>
      {#if errorSlot}
        {@render errorSlot()}
      {:else}
        Image unavailable
      {/if}
    </div>
  {/if}
</div>

<style>
  .smart-image {
    display: block;
    width: 100%;
    height: 100%;
  }

  .smart-image__img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 0.125rem;
  }

  .smart-image__placeholder,
  .smart-image__fallback {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    border-radius: 0.125rem;
    border: 1px solid hsl(var(--border));
    background: repeating-linear-gradient(
      45deg,
      hsl(var(--card)),
      hsl(var(--card)) 10px,
      hsl(var(--background)) 10px,
      hsl(var(--background)) 20px
    );
    color: hsl(var(--muted-foreground));
    font-size: 0.875rem;
    text-align: center;
    padding: 0.5rem;
  }

  .smart-image__placeholder {
    animation: pulse 1.2s ease-in-out infinite;
  }

  @keyframes pulse {
    0% {
      opacity: 0.65;
    }

    50% {
      opacity: 1;
    }

    100% {
      opacity: 0.65;
    }
  }
</style>
