<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { Bug, Settings } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Sheet } from '$lib/components/ui/sheet';
  import type { MoreMenuProps } from './types';
  import { isActive } from './utils';

  let { open = false, onClose, items }: MoreMenuProps = $props();

  const pathname = $derived(($page.url.pathname as string) || '');

  function handleItemClick() {
    onClose();
  }

  const topActions = [
    {
      id: 'settings',
      label: () => m.app_settings(),
      href: '/settings',
      icon: Settings
    },
    {
      id: 'debug',
      label: () => m.app_debug(),
      href: '/debug',
      icon: Bug
    }
  ];

  function handleOpenChange(newOpen: boolean) {
    if (!newOpen) {
      onClose();
    }
  }
</script>

<Sheet
  {open}
  side="bottom"
  class="!right-0 !bottom-20 !left-0 !mx-auto !max-h-[80vh] !w-[min(100%,26rem)] overflow-y-auto !rounded-t-xl !bg-background !opacity-100"
  onOpenChange={handleOpenChange}
>
  <div class="flex flex-col space-y-4 px-4 py-4">
    <div class="space-y-2">
      <p class="px-1 text-xs font-semibold tracking-wider text-muted-foreground uppercase">
        {m.mobile_more_top_actions()}
      </p>
      <div class="grid grid-cols-2 gap-2">
        {#each topActions as action (action.id)}
          <a
            href={resolve(action.href as '/settings')}
            onclick={() => handleItemClick()}
            class="flex min-h-11 items-center justify-center gap-2 rounded-lg border border-border bg-card px-3 py-2 text-sm font-semibold hover:bg-muted"
          >
            <action.icon size={18} />
            <span>{action.label()}</span>
          </a>
        {/each}
      </div>
    </div>

    <div class="space-y-2 border-t border-border pt-3">
      {#each items as item (item.id)}
        <a
          href={resolve(item.href as '/dashboard')}
          onclick={() => handleItemClick()}
          class="flex min-h-11 items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-colors"
          class:bg-primary={isActive(item, pathname)}
          class:text-primary-foreground={isActive(item, pathname)}
          class:text-foreground={!isActive(item, pathname)}
          class:hover:bg-muted={!isActive(item, pathname)}
          aria-current={isActive(item, pathname) ? 'page' : undefined}
        >
          <item.icon size={20} />
          <span>{item.label()}</span>
        </a>
      {/each}
    </div>
  </div>
</Sheet>
