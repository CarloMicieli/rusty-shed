<script lang="ts">
  import { onMount } from 'svelte';
  import { Heart, Star, Sparkles } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { wishlistStore } from '$lib/stores/wishlistStore';
  import { resolveTagIcon } from '$lib/config/icons';

  const {
    wishlists,
    wishlistItems,
    activeWishlist,
    activeWishlistId,
    selectWishlist,
    createWishlist,
    renameWishlist,
    deleteWishlist,
    setDefaultWishlist,
    removeItem
  } = wishlistStore;

  let isEditing = false;
  let nameDraft = '';

  $: if ($activeWishlist && !isEditing) {
    nameDraft = $activeWishlist.name;
  }

  onMount(() => {
    void wishlistStore.fetchWishlists();
  });

  function handleSelect(id: string) {
    void selectWishlist(id);
  }

  async function handleRenameBlur() {
    if (!$activeWishlist) return;
    if (nameDraft.trim() && nameDraft !== $activeWishlist.name) {
      await renameWishlist($activeWishlist.id, nameDraft.trim());
    } else {
      nameDraft = $activeWishlist.name;
    }
    isEditing = false;
  }

  function handleRenameKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      void handleRenameBlur();
    }
    if (event.key === 'Escape') {
      isEditing = false;
      if ($activeWishlist) nameDraft = $activeWishlist.name;
    }
  }

  async function handleSetDefault() {
    if ($activeWishlist) {
      await setDefaultWishlist($activeWishlist.id);
    }
  }

  async function handleDeleteList(id: string) {
    await deleteWishlist(id);
  }

  function iconForItem() {
    // Wishlist items currently lack tags; fall back to shared heart key.
    return resolveTagIcon('heart');
  }
</script>

<svelte:head>
  <title>{m.app_wishlists()}</title>
</svelte:head>

<div class="grid gap-6 lg:grid-cols-[320px,1fr]">
  <aside class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-4">
    <div class="flex items-center justify-between">
      <h2 class="h5 font-semibold tracking-tight">{m.app_wishlists()}</h2>
      <button
        class="variant-soft-primary btn btn-sm"
        on:click={() => void createWishlist('Create New List')}
      >
        <Sparkles size={16} />
        <span>Create New List</span>
      </button>
    </div>

    <div class="space-y-2">
      {#if $wishlists.length === 0}
        <p class="text-sm text-surface-400">Wishlist is empty</p>
      {:else}
        {#each $wishlists as wl (wl.id)}
          <div
            role="button"
            tabindex="0"
            class="btn w-full justify-between gap-3 text-left"
            class:variant-filled-primary={wl.id === $activeWishlistId}
            class:variant-ghost-surface={wl.id !== $activeWishlistId}
            on:click={() => handleSelect(wl.id)}
            on:keydown={(e) => e.key === 'Enter' && handleSelect(wl.id)}
          >
            <div class="flex items-center gap-2">
              <Heart size={16} />
              <div class="flex flex-col">
                <span class="font-semibold">{wl.name}</span>
                <span class="text-xs text-surface-400">{wl.count} items</span>
              </div>
            </div>
            <div class="flex items-center gap-2">
              {#if wl.is_default}
                <span class="variant-soft-primary badge text-[10px] uppercase">Default</span>
              {/if}
              <button
                class="btn-icon btn btn-icon-sm"
                type="button"
                on:click|stopPropagation={() => handleDeleteList(wl.id)}
              >
                ✕
              </button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </aside>

  <section class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-6">
    {#if $activeWishlist}
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex items-center gap-2">
          <Heart class="text-accent-500" size={20} />
          {#if isEditing}
            <input
              class="input-lg variant-ghost-surface input"
              bind:value={nameDraft}
              on:blur={handleRenameBlur}
              on:keydown={handleRenameKeydown}
            />
          {:else}
            <button
              type="button"
              class="h4 font-bold text-left"
              on:click={() => (isEditing = true)}
              on:keydown={(e) => e.key === 'Enter' && (isEditing = true)}
            >
              {nameDraft}
            </button>
          {/if}
        </div>
        <div class="flex items-center gap-2">
          <button
            class="variant-soft-primary btn"
            class:variant-filled-primary={$activeWishlist.is_default}
            on:click={handleSetDefault}
          >
            <Star size={16} />
            <span>Set as Default</span>
          </button>
        </div>
      </div>

      <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {#if $wishlistItems.length === 0}
          <div
            class="col-span-full rounded-xl border border-dashed border-surface-700/60 p-6 text-center text-surface-400"
          >
            Wishlist is empty
          </div>
        {:else}
          {#each $wishlistItems as item (item.id)}
            <div class="rounded-xl border border-surface-700/50 bg-surface-800 p-4 shadow-sm">
              <div class="mb-3 flex items-center gap-2">
                {#if iconForItem()}
                  <svelte:component this={iconForItem()} size={16} class="text-accent-400" />
                {:else}
                  <Heart size={16} class="text-accent-400" />
                {/if}
                <span class="text-sm font-semibold"
                  >{item.railway_model_id as unknown as string}</span
                >
              </div>
              <div class="flex items-center justify-between text-xs text-surface-400">
                <span>{item.status}</span>
                <span>{item.priority}</span>
              </div>
              <div class="mt-3 flex items-center gap-2">
                <button
                  class="variant-ghost-surface btn btn-sm"
                  on:click={() => void removeItem($activeWishlist.id, item.id)}
                >
                  Delete
                </button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    {:else}
      <div
        class="rounded-xl border border-dashed border-surface-700/60 p-8 text-center text-surface-400"
      >
        Wishlist is empty
      </div>
    {/if}
  </section>
</div>
