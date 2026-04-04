<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Input, Button } from '$lib/components';
  import { FormSelect } from '$lib/components/drawer';
  import { getFlag } from '$lib/utils/flags';
  import type { RailwayCompany, Category } from '$lib/bindings';
  import type { RollingStockFormEntry } from '../types';
  import { ROLLING_STOCK_CATEGORIES } from '../constants';
  import { categoryLabel } from '$lib/utils/enum-options';

  interface Props {
    /** Entry data bound two-way */
    entry: RollingStockFormEntry;
    /** Available railway companies for dropdown */
    railwayCompanies: RailwayCompany[];
    /** Whether remove button is enabled (disabled if only entry) */
    canRemove: boolean;
    /** Callback to remove this entry */
    onRemove: () => void;
  }

  let { entry = $bindable(), railwayCompanies, canRemove, onRemove }: Props = $props();
</script>

<div
  class="rolling-stock-entry grid grid-cols-1 gap-4 rounded-lg border border-border bg-card p-4 text-card-foreground"
>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Railway Company -->
    <div class="col-span-1">
      <FormSelect
        id="railway-company-{entry.id}"
        label={m.wishlist_field_railway_company()}
        options={railwayCompanies.map((c) => ({
          value: c.id,
          label: c.name,
          countryCode: c.countryCode,
          registeredCompanyName: c.registeredCompanyName
        }))}
        bind:value={entry.railwayCompanyId}
        placeholder="-- {m.wishlist_field_railway_company()} --"
        isSearchable
        required
      >
        {#snippet item(opt)}
          <div class="flex items-center gap-3">
            <div class="flex shrink-0 items-center gap-1.5">
              <span class="text-lg leading-none" aria-hidden="true">{getFlag(opt.countryCode)}</span
              >
              <span class="font-mono text-[10px] text-muted-foreground/50"
                >[{opt.countryCode ?? '??'}]</span
              >
            </div>
            <div class="flex min-w-0 flex-col leading-tight">
              <span class="truncate font-bold text-foreground">{opt.label}</span>
              {#if opt.registeredCompanyName}
                <span
                  class="truncate font-mono text-[9px] font-medium tracking-tight text-muted-foreground/70 uppercase"
                >
                  {opt.registeredCompanyName}
                </span>
              {/if}
            </div>
          </div>
        {/snippet}

        {#snippet trigger(opt)}
          {#if opt}
            <div class="flex items-center gap-2">
              <span class="text-base leading-none" aria-hidden="true"
                >{getFlag(opt.countryCode)}</span
              >
              <span class="font-bold text-foreground">{opt.label}</span>
            </div>
          {:else}
            <span class="text-muted-foreground">-- {m.wishlist_field_railway_company()} --</span>
          {/if}
        {/snippet}
      </FormSelect>
    </div>

    <!-- Series Code -->
    <div>
      <label for="series-code-{entry.id}" class="block space-y-1">
        <span class="text-[10px] text-muted-foreground uppercase"
          >{m.wishlist_field_series_code()}</span
        >
      </label>
      <Input
        id="series-code-{entry.id}"
        type="text"
        bind:value={entry.seriesCode}
        placeholder={m.rolling_stock_placeholder_series_code()}
        class="w-full border-border bg-transparent font-mono text-foreground placeholder:text-muted-foreground focus:border-amber-500"
        required
      />
    </div>
  </div>

  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <!-- Category -->
    <div>
      <label for="category-{entry.id}" class="block space-y-1">
        <span class="text-[10px] text-muted-foreground uppercase"
          >{m.wishlist_field_category()}</span
        >
      </label>
      <select
        id="category-{entry.id}"
        bind:value={entry.category}
        class="flex h-10 w-full rounded-md border border-border bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-amber-500 focus:ring-2 focus:ring-amber-500/30 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        required
      >
        <option value="">-- {m.wishlist_field_category()} --</option>
        {#each ROLLING_STOCK_CATEGORIES as cat (cat)}
          <option value={cat}>{categoryLabel(cat as Category)}</option>
        {/each}
      </select>
    </div>

    <!-- Road Number (optional) -->
    <div>
      <label for="road-number-{entry.id}" class="block space-y-1">
        <span class="text-[10px] text-muted-foreground uppercase"
          >{m.wishlist_field_road_number()}</span
        >
      </label>
      <Input
        id="road-number-{entry.id}"
        type="text"
        bind:value={entry.roadNumber}
        placeholder={m.rolling_stock_placeholder_road_number()}
        class="w-full border-border bg-transparent font-mono text-foreground placeholder:text-muted-foreground focus:border-amber-500"
      />
    </div>
  </div>

  <!-- Remove Button -->
  <div class="flex justify-end">
    <Button
      type="button"
      variant="ghost"
      size="sm"
      class="text-destructive hover:bg-destructive/10"
      disabled={!canRemove}
      onclick={onRemove}
      aria-label={m.wishlist_rolling_stock_remove()}
    >
      <X size={16} />
      <span>{m.wishlist_rolling_stock_remove()}</span>
    </Button>
  </div>
</div>
