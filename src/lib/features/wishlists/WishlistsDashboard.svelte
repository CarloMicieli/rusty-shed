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
    class="-mx-4 -mt-4 rounded-tl-[24px] border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8"
  >
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

  <div class="-mx-4 flex flex-1 flex-col md:flex-row lg:-mx-8">
    <!-- List Navigator (Left Column) -->
    {#if wishlists.length > 0 || wishlistService.isLoading}
      <aside class="flex-shrink-0 border-r border-border bg-card md:w-80">
        <div class="sticky top-4 p-4">
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
                class="aspect-[3/4] animate-pulse space-y-4 overflow-hidden rounded-2xl border-2 border-border/50 bg-card p-4"
              >
                <div class="h-2 w-1/2 rounded bg-muted"></div>
                <div class="h-4 w-3/4 rounded bg-muted"></div>
                <div class="aspect-[4/3] rounded-xl bg-muted"></div>
                <div class="flex gap-2">
                  <div class="h-8 flex-1 rounded-lg bg-muted"></div>
                  <div class="h-8 flex-1 rounded-lg bg-muted"></div>
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

            <WishlistValueBar items={wishlistItems} />

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
          <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
            <div
              class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 px-4 py-24 text-center"
            >
              <div class="relative">
                <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
                <div
                  class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
                >
                  <Heart size={56} class="text-zinc-600 opacity-50" />
                </div>
              </div>

              <div class="flex max-w-sm flex-col items-center gap-3 text-center">
                <h3 class="text-2xl font-bold text-zinc-200">
                  {m.app_wishlists()}
                </h3>
                <p class="text-sm leading-relaxed text-zinc-500">
                  {m.wishlists_empty_state()}
                </p>
              </div>

              <button
                type="button"
                class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
                onclick={handleCreate}
              >
                <div
                  class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
                ></div>
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
    onClose={handlePurchaseClose}
    onSuccess={handlePurchaseSuccess}
  />
{/if}
