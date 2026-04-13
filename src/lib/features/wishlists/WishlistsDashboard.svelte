<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Sparkles, Heart, LayoutGrid, Table, PackagePlus } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getWishlistContext } from './WishlistState.svelte';
  import { Button } from '$lib/components';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import WishlistDashboardHeader from './components/WishlistDashboardHeader.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';
  import WishlistTableView from './components/WishlistTableView.svelte';
  import AddWishlistItemDrawer from './AddWishlistItemDrawer.svelte';
  import PurchaseDialog from './components/PurchaseDialog.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import type { WishlistItem } from '$lib/bindings';

  const wishlistService = getWishlistContext();

  const wishlists = $derived(wishlistService.wishlists);
  const activeWishlist = $derived(wishlistService.activeWishlist);
  const activeWishlistId = $derived(wishlistService.activeWishlistId);
  const wishlistItems = $derived(wishlistService.wishlistItems);
  const otherTargets = $derived(wishlists.filter((w) => w.id !== activeWishlistId));

  // Drawer state
  let showAddModelDrawer = $state(false);

  // View toggle state
  let viewMode = $state<'grid' | 'table'>('grid');

  // Purchase dialog state
  let purchaseDialogOpen = $state(false);
  let purchaseDialogItem = $state<WishlistItem | null>(null);

  onMount(() => {
    void wishlistService.fetchWishlists();
  });

  /**
   * Generate the next available wishlist name following the pattern:
   * "My Wish List (1)", "My Wish List (2)", etc.
   * The base name is localized (e.g., "La Mia Lista dei Desideri" in Italian).
   */
  function getNextWishlistName(): string {
    const baseName = m.wishlists_default_name();
    const pattern = new RegExp(`^${baseName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')} \\((\\d+)\\)$`);

    // Find all existing wishlist names that match the pattern
    const numbers: number[] = [];
    for (const wishlist of wishlists) {
      const match = wishlist.name.match(pattern);
      if (match && match[1]) {
        numbers.push(parseInt(match[1], 10));
      }
    }

    // Find the next available number
    const nextNumber = numbers.length > 0 ? Math.max(...numbers) + 1 : 1;
    return `${baseName} (${nextNumber})`;
  }

  function handleCreate() {
    const name = getNextWishlistName();
    void wishlistService.createWishlist(name);
  }

  function handleSelect(id: string) {
    void wishlistService.selectWishlist(id);
  }

  function _handleDelete(id: string) {
    console.log('Wishlist delete clicked', id);
    void wishlistService.deleteWishlist(id);
  }

  function handleRename(name: string) {
    if (!activeWishlist) return;
    console.log('WishlistsDashboard: handleRename', { activeId: activeWishlist?.id, name });
    void wishlistService.renameWishlist(activeWishlist.id, name);
  }

  function handleSetDefault() {
    if (!activeWishlist) return;
    void wishlistService.setDefaultWishlist(activeWishlist.id);
  }

  function handleRemove(detail: { itemId: string; wishlistId: string }) {
    const { itemId, wishlistId } = detail;
    void wishlistService.removeItem(wishlistId, itemId);
  }

  function handleMove(detail: { itemId: string; fromId: string; toId: string }) {
    const { itemId, fromId, toId } = detail;
    void wishlistService.moveItemToList(itemId, fromId, toId);
  }

  function handlePurchaseTrigger(itemId: string) {
    const item = wishlistItems.find((i) => i.id === itemId) ?? null;
    if (!item) return;
    purchaseDialogItem = item;
    purchaseDialogOpen = true;
  }

  function handlePurchaseClose() {
    purchaseDialogOpen = false;
    purchaseDialogItem = null;
  }

  async function handlePurchaseSuccess() {
    handlePurchaseClose();
    if (activeWishlistId) {
      await wishlistService.onPurchaseSuccess(activeWishlistId);
    }
  }

  function openAddModelDrawer() {
    showAddModelDrawer = true;
  }

  function closeAddModelDrawer() {
    showAddModelDrawer = false;
  }

  function handleAddModelSuccess() {
    closeAddModelDrawer();
    // Refresh handled by WishlistState method
  }
</script>

<svelte:head>
  <title>{m.app_wishlists()}</title>
</svelte:head>

<div class="mb-10 flex flex-col">
  <!-- Page Header -->
  <div
    class="-mx-4 -mt-4 border-b border-layout-border bg-layout-surface px-6 py-4 lg:-mx-8 lg:-mt-8"
  >
    <PageHeader
      title={m.wishlists_title()}
      subtitle={m.wishlists_subtitle()}
      description={m.wishlists_description()}
    >
      {#snippet actions()}
        {#if wishlists.length > 0}
          <Button onclick={handleCreate} size="sm">
            <Sparkles size={18} />
            {m.wishlists_create_button()}
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  <div class="-mx-4 flex min-h-screen flex-col lg:-mx-8">
    <!-- Integrated Dashboard Header -->
    {#if wishlists.length > 0}
      <div class="border-b border-layout-border px-6 py-4">
        <WishlistDashboardHeader
          {wishlists}
          {activeWishlist}
          {activeWishlistId}
          items={wishlistItems}
          onSelect={handleSelect}
          onRename={handleRename}
          onSetDefault={handleSetDefault}
          onDelete={_handleDelete}
        />
      </div>
    {/if}

    <!-- Wishlist Content Area -->
    <div class="flex-1 bg-background">
      <div class="p-6">
        {#if wishlistService.isLoading}
          <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each Array.from({ length: 8 }) as _, i (i)}
              <div
                class="aspect-[3/4] animate-pulse space-y-4 overflow-hidden rounded-[8px] border border-layout-border bg-layout-surface p-4"
              >
                <div class="h-2 w-1/2 rounded bg-layout-border"></div>
                <div class="h-4 w-3/4 rounded bg-layout-border"></div>
                <div class="aspect-[4/3] rounded-[8px] bg-layout-border"></div>
                <div class="flex gap-2">
                  <div class="h-8 flex-1 rounded-[8px] bg-layout-border"></div>
                  <div class="h-8 flex-1 rounded-[8px] bg-layout-border"></div>
                </div>
              </div>
            {/each}
          </div>
        {:else if activeWishlist}
          <div class="space-y-4">
            <WishlistHeader wishlist={activeWishlist} onAddModel={openAddModelDrawer} />

            <!-- View Mode Toggle -->
            <div class="flex items-center justify-end gap-1">
              <button
                type="button"
                title={m.wishlist_view_grid()}
                class={[
                  'rounded-[8px] border p-1.5 transition-colors',
                  viewMode === 'grid'
                    ? 'border-primary bg-primary/15 text-primary'
                    : 'border-layout-border text-muted-foreground hover:border-primary/40 hover:text-primary'
                ].join(' ')}
                onclick={() => (viewMode = 'grid')}
              >
                <LayoutGrid size={16} />
              </button>
              <button
                type="button"
                title={m.wishlist_view_table()}
                class={[
                  'rounded-[8px] border p-1.5 transition-colors',
                  viewMode === 'table'
                    ? 'border-primary bg-primary/15 text-primary'
                    : 'border-layout-border text-muted-foreground hover:border-primary/40 hover:text-primary'
                ].join(' ')}
                onclick={() => (viewMode = 'table')}
              >
                <Table size={16} />
              </button>
            </div>

            <!-- Items: Grid or Table -->
            {#if viewMode === 'grid'}
              <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <WishlistItems
                  items={wishlistItems}
                  {activeWishlistId}
                  {otherTargets}
                  onRemove={handleRemove}
                  onMove={handleMove}
                  onPurchase={handlePurchaseTrigger}
                />
              </div>
            {:else}
              <WishlistTableView
                items={wishlistItems}
                {activeWishlistId}
                {otherTargets}
                onRemove={handleRemove}
                onMove={handleMove}
                onPurchase={handlePurchaseTrigger}
              />
            {/if}
          </div>
        {:else if wishlists.length > 0}
          <!-- Has wishlists but none is active-selected -->
          <EmptyState
            icon={PackagePlus}
            title={m.wishlists_no_model_title()}
            description={m.wishlists_no_model_description()}
            ctaLabel={m.wishlists_add_model_button()}
            onCta={openAddModelDrawer}
          />
        {:else}
          <!-- No wishlists at all -->
          <EmptyState
            icon={Heart}
            title={m.wishlists_empty_title()}
            description={m.wishlists_empty_state()}
            ctaLabel={m.wishlists_create_button()}
            onCta={handleCreate}
          />
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Add Railway Model Drawer -->
<AddWishlistItemDrawer
  open={showAddModelDrawer}
  preselectedWishlistId={activeWishlistId}
  onClose={closeAddModelDrawer}
  onSaved={handleAddModelSuccess}
/>

{#if purchaseDialogOpen && purchaseDialogItem && activeWishlistId}
  <PurchaseDialog
    open={purchaseDialogOpen}
    wishlistId={activeWishlistId}
    wishlistItemId={purchaseDialogItem.id}
    itemName={purchaseDialogItem.railwayModelId}
    initialPriceAmount={purchaseDialogItem.desiredPrice
      ? Number(purchaseDialogItem.desiredPrice.amount)
      : null}
    onClose={handlePurchaseClose}
    onSuccess={handlePurchaseSuccess}
  />
{/if}
