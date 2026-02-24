<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import { appVersion } from '$lib/stores/app';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { localeStore } from '$lib/stores/locale';
  import { NAVIGATION_ITEMS } from './navigation/config';
  import { isActive } from './navigation/utils';

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const locale = $derived($localeStore);
  const pathname = $derived(($page.url.pathname as string) || '');
</script>

{#key locale}
  <nav class="hidden h-full w-64 flex-col border-r border-border bg-sidebar p-4 lg:flex">
    <div class="mb-8 flex items-center gap-3 px-4">
      <svg
        class="text-primary"
        width="32"
        height="32"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M6 9v12m6-12v12m6-12v12M3 9h18" />
      </svg>
      <h2 class="h3 font-bold tracking-tight text-sidebar-foreground uppercase">
        {m.app_name()}
      </h2>
    </div>

    <ul class="space-y-2">
      {#each NAVIGATION_ITEMS as item (item.id)}
        <li>
          <a
            href={resolve(item.href as '/dashboard')}
            class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
            class:bg-primary={isActive(item, pathname)}
            class:text-primary-foreground={isActive(item, pathname)}
            class:text-sidebar-foreground={!isActive(item, pathname)}
            class:hover:bg-sidebar-accent={!isActive(item, pathname)}
            aria-current={isActive(item, pathname) ? 'page' : undefined}
          >
            <item.icon size={20} />
            <span class="tracking-wide">{item.label()}</span>
            {#if item.id === 'wishlists' && defaultWishlist}
              <Badge variant="outline" class="ml-auto">{defaultWishlist.count}</Badge>
            {/if}
          </a>
        </li>
      {/each}
    </ul>

    <div class="mt-auto space-y-2 border-t border-border pt-4">
      <a
        href={resolve('/settings')}
        class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
        class:bg-primary={pathname === '/settings'}
        class:text-primary-foreground={pathname === '/settings'}
        class:text-sidebar-foreground={pathname !== '/settings'}
        class:hover:bg-sidebar-accent={pathname !== '/settings'}
        aria-current={pathname === '/settings' ? 'page' : undefined}
      >
        <Settings size={20} />
        <span class="tracking-wide">{m.app_settings()}</span>
      </a>
      <div class="px-4 py-2 text-center text-xs tracking-widest text-muted-foreground uppercase">
        {m.app_version_prefix()}
        {$appVersion || '—'}
      </div>
    </div>
  </nav>
{/key}
