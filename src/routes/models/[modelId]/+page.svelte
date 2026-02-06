<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { commands } from '$lib/bindings';
  import { ArrowLeft } from 'lucide-svelte';
  import type {
    RailwayModelView,
    RailwayModelId,
    CollectionItemView,
    RailwayModelImageResponse
  } from '$lib/bindings';
  import ModelDetailsHeader from '$lib/components/model-details/ModelDetailsHeader.svelte';
  import ModelDetailsTabs from '$lib/components/model-details/ModelDetailsTabs.svelte';
  import ModelDetailsContent from '$lib/components/model-details/ModelDetailsContent.svelte';
  import RollingStockList from '$lib/components/model-details/RollingStockList.svelte';

  // Get model ID from route params
  const modelId = $page.params.modelId as RailwayModelId;

  // State
  let model = $state<RailwayModelView | null>(null);
  let collectionItem = $state<CollectionItemView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let activeTab = $state<'details' | 'rolling-stock'>('details');

  function handleTabChange(tab: 'details' | 'rolling-stock') {
    activeTab = tab;
  }

  function goBack() {
    goto('/my-collection');
  }

  // Load data on mount
  onMount(async () => {
    try {
      loading = true;
      error = null;

      // Fetch model data
      const modelResult = await commands.getRailwayModelById(modelId);
      if (modelResult.status === 'error') {
        const errorMsg = typeof modelResult.error === 'object' && 'DatabaseError' in modelResult.error
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
        collectionItem = collectionResult.data.items.find(
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
      <div class="mb-4 inline-block h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
      <p class="text-muted-foreground">Loading model details...</p>
    </div>
  </div>
{:else if error}
  <div class="flex h-full items-center justify-center p-8">
    <div class="text-center">
      <p class="text-destructive mb-4 text-lg font-semibold">{error}</p>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        onclick={goBack}
      >
        <ArrowLeft class="h-4 w-4" />
        Back to Collection
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
      Back to Collection
    </button>

    <!-- Hero Section with Image -->
    <ModelDetailsHeader {model} {imageResponse} />

    <!-- Tabs -->
    <ModelDetailsTabs {activeTab} onTabChange={handleTabChange} />

    <!-- Tab Content -->
    <div class="mt-6">
      {#if activeTab === 'details'}
        <ModelDetailsContent details={model.details} />
      {:else if activeTab === 'rolling-stock'}
        <RollingStockList rollingStocks={collectionItem?.rollingStocks} />
      {/if}
    </div>
  </div>
{/if}
