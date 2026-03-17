<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Heart } from 'lucide-svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { commands } from '$lib/bindings';
  import type { Manufacturer, WishlistPriority } from '$lib/bindings';
  import { CATEGORIES, SCALES, POWER_METHODS } from '$lib/features/wishlists/constants';
  import {
    DrawerShell,
    DrawerHeader,
    ModelInfoSection,
    WishlistSection
  } from '$lib/components/drawer';
  import { onMount } from 'svelte';

  const wishlistService = getWishlistContext();

  interface Props {
    open: boolean;
    preselectedWishlistId?: string | null;
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

  let { open, preselectedWishlistId = null, onClose, onSaved }: Props = $props();

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

  const currency = $derived(settingsState.settings.currency ?? 'EUR');

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

  const lockWishlist = $derived(!!preselectedWishlistId);

  $effect(() => {
    if (open && preselectedWishlistId) {
      form.wishlistId = preselectedWishlistId;
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
  }
</script>

<DrawerShell
  {open}
  onClose={resetAndClose}
  size="md"
  {hasChanges}
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

  <div class="space-y-5">
    <WishlistSection
      bind:wishlistId={form.wishlistId}
      bind:newListName={form.newListName}
      {wishlists}
      bind:priority={form.priority}
      bind:desiredPrice={form.desiredPrice}
      {currency}
      disabled={isSubmitting}
      disableWishlistSelection={lockWishlist}
    />

    <div class="border-t border-white/10 pt-2"></div>

    <ModelInfoSection
      bind:manufacturerId={form.manufacturerId}
      bind:productCode={form.productCode}
      bind:description={form.description}
      bind:category={form.category}
      bind:scale={form.scale}
      bind:powerMethod={form.powerMethod}
      bind:epoch={form.epoch}
      {manufacturers}
      isLoading={isLoadingData}
      disabled={isSubmitting}
    />
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
