<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Textarea } from '$lib/components/ui/textarea';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { commands } from '$lib/bindings';
  import type { Manufacturer, WishlistPriority } from '$lib/bindings';
  import { CATEGORIES, SCALES, POWER_METHODS, PRIORITIES } from '$lib/features/wishlists/constants';
  import { X } from 'lucide-svelte';
  import { onMount } from 'svelte';

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
    category: string;
    scale: string;
    powerMethod: string;
    epoch: string;
    priority: WishlistPriority;
    desiredPrice: string;
    desiredPriceCurrency: string;
    notes: string;
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
    desiredPriceCurrency: 'EUR',
    notes: ''
  });

  let manufacturers = $state<Manufacturer[]>([]);
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let formError = $state<string | null>(null);

  const isDropdownDisabled = $derived(form.newListName.trim() !== '');

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
        form.desiredPrice !== '' ? BigInt(Math.round(parseFloat(form.desiredPrice) * 100)) : null;

      const success = await wishlistService.addRailwayModelToWishlist({
        railwayModel: {
          manufacturerId: form.manufacturerId,
          productCode: form.productCode.trim(),
          description: form.description.trim(),
          category: form.category,
          scale: form.scale,
          epoch: form.epoch.trim(),
          powerMethod: form.powerMethod,
          rollingStocks: []
        },
        wishlistId: targetId,
        priority: form.priority,
        status: null,
        desiredPriceAmount: priceAmount,
        desiredPriceCurrency: priceAmount !== null ? form.desiredPriceCurrency : null,
        notes: form.notes.trim() || null,
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
      desiredPriceCurrency: 'EUR',
      notes: ''
    };
    formError = null;
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
</script>

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
      <!-- Wishlist selection -->
      <div class="space-y-2">
        <label
          for="wishlist-select"
          class="text-xs font-semibold tracking-wide text-muted-foreground uppercase"
        >
          {m.wishlist_modal_choose_or_create()}
        </label>
        <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
          <select
            id="wishlist-select"
            class="h-9 rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40 disabled:cursor-not-allowed disabled:opacity-50"
            bind:value={form.wishlistId}
            disabled={isDropdownDisabled}
            aria-label={m.wishlist_modal_select_list()}
          >
            <option value="" disabled>{m.wishlist_modal_select_placeholder()}</option>
            {#each wishlists as list (list.id)}
              <option value={list.id}>
                {list.name}
                {#if list.isDefault}
                  (default)
                {/if}
              </option>
            {/each}
          </select>
          <Input
            type="text"
            placeholder={m.wishlist_modal_new_list_placeholder()}
            bind:value={form.newListName}
          />
        </div>
      </div>

      <!-- Model Details section -->
      <div class="space-y-3">
        <p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
          {m.wishlist_modal_model_details()}
        </p>

        <!-- Manufacturer -->
        <div class="space-y-1">
          <label for="manufacturer" class="text-xs text-muted-foreground">
            {m.wishlist_modal_manufacturer()} *
          </label>
          {#if isLoadingData}
            <p class="text-sm text-muted-foreground">{m.wishlist_modal_loading()}</p>
          {:else}
            <select
              id="manufacturer"
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40 disabled:cursor-not-allowed disabled:opacity-50"
              bind:value={form.manufacturerId}
            >
              <option value="">{m.wishlist_modal_manufacturer_placeholder()}</option>
              {#each manufacturers as mfr (mfr.id)}
                <option value={mfr.id}>{mfr.name}</option>
              {/each}
            </select>
          {/if}
        </div>

        <!-- Product Code + Category -->
        <div class="grid grid-cols-2 gap-3">
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
          <div class="space-y-1">
            <label for="category" class="text-xs text-muted-foreground">
              {m.wishlist_modal_category()}
            </label>
            <select
              id="category"
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              bind:value={form.category}
            >
              {#each CATEGORIES as cat (cat)}
                <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                <option value={cat}>{(m as any)[getCategoryLabelKey(cat)]()}</option>
              {/each}
            </select>
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

        <!-- Scale + Power Method + Epoch -->
        <div class="grid grid-cols-3 gap-3">
          <div class="space-y-1">
            <label for="scale" class="text-xs text-muted-foreground">
              {m.wishlist_modal_scale()}
            </label>
            <select
              id="scale"
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              bind:value={form.scale}
            >
              {#each SCALES as scale (scale)}
                <option value={scale}>{scale}</option>
              {/each}
            </select>
          </div>
          <div class="space-y-1">
            <label for="power-method" class="text-xs text-muted-foreground">
              {m.wishlist_modal_power_method()}
            </label>
            <select
              id="power-method"
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              bind:value={form.powerMethod}
            >
              {#each POWER_METHODS as method (method)}
                <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                <option value={method}>{(m as any)[getPowerMethodLabelKey(method)]()}</option>
              {/each}
            </select>
          </div>
          <div class="space-y-1">
            <label for="epoch" class="text-xs text-muted-foreground">
              {m.wishlist_modal_epoch()}
            </label>
            <Input
              id="epoch"
              type="text"
              placeholder={m.wishlist_modal_epoch_placeholder()}
              bind:value={form.epoch}
            />
          </div>
        </div>
      </div>

      <!-- Wishlist Preferences section -->
      <div class="space-y-3">
        <p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
          {m.wishlist_modal_wishlist_prefs()}
        </p>

        <!-- Priority + Desired Price -->
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1">
            <label for="priority" class="text-xs text-muted-foreground">
              {m.wishlist_modal_priority()}
            </label>
            <select
              id="priority"
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              bind:value={form.priority}
            >
              {#each PRIORITIES as priority (priority)}
                <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                <option value={priority}>{(m as any)[getPriorityLabelKey(priority)]()}</option>
              {/each}
            </select>
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

        <!-- Notes -->
        <div class="space-y-1">
          <label for="wishlist-notes" class="text-xs text-muted-foreground">
            {m.wishlist_modal_notes_label()}
          </label>
          <Textarea
            id="wishlist-notes"
            rows={3}
            placeholder={m.wishlist_modal_notes_placeholder()}
            bind:value={form.notes}
          />
        </div>
      </div>

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
