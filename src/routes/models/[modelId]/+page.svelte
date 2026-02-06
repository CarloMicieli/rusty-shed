<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import type {
    RailwayModelView,
    RailwayModelId,
    CollectionItemView,
    RailwayModelImageResponse
  } from '$lib/bindings';

  // Get model ID from route params
  const modelId = $page.params.modelId as RailwayModelId;

  // State
  let model = $state<RailwayModelView | null>(null);
  let collectionItem = $state<CollectionItemView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let activeTab = $state<'details' | 'rolling-stock'>('details');

  // Load data on mount
  onMount(async () => {
    try {
      loading = true;
      error = null;

      // Fetch model data
      const modelResult = await commands.getRailwayModelById(modelId);
      if (modelResult.status === 'error') {
        throw new Error(modelResult.error.message || 'Failed to load model');
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
          (item) => item.railwayModel.id === modelId
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
          {model.manufacturer.name} | {model.productCode}
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

    <!-- Tabs -->
    <div class="border-b border-border">
      <nav class="-mb-px flex space-x-8" aria-label="Tabs">
        <button
          type="button"
          class="whitespace-nowrap border-b-2 px-1 py-4 text-sm font-medium transition-colors {activeTab ===
          'details'
            ? 'border-primary text-primary'
            : 'border-transparent text-muted-foreground hover:border-border hover:text-foreground'}"
          onclick={() => (activeTab = 'details')}
          aria-current={activeTab === 'details' ? 'page' : undefined}
        >
          {m.model_details_tab()}
        </button>
        <button
          type="button"
          class="whitespace-nowrap border-b-2 px-1 py-4 text-sm font-medium transition-colors {activeTab ===
          'rolling-stock'
            ? 'border-primary text-primary'
            : 'border-transparent text-muted-foreground hover:border-border hover:text-foreground'}"
          onclick={() => (activeTab = 'rolling-stock')}
          aria-current={activeTab === 'rolling-stock' ? 'page' : undefined}
        >
          {m.model_rolling_stock_tab()}
        </button>
      </nav>
    </div>

    <!-- Tab Content -->
    <div class="mt-6">
      {#if activeTab === 'details'}
        <div class="prose dark:prose-invert max-w-none">
          {#if model.details}
            <p>{model.details}</p>
          {:else}
            <p class="text-muted-foreground">{m.model_no_details()}</p>
          {/if}
        </div>
      {:else if activeTab === 'rolling-stock'}
        {#if collectionItem?.rollingStocks && collectionItem.rollingStocks.length > 0}
          <div class="space-y-4">
            {#each collectionItem.rollingStocks as rollingStock (rollingStock.id)}
              <div class="rounded-lg border border-border">
                <div class="p-4">
                  <h3 class="text-lg font-semibold">
                    {rollingStock.series || 'Unknown'} — {rollingStock.roadNumber || 'N/A'}
                  </h3>
                  {#if rollingStock.notes}
                    <p class="text-muted-foreground mt-2">{rollingStock.notes}</p>
                  {/if}
                  <dl class="mt-4 grid grid-cols-1 gap-x-4 gap-y-2 sm:grid-cols-2">
                    {#if rollingStock.series}
                      <div>
                        <dt class="text-sm font-medium text-muted-foreground">Series</dt>
                        <dd class="mt-1 text-sm">{rollingStock.series}</dd>
                      </div>
                    {/if}
                    {#if rollingStock.roadNumber}
                      <div>
                        <dt class="text-sm font-medium text-muted-foreground">Road Number</dt>
                        <dd class="mt-1 text-sm">{rollingStock.roadNumber}</dd>
                      </div>
                    {/if}
                    {#if rollingStock.livery}
                      <div>
                        <dt class="text-sm font-medium text-muted-foreground">Livery</dt>
                        <dd class="mt-1 text-sm">{rollingStock.livery}</dd>
                      </div>
                    {/if}
                    {#if rollingStock.railwayCompanyName}
                      <div>
                        <dt class="text-sm font-medium text-muted-foreground">Company</dt>
                        <dd class="mt-1 text-sm">{rollingStock.railwayCompanyName}</dd>
                      </div>
                    {/if}
                    {#if rollingStock.control}
                      <div>
                        <dt class="text-sm font-medium text-muted-foreground">Control</dt>
                        <dd class="mt-1 text-sm">{rollingStock.control}</dd>
                      </div>
                    {/if}
                  </dl>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="rounded-lg border border-dashed border-border p-8 text-center">
            <p class="text-muted-foreground">{m.model_no_rolling_stock()}</p>
          </div>
        {/if}
      {/if}
    </div>
  </div>
{/if}
