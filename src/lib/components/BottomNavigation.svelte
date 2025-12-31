<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { LayoutDashboard, Library, Heart, Box, Settings } from 'lucide-svelte';
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { collectionStore } from '$lib/stores/collectionStore.svelte';
  import { wishlistService } from '$lib/stores/WishlistService.svelte';

  const totalCount = $derived(collectionStore.totalCount);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const pathname = $derived(page.url.pathname as string);
</script>

<div
  class="pb-safe-area fixed right-0 bottom-0 left-0 z-50 border-t border-surface-700/50 bg-surface-900 lg:hidden"
>
  <div class="flex h-16 items-center justify-around">
    <a
      href={resolve('/my-dashboard')}
      class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
      class:text-accent-500={pathname === '/my-dashboard'}
      class:text-surface-400={pathname !== '/my-dashboard'}
    >
      <LayoutDashboard size={20} />
      <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_dashboard()}</span>
    </a>
    <a
      href={resolve('/my-collection')}
      class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
      class:text-accent-500={pathname === '/my-collection'}
      class:text-surface-400={pathname !== '/my-collection'}
    >
      <Library size={20} />
      <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_collection()}</span>
      <span class="variant-soft-surface badge">{totalCount}</span>
    </a>
    <a
      href={resolve('/my-wishlists')}
      class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
      class:text-accent-500={pathname === '/my-wishlists'}
      class:text-surface-400={pathname !== '/my-wishlists'}
    >
      <Heart size={20} />
      <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_wishlists()}</span>
      {#if defaultWishlist}
        <span class="variant-soft-surface badge">{defaultWishlist.count}</span>
      {/if}
    </a>
    <a
      href={resolve('/my-depot')}
      class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
      class:text-accent-500={pathname === '/my-depot'}
      class:text-surface-400={pathname !== '/my-depot'}
    >
      <Box size={20} />
      <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_depot()}</span>
    </a>
    <a
      href="/settings"
      class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
      class:text-accent-500={pathname === '/settings'}
      class:text-surface-400={pathname !== '/settings'}
    >
      <Settings size={20} />
      <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_settings()}</span>
    </a>
  </div>
</div>
