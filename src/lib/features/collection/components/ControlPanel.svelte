<script lang="ts">
  import { X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { FilterState } from '$lib/features/collection/CollectionState.svelte';

  interface ScaleOption {
    id: string;
    display: string;
  }

  interface Props {
    filters: FilterState;
    availableScales: ScaleOption[];
    availableCompanies: string[];
    availableCategories: string[];
    availableEpochs: string[];
    hasActiveFilters: boolean;
    onToggleScale?: (scale: string) => void;
    onToggleCompany?: (company: string) => void;
    onToggleCategory?: (category: string) => void;
    onToggleEpoch?: (epoch: string) => void;
    onClear?: () => void;
    onToggleSidebar?: () => void;
  }

  const {
    filters,
    availableScales,
    availableCompanies,
    availableCategories,
    availableEpochs,
    hasActiveFilters,
    onToggleScale,
    onToggleCompany,
    onToggleCategory,
    onToggleEpoch,
    onClear,
    onToggleSidebar
  }: Props = $props();

  function categoryLabel(cat: string): string {
    switch (cat) {
      case 'LOCOMOTIVES':
        return m.wishlist_category_locomotives();
      case 'FREIGHT_CARS':
        return m.wishlist_category_freight_cars();
      case 'PASSENGER_CARS':
        return m.wishlist_category_passenger_cars();
      case 'ELECTRIC_MULTIPLE_UNITS':
        return m.wishlist_category_electric_multiple_units();
      case 'RAILCARS':
        return m.wishlist_category_railcars();
      case 'TRAIN_SETS':
        return m.wishlist_category_train_sets();
      case 'STARTER_SETS':
        return m.wishlist_category_starter_sets();
      default:
        return cat;
    }
  }
</script>

<div class="flex h-full flex-col bg-[#0F0F0F]">
  <!-- Header -->
  <div class="flex flex-shrink-0 items-center justify-between border-b border-[#1F1F1F] px-4 py-3">
    <span
      class="text-[10px] font-semibold tracking-widest whitespace-nowrap text-[#808080] uppercase"
    >
      {m.collection_filters_title()}
    </span>
    <div class="flex items-center gap-2">
      {#if hasActiveFilters}
        <button
          type="button"
          class="rounded px-2 py-0.5 text-[10px] font-medium tracking-wide text-[#808080] uppercase transition-colors hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]"
          onclick={onClear}
        >
          {m.controlpanel_clear_all()}
        </button>
      {/if}
      <button
        type="button"
        class="rounded p-1 text-[#808080] transition-colors hover:text-[#D48A42]"
        onclick={onToggleSidebar}
        title={m.collection_toggle_filters_title()}
      >
        <X size={14} />
      </button>
    </div>
  </div>

  <!-- Scrollable filter sections -->
  <div class="flex-1 overflow-y-auto">
    <!-- Scales -->
    {#if availableScales.length > 0}
      <section class="border-b border-[#1F1F1F] px-4 py-3">
        <h4 class="mb-2 text-[10px] font-medium tracking-widest text-[#808080] uppercase">
          {m.controlpanel_section_scales()}
        </h4>
        <div class="flex flex-wrap gap-1.5">
          {#each availableScales as scale (scale.id)}
            <button
              type="button"
              class="cursor-pointer rounded-full border px-2.5 py-0.5 font-mono text-xs transition-colors
                {filters.scales.has(scale.id)
                ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                : 'border-[#1F1F1F] text-[#808080] hover:border-zinc-600 hover:text-zinc-300'}"
              onclick={() => onToggleScale?.(scale.id)}
            >
              {scale.display}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Companies -->
    {#if availableCompanies.length > 0}
      <section class="border-b border-[#1F1F1F] px-4 py-3">
        <h4 class="mb-2 text-[10px] font-medium tracking-widest text-[#808080] uppercase">
          {m.controlpanel_section_companies()}
        </h4>
        <div class="flex flex-wrap gap-1.5">
          {#each availableCompanies as company (company)}
            <button
              type="button"
              class="cursor-pointer rounded-full border px-2.5 py-0.5 text-xs transition-colors
                {filters.companies.has(company)
                ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                : 'border-[#1F1F1F] text-[#808080] hover:border-zinc-600 hover:text-zinc-300'}"
              onclick={() => onToggleCompany?.(company)}
            >
              {company}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Category -->
    {#if availableCategories.length > 0}
      <section class="border-b border-[#1F1F1F] px-4 py-3">
        <h4 class="mb-2 text-[10px] font-medium tracking-widest text-[#808080] uppercase">
          {m.controlpanel_section_categories()}
        </h4>
        <div class="flex flex-wrap gap-1.5">
          {#each availableCategories as category (category)}
            <button
              type="button"
              class="cursor-pointer rounded-full border px-2.5 py-0.5 text-xs transition-colors
                {filters.categories.has(category)
                ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                : 'border-[#1F1F1F] text-[#808080] hover:border-zinc-600 hover:text-zinc-300'}"
              onclick={() => onToggleCategory?.(category)}
            >
              {categoryLabel(category)}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Epochs -->
    {#if availableEpochs.length > 0}
      <section class="px-4 py-3">
        <h4 class="mb-2 text-[10px] font-medium tracking-widest text-[#808080] uppercase">
          {m.controlpanel_section_epochs()}
        </h4>
        <div class="flex flex-wrap gap-1.5">
          {#each availableEpochs as epoch (epoch)}
            <button
              type="button"
              class="cursor-pointer rounded-full border px-2.5 py-0.5 font-mono text-xs transition-colors
                {filters.epochs.has(epoch)
                ? 'border-[#D48A42] bg-[rgba(212,138,66,0.15)] text-[#D48A42]'
                : 'border-[#1F1F1F] text-[#808080] hover:border-zinc-600 hover:text-zinc-300'}"
              onclick={() => onToggleEpoch?.(epoch)}
            >
              {epoch}
            </button>
          {/each}
        </div>
      </section>
    {/if}
  </div>
</div>
