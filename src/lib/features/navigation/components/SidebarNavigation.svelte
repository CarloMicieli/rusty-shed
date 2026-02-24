<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import {
    LayoutDashboard,
    Library,
    Heart,
    Wallet,
    Box,
    Settings,
    TrainFront,
    Train,
    Wrench
  } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { appVersion } from '$lib/stores/app';
  import { Badge } from '$lib/components';
  import { getCollectionContext } from '$lib/features/collection/CollectionState.svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { localeStore } from '$lib/stores/locale';

  const collectionService = getCollectionContext();
  const wishlistService = getWishlistContext();

  const totalCount = $derived(collectionService.totalCount);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const locale = $derived($localeStore);
</script>

{#key locale}
  <nav
    class="border-surface-700/50 bg-surface-900 hidden h-full w-64 flex-col border-r p-4 lg:flex"
  >
    <div class="mb-8 flex items-center gap-3 px-4">
      <TrainFront class="text-accent-500" size={32} />
      <h2 class="h3 font-bold tracking-tight uppercase">{m.app_name()}</h2>
    </div>

    <ul class="space-y-2">
      <li>
        <a
          href={resolve('/dashboard')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/dashboard'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/dashboard'}
        >
          <LayoutDashboard size={20} />
          <span class="font-medium tracking-wide">{m.app_dashboard()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/collection')}
          class="hover:variant-soft-primary btn flex w-full items-center justify-between gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/collection'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/collection'}
        >
          <div class="flex items-center gap-3">
            <Library size={20} />
            <span class="font-medium tracking-wide">{m.app_collection()}</span>
          </div>
          <Badge variant="default">{totalCount}</Badge>
        </a>
      </li>
      <li>
        <a
          href={resolve('/wishlists')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/wishlists'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/wishlists'}
        >
          <Heart size={20} />
          <span class="font-medium tracking-wide">{m.app_wishlists()}</span>
          {#if defaultWishlist}
            <Badge variant="outline" class="ml-auto">{defaultWishlist.count}</Badge>
          {/if}
        </a>
      </li>
      <li>
        <a
          href={resolve('/finance')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/finance'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/finance'}
        >
          <Wallet size={20} />
          <span class="font-medium tracking-wide">{m.budget_title()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/railway-tracks')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string).startsWith(
            '/railway-tracks'
          )}
          class:variant-ghost-surface={!($page.url.pathname as string).startsWith(
            '/railway-tracks'
          )}
        >
          <Train size={20} />
          <span class="font-medium tracking-wide">{m.app_tracks()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/depot')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/depot'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/depot'}
        >
          <Box size={20} />
          <span class="font-medium tracking-wide">{m.app_depot()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/maintenance')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={($page.url.pathname as string) === '/maintenance'}
          class:variant-ghost-surface={($page.url.pathname as string) !== '/maintenance'}
        >
          <Wrench size={20} />
          <span class="font-medium tracking-wide">{m.app_maintenance()}</span>
        </a>
      </li>
    </ul>

    <div class="border-surface-700/50 mt-auto space-y-2 border-t pt-4">
      <a
        href={resolve('/settings')}
        class="hover:variant-soft-primary variant-ghost-surface btn w-full justify-start gap-3"
        class:variant-filled-primary={($page.url.pathname as string) === '/settings'}
      >
        <Settings size={20} />
        <span class="font-medium tracking-wide">{m.app_settings()}</span>
      </a>
      <div class="text-surface-400 px-4 py-2 text-center text-xs tracking-widest uppercase">
        {m.app_version_prefix()}
        {$appVersion || '—'}
      </div>
    </div>
  </nav>
{/key}
