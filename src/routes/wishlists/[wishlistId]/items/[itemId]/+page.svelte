<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { commands } from '$lib/bindings';
  import { ArrowLeft, ShoppingCart } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import WishlistItemSidebar from '$lib/features/wishlists/components/WishlistItemSidebar.svelte';
  import PurchaseDialog from '$lib/features/wishlists/components/PurchaseDialog.svelte';
  import { Button } from '$lib/components';
  import type {
    RailwayModelView,
    RailwayModelImageResponse,
    WishlistItem,
    WishlistItemView
  } from '$lib/bindings';

  const wishlistId = $page.params.wishlistId as string;
  const itemId = $page.params.itemId as string;

  let wishlistName = $state('');
  let wishlistItem = $state<WishlistItem | null>(null);
  let model = $state<RailwayModelView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let notFound = $state(false);
  let purchaseDialogOpen = $state(false);

  const displayModel = $derived(model ? toRailwayModel(model, null, imageResponse) : null);

  async function loadData() {
    const wishlistResult = await commands.getWishlistById(wishlistId);
    if (wishlistResult.status !== 'ok' || !wishlistResult.data) {
      notFound = true;
      return;
    }

    const wishlistView = wishlistResult.data;
    wishlistName = wishlistView.name;
    const foundView = wishlistView.items?.find((i) => i.id === itemId) ?? null;

    if (!foundView) {
      notFound = true;
      return;
    }

    notFound = false;
    wishlistItem = normalizeItem(foundView);

    const [modelResult, imageResult] = await Promise.all([
      commands.getRailwayModelById(wishlistItem.railwayModelId, getLocale()),
      commands.getRailwayModelImage(wishlistItem.railwayModelId)
    ]);

    if (modelResult.status === 'ok') model = modelResult.data;
    if (imageResult.status === 'ok') imageResponse = imageResult.data;
  }

  async function handleModelUpdated() {
    try {
      await loadData();
    } catch (e) {
      error = e instanceof Error ? e.message : m.wishlist_item_error();
    }
  }

  function goBack() {
    goto('/wishlists');
  }

  function normalizeItem(v: WishlistItemView): WishlistItem {
    return {
      id: v.id,
      railwayModelId: v.railway_model_id,
      priority: v.priority,
      status: v.status,
      addedDate: v.added_date,
      removedDate: v.removed_date,
      notes: v.notes,
      desiredPrice: v.desired_price,
      purchasedPrice: v.purchased_price
    };
  }

  async function reloadItem() {
    await loadData();
  }

  function handlePurchaseSuccess() {
    purchaseDialogOpen = false;
    void reloadItem();
  }

  onMount(async () => {
    try {
      await loadData();
    } catch (e) {
      error = e instanceof Error ? e.message : m.wishlist_item_error();
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>
    {wishlistItem ? `${wishlistName} — ${m.app_wishlists()}` : m.app_wishlists()}
  </title>
</svelte:head>

{#if loading}
  <div class="flex h-64 items-center justify-center">
    <div class="flex flex-col items-center gap-3">
      <div
        class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent"
      ></div>
      <p class="text-sm text-muted-foreground">{m.wishlist_item_loading()}</p>
    </div>
  </div>
{:else if notFound}
  <div class="flex h-64 flex-col items-center justify-center gap-4 text-center">
    <p class="text-lg font-semibold text-destructive">{m.wishlist_item_not_found()}</p>
    <p class="text-sm text-muted-foreground">{m.wishlist_item_not_found_message()}</p>
    <Button variant="ghost" size="sm" onclick={goBack}>
      <ArrowLeft class="h-4 w-4" />
      {m.wishlist_item_back()}
    </Button>
  </div>
{:else if error}
  <div class="flex h-64 flex-col items-center justify-center gap-4 text-center">
    <p class="text-lg font-semibold text-destructive">{error}</p>
    <Button variant="ghost" size="sm" onclick={goBack}>
      <ArrowLeft class="h-4 w-4" />
      {m.wishlist_item_back()}
    </Button>
  </div>
{:else if wishlistItem}
  <div class="mx-auto max-w-7xl">
    <!-- Back button -->
    <Button variant="ghost" size="sm" class="mb-6" onclick={goBack}>
      <ArrowLeft class="h-4 w-4" />
      {m.wishlist_item_back()}
    </Button>

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
      <div class="flex w-full shrink-0 flex-col gap-4 lg:w-80">
        <WishlistItemSidebar item={wishlistItem} {wishlistName} />
        {#if wishlistItem.status === 'WANTED' || wishlistItem.status === 'ON_ORDER'}
          <Button variant="default" class="w-full" onclick={() => (purchaseDialogOpen = true)}>
            <ShoppingCart class="h-4 w-4" />
            {m.purchase_dialog_submit()}
          </Button>
        {/if}
      </div>
    </div>
  </div>

  {#if purchaseDialogOpen}
    <PurchaseDialog
      open={purchaseDialogOpen}
      {wishlistId}
      wishlistItemId={itemId}
      itemName={model?.description ?? itemId}
      onClose={() => (purchaseDialogOpen = false)}
      onSuccess={handlePurchaseSuccess}
    />
  {/if}
{/if}
