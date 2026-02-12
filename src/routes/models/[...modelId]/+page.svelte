<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { commands } from '$lib/bindings';
  import { ArrowLeft } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type {
    RailwayModelView,
    RailwayModelId,
    CollectionItemView,
    RailwayModelImageResponse
  } from '$lib/bindings';
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';

  // Get model ID from route params - with [...modelId], it captures the full path
  // The param will be "trn:railway-model:manufacturer:product"
  const modelId = $page.params.modelId as RailwayModelId;

  // State
  let model = $state<RailwayModelView | null>(null);
  let collectionItem = $state<CollectionItemView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Derived reactive model for display
  let displayModel = $derived(
    model ? toRailwayModel(model, collectionItem, imageResponse) : null
  );

  function goBack() {
    goto('/my-collection');
  }

  async function handleImageUploaded(_imagePath: string) {
    // Refresh image after upload
    await refreshImage();
  }

  function handleImageError(errorMessage: string) {
    console.error('Image upload error:', errorMessage);
    error = errorMessage;
  }

  async function refreshImage() {
    try {
      const imageResult = await commands.getRailwayModelImage(modelId);
      if (imageResult.status === 'ok') {
        imageResponse = imageResult.data;
      }
    } catch (imgError) {
      console.warn('Failed to refresh image:', imgError);
    }
  }

  // Load data on mount
  onMount(async () => {
    try {
      loading = true;
      error = null;

      // Fetch model data
      const modelResult = await commands.getRailwayModelById(modelId);
      if (modelResult.status === 'error') {
        const errorMsg =
          typeof modelResult.error === 'object' && 'DatabaseError' in modelResult.error
            ? modelResult.error.DatabaseError
            : 'Failed to load model';
        throw new Error(errorMsg);
      }

      if (!modelResult.data) {
        throw new Error('Model not found');
      }

      model = modelResult.data;

      // Fetch image (don't fail if image loading fails)
      try {
        const imageResult = await commands.getRailwayModelImage(modelId);
        if (imageResult.status === 'ok') {
          imageResponse = imageResult.data;
        }
      } catch (imgError) {
        console.warn('Failed to load image:', imgError);
        // Continue without image
      }

      // Fetch collection data for rolling stock
      const collectionResult = await commands.getCollection();
      if (collectionResult.status === 'ok') {
        collectionItem =
          collectionResult.data.items.find(
            (item) => item.railwayModel.railwayModelId === modelId
          ) || null;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'An error occurred';
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <div class="flex h-full items-center justify-center p-8">
    <div class="text-center">
      <div
        class="mb-4 inline-block h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"
      ></div>
      <p class="text-muted-foreground">{m.model_loading()}</p>
    </div>
  </div>
{:else if error}
  <div class="flex h-full items-center justify-center p-8">
    <div class="text-center">
      <p class="mb-4 text-lg font-semibold text-destructive">{error}</p>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        onclick={goBack}
      >
        <ArrowLeft class="h-4 w-4" />
        {m.model_back_to_collection()}
      </button>
    </div>
  </div>
{:else if model}
  <div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
    <!-- Back Button -->
    <button
      type="button"
      class="mb-4 inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
      onclick={goBack}
    >
      <ArrowLeft class="h-4 w-4" />
      {m.model_back_to_collection()}
    </button>

    <!-- Railway Model Card -->
    {#if displayModel}
      <RailwayModelCard
        model={displayModel}
        editable={true}
        onImageUploaded={handleImageUploaded}
        onError={handleImageError}
      />
    {/if}
  </div>
{/if}
