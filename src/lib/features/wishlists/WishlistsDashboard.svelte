<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { wishlistStore } from '$lib/stores/wishlistStore';
  import WishlistSidebar from './components/WishlistSidebar.svelte';
  import WishlistHeader from './components/WishlistHeader.svelte';
  import WishlistItems from './components/WishlistItems.svelte';

  const wishlistsStore = wishlistStore.wishlists;
  const activeWishlistStore = wishlistStore.activeWishlist;
  const activeWishlistIdStore = wishlistStore.activeWishlistId;
  const wishlistItemsStore = wishlistStore.wishlistItems;

  const wishlists = $derived($wishlistsStore);
  const activeWishlist = $derived($activeWishlistStore);
  const activeWishlistId = $derived($activeWishlistIdStore);
  const wishlistItems = $derived($wishlistItemsStore);
  const otherTargets = $derived(wishlists.filter((w) => w.id !== activeWishlistId));

  if (typeof window !== 'undefined') {
    $effect(() => {
      void wishlistStore.fetchWishlists();
    });
  }

  function handleCreate() {
    void wishlistStore.createWishlist('Create New List');
  }

  function handleSelect(id: string) {
    void wishlistStore.selectWishlist(id);
  }

  function handleDelete(id: string) {
    void wishlistStore.deleteWishlist(id);
  }

  function handleRename(name: string) {
    if (!activeWishlist) return;
    void wishlistStore.renameWishlist(activeWishlist.id, name);
  }

  function handleSetDefault() {
    if (!activeWishlist) return;
    void wishlistStore.setDefaultWishlist(activeWishlist.id);
  }

  function handleRemove(detail: { itemId: string; wishlistId: string }) {
    const { itemId, wishlistId } = detail;
    void wishlistStore.removeItem(wishlistId, itemId);
  }

  function handleMove(detail: { itemId: string; fromId: string; toId: string }) {
    const { itemId, fromId, toId } = detail;
    void wishlistStore.moveItemToList(itemId, fromId, toId);
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
    onDelete={handleDelete}
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
