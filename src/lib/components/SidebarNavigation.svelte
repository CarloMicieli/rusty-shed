<!-- eslint-disable svelte/no-navigation-without-resolve -->
<script lang="ts">
  import { Settings, ChevronsLeft, ChevronsRight } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components/ui/badge';
  import { appState } from '$lib/stores/app.svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { NAVIGATION_ITEMS } from './navigation/config';
  import { isActive } from './navigation/utils';
  import type { SidebarNavigationProps } from './navigation/types';
  import AppLogo from './AppLogo.svelte';

  let { collapsed = false, onToggle }: SidebarNavigationProps = $props();

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const pathname = $derived(($page.url.pathname as string) || '');

  function navLinkClass(active: boolean): string {
    return active
      ? 'bg-sidebar-primary/10 text-sidebar-primary'
      : 'text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground';
  }
</script>

<nav class="relative flex h-full w-full flex-col overflow-hidden bg-sidebar">
  <!-- Content -->
  <div class="relative z-10 flex h-full flex-col p-4">
    <!-- Logo -->
    <div class="mb-8 flex items-center {collapsed ? 'justify-center px-0' : 'gap-3 px-4'}">
      <AppLogo size={32} class="shrink-0 text-sidebar-primary" />
      {#if !collapsed}
        <h1
          class="text-xl font-bold tracking-tight text-sidebar-primary uppercase"
          style="filter: drop-shadow(0 1px 2px var(--sidebar-name-shadow));"
        >
          {m.app_name()}
        </h1>
      {/if}
    </div>

    <!-- Nav items -->
    <ul class="space-y-2">
      {#each NAVIGATION_ITEMS as item (item.id)}
        <li class="relative overflow-hidden">
          {#if isActive(item, pathname)}
            <div class="absolute inset-y-0 left-0 w-[2px] bg-sidebar-primary"></div>
          {/if}
          <a
            href={resolve(item.href as '/dashboard')}
            class="flex w-full items-center rounded-lg py-2.5 text-sm font-medium transition-colors {collapsed
              ? 'justify-center px-2'
              : 'justify-start gap-3 px-4'} {navLinkClass(isActive(item, pathname))}"
            aria-current={isActive(item, pathname) ? 'page' : undefined}
            title={collapsed ? item.label() : undefined}
          >
            <item.icon size={20} class="shrink-0" />
            {#if !collapsed}
              <span class="tracking-wide">{item.label()}</span>
              {#if item.id === 'wishlists' && defaultWishlist}
                <Badge variant="outline" class="ml-auto">{defaultWishlist.count}</Badge>
              {/if}
            {/if}
          </a>
        </li>
      {/each}
    </ul>

    <!-- Footer -->
    <div class="mt-auto space-y-2 border-t border-sidebar-border pt-4">
      <!-- Toggle button -->
      <button
        onclick={onToggle}
        class="mb-2 flex w-full items-center rounded-lg p-2 text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-sidebar-foreground {collapsed
          ? 'justify-center'
          : 'justify-end px-4'}"
        aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      >
        {#if collapsed}
          <ChevronsRight size={16} />
        {:else}
          <ChevronsLeft size={16} />
        {/if}
      </button>

      <!-- Settings link -->
      <div class="relative overflow-hidden">
        {#if pathname === '/settings'}
          <div class="absolute inset-y-0 left-0 w-[2px] bg-sidebar-primary"></div>
        {/if}
        <a
          href={resolve('/settings')}
          class="flex w-full items-center rounded-lg py-2.5 text-sm font-medium transition-colors {collapsed
            ? 'justify-center px-2'
            : 'justify-start gap-3 px-4'} {navLinkClass(pathname === '/settings')}"
          aria-current={pathname === '/settings' ? 'page' : undefined}
          title={collapsed ? m.app_settings() : undefined}
        >
          <Settings size={20} class="shrink-0" />
          {#if !collapsed}
            <span class="tracking-wide">{m.app_settings()}</span>
          {/if}
        </a>
      </div>

      {#if !collapsed}
        <div class="px-4 py-2 text-center text-xs tracking-widest text-muted-foreground uppercase">
          {m.app_version_prefix()}
          {appState.version || '—'}
        </div>
      {/if}
    </div>
  </div>
</nav>
