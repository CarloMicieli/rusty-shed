<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { getWishlistContext } from './WishlistState.svelte';
  import WishlistSidebar from './components/WishlistSidebar.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';

  const wishlistService = getWishlistContext();

  const wishlists = $derived(wishlistService.wishlists);
  const activeWishlist = $derived(wishlistService.activeWishlist);
  const activeWishlistId = $derived(wishlistService.activeWishlistId);
  const wishlistItems = $derived(wishlistService.wishlistItems);
  const otherTargets = $derived(wishlists.filter((w) => w.id !== activeWishlistId));

  onMount(() => {
    void wishlistService.fetchWishlists();
  });

  function handleCreate() {
    void wishlistService.createWishlist('Create New List');
  }

  function handleSelect(id: string) {
    void wishlistService.selectWishlist(id);
  }

  function _handleDelete(id: string) {
    void wishlistService.deleteWishlist(id);
  }

  function handleRename(name: string) {
    if (!activeWishlist) return;
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
</script>

<svelte:head>
  <title>{m.app_wishlists()}</title>
</svelte:head>

<div class="grid gap-6 lg:grid-cols-[320px,1fr]">
  <WishlistSidebar
    {wishlists}
    activeId={activeWishlistId}
    onCreate={handleCreate}
    onSelect={handleSelect}
  />

  <section class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-6">
    <WishlistHeader
      wishlist={activeWishlist}
      onRename={handleRename}
      onSetDefault={handleSetDefault}
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
