<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { LayoutDashboard, Library, Heart, Wallet, Settings } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { localeStore } from '$lib/stores/locale';

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const pathname = $derived($page.url.pathname as string);
  const locale = $derived($localeStore);
</script>

<div
  class="pb-safe-area border-surface-700/50 bg-surface-900 fixed right-0 bottom-0 left-0 z-50 border-t lg:hidden"
>
  {#key locale}
    <div class="flex h-16 items-center justify-around">
      <a
        href={resolve('/dashboard')}
        class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
        class:text-accent-500={pathname === '/dashboard'}
        class:text-surface-400={pathname !== '/dashboard'}
      >
        <LayoutDashboard size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_dashboard()}</span>
      </a>
      <a
        href={resolve('/collection')}
        class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
        class:text-accent-500={pathname === '/collection'}
        class:text-surface-400={pathname !== '/collection'}
      >
        <Library size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_collection()}</span>
      </a>
      <a
        href={resolve('/wishlists')}
        class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
        class:text-accent-500={pathname === '/wishlists'}
        class:text-surface-400={pathname !== '/wishlists'}
      >
        <Heart size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_wishlists()}</span>
        {#if defaultWishlist}
          <Badge variant="outline">{defaultWishlist.count}</Badge>
        {/if}
      </a>
      <a
        href={resolve('/finance')}
        class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
        class:text-accent-500={pathname === '/finance'}
        class:text-surface-400={pathname !== '/finance'}
      >
        <Wallet size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.budget_title()}</span>
      </a>
      <a
        href={resolve('/settings')}
        class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
        class:text-accent-500={pathname === '/settings'}
        class:text-surface-400={pathname !== '/settings'}
      >
        <Settings size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_settings()}</span>
      </a>
    </div>
  {/key}
</div>
