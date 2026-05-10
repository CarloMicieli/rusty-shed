<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import FormInput from './FormInput.svelte';
  import FormSelect from './FormSelect.svelte';
  import ManufacturerSelect from './ManufacturerSelect.svelte';
  import type { Manufacturer } from '$lib/bindings';
  import { categoryOptions } from '$lib/utils/enum-options';

  /**
   * Optional fields configuration - allows customizing which fields render.
   * By default, all model fields are rendered for consistency.
   */
  interface FieldsConfig {
    manufacturer?: boolean;
    productCode?: boolean;
    description?: boolean;
    category?: boolean;
  }

  interface Props {
    /**
     * Manufacturer ID - bindable for two-way binding with parent form state.
     */
    manufacturerId: string | null;
    /**
     * Product code - bindable for two-way binding with parent form state.
     */
    productCode: string;
    /**
     * Description - bindable for two-way binding with parent form state.
     */
    description: string;
    /**
     * Category - bindable for two-way binding with parent form state.
     */
    category: string | null;
    /**
     * List of available manufacturers for the dropdown.
     */
    manufacturers: Manufacturer[];
    /**
     * Error messages for each field (optional).
     */
    errors?: {
      manufacturerId?: string;
      productCode?: string;
      description?: string;
      category?: string;
    };
    /**
     * Whether form is disabled (e.g., during submission).
     */
    disabled?: boolean;
    /**
     * Whether to show required field indicators (*).
     */
    showRequired?: boolean;
    /**
     * Optional configuration to control which fields render.
     * By default, all fields render for consistency.
     */
    fieldsConfig?: FieldsConfig;
    /**
     * Optional ID prefix for form inputs (for accessibility).
     */
    idPrefix?: string;
    /**
     * Whether manufacturer select is loading (shows loading state).
     */
    manufacturerLoading?: boolean;
  }

  let {
    manufacturerId = $bindable<string | null>(),
    productCode = $bindable(),
    description = $bindable(),
    category = $bindable<string | null>(),
    manufacturers,
    errors = {},
    disabled = false,
    showRequired = true,
    fieldsConfig = {
      manufacturer: true,
      productCode: true,
      description: true,
      category: true
    },
    idPrefix = 'model-field',
    manufacturerLoading = false
  }: Props = $props();
</script>

<!-- Manufacturer + Product Code: 2-column grid -->
<div class="grid grid-cols-[2fr_1fr] gap-3">
  {#if fieldsConfig.manufacturer}
    <ManufacturerSelect
      id={`${idPrefix}-manufacturer`}
      bind:manufacturerId
      {manufacturers}
      isLoading={manufacturerLoading}
      {disabled}
      required={showRequired}
      error={errors.manufacturerId}
    />
  {/if}

  {#if fieldsConfig.productCode}
    <FormInput
      label={m.wishlist_modal_product_code()}
      id={`${idPrefix}-product-code`}
      type="text"
      placeholder={m.acquisition_item_product_code_placeholder()}
      bind:value={productCode}
      {disabled}
      required={showRequired}
      error={errors.productCode}
    />
  {/if}
</div>

<!-- Description: full width -->
{#if fieldsConfig.description}
  <FormInput
    label={m.wishlist_modal_description()}
    id={`${idPrefix}-description`}
    type="text"
    placeholder={m.acquisition_item_description_placeholder()}
    bind:value={description}
    {disabled}
    required={showRequired}
    error={errors.description}
  />
{/if}

<!-- Category: single column -->
{#if fieldsConfig.category}
  <FormSelect
    label={m.wishlist_modal_category()}
    options={categoryOptions()}
    bind:value={category}
    placeholder={m.acquisition_item_category_placeholder()}
    {disabled}
    required={showRequired}
    error={errors.category}
  />
{/if}
