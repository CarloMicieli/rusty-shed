<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { collectionStore } from '$lib/state/collection.svelte';
  import { commands } from '$lib/bindings';
  import { ArrowLeft } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import CollectionItemSidebar from '$lib/features/collection/components/CollectionItemSidebar.svelte';
  import type {
    RailwayModelView,
    RailwayModelImageResponse,
    SellerView,
    CollectionItemView
  } from '$lib/bindings';

  const itemId = $page.params.itemId as string;

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

  function goBack() {
    goto('/collection');
  }

  onMount(async () => {
    try {
      // 1. Ensure collection is loaded (cache-first)
      await collectionStore.fetch();
      const found = collectionStore.getItemById(itemId);

      if (!found) {
        notFound = true;
        return;
      }

      collectionItem = found;

      const railwayModelId = collectionItem.railwayModel.railwayModelId;
      const sellerId =
        collectionItem.purchaseInfo?.kind === 'purchased'
          ? collectionItem.purchaseInfo.data.seller
          : null;

      // 2. Parallel fetch: model card data + seller
      const [modelResult, imageResult, sellerResult] = await Promise.all([
        commands.getRailwayModelById(railwayModelId, getLocale()),
        commands.getRailwayModelImage(railwayModelId),
        sellerId
          ? commands.getSellerById(sellerId)
          : Promise.resolve({ status: 'ok' as const, data: null })
      ]);

      if (modelResult.status === 'ok') model = modelResult.data;
      if (imageResult.status === 'ok') imageResponse = imageResult.data;
      if (sellerResult.status === 'ok') seller = sellerResult.data;
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
    <button
      type="button"
      class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
      onclick={goBack}
    >
      <ArrowLeft class="h-4 w-4" />
      {m.collection_item_back()}
    </button>
  </div>
{:else if error}
  <div class="flex h-64 flex-col items-center justify-center gap-4 text-center">
    <p class="text-lg font-semibold text-destructive">{error}</p>
    <button
      type="button"
      class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
      onclick={goBack}
    >
      <ArrowLeft class="h-4 w-4" />
      {m.collection_item_back()}
    </button>
  </div>
{:else if collectionItem}
  <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
    <!-- Back button -->
    <button
      type="button"
      class="mb-6 inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
      onclick={goBack}
    >
      <ArrowLeft class="h-4 w-4" />
      {m.collection_item_back()}
    </button>

    <!-- Two-panel layout -->
    <div class="flex flex-col gap-6 lg:flex-row">
      <!-- Left panel: Railway model card -->
      <div class="min-w-0 flex-1">
        {#if displayModel}
          <RailwayModelCard model={displayModel} editable={true} />
        {/if}
      </div>

      <!-- Right panel: Sidebar -->
      <CollectionItemSidebar item={collectionItem} {seller} />
    </div>
  </div>
{/if}
