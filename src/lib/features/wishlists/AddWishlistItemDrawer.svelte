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
    WishlistPreferencesSection
  } from '$lib/components/drawer';
  import { onMount } from 'svelte';
  import { superForm } from 'sveltekit-superforms';
  import { zod } from '$lib/vendor/superforms-adapters';
  import { wishlistFormSchema } from '$lib/schemas/wishlist-form';

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
      manufacturerId: null as string | null,
      productCode: '',
      description: '',
      category: null as string | null,
      scale: settingsState.settings?.favouriteScale || SCALES[0],
      powerMethod: settingsState.settings?.powerMethod || POWER_METHODS[0],
      epoch: null as string | null,
      priority: 'NORMAL' as WishlistPriority,
      desiredPrice: null as number | null
    };
  }

  let manufacturers = $state<Manufacturer[]>([]);
  let isLoadingData = $state(false);
  let isSubmitting = $state(false);
  let asyncError = $state<string | null>(null);

  const { form, errors, tainted, reset, isTainted, validateForm } = superForm(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    makeInitial() as any,
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zod(wishlistFormSchema as any)
    }
  );

  const currency = $derived(settingsState.settings.currency ?? 'EUR');
  const lockWishlist = $derived(!!preselectedWishlistId);
  const hasChanges = $derived(isTainted($tainted));

  const mappedErrors = $derived({
    desiredPrice: $errors.desiredPrice?.[0] as string | undefined
  });

  // Show first validation error or async error in banner
  const formError = $derived(
    asyncError ??
      (($errors.manufacturerId?.[0] ?? $errors.productCode?.[0] ?? $errors.description?.[0]) as
        | string
        | undefined) ??
      null
  );

  $effect.pre(() => {
    if (open) {
      asyncError = null;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: makeInitial() as any });
    }
  });

  $effect(() => {
    if (defaultWishlist && ($form.wishlistId as string) === '' && !$form.newListName) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: makeInitial() as any });
    }
  });

  $effect(() => {
    if (open && preselectedWishlistId) {
      $form.wishlistId = preselectedWishlistId;
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

  function resetAndClose() {
    onClose();
    asyncError = null;
  }

  async function handleSubmit() {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result = await validateForm({ update: true } as any);
    if (!result.valid) return;

    isSubmitting = true;
    asyncError = null;
    try {
      let targetId = $form.wishlistId as string;

      if (($form.newListName as string).trim()) {
        const created = await wishlistService.createWishlist(
          ($form.newListName as string).trim(),
          false,
          true
        );
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

      const priceAmount = $form.desiredPrice !== null ? ($form.desiredPrice as number) : null;

      const success = await wishlistService.addRailwayModelToWishlist({
        railwayModel: {
          manufacturerId: $form.manufacturerId as string,
          productCode: ($form.productCode as string).trim(),
          description: ($form.description as string).trim(),
          category: ($form.category as string | null) ?? '',
          scale: $form.scale as string,
          epoch: ($form.epoch as string | null) ?? '',
          powerMethod: $form.powerMethod as string,
          rollingStocks: []
        },
        wishlistId: targetId,
        priority: $form.priority as WishlistPriority,
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
</script>

<DrawerShell
  {open}
  onClose={resetAndClose}
  size="xl"
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

  <form class="contents" onsubmit={(e) => e.preventDefault()}>
    <div class="space-y-6">
      <!-- Section 1: Choose or Create Wishlist -->
      <WishlistPickerSection
        bind:wishlistId={$form.wishlistId}
        bind:newListName={$form.newListName}
        {wishlists}
        disabled={isSubmitting}
        disableWishlistSelection={lockWishlist}
      />

      <!-- Section 2: Model Details -->
      <ModelInfoSection
        bind:manufacturerId={$form.manufacturerId}
        bind:productCode={$form.productCode}
        bind:description={$form.description}
        bind:category={$form.category}
        bind:scale={$form.scale}
        bind:powerMethod={$form.powerMethod}
        bind:epoch={$form.epoch}
        {manufacturers}
        isLoading={isLoadingData}
        disabled={isSubmitting}
      />

      <!-- Section 3: Wishlist Preferences -->
      <div class="overflow-hidden rounded-sm border border-border bg-card p-4">
        <section>
          <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
            {m.drawer_section_wishlist()}
          </p>
          <WishlistPreferencesSection
            bind:priority={$form.priority}
            bind:desiredPrice={$form.desiredPrice}
            {currency}
            errors={mappedErrors}
            disabled={isSubmitting}
          />
        </section>
      </div>
    </div>
  </form>

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
        class="variant-steampunk-lever rounded-sm bg-primary font-bebas font-bold text-primary-foreground shadow-[2px_2px_0px_0px_rgba(0,0,0,0.2)] hover:bg-primary/90"
        onclick={handleSubmit}
        disabled={isSubmitting}
      >
        {isSubmitting ? m.wishlist_modal_saving() : m.wishlist_modal_save()}
      </Button>
    </div>
  {/snippet}
</DrawerShell>
