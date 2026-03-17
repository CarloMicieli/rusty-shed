<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { DrawerSectionBar, FormInput, FormSelect } from '$lib/components/drawer';
  import type { Manufacturer } from '$lib/bindings';
  import { CATEGORIES, SCALES, POWER_METHODS, EPOCHS } from '$lib/features/wishlists/constants';

  interface Props {
    manufacturerId: string;
    productCode: string;
    description: string;
    category: string;
    scale: string;
    powerMethod: string;
    epoch: string | null;
    manufacturers: Manufacturer[];
    errors?: {
      manufacturerId?: string;
      productCode?: string;
      description?: string;
    };
    isLoading?: boolean;
    disabled?: boolean;
    expanded?: boolean;
  }

  let {
    manufacturerId = $bindable(),
    productCode = $bindable(),
    description = $bindable(),
    category = $bindable(),
    scale = $bindable(),
    powerMethod = $bindable(),
    epoch = $bindable<string | null>(),
    manufacturers,
    errors = {},
    isLoading = false,
    disabled = false,
    expanded = $bindable(true)
  }: Props = $props();

  const SCALE_DISPLAY_MAP: Record<string, string> = {
    H0: 'H0 (1:87)',
    H0m: 'H0m (1:87)',
    H0e: 'H0e (1:87)',
    N: 'N (1:160)',
    TT: 'TT (1:120)',
    Z: 'Z (1:220)',
    G: 'G (1:22.5)',
    Scale1: 'Scale 1 (1:32)',
    Scale0: 'Scale 0 (1:43)',
    Scale00: 'Scale 00 (1:76)'
  };

  const CATEGORY_LABELS: Record<string, () => string> = {
    LOCOMOTIVES: m.wishlist_category_locomotives,
    TRAIN_SETS: m.wishlist_category_train_sets,
    STARTER_SETS: m.wishlist_category_starter_sets,
    FREIGHT_CARS: m.wishlist_category_freight_cars,
    PASSENGER_CARS: m.wishlist_category_passenger_cars,
    ELECTRIC_MULTIPLE_UNITS: m.wishlist_category_electric_multiple_units,
    RAILCARS: m.wishlist_category_railcars
  };

  const POWER_METHOD_LABELS: Record<string, () => string> = {
    AC: m.wishlist_power_ac,
    DC: m.wishlist_power_dc,
    TRIX_EXPRESS: m.wishlist_power_trix_express
  };

  function getCategoryLabel(cat: string): string {
    return CATEGORY_LABELS[cat]?.() ?? cat;
  }

  function getPowerMethodLabel(method: string): string {
    return POWER_METHOD_LABELS[method]?.() ?? method;
  }

  const selectedManufacturer = $derived(manufacturers.find((mfr) => mfr.id === manufacturerId));
</script>

<div class="space-y-3">
  <DrawerSectionBar
    label={m.drawer_section_model_info()}
    {expanded}
    onToggle={() => (expanded = !expanded)}
  />

  {#if expanded}
    <!-- Manufacturer + Product Code -->
    <div class="grid grid-cols-[2fr_1fr] gap-3">
      <!-- Manufacturer: kept inline due to loading state -->
      <div class="space-y-1">
        <span class="text-xs text-zinc-400">{m.wishlist_modal_manufacturer()} *</span>
        {#if isLoading}
          <p class="text-sm text-zinc-500">{m.wishlist_modal_loading()}</p>
        {:else}
          <Select.Root
            type="single"
            value={manufacturerId || undefined}
            {disabled}
            onValueChange={(v) => {
              manufacturerId = v;
            }}
          >
            <Select.Trigger
              class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]"
              aria-label={m.wishlist_modal_manufacturer()}
            >
              {#if selectedManufacturer}
                {selectedManufacturer.name}
              {:else}
                <span class="text-zinc-500">{m.wishlist_modal_manufacturer_placeholder()}</span>
              {/if}
            </Select.Trigger>
            <Select.Content>
              {#each manufacturers as mfr (mfr.id)}
                <Select.Item value={mfr.id} label={mfr.name} />
              {/each}
            </Select.Content>
          </Select.Root>
        {/if}
        {#if errors.manufacturerId}
          <p class="text-xs text-destructive">{errors.manufacturerId}</p>
        {/if}
      </div>

      <FormInput
        label={m.wishlist_modal_product_code()}
        id="model-info-product-code"
        type="text"
        placeholder={m.wishlist_modal_product_code_placeholder()}
        bind:value={productCode}
        {disabled}
        required
        error={errors.productCode}
      />
    </div>

    <!-- Description -->
    <FormInput
      label={m.wishlist_modal_description()}
      id="model-info-description"
      type="text"
      placeholder={m.wishlist_modal_description_placeholder()}
      bind:value={description}
      {disabled}
      required
      error={errors.description}
    />

    <!-- Category -->
    <FormSelect
      label={m.wishlist_modal_category()}
      options={CATEGORIES.map((cat) => ({ value: cat, label: getCategoryLabel(cat) }))}
      bind:value={category}
      {disabled}
    />

    <!-- Scale + Power Method -->
    <div class="grid grid-cols-2 gap-3">
      <FormSelect
        label={m.wishlist_modal_scale()}
        options={SCALES.map((s) => ({ value: s, label: SCALE_DISPLAY_MAP[s] ?? s }))}
        bind:value={scale}
        {disabled}
      />
      <FormSelect
        label={m.wishlist_modal_power_method()}
        options={POWER_METHODS.map((method) => ({
          value: method,
          label: getPowerMethodLabel(method)
        }))}
        bind:value={powerMethod}
        {disabled}
      />
    </div>

    <!-- Epoch -->
    <FormSelect
      label={m.wishlist_modal_epoch()}
      options={EPOCHS.map((ep) => ({ value: ep, label: ep }))}
      bind:value={epoch}
      placeholder={m.wishlist_modal_epoch_placeholder()}
      {disabled}
    />
  {/if}
</div>
