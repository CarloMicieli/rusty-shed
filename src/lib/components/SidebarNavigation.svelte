<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { Settings } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components/ui/badge';
  import { appVersion } from '$lib/stores/app';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { localeStore } from '$lib/stores/locale';
  import { NAVIGATION_ITEMS } from './navigation/config';
  import { isActive } from './navigation/utils';

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const locale = $derived($localeStore);
  const pathname = $derived(($page.url.pathname as string) || '');

  function navLinkClass(active: boolean): string {
    return active
      ? 'bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
      : 'text-sidebar-foreground hover:bg-sidebar-accent';
  }
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
        <li class="relative overflow-hidden">
          {#if isActive(item, pathname)}
            <div class="absolute inset-y-0 left-0 w-[2px] bg-[#D48A42]"></div>
          {/if}
          <a
            href={resolve(item.href as '/dashboard')}
            class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors {navLinkClass(
              isActive(item, pathname)
            )}"
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
      <div class="relative overflow-hidden">
        {#if pathname === '/settings'}
          <div class="absolute inset-y-0 left-0 w-[2px] bg-[#D48A42]"></div>
        {/if}
        <a
          href={resolve('/settings')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors {navLinkClass(
            pathname === '/settings'
          )}"
          aria-current={pathname === '/settings' ? 'page' : undefined}
        >
          <Settings size={20} />
          <span class="tracking-wide">{m.app_settings()}</span>
        </a>
      </div>
      <div class="px-4 py-2 text-center text-xs tracking-widest text-muted-foreground uppercase">
        {m.app_version_prefix()}
        {$appVersion || '—'}
      </div>
    </div>
  </nav>
{/key}
