<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { commands } from '$lib/bindings';
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

      // Fetch image
      const imageResult = await commands.getRailwayModelImage(modelId);
      if (imageResult.status === 'ok') {
        imageResponse = imageResult.data;
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
  <div class="flex h-full items-center justify-center">
    <p class="text-muted-foreground">Loading...</p>
  </div>
{:else if error}
  <div class="flex h-full items-center justify-center">
    <p class="text-destructive">{error}</p>
  </div>
{:else if model}
  <div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
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
