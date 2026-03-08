<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Textarea, Button } from '$lib/components';
  import { getWishlistContext } from '../WishlistState.svelte';
  import type { AddRailwayModelFormState, RollingStockFormEntry } from '../types';
  import type {
    Manufacturer,
    RailwayCompany,
    WishlistPreview,
    AddRailwayModelToWishListArgs
  } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import RailwayModelBaseForm from '$lib/shared/components/RailwayModelBaseForm.svelte';
  import { CATEGORIES, SCALES, POWER_METHODS, PRIORITIES } from '../constants';

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

  // Check if form has unsaved changes (for future use)
  let _hasChanges = $derived(
    form.productCode.trim() !== '' ||
      form.description.trim() !== '' ||
      form.manufacturerId !== '' ||
      form.category !== '' ||
      form.scale !== '' ||
      form.powerMethod !== '' ||
      (form.epoch !== null && form.epoch.trim() !== '') ||
      form.rollingStocks.length > 0
  );

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
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) {
        mainElement.style.overflow = '';
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
      scale: '',
      powerMethod: '',
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
    return labelMap[category] || category;
  }

  // Helper to get priority label
  function getPriorityLabelKey(priority: string): string {
    const labelMap: Record<string, string> = {
      LOW: 'wishlist_priority_low',
      NORMAL: 'wishlist_priority_normal',
      HIGH: 'wishlist_priority_high'
    };
    return labelMap[priority] || priority;
  }

  // Helper to get power method label
  function getPowerMethodLabelKey(method: string): string {
    const labelMap: Record<string, string> = {
      AC: 'wishlist_power_ac',
      DC: 'wishlist_power_dc',
      TRIX_EXPRESS: 'wishlist_power_trix_express'
    };
    return labelMap[method] || method;
  }
</script>

<!-- Drawer Overlay -->
{#if open}
  <div
    class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>
{/if}

<!-- Drawer -->
<div
  class="drawer fixed top-0 right-0 z-50 flex h-full w-full max-w-2xl transform flex-col border-l border-border bg-background shadow-xl transition-transform duration-300 {open
    ? 'translate-x-0'
    : 'translate-x-full'}"
  role="dialog"
  aria-modal="true"
  aria-labelledby="drawer-title"
>
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-border p-4">
    <div>
      <h2 id="drawer-title" class="text-xl font-bold text-foreground">
        {m.wishlist_drawer_title()}
      </h2>
      <p class="mt-1 text-sm text-muted-foreground">{m.wishlist_drawer_subtitle()}</p>
    </div>
    <Button
      type="button"
      variant="ghost"
      size="icon-sm"
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
      <form onsubmit={(e) => e.preventDefault()} class="space-y-6">
        <!-- Wishlist Selection -->
        <div>
          <label for="wishlist" class="block space-y-1">
            <span class="text-sm font-medium text-muted-foreground">
              {m.wishlist_field_wishlist()}
              <span class="text-error-500">*</span>
            </span>
          </label>
          <select
            id="wishlist"
            bind:value={form.wishlistId}
            class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
            required
          >
            <option value="">-- {m.wishlist_field_wishlist()} --</option>
            {#each wishlists as wishlist (wishlist.id)}
              <option value={wishlist.id}>{wishlist.name}</option>
            {/each}
          </select>
        </div>

        <!-- Base Railway Model Form (shared component) -->
        <RailwayModelBaseForm
          {manufacturers}
          categoryOptions={CATEGORIES}
          scaleOptions={SCALES}
          powerMethodOptions={POWER_METHODS}
          {form}
          onAddRollingStock={addRollingStock}
          {getCategoryLabelKey}
          {getPowerMethodLabelKey}
        >
          {#if form.rollingStocks.length === 0}
            <p class="text-sm text-muted-foreground">No rolling stocks added yet.</p>
          {:else}
            {#each form.rollingStocks as entry, i (entry.id)}
              <RollingStockEntry
                bind:entry={form.rollingStocks[i]}
                {railwayCompanies}
                canRemove={form.rollingStocks.length > 0}
                onRemove={() => removeRollingStock(entry.id)}
              />
            {/each}
          {/if}
        </RailwayModelBaseForm>

        <!-- Wishlist Item Details -->
        <div class="space-y-4 rounded-lg border border-border bg-card p-4 text-card-foreground">
          <h3 class="text-lg font-semibold text-foreground">Wishlist Details</h3>

          <!-- Priority -->
          <div>
            <label for="priority" class="block space-y-1">
              <span class="text-sm font-medium text-muted-foreground"
                >{m.wishlist_field_priority()}</span
              >
            </label>
            <select
              id="priority"
              bind:value={form.priority}
              class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
            >
              {#each PRIORITIES as priority (priority)}
                <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                <option value={priority}>{(m as any)[getPriorityLabelKey(priority)]()}</option>
              {/each}
            </select>
          </div>

          <!-- Desired Price -->
          <div>
            <label for="desired-price" class="block space-y-1">
              <span class="text-sm font-medium text-muted-foreground"
                >{m.wishlist_field_desired_price()}</span
              >
            </label>
            <input
              id="desired-price"
              type="number"
              step="0.01"
              min="0"
              bind:value={form.desiredPriceAmount}
              class="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="0.00"
            />
          </div>

          <!-- Notes -->
          <div>
            <label for="notes" class="block space-y-1">
              <span class="text-sm font-medium text-muted-foreground"
                >{m.wishlist_field_notes()}</span
              >
            </label>
            <Textarea
              id="notes"
              bind:value={form.notes}
              class="w-full"
              rows={3}
              placeholder="Additional notes..."
            />
          </div>
        </div>
      </form>
    {/if}
  </div>

  <!-- Footer -->
  <div class="flex items-center justify-end gap-2 border-t border-border p-4">
    <Button type="button" variant="ghost" onclick={handleCloseRequest}>
      {m.wishlist_drawer_cancel()}
    </Button>
    <Button
      type="button"
      variant="default"
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
