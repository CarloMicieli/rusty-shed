<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { FormInput, FormSelect } from '$lib/components/drawer';
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

  const selectedManufacturer = $derived(manufacturers.find((mfr) => mfr.id === manufacturerId));
</script>

<div class="overflow-hidden rounded-sm border border-border bg-card p-4">
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
      {m.drawer_section_model_info()}
    </p>
    <div class="space-y-4">
      <!-- Manufacturer + Product Code -->
      <div class="grid grid-cols-[2fr_1fr] gap-3">
        <!-- Manufacturer: kept inline due to loading state -->
        <div class="space-y-1">
          <span class="text-[10px] font-bold text-muted-foreground uppercase"
            >{m.wishlist_modal_manufacturer()} *</span
          >
          {#if isLoading}
            <p class="font-mono text-xs text-muted-foreground">{m.wishlist_modal_loading()}</p>
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
                class="w-full border-border bg-background text-foreground"
                aria-label={m.wishlist_modal_manufacturer()}
              >
                {#if selectedManufacturer}
                  {selectedManufacturer.name}
                {:else}
                  <span class="text-muted-foreground"
                    >{m.wishlist_modal_manufacturer_placeholder()}</span
                  >
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
        options={categoryOptions()}
        bind:value={category}
        placeholder={m.rolling_stock_select_category()}
        {disabled}
        error={errors.category}
      />

      <!-- Scale + Power Method -->
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

      <!-- Epoch -->
      <EpochPicker
        label={m.wishlist_modal_epoch()}
        bind:value={epoch}
        {disabled}
        error={errors.epoch}
      />
    </div>
  </section>
</div>
