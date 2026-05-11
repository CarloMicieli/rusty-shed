<script lang="ts" module>
  import type { PrototypeView as CachedPrototypeView } from '$lib/bindings';
  import { commands } from '$lib/bindings';

  let cachedPrototypes: CachedPrototypeView[] | null = null;
  let prototypeRequest: Promise<CachedPrototypeView[]> | null = null;

  async function loadPrototypes(): Promise<CachedPrototypeView[]> {
    if (cachedPrototypes) return cachedPrototypes;
    if (!prototypeRequest) {
      prototypeRequest = commands.getPrototypes(null).then((result) => {
        if (result.status === 'ok') {
          cachedPrototypes = result.data.flatMap((group) => group.prototypes);
          return cachedPrototypes;
        }
        return [];
      });
    }

    return prototypeRequest;
  }
</script>

<script lang="ts">
  import { onMount } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Input, Button } from '$lib/components';
  import { FormSelect } from '$lib/components/drawer';
  import type { PrototypeView, RailwayCompany, RollingStockCategory } from '$lib/bindings';
  import type { RollingStockFormEntry } from '$lib/features/collection/types/AddModelFormTypes';
  import rollingStockCategories from '$lib/data/constants/rollingStockCategories.json';
  import { getSubcategoryOptions } from '$lib/components/model-details/components/constants';
  import { getFlag } from '$lib/utils/flags';

  interface Props {
    /** Entry data bound two-way */
    entry: RollingStockFormEntry;
    /** Available railway companies for dropdown */
    railwayCompanies: RailwayCompany[];
    /** Whether remove button is enabled (disabled if only entry) */
    canRemove: boolean;
    /** Callback to remove this entry */
    onRemove: () => void;
    /** Validation errors for this entry */
    errors?: {
      railwayCompanyId?: string;
      seriesCode?: string;
      category?: string;
    };
    /** Enable mechanical dark mode styling */
    dark?: boolean;
  }

  let {
    entry = $bindable(),
    railwayCompanies,
    canRemove,
    onRemove,
    errors,
    dark = false
  }: Props = $props();

  const darkInput =
    'flex h-10 w-full rounded-md border border-layout-border bg-transparent px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-2 focus:ring-primary/30 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50';

  // Subcategory options depend on selected category
  const subcategoryOptions = $derived(
    getSubcategoryOptions(entry.category as RollingStockCategory | null).map((o) => ({
      value: o.id,
      label: o.label
    }))
  );

  // When category changes, reset subcategory
  $effect(() => {
    if (subcategoryOptions.length === 0 && entry.subcategory) {
      entry.subcategory = null;
    }
  });

  // Map options to { value, label }[] for FormSelect
  const companyOptions = $derived(
    railwayCompanies.map((c) => ({
      value: c.id,
      label: c.name,
      countryCode: c.countryCode,
      registeredCompanyName: c.registeredCompanyName
    }))
  );

  const categoryLabelMap: Record<string, () => string> = {
    enum_category_locomotives: m.enum_category_locomotives,
    enum_category_passenger_cars: m.enum_category_passenger_cars,
    enum_category_freight_cars: m.enum_category_freight_cars,
    enum_category_railcars: m.enum_category_railcars,
    enum_category_electric_multiple_units: m.enum_category_electric_multiple_units
  };

  const categoryOptions = $derived(
    rollingStockCategories.map((cat) => ({
      value: cat.id,
      label: categoryLabelMap[cat.labelKey]?.() ?? cat.labelKey
    }))
  );

  let prototypes = $state<PrototypeView[]>([]);
  let searchPrototypeData = $state('');
  let prototypeSearchOpen = $state(false);
  let isPrototypeLoading = $state(false);

  onMount(async () => {
    isPrototypeLoading = true;
    prototypes = await loadPrototypes();
    isPrototypeLoading = false;
  });

  const filteredPrototypes = $derived.by(() => {
    const query = searchPrototypeData.trim().toLowerCase();
    if (!query) return [];

    return prototypes
      .filter(
        (prototype) =>
          prototype.series_code.toLowerCase().includes(query) ||
          (prototype.friendly_name ?? '').toLowerCase().includes(query)
      )
      .slice(0, 8);
  });

  function mapPrototypeCategory(specificationType: string): RollingStockCategory | null {
    switch (specificationType) {
      case 'LOCOMOTIVE':
      case 'PASSENGER_CAR':
      case 'FREIGHT_CAR':
      case 'RAILCAR':
      case 'ELECTRIC_MULTIPLE_UNIT':
        return specificationType;
      default:
        return null;
    }
  }

  function selectPrototype(prototype: PrototypeView) {
    entry.seriesCode = prototype.series_code;

    const mappedCategory = mapPrototypeCategory(prototype.specification_type);
    if (mappedCategory) {
      entry.category = mappedCategory;
    }

    searchPrototypeData = prototype.friendly_name
      ? `${prototype.series_code} - ${prototype.friendly_name}`
      : prototype.series_code;
    prototypeSearchOpen = false;
  }
</script>

<div
  class="rolling-stock-entry grid grid-cols-1 gap-4 rounded-lg border p-4"
  class:border-layout-border={dark}
  class:bg-layout-surface={dark}
  class:border-border={!dark}
  class:bg-card={!dark}
  class:text-card-foreground={!dark}
>
  <div class="space-y-1">
    <label
      for="prototype-search-{entry.uid}"
      class="block text-[10px] text-muted-foreground uppercase"
    >
      {m.add_model_search_prototype_data()}
    </label>
    <div class="relative">
      <Input
        id="prototype-search-{entry.uid}"
        type="text"
        bind:value={searchPrototypeData}
        placeholder={m.rolling_stock_prototype_search_placeholder()}
        class={dark ? `${darkInput} font-mono` : 'w-full font-mono'}
        onfocus={() => (prototypeSearchOpen = true)}
        oninput={() => (prototypeSearchOpen = true)}
        onblur={() => {
          setTimeout(() => {
            prototypeSearchOpen = false;
          }, 120);
        }}
      />

      {#if prototypeSearchOpen && searchPrototypeData.trim().length > 0}
        <div
          class="absolute z-20 mt-1 max-h-52 w-full overflow-y-auto rounded-md border border-layout-border bg-zinc-900 shadow-xl"
        >
          {#if isPrototypeLoading}
            <p class="px-3 py-2 text-xs text-muted-foreground">
              {m.add_model_searching_prototypes()}
            </p>
          {:else if filteredPrototypes.length === 0}
            <p class="px-3 py-2 text-xs text-muted-foreground">
              {m.rolling_stock_prototype_no_results()}
            </p>
          {:else}
            {#each filteredPrototypes as prototype (prototype.id)}
              <button
                type="button"
                class="flex w-full items-center justify-between px-3 py-2 text-left hover:bg-primary/15"
                onclick={() => selectPrototype(prototype)}
              >
                <span class="font-mono text-xs text-foreground">{prototype.series_code}</span>
                <span class="truncate pl-3 text-xs text-muted-foreground"
                  >{prototype.friendly_name ?? prototype.company_name}</span
                >
              </button>
            {/each}
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Row 1: Railway Company | (empty) -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <FormSelect
      id="railway-company-{entry.uid}"
      label={m.add_model_railway_company()}
      options={companyOptions}
      bind:value={entry.railwayCompanyId}
      placeholder="-- {m.add_model_railway_company()} --"
      error={errors?.railwayCompanyId}
      isSearchable
      required
    >
      {#snippet item(opt)}
        <div class="flex items-center gap-3">
          <div class="flex shrink-0 items-center gap-1.5">
            <span class="text-lg leading-none" aria-hidden="true">{getFlag(opt.countryCode)}</span>
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
            <span class="text-base leading-none" aria-hidden="true">{getFlag(opt.countryCode)}</span
            >
            <span class="font-bold text-foreground">{opt.label}</span>
          </div>
        {:else}
          <span class="text-muted-foreground">-- {m.add_model_railway_company()} --</span>
        {/if}
      {/snippet}
    </FormSelect>
    <div></div>
  </div>

  <!-- Row 2: Series Code | Road Number -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <div>
      <label for="series-code-{entry.uid}" class="block space-y-1">
        {#if dark}
          <span class="text-[10px] text-muted-foreground uppercase"
            >{m.add_model_series_code()}</span
          >
        {:else}
          <span class="text-sm text-muted-foreground">{m.add_model_series_code()}</span>
        {/if}
      </label>
      <Input
        id="series-code-{entry.uid}"
        type="text"
        bind:value={entry.seriesCode}
        placeholder={m.rolling_stock_placeholder_series_code()}
        class={dark ? `${darkInput} font-mono` : 'w-full font-mono'}
        aria-describedby={errors?.seriesCode ? `series-code-error-{entry.uid}` : undefined}
      />
      {#if errors?.seriesCode}
        <p id="series-code-error-{entry.uid}" class="text-error-500 mt-1 text-sm">
          {errors.seriesCode}
        </p>
      {/if}
    </div>

    <div>
      <label for="road-number-{entry.uid}" class="block space-y-1">
        {#if dark}
          <span class="text-[10px] text-muted-foreground uppercase"
            >{m.add_model_road_number()}</span
          >
          <span class="ml-1 text-muted-foreground/50">(optional)</span>
        {:else}
          <span class="text-sm text-muted-foreground">{m.add_model_road_number()}</span>
          <span class="ml-1 text-xs text-muted-foreground/60">(optional)</span>
        {/if}
      </label>
      <Input
        id="road-number-{entry.uid}"
        type="text"
        bind:value={entry.roadNumber}
        placeholder={m.rolling_stock_placeholder_road_number()}
        class={dark ? `${darkInput} font-mono` : 'w-full font-mono'}
      />
    </div>
  </div>

  <!-- Row 3: Category | Subcategory (shown only when options exist) -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
    <FormSelect
      id="category-{entry.uid}"
      label={m.add_model_rs_category()}
      options={categoryOptions}
      bind:value={entry.category}
      placeholder="-- {m.add_model_rs_category()} --"
      error={errors?.category}
      required
    />

    {#if subcategoryOptions.length > 0}
      <FormSelect
        id="subcategory-{entry.uid}"
        label={m.add_model_rs_subcategory()}
        options={subcategoryOptions}
        bind:value={entry.subcategory}
        placeholder="-- {m.add_model_rs_subcategory()} --"
      />
    {:else}
      <div></div>
    {/if}
  </div>

  <!-- Remove Button -->
  <div class="flex justify-end">
    {#if dark}
      <button
        type="button"
        class="text-[10px] tracking-widest text-muted-foreground/60 uppercase hover:text-red-400 disabled:pointer-events-none disabled:opacity-30"
        disabled={!canRemove}
        onclick={onRemove}
        aria-label={m.add_model_remove_rolling_stock()}
      >
        {m.add_model_remove_rolling_stock()}
      </button>
    {:else}
      <Button
        type="button"
        variant="ghost"
        size="sm"
        class="text-destructive hover:bg-destructive/10"
        disabled={!canRemove}
        onclick={onRemove}
        aria-label={m.add_model_remove_rolling_stock()}
      >
        <X size={16} />
        <span>{m.add_model_remove_rolling_stock()}</span>
      </Button>
    {/if}
  </div>
</div>
