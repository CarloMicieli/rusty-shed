<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { WishlistPriority } from '$lib/bindings';
  import { PRIORITIES } from '$lib/features/wishlists/constants';

  interface Props {
    priority: WishlistPriority;
    disabled?: boolean;
  }

  let { priority = $bindable(), disabled = false }: Props = $props();

  const PRIORITY_LABELS: Record<string, () => string> = {
    LOW: m.wishlist_priority_low,
    NORMAL: m.wishlist_priority_normal,
    HIGH: m.wishlist_priority_high
  };

  function getPriorityLabel(value: string): string {
    return PRIORITY_LABELS[value]?.() ?? value;
  }
</script>

<div class="space-y-1">
  <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
    {m.wishlist_modal_priority()}
  </span>
  <div class="flex gap-2">
    {#each PRIORITIES as option (option)}
      <button
        type="button"
        class={[
          'flex-1 rounded-sm px-3 py-1.5 text-xs font-semibold transition-colors',
          priority === option
            ? option === 'NORMAL'
              ? 'bg-primary text-primary-foreground'
              : 'border border-primary/60 bg-primary/10 text-primary'
            : 'border border-border text-muted-foreground hover:bg-primary/10'
        ].join(' ')}
        onclick={() => (priority = option)}
        {disabled}
      >
        {getPriorityLabel(option)}
      </button>
    {/each}
  </div>
</div>
