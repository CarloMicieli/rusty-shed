<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { WishlistPickerSection } from '$lib/components/drawer';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { commands } from '$lib/bindings';
  import type { Category, Manufacturer, PowerMethod, Scale, WishlistPriority } from '$lib/bindings';
  import { PRIORITIES, EPOCHS } from '$lib/features/wishlists/constants';
  import {
    CATEGORIES,
    SCALES,
    POWER_METHODS,
    categoryOptions,
    categoryLabel,
    scaleOptions,
    powerMethodOptions,
    powerMethodLabel,
    SCALE_DISPLAY_MAP
  } from '$lib/utils/enum-options';
  import { X } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { localeState } from '$lib/stores/locale.svelte';

  const wishlistService = getWishlistContext();

  interface Props {
    onClose: () => void;
    onSaved: () => void;
  }

  interface WishlistItemFormState {
    wishlistId: string;
    newListName: string;
    manufacturerId: string;
    productCode: string;
    description: string;
    category: Category;
    scale: Scale;
    powerMethod: PowerMethod;
    epoch: string;
    priority: WishlistPriority;
    desiredPrice: string;
    desiredPriceCurrency: string;
  }

  let { onClose, onSaved }: Props = $props();

  const wishlists = $derived(wishlistService.wishlists);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);

  let form = $state<WishlistItemFormState>({
    wishlistId: '',
    newListName: '',
    manufacturerId: '',
    productCode: '',
    description: '',
    category: CATEGORIES[0],
    scale: SCALES[0],
    powerMethod: POWER_METHODS[0],
    epoch: '',
    priority: 'NORMAL',
    desiredPrice: '',
    desiredPriceCurrency: 'EUR'
  });

  let manufacturers = $state<Manufacturer[]>([]);
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let formError = $state<string | null>(null);

  $effect(() => {
    if (defaultWishlist && form.wishlistId === '') {
      form.wishlistId = defaultWishlist.id;
    }
  });

  onMount(async () => {
    isLoadingData = true;
    const result = await commands.getManufacturers();
    if (result.status === 'ok') {
      manufacturers = result.data;
    }
    isLoadingData = false;
  });

  async function handleSubmit() {
    formError = null;

    if (!form.manufacturerId) {
      formError = m.wishlist_modal_missing_manufacturer();
      return;
    }
    if (!form.productCode.trim()) {
      formError = m.wishlist_modal_missing_product_code();
      return;
    }
    if (!form.description.trim()) {
      formError = m.wishlist_modal_missing_description();
      return;
    }
    if (form.desiredPrice !== '' && parseFloat(form.desiredPrice) <= 0) {
      formError = m.wishlist_modal_invalid_price();
      return;
    }

    isSubmitting = true;
    try {
      let targetId = form.wishlistId;

      if (form.newListName.trim()) {
        const created = await wishlistService.createWishlist(form.newListName.trim(), false);
        if (!created) {
          formError = m.wishlist_modal_create_failed();
          return;
        }
        targetId = created.id;
      }

      if (!targetId) {
        formError = m.wishlist_modal_select_list_error();
        return;
      }

      const priceAmount =
        form.desiredPrice !== ''
          ? (Math.round(parseFloat(form.desiredPrice) * 100) as number)
          : null;

      const success = await wishlistService.addRailwayModelToWishlist({
        railwayModel: {
          manufacturerId: form.manufacturerId,
          productCode: form.productCode.trim(),
          description: form.description.trim(),
          category: form.category,
          scale: form.scale,
          epoch: form.epoch,
          powerMethod: form.powerMethod,
          rollingStocks: []
        },
        wishlistId: targetId,
        priority: form.priority,
        status: null,
        desiredPriceAmount: priceAmount,
        desiredPriceCurrency: priceAmount !== null ? form.desiredPriceCurrency : null,
        notes: null,
        addedDate: null
      });

      if (!success) {
        formError = m.wishlist_modal_add_failed();
        return;
      }

      onSaved();
      close();
    } finally {
      isSubmitting = false;
    }
  }

  function close() {
    onClose();
    form = {
      wishlistId: defaultWishlist?.id ?? '',
      newListName: '',
      manufacturerId: '',
      productCode: '',
      description: '',
      category: CATEGORIES[0],
      scale: SCALES[0],
      powerMethod: POWER_METHODS[0],
      epoch: '',
      priority: 'NORMAL',
      desiredPrice: '',
      desiredPriceCurrency: 'EUR'
    };
    formError = null;
  }

  const PRIORITY_LABEL_FN_MAP: Record<WishlistPriority, () => string> = {
    LOW: m.wishlist_priority_low,
    NORMAL: m.wishlist_priority_normal,
    HIGH: m.wishlist_priority_high
  };

  function priorityLabel(priority: WishlistPriority): string {
    return PRIORITY_LABEL_FN_MAP[priority]();
  }

  const selectedManufacturer = $derived(
    manufacturers.find((mfr) => mfr.id === form.manufacturerId)
  );

  const categoryOptionList = $derived.by(() => {
    const locale = localeState.activeLocale;
    if (locale === 'it') {
      return categoryOptions();
    }
    return categoryOptions();
  });

  const scaleOptionList = $derived.by(() => {
    const locale = localeState.activeLocale;
    if (locale === 'it') {
      return scaleOptions();
    }
    return scaleOptions();
  });

  const powerMethodOptionList = $derived.by(() => {
    const locale = localeState.activeLocale;
    if (locale === 'it') {
      return powerMethodOptions();
    }
    return powerMethodOptions();
  });
</script>

{#snippet wishlistSelectionRow()}
  <WishlistPickerSection
    bind:wishlistId={form.wishlistId}
    bind:newListName={form.newListName}
    {wishlists}
    disabled={isSubmitting}
  />
{/snippet}

{#snippet modelDetailsSection()}
  <div class="space-y-3">
    <p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
      {m.wishlist_modal_model_details()}
    </p>

    <!-- Manufacturer + Product Code -->
    <div class="grid grid-cols-[2fr_1fr] gap-3">
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">
          {m.wishlist_modal_manufacturer()} *
        </span>
        {#if isLoadingData}
          <p class="text-sm text-muted-foreground">{m.wishlist_modal_loading()}</p>
        {:else}
          <Select.Root
            type="single"
            value={form.manufacturerId || undefined}
            onValueChange={(v) => {
              form.manufacturerId = v;
            }}
          >
            <Select.Trigger class="w-full" aria-label={m.wishlist_modal_manufacturer()}>
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
      </div>
      <div class="space-y-1">
        <label for="product-code" class="text-xs text-muted-foreground">
          {m.wishlist_modal_product_code()} *
        </label>
        <Input
          id="product-code"
          type="text"
          placeholder={m.wishlist_modal_product_code_placeholder()}
          bind:value={form.productCode}
        />
      </div>
    </div>

    <!-- Description -->
    <div class="space-y-1">
      <label for="description" class="text-xs text-muted-foreground">
        {m.wishlist_modal_description()} *
      </label>
      <Input
        id="description"
        type="text"
        placeholder={m.wishlist_modal_description_placeholder()}
        bind:value={form.description}
      />
    </div>

    <!-- Category -->
    <div class="space-y-1">
      <span class="text-xs text-muted-foreground">
        {m.wishlist_modal_category()}
      </span>
      <Select.Root
        type="single"
        value={form.category || undefined}
        onValueChange={(v) => {
          form.category = v as Category;
        }}
      >
        <Select.Trigger class="w-full">
          {#if form.category}
            {categoryLabel(form.category)}
          {:else}
            <span class="text-muted-foreground">—</span>
          {/if}
        </Select.Trigger>
        <Select.Content>
          {#each categoryOptionList as opt (opt.value)}
            <Select.Item value={opt.value} label={opt.label} />
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <!-- Scale + Power Method + Epoch -->
    <div class="grid grid-cols-3 gap-3">
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">
          {m.wishlist_modal_scale()}
        </span>
        <Select.Root
          type="single"
          value={form.scale || undefined}
          onValueChange={(v) => {
            form.scale = v as Scale;
          }}
        >
          <Select.Trigger class="w-full">
            {#if form.scale}
              {SCALE_DISPLAY_MAP[form.scale] ?? form.scale}
            {:else}
              <span class="text-muted-foreground">—</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each scaleOptionList as opt (opt.value)}
              <Select.Item value={opt.value} label={opt.label} />
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">
          {m.wishlist_modal_power_method()}
        </span>
        <Select.Root
          type="single"
          value={form.powerMethod || undefined}
          onValueChange={(v) => {
            form.powerMethod = v as PowerMethod;
          }}
        >
          <Select.Trigger class="w-full">
            {#if form.powerMethod}
              {powerMethodLabel(form.powerMethod)}
            {:else}
              <span class="text-muted-foreground">—</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each powerMethodOptionList as opt (opt.value)}
              <Select.Item value={opt.value} label={opt.label} />
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">
          {m.wishlist_modal_epoch()}
        </span>
        <Select.Root
          type="single"
          value={form.epoch || undefined}
          onValueChange={(v) => {
            form.epoch = v;
          }}
        >
          <Select.Trigger class="w-full">
            {#if form.epoch}
              {form.epoch}
            {:else}
              <span class="text-muted-foreground">{m.wishlist_modal_epoch_placeholder()}</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each EPOCHS as epoch (epoch)}
              <Select.Item value={epoch} label={epoch} />
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
  </div>
{/snippet}

{#snippet wishlistPreferencesSection()}
  <div class="space-y-3">
    <p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
      {m.wishlist_modal_wishlist_prefs()}
    </p>

    <div class="grid grid-cols-2 gap-3">
      <div class="space-y-1">
        <span class="text-xs text-muted-foreground">
          {m.wishlist_modal_priority()}
        </span>
        <Select.Root
          type="single"
          value={form.priority || undefined}
          onValueChange={(v) => {
            form.priority = v as WishlistPriority;
          }}
        >
          <Select.Trigger class="w-full">
            {#if form.priority}
              {priorityLabel(form.priority)}
            {:else}
              <span class="text-muted-foreground">—</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each PRIORITIES as priority (priority)}
              <Select.Item value={priority} label={priorityLabel(priority)} />
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="space-y-1">
        <label for="desired-price" class="text-xs text-muted-foreground">
          {m.wishlist_modal_desired_price()}
        </label>
        <Input
          id="desired-price"
          type="number"
          step="0.01"
          min="0"
          placeholder={m.wishlist_modal_price_placeholder()}
          bind:value={form.desiredPrice}
        />
      </div>
    </div>
  </div>
{/snippet}

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
  <div class="flex w-full max-w-2xl flex-col rounded-lg border border-border/70 bg-card shadow-xl">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border px-4 py-3">
      <h3 class="text-base font-semibold tracking-wide uppercase">{m.wishlist_modal_title()}</h3>
      <Button variant="ghost" size="sm" onclick={close} aria-label="close">
        <X size={16} />
      </Button>
    </div>

    <!-- Scrollable body -->
    <div class="max-h-[80vh] space-y-5 overflow-y-auto p-4">
      {@render wishlistSelectionRow()}
      {@render modelDetailsSection()}
      {@render wishlistPreferencesSection()}

      {#if formError}
        <div
          class="rounded-md border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
        >
          {formError}
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-2 border-t border-border px-4 py-3">
      <Button variant="ghost" size="sm" onclick={close} disabled={isSubmitting}>
        {m.wishlist_modal_cancel()}
      </Button>
      <Button variant="default" size="sm" onclick={handleSubmit} disabled={isSubmitting}>
        {isSubmitting ? m.wishlist_modal_saving() : m.wishlist_modal_save()}
      </Button>
    </div>
  </div>
</div>
