<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Sparkles, Heart, LayoutGrid, Table } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getWishlistContext } from './WishlistState.svelte';
  import { Button } from '$lib/components';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import WishlistSidebar from './components/WishlistSidebar.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';
  import WishlistTableView from './components/WishlistTableView.svelte';
  import WishlistValueBar from './components/WishlistValueBar.svelte';
  import AddWishlistItemDrawer from './AddWishlistItemDrawer.svelte';
  import PurchaseDialog from './components/PurchaseDialog.svelte';
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
  <div class="-mx-4 -mt-4 border-b border-[#1F1F1F] bg-[#0F0F0F] px-6 py-4 lg:-mx-8 lg:-mt-8">
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

  <div class="-mx-4 flex min-h-screen flex-col md:flex-row lg:-mx-8">
    <!-- List Navigator (Full-Height Left Column) -->
    {#if wishlists.length > 0 || wishlistService.isLoading}
      <aside
        class="w-full shrink-0 border-b border-[#1F1F1F] bg-[#0F0F0F] md:w-64 md:border-r md:border-b-0"
      >
        <div class="sticky top-0 p-3 pt-4">
          <WishlistSidebar
            {wishlists}
            activeId={activeWishlistId}
            onSelect={handleSelect}
            onDelete={_handleDelete}
          />
        </div>
      </aside>
    {/if}

    <!-- Wishlist Content Area (Right Column) -->
    <div class="flex-1 bg-background">
      <div class="p-6">
        {#if wishlistService.isLoading}
          <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each Array.from({ length: 8 }) as _, i (i)}
              <div
                class="aspect-[3/4] animate-pulse space-y-4 overflow-hidden rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] p-4"
              >
                <div class="h-2 w-1/2 rounded bg-[#1F1F1F]"></div>
                <div class="h-4 w-3/4 rounded bg-[#1F1F1F]"></div>
                <div class="aspect-[4/3] rounded-[8px] bg-[#1F1F1F]"></div>
                <div class="flex gap-2">
                  <div class="h-8 flex-1 rounded-[8px] bg-[#1F1F1F]"></div>
                  <div class="h-8 flex-1 rounded-[8px] bg-[#1F1F1F]"></div>
                </div>
              </div>
            {/each}
          </div>
        {:else if activeWishlist}
          <div class="space-y-4">
            <WishlistHeader
              wishlist={activeWishlist}
              onRename={handleRename}
              onSetDefault={handleSetDefault}
              onAddModel={openAddModelDrawer}
              onDelete={_handleDelete}
            />

            <WishlistValueBar items={wishlistItems} />

            <!-- View Mode Toggle -->
            <div class="flex items-center justify-end gap-1">
              <button
                type="button"
                title={m.wishlist_view_grid()}
                class={[
                  'rounded-[8px] border p-1.5 transition-colors',
                  viewMode === 'grid'
                    ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                    : 'border-[#1F1F1F] text-[#808080] hover:border-[#D48A42]/40 hover:text-[#D48A42]'
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
                    ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                    : 'border-[#1F1F1F] text-[#808080] hover:border-[#D48A42]/40 hover:text-[#D48A42]'
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
        {:else}
          <div class="rounded-[8px] border border-[#1F1F1F] bg-[#0F0F0F] p-4">
            <div
              class="flex flex-col items-center justify-center gap-8 rounded-[8px] border border-[#1F1F1F] px-4 py-24 text-center"
            >
              <div class="relative">
                <div class="absolute inset-0 rounded-full bg-[#D48A42]/5 blur-3xl"></div>
                <div
                  class="relative flex h-32 w-32 items-center justify-center rounded-full border border-[#1F1F1F] bg-[#0F0F0F]"
                >
                  <Heart size={56} class="text-[#808080] opacity-40" />
                </div>
              </div>

              <div class="flex max-w-sm flex-col items-center gap-3 text-center">
                <h3 class="text-2xl font-bold text-[#E0E0E0]">
                  {m.app_wishlists()}
                </h3>
                <p class="text-sm leading-relaxed text-[#808080]">
                  {m.wishlists_empty_state()}
                </p>
              </div>

              <button
                type="button"
                class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-[8px] border border-[#D48A42] bg-[#D48A42] px-8 py-3 font-bold tracking-wide text-black transition-all hover:bg-[#D48A42]/90 hover:shadow-[0_0_20px_rgba(212,138,66,0.3)] active:scale-95"
                onclick={handleCreate}
              >
                <Heart class="h-5 w-5" />
                <span>{m.wishlists_create_button()}</span>
              </button>
            </div>
          </div>
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
