<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Train } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import { getWishlistContext } from '../WishlistState.svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import type { AddRailwayModelFormState, RollingStockFormEntry } from '../types';
  import type {
    Manufacturer,
    RailwayCompany,
    WishlistPreview,
    AddRailwayModelToWishListArgs,
    Scale,
    PowerMethod
  } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import {
    DrawerShell,
    DrawerHeader,
    ModelInfoSection,
    WishlistSection,
    RollingStockSection
  } from '$lib/components/drawer';

  interface Props {
    open: boolean;
    preselectedWishlistId?: string | null;
    wishlists: WishlistPreview[];
    onClose: () => void;
    onSuccess: () => void;
  }

  let { open, preselectedWishlistId = null, wishlists, onClose, onSuccess }: Props = $props();

  const wishlistService = getWishlistContext();

  let manufacturers = $state<Manufacturer[]>([]);
  let railwayCompanies = $state<RailwayCompany[]>([]);
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);
  let form = $state<AddRailwayModelFormState>(createDefaultFormState());

  let isFormValid = $derived.by(() => {
    return (
      form.wishlistId !== '' &&
      form.manufacturerId !== '' &&
      form.productCode.trim() !== '' &&
      form.description.trim() !== '' &&
      form.category !== '' &&
      form.scale !== '' &&
      form.powerMethod !== '' &&
      form.epoch !== null &&
      validateRollingStocks()
    );
  });

  let hasLoadedData = $state(false);

  $effect(() => {
    if (open && !hasLoadedData) {
      hasLoadedData = true;
      handleOpen();
    } else if (!open) {
      hasLoadedData = false;
    }
  });

  function createDefaultFormState(): AddRailwayModelFormState {
    return {
      wishlistId: '',
      manufacturerId: '',
      productCode: '',
      description: '',
      category: '',
      scale: (settingsState.settings?.favouriteScale as Scale) || '',
      powerMethod: (settingsState.settings?.powerMethod as PowerMethod) || '',
      epoch: null,
      desiredPriceAmount: '',
      desiredPriceCurrency: 'EUR',
      priority: 'NORMAL',
      notes: '',
      rollingStocks: []
    };
  }

  function validateRollingStocks(): boolean {
    if (form.rollingStocks.length === 0) return true;
    return form.rollingStocks.every(
      (rs) => rs.railwayCompanyId !== '' && rs.seriesCode.trim() !== '' && rs.category !== ''
    );
  }

  async function handleOpen() {
    form = createDefaultFormState();
    if (preselectedWishlistId) {
      form.wishlistId = preselectedWishlistId;
    }
    await loadDropdownData();
  }

  async function loadDropdownData() {
    isLoadingData = true;
    try {
      const [mfrsResult, companiesResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getRailwayCompanies()
      ]);
      if (mfrsResult.status === 'ok') manufacturers = mfrsResult.data;
      if (companiesResult.status === 'ok') railwayCompanies = companiesResult.data;
    } catch (error) {
      console.error('Failed to load dropdown data:', error);
    } finally {
      isLoadingData = false;
    }
  }

  function addRollingStock() {
    const newEntry: RollingStockFormEntry = {
      id: crypto.randomUUID(),
      railwayCompanyId: '',
      seriesCode: '',
      category: '',
      roadNumber: ''
    };
    form.rollingStocks = [...form.rollingStocks, newEntry];
  }

  function removeRollingStock(id: string) {
    form.rollingStocks = form.rollingStocks.filter((rs) => rs.id !== id);
  }

  function toAddRailwayModelArgs(
    formData: AddRailwayModelFormState
  ): AddRailwayModelToWishListArgs {
    return {
      wishlistId: formData.wishlistId,
      railwayModel: {
        manufacturerId: formData.manufacturerId,
        productCode: formData.productCode,
        description: formData.description,
        category: formData.category as string,
        scale: formData.scale as string,
        epoch: formData.epoch as string,
        powerMethod: formData.powerMethod as string,
        rollingStocks: formData.rollingStocks.map((rs) => ({
          railwayCompanyId: rs.railwayCompanyId,
          seriesCode: rs.seriesCode,
          roadNumber: rs.roadNumber || null,
          locomotiveType: null,
          category: rs.category as string
        }))
      },
      priority: formData.priority,
      status: 'WANTED',
      desiredPriceAmount: desiredPriceInCents as unknown as bigint,
      desiredPriceCurrency: desiredPriceInCents !== null ? formData.desiredPriceCurrency : null,
      notes: formData.notes || null,
      addedDate: new Date().toISOString().split('T')[0]
    };
  }

  async function handleSubmit() {
    if (!isFormValid) return;

    isSubmitting = true;
    try {
      const args = toAddRailwayModelArgs(form);
      const success = await wishlistService.addRailwayModelToWishlist(args);
      if (success) {
        onSuccess();
      }
    } catch (error) {
      console.error('Error submitting railway model:', error);
    } finally {
      isSubmitting = false;
    }
  }

  // Local price state in integer cents (for WishlistSection); converted to string on submit
  let desiredPriceInCents = $state<number | null>(null);
</script>

<DrawerShell {open} {onClose} size="lg" labelledby="add-railway-model-drawer-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="add-railway-model-drawer-title"
      title={m.wishlist_drawer_title()}
      subtitle={m.wishlist_drawer_subtitle()}
      icon={Train}
      onClose={requestClose}
    />
  {/snippet}

  {#if isLoadingData}
    <p class="text-center text-muted-foreground">{m.wishlist_loading_data()}</p>
  {:else}
    <div class="space-y-6">
      <WishlistSection
        bind:wishlistId={form.wishlistId}
        {wishlists}
        bind:priority={form.priority}
        bind:desiredPrice={desiredPriceInCents}
        currency={form.desiredPriceCurrency}
        bind:notes={form.notes}
        disabled={isSubmitting}
      />

      <ModelInfoSection
        bind:manufacturerId={form.manufacturerId}
        bind:productCode={form.productCode}
        bind:description={form.description}
        bind:category={form.category}
        bind:scale={form.scale}
        bind:powerMethod={form.powerMethod}
        bind:epoch={form.epoch}
        {manufacturers}
        disabled={isSubmitting}
      />

      <RollingStockSection
        bind:entries={form.rollingStocks}
        {railwayCompanies}
        onadd={addRollingStock}
        onremove={removeRollingStock}
        disabled={isSubmitting}
      />
    </div>
  {/if}

  {#snippet footer({ requestClose })}
    <div class="flex items-center justify-end gap-2 p-4">
      <Button type="button" variant="ghost" class="text-zinc-500" onclick={requestClose}>
        {m.wishlist_drawer_cancel()}
      </Button>
      <Button
        type="button"
        variant="default"
        class="bg-[#D48A42] font-bold text-black hover:bg-[#D48A42]/90"
        disabled={!isFormValid || isSubmitting}
        onclick={handleSubmit}
      >
        {#if isSubmitting}
          <span>{m.wishlist_toast_adding()}</span>
        {:else}
          <span>{m.wishlist_drawer_submit()}</span>
        {/if}
      </Button>
    </div>
  {/snippet}
</DrawerShell>
