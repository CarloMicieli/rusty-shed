<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { collectionStore } from '$lib/state/collection.svelte';
  import { commands } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import DetailBackLink from '$lib/components/DetailBackLink.svelte';
  import CollectionItemSidebar from '$lib/features/collection/components/CollectionItemSidebar.svelte';
  import type {
    RailwayModelView,
    RailwayModelImageResponse,
    SellerView,
    CollectionItemView
  } from '$lib/bindings';

  const itemId = `trn:collection-item:${$page.params.itemId}`;

  let collectionItem = $state<CollectionItemView | null>(null);
  let model = $state<RailwayModelView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let seller = $state<SellerView | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let notFound = $state(false);

  const displayModel = $derived(
    model ? toRailwayModel(model, collectionItem, imageResponse) : null
  );

  async function loadData(options: { forceCollectionRefresh?: boolean } = {}) {
    const { forceCollectionRefresh = false } = options;

    if (forceCollectionRefresh) {
      await collectionStore.refresh();
    } else {
      await collectionStore.fetch();
    }

    const found = collectionStore.getItemById(itemId);
    if (!found) {
      notFound = true;
      return;
    }

    notFound = false;
    collectionItem = found;

    const railwayModelId = collectionItem.railwayModel.railwayModelId;
    const sellerId = (() => {
      const info = collectionItem.purchaseInfo;
      if (!info) return null;
      switch (info.kind) {
        case 'purchased':
          return info.data.seller;
        case 'preOrdered':
          return info.data.seller;
        case 'sold':
          return info.data.seller;
      }
    })();

    const result = await commands.getCollectionItemDetails(railwayModelId, sellerId, getLocale());

    if (result.status === 'ok') {
      model = result.data.model;
      imageResponse = result.data.image;
      seller = result.data.seller;
    }
  }

  async function handleModelUpdated() {
    try {
      await loadData({ forceCollectionRefresh: true });
    } catch (e) {
      error = e instanceof Error ? e.message : m.collection_item_error();
    }
  }

  onMount(async () => {
    try {
      await loadData();
    } catch (e) {
      error = e instanceof Error ? e.message : m.collection_item_error();
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>
    {collectionItem
      ? `${collectionItem.railwayModel.description} — ${m.app_collection()}`
      : m.app_collection()}
  </title>
</svelte:head>

{#if loading}
  <div class="flex h-64 items-center justify-center">
    <div class="flex flex-col items-center gap-3">
      <div
        class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent"
      ></div>
      <p class="text-sm text-muted-foreground">{m.collection_item_loading()}</p>
    </div>
  </div>
{:else if notFound}
  <div class="flex h-64 flex-col items-center justify-center gap-4 text-center">
    <p class="text-lg font-semibold text-destructive">{m.collection_item_not_found()}</p>
    <p class="text-sm text-muted-foreground">{m.collection_item_not_found_message()}</p>
    <DetailBackLink path="/collection" ariaLabel={m.collection_item_back()} />
  </div>
{:else if error}
  <div class="flex h-64 flex-col items-center justify-center gap-4 text-center">
    <p class="text-lg font-semibold text-destructive">{error}</p>
    <DetailBackLink path="/collection" ariaLabel={m.collection_item_back()} />
  </div>
{:else if collectionItem}
  <div class="w-full max-w-full">
    <!-- Back button -->
    <DetailBackLink path="/collection" ariaLabel={m.collection_item_back()} class="mb-6" />

    <!-- Two-panel layout -->
    <div class="flex flex-col gap-6 lg:flex-row">
      <!-- Left panel: Railway model card -->
      <div class="min-w-0 flex-1">
        {#if displayModel}
          <RailwayModelCard
            model={displayModel}
            editable={true}
            onModelUpdated={handleModelUpdated}
          />
        {/if}
      </div>

      <!-- Right panel: Sidebar -->
      <CollectionItemSidebar item={collectionItem} {seller} onItemUpdated={handleModelUpdated} />
    </div>
  </div>
{/if}
