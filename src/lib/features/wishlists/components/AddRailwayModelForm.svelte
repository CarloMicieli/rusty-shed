<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Textarea } from '$lib/components';
  import type { Manufacturer, RailwayCompany, WishlistPreview } from '$lib/bindings';
  import type { AddRailwayModelFormState } from '../types';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import RailwayModelBaseForm from '$lib/shared/components/RailwayModelBaseForm.svelte';
  import { CATEGORIES, SCALES, POWER_METHODS, PRIORITIES } from '../constants';
  import { getCurrencySymbol } from '$lib/utils/currency';

  interface Props {
    /** Form state, bound two-way by parent */
    form: AddRailwayModelFormState;
    /** Available wishlists for the wishlist selector */
    wishlists: WishlistPreview[];
    /** Reference data for dropdowns */
    manufacturers: Manufacturer[];
    railwayCompanies: RailwayCompany[];
    /** Callback to add a new rolling stock entry */
    onAddRollingStock: () => void;
    /** Callback to remove a rolling stock entry by id */
    onRemoveRollingStock: (id: string | number) => void;
    /** Label key helpers forwarded from parent */
    getCategoryLabelKey: (category: string) => string;
    getPowerMethodLabelKey: (method: string) => string;
  }

  let {
    form = $bindable(),
    wishlists,
    manufacturers,
    railwayCompanies,
    onAddRollingStock,
    onRemoveRollingStock,
    getCategoryLabelKey,
    getPowerMethodLabelKey
  }: Props = $props();

  const currencySymbol = $derived(getCurrencySymbol(form.desiredPriceCurrency));

  // Helper to get priority label — kept local since it is only used here
  function getPriorityLabelKey(priority: string): string {
    const labelMap: Record<string, string> = {
      LOW: 'wishlist_priority_low',
      NORMAL: 'wishlist_priority_normal',
      HIGH: 'wishlist_priority_high'
    };
    return labelMap[priority] ?? priority;
  }
</script>

<form onsubmit={(e) => e.preventDefault()} class="space-y-6">
  <!-- Wishlist Selection -->
  <div>
    <label for="wishlist" class="block space-y-1">
      <span class="text-[10px] text-[#808080] uppercase">
        {m.wishlist_field_wishlist()}
        <span class="text-error-500">*</span>
      </span>
    </label>
    <select
      id="wishlist"
      bind:value={form.wishlistId}
      class="flex h-10 w-full rounded-md border border-[#1F1F1F] bg-transparent px-3 py-2 text-sm text-[#E0E0E0] placeholder:text-[#808080] focus:border-[#D48A42] focus:ring-2 focus:ring-[#D48A42]/30 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      required
    >
      <option value="">-- {m.wishlist_field_wishlist()} --</option>
      {#each wishlists as wishlist (wishlist.id)}
        <option value={wishlist.id}>{wishlist.name}</option>
      {/each}
    </select>
  </div>

  <!-- Base Railway Model Form (shared component) -->
  <RailwayModelBaseForm
    {manufacturers}
    categoryOptions={CATEGORIES}
    scaleOptions={SCALES}
    powerMethodOptions={POWER_METHODS}
    {form}
    {onAddRollingStock}
    {getCategoryLabelKey}
    {getPowerMethodLabelKey}
    dark={true}
  >
    {#if form.rollingStocks.length === 0}
      <div class="rounded-lg border border-dashed border-[#1F1F1F] p-4">
        <p class="text-sm text-[#808080]">No rolling stocks added yet.</p>
      </div>
    {:else}
      {#each form.rollingStocks as entry, i (entry.id)}
        <RollingStockEntry
          bind:entry={form.rollingStocks[i]}
          {railwayCompanies}
          canRemove={form.rollingStocks.length > 0}
          onRemove={() => onRemoveRollingStock(entry.id)}
        />
      {/each}
    {/if}
  </RailwayModelBaseForm>

  <!-- Wishlist Item Details -->
  <div class="space-y-4 rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
    <p class="text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.wishlist_modal_wishlist_prefs()}
    </p>

    <!-- Priority -->
    <div class="space-y-1">
      <span class="text-[10px] text-[#808080] uppercase">{m.wishlist_field_priority()}</span>
      <div class="flex gap-1">
        {#each PRIORITIES as p (p)}
          <button
            type="button"
            class={[
              'flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors',
              form.priority === p
                ? 'bg-[#D48A42] font-bold text-black'
                : 'border border-[#1F1F1F] bg-transparent text-[#808080] hover:bg-[rgba(212,138,66,0.15)]'
            ].join(' ')}
            onclick={() => (form.priority = p)}
          >
            <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
            {(m as any)[getPriorityLabelKey(p)]()}
          </button>
        {/each}
      </div>
    </div>

    <!-- Desired Price -->
    <div>
      <label for="desired-price" class="block space-y-1">
        <span class="text-[10px] text-[#808080] uppercase">{m.wishlist_field_desired_price()}</span>
      </label>
      <div class="relative flex items-center">
        <input
          id="desired-price"
          type="number"
          step="0.01"
          min="0"
          bind:value={form.desiredPriceAmount}
          class="flex h-10 w-full rounded-md border border-[#1F1F1F] bg-transparent pr-10 text-right font-mono text-sm text-[#E0E0E0] placeholder:text-[#808080] focus:border-[#D48A42] focus:ring-2 focus:ring-[#D48A42]/30 focus:outline-none"
          placeholder="0.00"
        />
        <span class="pointer-events-none absolute right-3 text-sm text-[#808080]">
          {currencySymbol}
        </span>
      </div>
    </div>

    <!-- Notes -->
    <div>
      <label for="notes" class="block space-y-1">
        <span class="text-[10px] text-[#808080] uppercase">{m.wishlist_field_notes()}</span>
      </label>
      <Textarea
        id="notes"
        bind:value={form.notes}
        class="w-full border-[#1F1F1F] bg-transparent text-[#E0E0E0] placeholder:text-[#808080] focus:border-[#D48A42] focus:ring-[#D48A42]/30"
        rows={3}
        placeholder="Additional notes..."
      />
    </div>
  </div>
</form>
