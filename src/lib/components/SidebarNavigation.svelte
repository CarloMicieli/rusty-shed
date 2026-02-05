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
    Cpu,
    Wrench
  } from 'lucide-svelte';
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import { appVersion } from '$lib/stores/app';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { localeStore } from '$lib/stores/locale';

  const wishlistService = getWishlistContext();

  const defaultWishlist = $derived(wishlistService.defaultWishlist);
  const locale = $derived($localeStore);
</script>

{#key locale}
  <nav class="hidden h-full w-64 flex-col border-r border-border bg-sidebar p-4 lg:flex">
    <div class="mb-8 flex items-center gap-3 px-4">
      <TrainFront class="text-primary" size={32} />
      <h2 class="h3 font-bold tracking-tight text-sidebar-foreground uppercase">
        {m.app_name()}
      </h2>
    </div>

    <ul class="space-y-2">
      <li>
        <a
          href={resolve('/my-dashboard')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-dashboard'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-dashboard'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-dashboard'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-dashboard'}
        >
          <LayoutDashboard size={20} />
          <span class="tracking-wide">{m.app_dashboard()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-collection')}
          class="flex w-full items-center gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-collection'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-collection'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-collection'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-collection'}
        >
          <Library size={20} />
          <span class="tracking-wide">{m.app_collection()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-wishlists')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-wishlists'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-wishlists'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-wishlists'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-wishlists'}
        >
          <Heart size={20} />
          <span class="tracking-wide">{m.app_wishlists()}</span>
          {#if defaultWishlist}
            <Badge variant="outline" class="ml-auto">{defaultWishlist.count}</Badge>
          {/if}
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-budget')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-budget'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-budget'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-budget'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-budget'}
        >
          <Wallet size={20} />
          <span class="tracking-wide">{m.budget_title()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-tracks')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string).startsWith('/my-tracks')}
          class:text-primary-foreground={($page.url.pathname as string).startsWith('/my-tracks')}
          class:text-sidebar-foreground={!($page.url.pathname as string).startsWith('/my-tracks')}
          class:hover:bg-sidebar-accent={!($page.url.pathname as string).startsWith('/my-tracks')}
        >
          <Train size={20} />
          <span class="tracking-wide">{m.app_tracks()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-depot')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-depot'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-depot'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-depot'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-depot'}
        >
          <Box size={20} />
          <span class="tracking-wide">{m.app_depot()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-digital-roster')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-digital-roster'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-digital-roster'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-digital-roster'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-digital-roster'}
        >
          <Cpu size={20} />
          <span class="tracking-wide">{m.app_digital_roster()}</span>
        </a>
      </li>
      <li>
        <a
          href={resolve('/my-maintenance')}
          class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
          class:bg-primary={($page.url.pathname as string) === '/my-maintenance'}
          class:text-primary-foreground={($page.url.pathname as string) === '/my-maintenance'}
          class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-maintenance'}
          class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-maintenance'}
        >
          <Wrench size={20} />
          <span class="tracking-wide">{m.app_maintenance()}</span>
        </a>
      </li>
    </ul>

    <div class="mt-auto space-y-2 border-t border-border pt-4">
      <a
        href={resolve('/my-settings')}
        class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
        class:bg-primary={($page.url.pathname as string) === '/my-settings'}
        class:text-primary-foreground={($page.url.pathname as string) === '/my-settings'}
        class:text-sidebar-foreground={($page.url.pathname as string) !== '/my-settings'}
        class:hover:bg-sidebar-accent={($page.url.pathname as string) !== '/my-settings'}
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
