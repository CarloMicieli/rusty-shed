<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Heart } from 'lucide-svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { commands } from '$lib/bindings';
  import type { Manufacturer, WishlistPriority } from '$lib/bindings';
  import { SCALES, POWER_METHODS } from '$lib/features/wishlists/constants';
  import {
    DrawerShell,
    DrawerHeader,
    ModelInfoSection,
    WishlistPickerSection,
    WishlistPreferencesSection,
    createDrawerForm
  } from '$lib/components/drawer';
  import { onMount } from 'svelte';

  const wishlistService = getWishlistContext();

  interface Props {
    open: boolean;
    preselectedWishlistId?: string | null;
    onClose: () => void;
    onSaved: () => void;
  }

  let { open, preselectedWishlistId = null, onClose, onSaved }: Props = $props();

  const wishlists = $derived(wishlistService.wishlists);
  const defaultWishlist = $derived(wishlistService.defaultWishlist);

  function makeInitial() {
    return {
      wishlistId: defaultWishlist?.id ?? '',
      newListName: '',
      manufacturerId: '',
      productCode: '',
      description: '',
      category: '',
      scale: settingsState.settings?.favouriteScale || SCALES[0],
      powerMethod: settingsState.settings?.powerMethod || POWER_METHODS[0],
      epoch: '',
      priority: 'NORMAL' as WishlistPriority,
      desiredPrice: null as number | null
    };
  }

  const f = createDrawerForm({
    initial: makeInitial,
    validate: (v) => ({
      manufacturerId: !v.manufacturerId ? m.wishlist_modal_missing_manufacturer() : undefined,
      productCode: !v.productCode.trim() ? m.wishlist_modal_missing_product_code() : undefined,
      description: !v.description.trim() ? m.wishlist_modal_missing_description() : undefined,
      desiredPrice:
        v.desiredPrice !== null && v.desiredPrice <= 0
          ? m.wishlist_modal_invalid_price()
          : undefined
    })
  });

  let manufacturers = $state<Manufacturer[]>([]);
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let asyncError = $state<string | null>(null);

  const currency = $derived(settingsState.settings.currency ?? 'EUR');
  const lockWishlist = $derived(!!preselectedWishlistId);

  // Show first validation error or async error in banner
  const formError = $derived(
    asyncError ?? (Object.values(f.errors).find((e) => !!e) as string | undefined) ?? null
  );

  $effect(() => {
    if (defaultWishlist && f.values.wishlistId === '') {
      f.values.wishlistId = defaultWishlist.id;
    }
  });

  $effect(() => {
    if (open && preselectedWishlistId) {
      f.values.wishlistId = preselectedWishlistId;
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
    asyncError = null;
    f.touch();

    if (!f.isValid) return;

    isSubmitting = true;
    try {
      let targetId = f.values.wishlistId;

      if (f.values.newListName.trim()) {
        const created = await wishlistService.createWishlist(f.values.newListName.trim(), false);
        if (!created) {
          asyncError = m.wishlist_modal_create_failed();
          return;
        }
        targetId = created.id;
      }

      if (!targetId) {
        asyncError = m.wishlist_modal_select_list_error();
        return;
      }

      const priceAmount =
        f.values.desiredPrice !== null ? (f.values.desiredPrice as unknown as bigint) : null;

      const success = await wishlistService.addRailwayModelToWishlist({
        railwayModel: {
          manufacturerId: f.values.manufacturerId,
          productCode: f.values.productCode.trim(),
          description: f.values.description.trim(),
          category: f.values.category,
          scale: f.values.scale,
          epoch: f.values.epoch,
          powerMethod: f.values.powerMethod,
          rollingStocks: []
        },
        wishlistId: targetId,
        priority: f.values.priority,
        status: null,
        desiredPriceAmount: priceAmount,
        desiredPriceCurrency: priceAmount !== null ? currency : null,
        notes: null,
        addedDate: null
      });

      if (!success) {
        asyncError = m.wishlist_modal_add_failed();
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
    f.reset(makeInitial());
    asyncError = null;
  }
</script>

<DrawerShell
  {open}
  onClose={resetAndClose}
  size="xl"
  hasChanges={f.isDirty}
  labelledby="wishlist-item-drawer-title"
  error={formError}
  discardTitle={m.wishlist_add_item_drawer_discard_title()}
  discardDescription={m.wishlist_add_item_drawer_discard_description()}
  discardConfirm={m.wishlist_add_item_drawer_discard_confirm()}
  discardCancel={m.wishlist_add_item_drawer_discard_cancel()}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="wishlist-item-drawer-title"
      title={m.wishlist_modal_title()}
      subtitle={m.wishlist_add_item_drawer_subtitle()}
      icon={Heart}
      onClose={requestClose}
    />
  {/snippet}

  <div class="space-y-6">
    <!-- Section 1: Choose or Create Wishlist -->
    <WishlistPickerSection
      bind:wishlistId={f.values.wishlistId}
      bind:newListName={f.values.newListName}
      {wishlists}
      disabled={isSubmitting}
      disableWishlistSelection={lockWishlist}
    />

    <!-- Section 2: Model Details -->
    <ModelInfoSection
      bind:manufacturerId={f.values.manufacturerId}
      bind:productCode={f.values.productCode}
      bind:description={f.values.description}
      bind:category={f.values.category}
      bind:scale={f.values.scale}
      bind:powerMethod={f.values.powerMethod}
      bind:epoch={f.values.epoch}
      {manufacturers}
      isLoading={isLoadingData}
      disabled={isSubmitting}
    />

    <!-- Section 3: Wishlist Preferences -->
    <div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
      <section>
        <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
          {m.drawer_section_wishlist()}
        </p>
        <WishlistPreferencesSection
          bind:priority={f.values.priority}
          bind:desiredPrice={f.values.desiredPrice}
          {currency}
          errors={f.errors}
          disabled={isSubmitting}
        />
      </section>
    </div>
  </div>

  {#snippet footer({ requestClose })}
    <div class="flex items-center justify-end gap-2 p-4">
      <Button
        type="button"
        variant="ghost"
        size="sm"
        onclick={requestClose}
        disabled={isSubmitting}
      >
        {m.wishlist_modal_cancel()}
      </Button>
      <Button
        type="button"
        variant="default"
        size="sm"
        class="bg-[#D48A42] font-bold text-black hover:bg-[#D48A42]/90"
        onclick={handleSubmit}
        disabled={isSubmitting}
      >
        {isSubmitting ? m.wishlist_modal_saving() : m.wishlist_modal_save()}
      </Button>
    </div>
  {/snippet}
</DrawerShell>
