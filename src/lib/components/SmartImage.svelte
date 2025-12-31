<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';

  type ImageCategory = 'static' | 'railway_model';

  let {
    id,
    category,
    alt = '',
    class: className = '',
    placeholder,
    error: errorSlot
  } = $props<{
    id: string;
    category: ImageCategory;
    alt?: string;
    class?: string;
    placeholder?: () => unknown;
    error?: () => unknown;
  }>();

  let resolvedSrc = $state<string | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  const renderPlaceholder = () => (placeholder ? placeholder() : 'Loading image...');
  const renderError = () => (errorSlot ? errorSlot() : 'Image unavailable');

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
      const path = await invoke<string>('get_image_path', { id, category });
      resolvedSrc = convertFileSrc(path);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unable to load image';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    resolveImageSource();
  });
</script>

<div class="smart-image">
  {#if loading}
    <div class="smart-image__placeholder" aria-busy="true">
      {@render renderPlaceholder()}
    </div>
  {:else if error}
    <div class="smart-image__fallback" role="img" aria-label={alt || 'Image unavailable'}>
      {@render renderError()}
    </div>
  {:else if resolvedSrc}
    <img
      src={resolvedSrc}
      alt={alt || id}
      class={`smart-image__img${className ? ` ${className}` : ''}`}
    />
  {:else}
    <div class="smart-image__fallback" role="img" aria-label={alt || id}>
      {@render renderError()}
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
    border-radius: 0.5rem;
  }

  .smart-image__placeholder,
  .smart-image__fallback {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    border-radius: 0.5rem;
    background: repeating-linear-gradient(45deg, #262626, #262626 10px, #1f1f1f 10px, #1f1f1f 20px);
    color: #c7c7c7;
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
