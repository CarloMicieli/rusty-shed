<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { Input } from '$lib/components/ui/input';
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
    disabled = false
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
  <div class="flex items-center gap-2 border border-white/10 px-3 py-2">
    <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
      {m.drawer_section_model_info()}
    </span>
  </div>

  <!-- Manufacturer + Product Code -->
  <div class="grid grid-cols-[2fr_1fr] gap-3">
    <div class="space-y-1">
      <span class="text-xs text-zinc-400">
        {m.wishlist_modal_manufacturer()} *
      </span>
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
          <Select.Trigger class="w-full" aria-label={m.wishlist_modal_manufacturer()}>
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

    <div class="space-y-1">
      <label for="model-info-product-code" class="text-xs text-zinc-400">
        {m.wishlist_modal_product_code()} *
      </label>
      <Input
        id="model-info-product-code"
        type="text"
        placeholder={m.wishlist_modal_product_code_placeholder()}
        bind:value={productCode}
        {disabled}
      />
      {#if errors.productCode}
        <p class="text-xs text-destructive">{errors.productCode}</p>
      {/if}
    </div>
  </div>

  <!-- Description -->
  <div class="space-y-1">
    <label for="model-info-description" class="text-xs text-zinc-400">
      {m.wishlist_modal_description()} *
    </label>
    <Input
      id="model-info-description"
      type="text"
      placeholder={m.wishlist_modal_description_placeholder()}
      bind:value={description}
      {disabled}
    />
    {#if errors.description}
      <p class="text-xs text-destructive">{errors.description}</p>
    {/if}
  </div>

  <!-- Category -->
  <div class="space-y-1">
    <span class="text-xs text-zinc-400">
      {m.wishlist_modal_category()}
    </span>
    <Select.Root
      type="single"
      value={category || undefined}
      {disabled}
      onValueChange={(v) => {
        category = v;
      }}
    >
      <Select.Trigger class="w-full">
        {#if category}
          {getCategoryLabel(category)}
        {:else}
          <span class="text-zinc-500">—</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each CATEGORIES as cat (cat)}
          <Select.Item value={cat} label={getCategoryLabel(cat)} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <!-- Scale + Power Method -->
  <div class="grid grid-cols-2 gap-3">
    <div class="space-y-1">
      <span class="text-xs text-zinc-400">
        {m.wishlist_modal_scale()}
      </span>
      <Select.Root
        type="single"
        value={scale || undefined}
        {disabled}
        onValueChange={(v) => {
          scale = v;
        }}
      >
        <Select.Trigger class="w-full">
          {#if scale}
            {SCALE_DISPLAY_MAP[scale] ?? scale}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each SCALES as s (s)}
            <Select.Item value={s} label={SCALE_DISPLAY_MAP[s] ?? s} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <div class="space-y-1">
      <span class="text-xs text-zinc-400">
        {m.wishlist_modal_power_method()}
      </span>
      <Select.Root
        type="single"
        value={powerMethod || undefined}
        {disabled}
        onValueChange={(v) => {
          powerMethod = v;
        }}
      >
        <Select.Trigger class="w-full">
          {#if powerMethod}
            {getPowerMethodLabel(powerMethod)}
          {:else}
            <span class="text-zinc-500">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each POWER_METHODS as method (method)}
            <Select.Item value={method} label={getPowerMethodLabel(method)} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </div>

  <!-- Epoch -->
  <div class="space-y-1">
    <span class="text-xs text-zinc-400">
      {m.wishlist_modal_epoch()}
    </span>
    <Select.Root
      type="single"
      value={epoch ?? undefined}
      {disabled}
      onValueChange={(v) => {
        epoch = v;
      }}
    >
      <Select.Trigger class="w-full">
        {#if epoch}
          {epoch}
        {:else}
          <span class="text-zinc-500">{m.wishlist_modal_epoch_placeholder()}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each EPOCHS as ep (ep)}
          <Select.Item value={ep} label={ep} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
</div>
