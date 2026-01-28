<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { LayoutDashboard, Library, Heart, Box, Settings, TrainFront } from 'lucide-svelte';
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { appVersion } from '$lib/stores/app';
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
    class="hidden h-full w-64 flex-col border-r border-surface-700/50 bg-surface-900 p-4 lg:flex"
  >
    <div class="mb-8 flex items-center gap-3 px-4">
      <TrainFront class="text-accent-500" size={32} />
      <h2 class="h3 font-bold tracking-tight uppercase">{m.app_name()}</h2>
    </div>

    <ul class="space-y-2">
      <li>
        <a
          href={resolve('/my-dashboard')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={(page.url.pathname as string) === '/my-dashboard'}
          class:variant-ghost-surface={(page.url.pathname as string) !== '/my-dashboard'}
        >
          <LayoutDashboard size={20} />
          <span class="font-medium tracking-wide">{m.app_dashboard()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-collection')}
          class="hover:variant-soft-primary btn flex w-full items-center justify-between gap-3"
          class:variant-filled-primary={(page.url.pathname as string) === '/my-collection'}
          class:variant-ghost-surface={(page.url.pathname as string) !== '/my-collection'}
        >
          <div class="flex items-center gap-3">
            <Library size={20} />
            <span class="font-medium tracking-wide">{m.app_collection()}</span>
          </div>
          <span class="variant-filled-primary badge">{totalCount}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-wishlists')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={(page.url.pathname as string) === '/my-wishlists'}
          class:variant-ghost-surface={(page.url.pathname as string) !== '/my-wishlists'}
        >
          <Heart size={20} />
          <span class="font-medium tracking-wide">{m.app_wishlists()}</span>
          {#if defaultWishlist}
            <span class="variant-soft-surface ml-auto badge">{defaultWishlist.count}</span>
          {/if}
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-depot')}
          class="hover:variant-soft-primary btn w-full justify-start gap-3"
          class:variant-filled-primary={(page.url.pathname as string) === '/my-depot'}
          class:variant-ghost-surface={(page.url.pathname as string) !== '/my-depot'}
        >
          <Box size={20} />
          <span class="font-medium tracking-wide">{m.app_depot()}</span>
        </a>
      </li>
    </ul>

    <div class="mt-auto space-y-2 border-t border-surface-700/50 pt-4">
      <a
        href={resolve('/my-settings')}
        class="hover:variant-soft-primary variant-ghost-surface btn w-full justify-start gap-3"
        class:variant-filled-primary={(page.url.pathname as string) === '/my-settings'}
      >
        <Settings size={20} />
        <span class="font-medium tracking-wide">{m.app_settings()}</span>
      </a>
      <div class="px-4 py-2 text-center text-xs tracking-widest text-surface-400 uppercase">
        {m.app_version_prefix()}
        {$appVersion || '—'}
      </div>
    </div>
  </nav>
{/key}
