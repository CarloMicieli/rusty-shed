<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { Ellipsis } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components/ui/badge';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { PRIMARY_ITEMS, SECONDARY_ITEMS } from './navigation/config';
  import { isActive, isMoreButtonActive } from './navigation/utils';
  import MoreMenu from './navigation/MoreMenu.svelte';

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const pathname = $derived($page.url.pathname as string);

  let moreMenuOpen = $state(false);

  function toggleMoreMenu() {
    moreMenuOpen = !moreMenuOpen;
  }

  function closeMoreMenu() {
    moreMenuOpen = false;
  }

  // Helper function to get inactive text color
  function getInactiveClass(active: boolean) {
    return active ? 'text-primary' : 'text-muted-foreground';
  }
</script>

<div
  class="pb-safe-area fixed right-0 bottom-0 left-0 z-50 border-t-2 border-primary bg-card lg:hidden"
  style="box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.5);"
>
  <div class="flex h-16 items-center justify-around px-1">
    {#each PRIMARY_ITEMS as item (item.id)}
      <a
        href={resolve(item.href as '/dashboard')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          isActive(item, pathname)
        )}"
        aria-current={isActive(item, pathname) ? 'page' : undefined}
      >
        {#if isActive(item, pathname)}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <item.icon size={20} />
        <span
          class="max-w-[5.5rem] truncate text-[10px] leading-tight font-bold tracking-wider uppercase"
        >
          {item.label()}
        </span>
        {#if item.id === 'wishlists' && defaultWishlist}
          <Badge variant="outline" class="text-[9px]">{defaultWishlist.count}</Badge>
        {/if}
      </a>
    {/each}

    <!-- More button as 5th slot -->
    <button
      onclick={toggleMoreMenu}
      class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
        isMoreButtonActive(SECONDARY_ITEMS, pathname)
      )}"
      aria-label={m.app_more_aria()}
      aria-expanded={moreMenuOpen}
    >
      {#if isMoreButtonActive(SECONDARY_ITEMS, pathname)}
        <div
          class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
          style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
        ></div>
      {/if}
      <Ellipsis size={20} />
      <span
        class="max-w-[5.5rem] truncate text-[10px] leading-tight font-bold tracking-wider uppercase"
      >
        {m.app_more()}
      </span>
    </button>
  </div>

  <!-- More Menu Bottom Sheet -->
  <MoreMenu open={moreMenuOpen} onClose={closeMoreMenu} items={SECONDARY_ITEMS} />
</div>
