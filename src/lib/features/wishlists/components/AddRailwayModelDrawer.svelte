<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
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
  import AddRailwayModelForm from './AddRailwayModelForm.svelte';

  interface Props {
    /** Controls drawer visibility */
    open: boolean;
    /** Pre-selected wishlist ID (optional, for contextual opening) */
    preselectedWishlistId?: string | null;
    /** Available wishlists for the dropdown */
    wishlists: WishlistPreview[];
    /** Callback when drawer requests close */
    onClose: () => void;
    /** Callback when model is successfully added */
    onSuccess: () => void;
  }

  let { open, preselectedWishlistId = null, wishlists, onClose, onSuccess }: Props = $props();

  const wishlistService = getWishlistContext();

  // Reference data (loaded from backend)
  let manufacturers = $state<Manufacturer[]>([]);
  let railwayCompanies = $state<RailwayCompany[]>([]);

  // UI state
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);

  // Form state
  let form = $state<AddRailwayModelFormState>(createDefaultFormState());

  // Validate form
  let isFormValid = $derived.by(() => {
    const valid =
      form.wishlistId !== '' &&
      form.manufacturerId !== '' &&
      form.productCode.trim() !== '' &&
      form.description.trim() !== '' &&
      form.category !== '' &&
      form.scale !== '' &&
      form.powerMethod !== '' &&
      form.epoch !== null &&
      validateRollingStocks();

    // Debug logging
    if (!valid) {
      console.log('Form validation failed:', {
        wishlistId: form.wishlistId !== '',
        manufacturerId: form.manufacturerId !== '',
        productCode: form.productCode.trim() !== '',
        description: form.description.trim() !== '',
        category: form.category !== '',
        scale: form.scale !== '',
        powerMethod: form.powerMethod !== '',
        epoch: form.epoch !== null,
        rollingStocks: validateRollingStocks(),
        epochValue: form.epoch
      });
    }

    return valid;
  });

  // Track if we've already loaded data for this drawer open session
  let hasLoadedData = $state(false);

  // Effect to handle drawer open/close
  $effect(() => {
    if (open && !hasLoadedData) {
      hasLoadedData = true;
      handleOpen();
    } else if (!open) {
      // Reset when drawer closes
      hasLoadedData = false;
    }

    // Lock/unlock scroll on both body and main content area
    const mainElement = document.querySelector('main');

    if (open) {
      document.body.style.overflow = 'hidden';
      if (mainElement) {
        mainElement.style.overflow = 'hidden';
      }
    } else {
      document.body.style.overflow = '';
      if (mainElement) {
        mainElement.style.overflow = '';
      }
    }

    // Cleanup on unmount
    return () => {
      const mainEl = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainEl) {
        mainEl.style.overflow = '';
      }
    };
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
    // Rolling stocks are optional, but if present, must be valid
    if (form.rollingStocks.length === 0) return true;

    return form.rollingStocks.every(
      (rs) => rs.railwayCompanyId !== '' && rs.seriesCode.trim() !== '' && rs.category !== ''
    );
  }

  async function handleOpen() {
    // Reset form
    form = createDefaultFormState();

    // Apply preselection if provided
    if (preselectedWishlistId) {
      form.wishlistId = preselectedWishlistId;
    }

    // Load dropdown data
    await loadDropdownData();
  }

  async function loadDropdownData() {
    isLoadingData = true;
    try {
      const [mfrsResult, companiesResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getRailwayCompanies()
      ]);

      if (mfrsResult.status === 'ok') {
        manufacturers = mfrsResult.data;
      }

      if (companiesResult.status === 'ok') {
        railwayCompanies = companiesResult.data;
      }
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

  function removeRollingStock(id: string | number) {
    form.rollingStocks = form.rollingStocks.filter((rs) => rs.id !== id);
  }

  function toAddRailwayModelArgs(
    formData: AddRailwayModelFormState
  ): AddRailwayModelToWishListArgs {
    // Parse price amount from decimal to cents
    // Use number type for JSON serialization (safe for typical price values)
    const desiredPriceInCents = formData.desiredPriceAmount
      ? Math.round(parseFloat(formData.desiredPriceAmount) * 100)
      : null;

    return {
      wishlistId: formData.wishlistId,
      railwayModel: {
        manufacturerId: formData.manufacturerId,
        productCode: formData.productCode,
        description: formData.description,
        category: formData.category as string,
        scale: formData.scale as string,
        epoch: formData.epoch as string, // Safe cast: validation ensures epoch is not null
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
      desiredPriceCurrency: formData.desiredPriceAmount ? formData.desiredPriceCurrency : null,
      notes: formData.notes || null,
      addedDate: new Date().toISOString().split('T')[0]
    };
  }

  async function handleSubmit() {
    if (!isFormValid) {
      console.error('Form validation failed');
      return;
    }

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

  function handleCloseRequest() {
    onClose();
  }

  // Helper to get category label
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

  // Helper to get power method label
  function getPowerMethodLabelKey(method: string): string {
    const labelMap: Record<string, string> = {
      AC: 'wishlist_power_ac',
      DC: 'wishlist_power_dc',
      TRIX_EXPRESS: 'wishlist_power_trix_express'
    };
    return labelMap[method] ?? method;
  }
</script>

<!-- Drawer Overlay -->
{#if open}
  <div
    class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>
{/if}

<!-- Drawer -->
<div
  class="drawer fixed top-0 right-0 z-50 flex h-full w-full max-w-2xl transform flex-col border-l border-[#1F1F1F] bg-[#0F0F0F] shadow-2xl transition-transform duration-300 {open
    ? 'translate-x-0'
    : 'translate-x-full'}"
  role="dialog"
  aria-modal="true"
  aria-labelledby="drawer-title"
>
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-[#1F1F1F] p-4">
    <div>
      <h2 id="drawer-title" class="text-xl font-bold text-[#E0E0E0]">
        {m.wishlist_drawer_title()}
      </h2>
      <p class="mt-1 text-sm text-[#808080]">{m.wishlist_drawer_subtitle()}</p>
    </div>
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
      class="text-[#808080] hover:text-[#E0E0E0]"
      onclick={handleCloseRequest}
      aria-label={m.wishlist_drawer_cancel()}
    >
      <X size={20} />
    </Button>
  </div>

  <!-- Form Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if isLoadingData}
      <p class="text-center text-muted-foreground">{m.wishlist_loading_data()}</p>
    {:else}
      <AddRailwayModelForm
        bind:form
        {wishlists}
        {manufacturers}
        {railwayCompanies}
        onAddRollingStock={addRollingStock}
        onRemoveRollingStock={removeRollingStock}
        {getCategoryLabelKey}
        {getPowerMethodLabelKey}
      />
    {/if}
  </div>

  <!-- Footer -->
  <div class="flex items-center justify-end gap-2 border-t border-[#1F1F1F] p-4">
    <Button type="button" variant="ghost" class="text-[#808080]" onclick={handleCloseRequest}>
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
</div>

<style>
  .drawer {
    max-height: 100vh;
  }
</style>
