<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Textarea } from '$lib/components';
  import type { Manufacturer, RailwayCompany, WishlistPreview } from '$lib/bindings';
  import type { AddRailwayModelFormState } from '../types';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import RailwayModelBaseForm from '$lib/shared/components/RailwayModelBaseForm.svelte';
  import { CATEGORIES, SCALES, POWER_METHODS, PRIORITIES } from '../constants';

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
      <span class="text-sm font-medium text-muted-foreground">
        {m.wishlist_field_wishlist()}
        <span class="text-error-500">*</span>
      </span>
    </label>
    <select
      id="wishlist"
      bind:value={form.wishlistId}
      class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
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
  >
    {#if form.rollingStocks.length === 0}
      <p class="text-sm text-muted-foreground">No rolling stocks added yet.</p>
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
  <div class="space-y-4 rounded-lg border border-border bg-card p-4 text-card-foreground">
    <h3 class="text-lg font-semibold text-foreground">{m.wishlist_details_section_title()}</h3>

    <!-- Priority -->
    <div>
      <label for="priority" class="block space-y-1">
        <span class="text-sm font-medium text-muted-foreground">{m.wishlist_field_priority()}</span>
      </label>
      <select
        id="priority"
        bind:value={form.priority}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
      >
        {#each PRIORITIES as priority (priority)}
          <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
          <option value={priority}>{(m as any)[getPriorityLabelKey(priority)]()}</option>
        {/each}
      </select>
    </div>

    <!-- Desired Price -->
    <div>
      <label for="desired-price" class="block space-y-1">
        <span class="text-sm font-medium text-muted-foreground"
          >{m.wishlist_field_desired_price()}</span
        >
      </label>
      <input
        id="desired-price"
        type="number"
        step="0.01"
        min="0"
        bind:value={form.desiredPriceAmount}
        class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        placeholder="0.00"
      />
    </div>

    <!-- Notes -->
    <div>
      <label for="notes" class="block space-y-1">
        <span class="text-sm font-medium text-muted-foreground">{m.wishlist_field_notes()}</span>
      </label>
      <Textarea
        id="notes"
        bind:value={form.notes}
        class="w-full"
        rows={3}
        placeholder="Additional notes..."
      />
    </div>
  </div>
</form>
