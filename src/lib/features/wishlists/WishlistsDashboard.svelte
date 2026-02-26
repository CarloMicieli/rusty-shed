<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Sparkles, Heart } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getWishlistContext } from './WishlistState.svelte';
  import { Button } from '$lib/components';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import WishlistSidebar from './components/WishlistSidebar.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';
  import AddRailwayModelDrawer from './components/AddRailwayModelDrawer.svelte';
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

<div class="flex h-screen flex-col overflow-hidden bg-background">
  <!-- Page Header -->
  <div class="flex-shrink-0 border-b border-border px-6 py-4">
    <PageHeader
      title={m.wishlists_title()}
      subtitle={m.wishlists_subtitle()}
      description={m.wishlists_description()}
    >
      {#snippet actions()}
        <Button onclick={handleCreate} size="sm">
          <Sparkles size={18} />
          {m.wishlists_create_button()}
        </Button>
      {/snippet}
    </PageHeader>
  </div>

  <div class="flex flex-1 overflow-hidden">
    <!-- List Navigator (Left Column) -->
    <aside class="w-80 flex-shrink-0 overflow-y-auto border-r border-white/10 bg-[#0c0c0c]">
      <div class="p-4">
        <WishlistSidebar
          {wishlists}
          activeId={activeWishlistId}
          onSelect={handleSelect}
          onDelete={_handleDelete}
        />
      </div>
    </aside>

    <!-- Wishlist Content Area (Right Column) -->
    <main class="flex-1 overflow-y-auto bg-background">
      <div class="p-6">
        {#if wishlistService.isLoading}
          <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each Array.from({ length: 8 }) as _, i (i)}
              <div
                class="aspect-[3/4] animate-pulse space-y-4 overflow-hidden rounded-2xl border-2 border-white/5 bg-[#0c0c0c] p-4"
              >
                <div class="h-2 w-1/2 rounded bg-white/5"></div>
                <div class="h-4 w-3/4 rounded bg-white/5"></div>
                <div class="aspect-[4/3] rounded-xl bg-white/5"></div>
                <div class="flex gap-2">
                  <div class="h-8 flex-1 rounded-lg bg-white/5"></div>
                  <div class="h-8 flex-1 rounded-lg bg-white/5"></div>
                </div>
              </div>
            {/each}
          </div>
        {:else if activeWishlist}
          <div class="space-y-6">
            <WishlistHeader
              wishlist={activeWishlist}
              onRename={handleRename}
              onSetDefault={handleSetDefault}
              onAddModel={openAddModelDrawer}
              onDelete={_handleDelete}
            />

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
          </div>
        {:else}
          <div class="flex h-[60vh] flex-col items-center justify-center space-y-6 text-center">
            <div class="relative">
              <div
                class="absolute inset-0 -m-4 animate-pulse rounded-full bg-amber-500/10 blur-2xl"
              ></div>
              <Heart size={64} class="relative text-zinc-800" />
            </div>
            <div class="space-y-2">
              <h2 class="text-3xl font-bold tracking-tight text-white">Rusty Shed Wishlists</h2>
              <p class="mx-auto max-w-xs text-zinc-500">
                Select a wishlist from the left or create a new one to start building your dream
                collection.
              </p>
            </div>
            <Button variant="outline" class="border-zinc-800 text-zinc-400" onclick={handleCreate}>
              Create your first list
            </Button>
          </div>
        {/if}
      </div>
    </main>
  </div>
</div>

<!-- Add Railway Model Drawer -->
<AddRailwayModelDrawer
  open={showAddModelDrawer}
  preselectedWishlistId={activeWishlistId}
  {wishlists}
  onClose={closeAddModelDrawer}
  onSuccess={handleAddModelSuccess}
/>

{#if purchaseDialogOpen && purchaseDialogItem && activeWishlistId}
  <PurchaseDialog
    open={purchaseDialogOpen}
    wishlistId={activeWishlistId}
    wishlistItemId={purchaseDialogItem.id}
    itemName={purchaseDialogItem.railwayModelId}
    onClose={handlePurchaseClose}
    onSuccess={handlePurchaseSuccess}
  />
{/if}
