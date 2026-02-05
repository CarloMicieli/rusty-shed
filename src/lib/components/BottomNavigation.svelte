<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import {
    LayoutDashboard,
    Library,
    Heart,
    Wallet,
    Box,
    Settings,
    Train,
    Cpu
  } from 'lucide-svelte';
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

  // Helper function to get inactive text color
  function getInactiveClass(isActive: boolean) {
    return isActive ? 'text-primary' : 'text-muted-foreground';
  }
</script>

<div
  class="pb-safe-area fixed right-0 bottom-0 left-0 z-50 border-t-2 border-primary bg-card lg:hidden"
  style="box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.5);"
>
  {#key locale}
    <div class="flex h-16 items-center justify-around">
      <a
        href={resolve('/my-dashboard')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-dashboard'
        )}"
      >
        {#if pathname === '/my-dashboard'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <LayoutDashboard size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_dashboard()}</span>
      </a>
      <a
        href={resolve('/my-collection')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-collection'
        )}"
      >
        {#if pathname === '/my-collection'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Library size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_collection()}</span>
      </a>
      <a
        href={resolve('/my-wishlists')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-wishlists'
        )}"
      >
        {#if pathname === '/my-wishlists'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Heart size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_wishlists()}</span>
        {#if defaultWishlist}
          <Badge variant="outline" class="text-[9px]">{defaultWishlist.count}</Badge>
        {/if}
      </a>
      <a
        href={resolve('/my-budget')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-budget'
        )}"
      >
        {#if pathname === '/my-budget'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Wallet size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.budget_title()}</span>
      </a>
      <a
        href={resolve('/my-tracks')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname.startsWith('/my-tracks')
        )}"
      >
        {#if pathname.startsWith('/my-tracks')}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Train size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_tracks()}</span>
      </a>
      <a
        href={resolve('/my-depot')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-depot'
        )}"
      >
        {#if pathname === '/my-depot'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Box size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_depot()}</span>
      </a>
      <a
        href={resolve('/my-digital-roster')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-digital-roster'
        )}"
      >
        {#if pathname === '/my-digital-roster'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Cpu size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_digital_roster()}</span>
      </a>
      <a
        href={resolve('/my-settings')}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95 {getInactiveClass(
          pathname === '/my-settings'
        )}"
      >
        {#if pathname === '/my-settings'}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            style="box-shadow: 0 0 8px currentColor, 0 0 12px currentColor;"
          ></div>
        {/if}
        <Settings size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_settings()}</span>
      </a>
    </div>
  {/key}
</div>
