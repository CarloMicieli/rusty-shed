<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { CurrencyInput } from '$lib/components';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { getCurrencySymbol } from '$lib/utils/currency';
  import { commands } from '$lib/bindings';
  import type { Manufacturer, WishlistPriority } from '$lib/bindings';
  import {
    CATEGORIES,
    SCALES,
    POWER_METHODS,
    PRIORITIES,
    EPOCHS
  } from '$lib/features/wishlists/constants';
  import { X, Heart } from 'lucide-svelte';
  import { onMount } from 'svelte';

  const wishlistService = getWishlistContext();

  interface Props {
    open: boolean;
    onClose: () => void;
    onSaved: () => void;
  }

  interface WishlistItemFormState {
    wishlistId: string;
    newListName: string;
    manufacturerId: string;
    productCode: string;
    description: string;
    category: string;
    scale: string;
    powerMethod: string;
    epoch: string;
    priority: WishlistPriority;
    desiredPrice: number | null;
  }

  let { open, onClose, onSaved }: Props = $props();

  const wishlists = $derived(wishlistService.wishlists);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);

  function makeDefaultForm(): WishlistItemFormState {
    return {
      wishlistId: defaultWishlist?.id ?? '',
      newListName: '',
      manufacturerId: '',
      productCode: '',
      description: '',
      category: CATEGORIES[0],
      scale: settingsState.settings?.favouriteScale || SCALES[0],
      powerMethod: settingsState.settings?.powerMethod || POWER_METHODS[0],
      epoch: '',
      priority: 'NORMAL',
      desiredPrice: null
    };
  }

  let form = $state<WishlistItemFormState>(makeDefaultForm());

  let manufacturers = $state<Manufacturer[]>([]);
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let formError = $state<string | null>(null);
  let showDiscardDialog = $state(false);

  const isDropdownDisabled = $derived(form.newListName.trim() !== '' || wishlists.length === 0);
  const currency = $derived(settingsState.settings.currency ?? 'EUR');
  const currencySymbol = $derived(getCurrencySymbol(currency));
  const selectedWishlist = $derived(wishlists.find((l) => l.id === form.wishlistId));
  const selectedManufacturer = $derived(
    manufacturers.find((mfr) => mfr.id === form.manufacturerId)
  );

  let hasChanges = $derived(
    form.manufacturerId !== '' ||
      form.productCode.trim() !== '' ||
      form.description.trim() !== '' ||
      form.newListName.trim() !== ''
  );

  $effect(() => {
    if (defaultWishlist && form.wishlistId === '') {
      form.wishlistId = defaultWishlist.id;
    }
  });

  // Lock body scroll when drawer is open
  $effect(() => {
    if (open) {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = 'hidden';
      if (mainElement) mainElement.style.overflow = 'hidden';
    } else {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    }
    return () => {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    };
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
    if (form.desiredPrice !== null && form.desiredPrice <= 0) {
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
        form.desiredPrice !== null ? (form.desiredPrice as unknown as bigint) : null;

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
        desiredPriceCurrency: priceAmount !== null ? currency : null,
        notes: null,
        addedDate: null
      });

      if (!success) {
        formError = m.wishlist_modal_add_failed();
        return;
      }

      onSaved();
      resetAndClose();
    } finally {
      isSubmitting = false;
    }
  }

  function resetAndClose() {
    onClose();
    form = makeDefaultForm();
    formError = null;
    showDiscardDialog = false;
  }

  function handleCloseRequest() {
    if (hasChanges) {
      showDiscardDialog = true;
    } else {
      resetAndClose();
    }
  }

  function handleDiscardConfirm() {
    showDiscardDialog = false;
    resetAndClose();
  }

  function handleDiscardCancel() {
    showDiscardDialog = false;
  }

  function getCategoryLabelKey(category: string): string {
    const labelMap: Record<string, string> = {
      LOCOMOTIVES: 'wishlist_category_locomotives',
      TRAIN_SETS: 'wishlist_category_train_sets',
      STARTER_SETS: 'wishlist_category_starter_sets',
      FREIGHT_CARS: 'wishlist_category_freight_cars',
      PASSENGER_CARS: 'wishlist_category_passenger_cars',
      ELECTRIC_MULTIPLE_UNITS: 'wishlist_category_electric_multiple_units',
      RAILCARS: 'wishlist_category_railcars'
    };
    return labelMap[category] ?? category;
  }

  function getPowerMethodLabelKey(method: string): string {
    const labelMap: Record<string, string> = {
      AC: 'wishlist_power_ac',
      DC: 'wishlist_power_dc',
      TRIX_EXPRESS: 'wishlist_power_trix_express'
    };
    return labelMap[method] ?? method;
  }

  function getPriorityLabelKey(priority: string): string {
    const labelMap: Record<string, string> = {
      LOW: 'wishlist_priority_low',
      NORMAL: 'wishlist_priority_normal',
      HIGH: 'wishlist_priority_high'
    };
    return labelMap[priority] ?? priority;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const messages = m as any as Record<string, () => string>;
  function getMessage(key: string): string {
    return messages[key]();
  }
</script>

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>

  <!-- Drawer panel -->
  <div
    class="fixed inset-y-0 right-0 z-50 flex w-full max-w-lg flex-col bg-zinc-950 shadow-2xl"
    role="dialog"
    aria-modal="true"
    aria-labelledby="wishlist-item-drawer-title"
  >
    <!-- Sticky header -->
    <div class="flex items-center justify-between border-b border-white/10 p-4">
      <div class="flex items-center gap-3">
        <div class="rounded-lg bg-rose-500/10 p-2">
          <Heart class="h-5 w-5 text-rose-500" />
        </div>
        <div>
          <h2 id="wishlist-item-drawer-title" class="text-lg font-semibold text-zinc-100">
            {m.wishlist_modal_title()}
          </h2>
          <p class="text-sm text-zinc-500">{m.wishlist_add_item_drawer_subtitle()}</p>
        </div>
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        onclick={handleCloseRequest}
        aria-label="close"
      >
        <X size={16} />
      </Button>
    </div>

    <!-- Scrollable body -->
    <div class="flex-1 space-y-5 overflow-y-auto p-4">
      <!-- Section: Choose or Create Wishlist -->
      <div class="space-y-2">
        <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          {m.wishlist_modal_choose_or_create()}
        </span>
        <div class="grid grid-cols-2 gap-3">
          <Select.Root
            type="single"
            value={form.wishlistId || undefined}
            disabled={isDropdownDisabled}
            onValueChange={(v) => {
              form.wishlistId = v;
            }}
          >
            <Select.Trigger class="w-full" aria-label={m.wishlist_modal_select_list()}>
              {#if selectedWishlist}
                {selectedWishlist.name}{#if selectedWishlist.isDefault}
                  (default){/if}
              {:else}
                <span class="text-zinc-500">{m.wishlist_modal_select_placeholder()}</span>
              {/if}
            </Select.Trigger>
            <Select.Content>
              {#each wishlists as list (list.id)}
                <Select.Item value={list.id} label={list.name}>
                  {list.name}{#if list.isDefault}
                    (default){/if}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
          <Input
            type="text"
            placeholder={m.wishlist_modal_new_list_placeholder()}
            bind:value={form.newListName}
          />
        </div>
      </div>

      <div class="border-t border-white/10 pt-4"></div>

      <!-- Section: Model Details -->
      <div class="space-y-3">
        <p class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          {m.wishlist_modal_model_details()}
        </p>

        <!-- Manufacturer + Product Code -->
        <div class="grid grid-cols-[2fr_1fr] gap-3">
          <div class="space-y-1">
            <span class="text-xs text-zinc-400">
              {m.wishlist_modal_manufacturer()} *
            </span>
            {#if isLoadingData}
              <p class="text-sm text-zinc-500">{m.wishlist_modal_loading()}</p>
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
          </div>
          <div class="space-y-1">
            <label for="wishlist-drawer-product-code" class="text-xs text-zinc-400">
              {m.wishlist_modal_product_code()} *
            </label>
            <Input
              id="wishlist-drawer-product-code"
              type="text"
              placeholder={m.wishlist_modal_product_code_placeholder()}
              bind:value={form.productCode}
            />
          </div>
        </div>

        <!-- Description -->
        <div class="space-y-1">
          <label for="wishlist-drawer-description" class="text-xs text-zinc-400">
            {m.wishlist_modal_description()} *
          </label>
          <Input
            id="wishlist-drawer-description"
            type="text"
            placeholder={m.wishlist_modal_description_placeholder()}
            bind:value={form.description}
          />
        </div>

        <!-- Category (full width) -->
        <div class="space-y-1">
          <span class="text-xs text-zinc-400">
            {m.wishlist_modal_category()}
          </span>
          <Select.Root
            type="single"
            value={form.category || undefined}
            onValueChange={(v) => {
              form.category = v;
            }}
          >
            <Select.Trigger class="w-full">
              {#if form.category}
                {getMessage(getCategoryLabelKey(form.category))}
              {:else}
                <span class="text-zinc-500">—</span>
              {/if}
            </Select.Trigger>
            <Select.Content>
              {#each CATEGORIES as cat (cat)}
                <Select.Item value={cat} label={getMessage(getCategoryLabelKey(cat))} />
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
              value={form.scale || undefined}
              onValueChange={(v) => {
                form.scale = v;
              }}
            >
              <Select.Trigger class="w-full">
                {#if form.scale}
                  {form.scale}
                {:else}
                  <span class="text-zinc-500">—</span>
                {/if}
              </Select.Trigger>
              <Select.Content>
                {#each SCALES as scale (scale)}
                  <Select.Item value={scale} label={scale} />
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
              value={form.powerMethod || undefined}
              onValueChange={(v) => {
                form.powerMethod = v;
              }}
            >
              <Select.Trigger class="w-full">
                {#if form.powerMethod}
                  {getMessage(getPowerMethodLabelKey(form.powerMethod))}
                {:else}
                  <span class="text-zinc-500">—</span>
                {/if}
              </Select.Trigger>
              <Select.Content>
                {#each POWER_METHODS as method (method)}
                  <Select.Item value={method} label={getMessage(getPowerMethodLabelKey(method))} />
                {/each}
              </Select.Content>
            </Select.Root>
          </div>
        </div>

        <!-- Epoch (full width) -->
        <div class="space-y-1">
          <span class="text-xs text-zinc-400">
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
                <span class="text-zinc-500">{m.wishlist_modal_epoch_placeholder()}</span>
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

      <div class="border-t border-white/10 pt-4"></div>

      <!-- Section: Wishlist Preferences -->
      <div class="space-y-3">
        <p class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
          {m.wishlist_modal_wishlist_prefs()}
        </p>

        <!-- Priority: colored button group -->
        <div class="space-y-1">
          <span class="text-xs text-zinc-400">
            {m.wishlist_modal_priority()}
          </span>
          <div class="flex gap-2">
            {#each PRIORITIES as p (p)}
              <button
                type="button"
                class={[
                  'flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors',
                  form.priority === p
                    ? p === 'LOW'
                      ? 'bg-zinc-700 text-zinc-200 ring-1 ring-zinc-500'
                      : p === 'NORMAL'
                        ? 'bg-blue-900 text-blue-200 ring-1 ring-blue-600'
                        : 'bg-amber-900 text-amber-200 ring-1 ring-amber-600'
                    : 'bg-zinc-900 text-zinc-500 hover:bg-zinc-800'
                ].join(' ')}
                onclick={() => (form.priority = p)}
              >
                {getMessage(getPriorityLabelKey(p))}
              </button>
            {/each}
          </div>
        </div>

        <!-- Desired Price -->
        <div class="space-y-1">
          <label for="wishlist-drawer-desired-price" class="text-xs text-zinc-400">
            {m.wishlist_modal_desired_price()}
          </label>
          <CurrencyInput
            id="wishlist-drawer-desired-price"
            bind:value={form.desiredPrice}
            symbol={currencySymbol}
            label={m.wishlist_modal_desired_price()}
          />
        </div>
      </div>

      {#if formError}
        <div
          class="rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
        >
          {formError}
        </div>
      {/if}
    </div>

    <!-- Sticky footer -->
    <div class="flex items-center justify-end gap-2 border-t border-white/10 p-4">
      <Button
        type="button"
        variant="ghost"
        size="sm"
        onclick={handleCloseRequest}
        disabled={isSubmitting}
      >
        {m.wishlist_modal_cancel()}
      </Button>
      <Button
        type="button"
        variant="default"
        size="sm"
        onclick={handleSubmit}
        disabled={isSubmitting}
      >
        {isSubmitting ? m.wishlist_modal_saving() : m.wishlist_modal_save()}
      </Button>
    </div>
  </div>

  <!-- Discard changes confirmation dialog -->
  {#if showDiscardDialog}
    <div
      class="fixed inset-0 z-[60] flex items-center justify-center bg-background/80 backdrop-blur-sm"
    >
      <div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl">
        <h3 class="mb-2 text-lg font-bold text-foreground">
          {m.wishlist_add_item_drawer_discard_title()}
        </h3>
        <p class="mb-4 text-muted-foreground">
          {m.wishlist_add_item_drawer_discard_description()}
        </p>
        <div class="flex justify-end gap-3">
          <Button type="button" variant="ghost" onclick={handleDiscardCancel}>
            {m.wishlist_add_item_drawer_discard_cancel()}
          </Button>
          <Button type="button" variant="destructive" onclick={handleDiscardConfirm}>
            {m.wishlist_add_item_drawer_discard_confirm()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
