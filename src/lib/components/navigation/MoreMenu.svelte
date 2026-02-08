<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { Sheet } from '$lib/components/ui/sheet';
  import type { MoreMenuProps } from './types';
  import { isActive } from './utils';

  let { open = false, onClose, items }: MoreMenuProps = $props();

  const pathname = $derived(($page.url.pathname as string) || '');

  function handleItemClick() {
    onClose();
  }

  function handleOpenChange(newOpen: boolean) {
    if (!newOpen) {
      onClose();
    }
  }
</script>

<Sheet
  {open}
  side="bottom"
  class="!right-4 !bottom-24 !left-auto !max-h-64 !w-72 overflow-y-auto !rounded-t-lg !bg-background !opacity-100"
  onOpenChange={handleOpenChange}
>
  <div class="flex flex-col space-y-2 px-4 py-4">
    <div class="space-y-2">
      {#each items as item (item.id)}
        <a
          href={resolve(item.href as '/my-dashboard')}
          onclick={() => handleItemClick()}
          class="flex items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-colors"
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
