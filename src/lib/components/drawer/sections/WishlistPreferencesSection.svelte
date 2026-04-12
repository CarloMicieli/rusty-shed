<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { FormPrice } from '$lib/components/drawer';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import type { WishlistPriority } from '$lib/bindings';
  import { PRIORITIES } from '$lib/features/wishlists/constants';

  interface Props {
    priority: WishlistPriority;
    desiredPrice: number | null;
    currency: string;
    notes?: string;
    disabled?: boolean;
    errors?: {
      desiredPrice?: string;
    };
  }

  let {
    priority = $bindable(),
    desiredPrice = $bindable(),
    currency,
    notes = $bindable(undefined),
    disabled = false,
    errors = {}
  }: Props = $props();

  const PRIORITY_LABELS: Record<string, () => string> = {
    LOW: m.wishlist_priority_low,
    NORMAL: m.wishlist_priority_normal,
    HIGH: m.wishlist_priority_high
  };

  function getPriorityLabel(p: string): string {
    return PRIORITY_LABELS[p]?.() ?? p;
  }

  const currencySymbol = $derived(regionalManager.getCurrencySymbol(currency));
</script>

<div class="space-y-3">
  <!-- Priority tri-state toggle -->
  <div class="space-y-1">
    <span class="text-[10px] font-bold text-muted-foreground uppercase">
      {m.wishlist_modal_priority()}
    </span>
    <div class="flex gap-2">
      {#each PRIORITIES as p (p)}
        <button
          type="button"
          class={[
            'flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors',
            priority === p
              ? p === 'NORMAL'
                ? 'bg-primary text-primary-foreground'
                : 'border border-primary/60 bg-primary/10 text-primary'
              : 'border border-layout-border text-muted-foreground hover:bg-primary/10'
          ].join(' ')}
          onclick={() => (priority = p)}
          {disabled}
        >
          {getPriorityLabel(p)}
        </button>
      {/each}
    </div>
  </div>

  <!-- Desired price -->
  <FormPrice
    label={m.wishlist_modal_desired_price()}
    id="wishlist-section-desired-price"
    bind:value={desiredPrice}
    symbol={currencySymbol}
    inputClass="bg-background border-border rounded-sm text-foreground placeholder:text-muted-foreground"
    {disabled}
    error={errors.desiredPrice}
  />

  <!-- Notes (optional) -->
  {#if notes !== undefined}
    <div class="space-y-1">
      <label
        for="wishlist-preferences-notes"
        class="text-[10px] font-bold text-muted-foreground uppercase"
      >
        {m.wishlist_field_notes()}
      </label>
      <textarea
        id="wishlist-preferences-notes"
        bind:value={notes}
        {disabled}
        rows={3}
        placeholder={m.wishlist_modal_notes_placeholder()}
        class="flex w-full rounded-sm border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-1 focus:ring-primary focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      ></textarea>
    </div>
  {/if}
</div>
