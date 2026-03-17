<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { DrawerSectionBar, DrawerInput, FormPrice } from '$lib/components/drawer';
  import { getCurrencySymbol } from '$lib/utils/currency';
  import type { WishlistPreview, WishlistPriority } from '$lib/bindings';
  import { PRIORITIES } from '$lib/features/wishlists/constants';

  interface Props {
    wishlistId: string;
    newListName?: string;
    wishlists: WishlistPreview[];
    priority: WishlistPriority;
    desiredPrice: number | null;
    currency: string;
    notes?: string;
    errors?: {
      wishlistId?: string;
      desiredPrice?: string;
    };
    disabled?: boolean;
    disableWishlistSelection?: boolean;
    expanded?: boolean;
  }

  let {
    wishlistId = $bindable(),
    newListName = $bindable(''),
    wishlists,
    priority = $bindable(),
    desiredPrice = $bindable(),
    currency,
    notes = $bindable(undefined),
    errors = {},
    disabled = false,
    disableWishlistSelection = false,
    expanded = $bindable(true)
  }: Props = $props();

  const PRIORITY_LABELS: Record<string, () => string> = {
    LOW: m.wishlist_priority_low,
    NORMAL: m.wishlist_priority_normal,
    HIGH: m.wishlist_priority_high
  };

  function getPriorityLabel(p: string): string {
    return PRIORITY_LABELS[p]?.() ?? p;
  }

  const isDropdownDisabled = $derived(
    disabled || disableWishlistSelection || newListName.trim() !== '' || wishlists.length === 0
  );
  const selectedWishlist = $derived(wishlists.find((l) => l.id === wishlistId));
  const currencySymbol = $derived(getCurrencySymbol(currency));
</script>

<div class="space-y-3">
  <DrawerSectionBar
    label={m.drawer_section_wishlist()}
    {expanded}
    onToggle={() => (expanded = !expanded)}
  />

  {#if expanded}
    <!-- Wishlist selector + new list name -->
    <div class="space-y-1">
      <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
        {m.wishlist_modal_choose_or_create()}
      </span>
      <div class="grid grid-cols-2 gap-3">
        <Select.Root
          type="single"
          value={wishlistId || undefined}
          disabled={isDropdownDisabled}
          onValueChange={(v) => {
            wishlistId = v;
          }}
        >
          <Select.Trigger
            class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]"
            aria-label={m.wishlist_modal_select_list()}
          >
            {#if selectedWishlist}
              {selectedWishlist.name}{#if selectedWishlist.isDefault}
                (default){/if}
            {:else}
              <span class="text-zinc-500">{m.wishlist_modal_select_placeholder()}</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each wishlists as list (list.id)}
              <Select.Item value={list.id} label={list.name}>
                {list.name}{#if list.isDefault}
                  (default){/if}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>

        {#if !disableWishlistSelection}
          <DrawerInput
            type="text"
            placeholder={m.wishlist_modal_new_list_placeholder()}
            bind:value={newListName}
            {disabled}
          />
        {/if}
      </div>
      {#if errors.wishlistId}
        <p class="text-xs text-destructive">{errors.wishlistId}</p>
      {/if}
    </div>

    <!-- Priority tri-state toggle -->
    <div class="space-y-1">
      <span class="text-xs text-zinc-400">
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
                  ? 'bg-[#D48A42] text-black'
                  : 'border border-[#D48A42]/60 bg-[#D48A42]/10 text-[#D48A42]'
                : 'border border-[#1F1F1F] text-[#808080] hover:bg-[#D48A42]/10'
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
      inputClass="bg-[#0F0F0F] border-[#1F1F1F] rounded-[8px] text-[#E0E0E0] placeholder:text-[#808080]"
      {disabled}
      error={errors.desiredPrice}
    />

    <!-- Notes (optional) -->
    {#if notes !== undefined}
      <div class="space-y-1">
        <label for="wishlist-section-notes" class="text-xs text-zinc-400">
          {m.wishlist_field_notes()}
        </label>
        <textarea
          id="wishlist-section-notes"
          bind:value={notes}
          {disabled}
          rows={3}
          placeholder="Additional notes..."
          class="flex w-full rounded-md border border-[#1F1F1F] bg-[#0F0F0F] px-3 py-2 text-sm text-[#E0E0E0] placeholder:text-[#808080] focus:border-[#D48A42]/60 focus:ring-2 focus:ring-[#D48A42]/20 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        ></textarea>
      </div>
    {/if}
  {/if}
</div>
