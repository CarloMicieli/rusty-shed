<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Sparkles } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getWishlistContext } from './WishlistState.svelte';
  import { Button } from '$lib/components';
  import WishlistSidebar from './components/WishlistSidebar.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';
  import AddRailwayModelDrawer from './components/AddRailwayModelDrawer.svelte';

  const wishlistService = getWishlistContext();

  const wishlists = $derived(wishlistService.wishlists);
  const activeWishlist = $derived(wishlistService.activeWishlist);
  const activeWishlistId = $derived(wishlistService.activeWishlistId);
  const wishlistItems = $derived(wishlistService.wishlistItems);
  const otherTargets = $derived(wishlists.filter((w) => w.id !== activeWishlistId));

  // Drawer state
  let showAddModelDrawer = $state(false);

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

<div class="space-y-6">
  <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
    <div>
      <p class="text-sm tracking-[0.2em] text-muted-foreground uppercase">{m.app_wishlists()}</p>
      <h1 class="h2 font-bold">{m.wishlists_title()}</h1>
      <p class="text-sm text-muted-foreground">{m.wishlists_subtitle()}</p>
    </div>
    <div class="flex flex-col gap-3 md:flex-row md:items-center">
      <Button onclick={handleCreate}>
        <Sparkles size={18} />
        {m.wishlists_create_button()}
      </Button>
    </div>
  </div>

  <div class="grid gap-6 lg:grid-cols-[320px,1fr]">
    <WishlistSidebar
      {wishlists}
      activeId={activeWishlistId}
      onSelect={handleSelect}
      onDelete={_handleDelete}
    />

    <section class="space-y-4 rounded-2xl border border-border bg-card p-6">
      <WishlistHeader
        wishlist={activeWishlist}
        onRename={handleRename}
        onSetDefault={handleSetDefault}
        onAddModel={openAddModelDrawer}
      />

      <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <WishlistItems
          items={wishlistItems}
          {activeWishlistId}
          {otherTargets}
          onRemove={handleRemove}
          onMove={handleMove}
        />
      </div>
    </section>
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
