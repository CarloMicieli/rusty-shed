<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import FormInput from './FormInput.svelte';
  import FormSelect from './FormSelect.svelte';
  import ManufacturerSelect from './ManufacturerSelect.svelte';
  import EpochPicker from '$lib/components/drawer/EpochPicker.svelte';
  import type { Manufacturer } from '$lib/bindings';
  import { scaleOptions, categoryOptions, powerMethodOptions } from '$lib/utils/enum-options';

  interface Props {
    manufacturerId: string | null;
    productCode: string;
    description: string;
    category: string | null;
    scale: string | null;
    powerMethod: string | null;
    epoch: string | null;
    manufacturers: Manufacturer[];
    errors?: {
      manufacturerId?: string;
      productCode?: string;
      description?: string;
      category?: string;
      scale?: string;
      powerMethod?: string;
      epoch?: string;
    };
    isLoading?: boolean;
    disabled?: boolean;
  }

  let {
    manufacturerId = $bindable<string | null>(),
    productCode = $bindable(),
    description = $bindable(),
    category = $bindable<string | null>(),
    scale = $bindable<string | null>(),
    powerMethod = $bindable<string | null>(),
    epoch = $bindable<string | null>(),
    manufacturers,
    errors = {},
    isLoading = false,
    disabled = false
  }: Props = $props();
</script>

<div class="space-y-4">
  <div class="grid grid-cols-[2fr_1fr] gap-3">
    <ManufacturerSelect
      id="model-info-manufacturer"
      bind:manufacturerId
      {manufacturers}
      {isLoading}
      {disabled}
      required
      error={errors.manufacturerId}
    />

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

  <FormSelect
    label={m.wishlist_modal_category()}
    options={categoryOptions()}
    bind:value={category}
    placeholder={m.rolling_stock_select_category()}
    {disabled}
    error={errors.category}
  />

  <div class="grid grid-cols-2 gap-3">
    <FormSelect
      label={m.wishlist_modal_scale()}
      options={scaleOptions()}
      bind:value={scale}
      {disabled}
      error={errors.scale}
    />
    <FormSelect
      label={m.wishlist_modal_power_method()}
      options={powerMethodOptions()}
      bind:value={powerMethod}
      {disabled}
      error={errors.powerMethod}
    />
  </div>

  <EpochPicker
    label={m.wishlist_modal_epoch()}
    bind:value={epoch}
    {disabled}
    error={errors.epoch}
  />
</div>
