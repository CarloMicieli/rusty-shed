<script lang="ts">
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import { slide } from 'svelte/transition';
  import * as m from '$lib/paraglide/messages';
  import { FormInput, FormSelect } from '$lib/components/drawer';
  import PrototypeLibraryPicker from './PrototypeLibraryPicker.svelte';
  import type { PrototypeView } from '$lib/bindings';

  interface Props {
    // Identification fields (all bindable)
    railwayCompanyId?: string | null;
    companyOptions?: { value: string; label: string }[];
    seriesCode: string;
    series: string;
    roadNumber: string;
    friendlyName: string;
    livery: string;
    depot: string;
    // Prototype picker (optional — pass all three to show the picker)
    category?: string;
    selectedPrototypeId?: string;
    onPrototypeSelect?: (p: PrototypeView) => void;
    onPrototypeClear?: () => void;
  }

  let {
    railwayCompanyId = $bindable<string | null>(null),
    companyOptions = [],
    seriesCode = $bindable(),
    series = $bindable(),
    roadNumber = $bindable(),
    friendlyName = $bindable(),
    livery = $bindable(),
    depot = $bindable(),
    category,
    selectedPrototypeId,
    onPrototypeSelect,
    onPrototypeClear
  }: Props = $props();

  const showPicker = $derived(
    category !== undefined && onPrototypeSelect !== undefined && onPrototypeClear !== undefined
  );

  let open = $state(true);
</script>

<div class="overflow-hidden rounded-sm border border-border bg-card">
  <button
    type="button"
    onclick={() => (open = !open)}
    class="flex w-full items-center justify-between px-4 py-3 transition-colors hover:bg-white/5"
  >
    <span class="font-bebas text-sm tracking-widest text-muted-foreground uppercase">
      {m.rolling_stock_create_section_prototype()}
    </span>
    <ChevronDown
      size={14}
      class="text-muted-foreground transition-transform duration-200 {open ? 'rotate-180' : ''}"
    />
  </button>

  {#if open}
    <div class="px-4 pb-4" transition:slide={{ duration: 200 }}>
      <div class="space-y-4">
        {#if showPicker}
          <PrototypeLibraryPicker
            category={category ?? ''}
            selectedId={selectedPrototypeId ?? ''}
            onSelect={onPrototypeSelect!}
            onClear={onPrototypeClear!}
          />
          <div class="border-t border-border/40"></div>
        {/if}

        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <FormSelect
              id="proto-company"
              label={m.rolling_stock_field_railway_company()}
              options={companyOptions}
              bind:value={railwayCompanyId}
              placeholder={m.rolling_stock_select_company()}
              isSearchable
              disabled={companyOptions.length === 0}
              required
            />
          </div>
          <FormInput
            id="proto-series-code"
            label={m.rolling_stock_field_series_code()}
            bind:value={seriesCode}
            placeholder={m.rolling_stock_placeholder_series_code()}
            class="font-mono"
            required
          />
          <FormInput
            id="proto-series"
            label={m.rolling_stock_field_series()}
            bind:value={series}
            placeholder={m.rolling_stock_placeholder_series()}
          />
          <FormInput
            id="proto-friendly-name"
            label={m.rolling_stock_field_friendly_name()}
            bind:value={friendlyName}
            placeholder={m.rolling_stock_placeholder_friendly_name()}
          />
          <FormInput
            id="proto-road-number"
            label={m.rolling_stock_field_road_number()}
            bind:value={roadNumber}
            placeholder={m.rolling_stock_placeholder_road_number()}
            class="font-mono"
          />
          <FormInput
            id="proto-livery"
            label={m.rolling_stock_field_livery()}
            bind:value={livery}
            placeholder={m.rolling_stock_placeholder_livery()}
          />
          <FormInput
            id="proto-depot"
            label={m.rolling_stock_field_depot()}
            bind:value={depot}
            placeholder={m.rolling_stock_placeholder_depot()}
          />
        </div>
      </div>
    </div>
  {/if}
</div>
