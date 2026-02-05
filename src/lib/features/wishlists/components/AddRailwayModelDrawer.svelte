<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X, Plus } from 'lucide-svelte';
  import { Textarea } from '$lib/components';
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
  import { CATEGORIES, SCALES, POWER_METHODS, PRIORITIES } from '../constants';
  import epochs from '$lib/data/constants/epochs.json';

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

  function removeRollingStock(id: string) {
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
    class="fixed inset-0 z-50 bg-black/50"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>
{/if}

<!-- Drawer -->
<div
  class="drawer bg-surface-800 fixed top-0 right-0 z-50 flex h-full w-full max-w-2xl transform flex-col shadow-xl transition-transform duration-300 {open
    ? 'translate-x-0'
    : 'translate-x-full'}"
  role="dialog"
  aria-modal="true"
  aria-labelledby="drawer-title"
>
  <!-- Header -->
  <div class="border-surface-700 flex items-center justify-between border-b p-4">
    <div>
      <h2 id="drawer-title" class="text-surface-50 text-xl font-bold">
        {m.wishlist_drawer_title()}
      </h2>
      <p class="text-surface-400 mt-1 text-sm">{m.wishlist_drawer_subtitle()}</p>
    </div>
    <button
      type="button"
      class="variant-ghost btn-icon"
      onclick={handleCloseRequest}
      aria-label={m.wishlist_drawer_cancel()}
    >
      <X size={20} />
    </button>
  </div>

  <!-- Form Content -->
  <div class="flex-1 overflow-y-auto p-4">
    {#if isLoadingData}
      <p class="text-surface-400 text-center">{m.wishlist_loading_data()}</p>
    {:else}
      <form onsubmit={(e) => e.preventDefault()} class="space-y-6">
        <!-- Wishlist Selection -->
        <div>
          <label for="wishlist" class="block space-y-1">
            <span class="text-sm font-medium text-surface-300">
              {m.wishlist_field_wishlist()}
              <span class="text-error-500">*</span>
            </span>
          </label>
          <select id="wishlist" bind:value={form.wishlistId} class="input w-full" required>
            <option value="">-- {m.wishlist_field_wishlist()} --</option>
            {#each wishlists as wishlist (wishlist.id)}
              <option value={wishlist.id}>{wishlist.name}</option>
            {/each}
          </select>
        </div>

        <!-- Railway Model Section -->
        <div class="border-surface-700 bg-surface-900 space-y-4 rounded-lg border p-4">
          <h3 class="text-lg font-semibold text-surface-100">Railway Model Details</h3>

          <!-- Manufacturer -->
          <div>
            <label for="manufacturer" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300">
                {m.wishlist_field_manufacturer()}
                <span class="text-error-500">*</span>
              </span>
            </label>
            <select
              id="manufacturer"
              bind:value={form.manufacturerId}
              class="input w-full"
              required
            >
              <option value="">-- {m.wishlist_field_manufacturer()} --</option>
              {#each manufacturers as mfr (mfr.id)}
                <option value={mfr.id}>{mfr.name}</option>
              {/each}
            </select>
          </div>

          <!-- Product Code -->
          <div>
            <label for="product-code" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300">
                {m.wishlist_field_product_code()}
                <span class="text-error-500">*</span>
              </span>
            </label>
            <input
              id="product-code"
              type="text"
              bind:value={form.productCode}
              class="input w-full font-mono"
              placeholder="e.g., 37171"
              required
            />
          </div>

          <!-- Description -->
          <div>
            <label for="description" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300">
                {m.wishlist_field_description()}
                <span class="text-error-500">*</span>
              </span>
            </label>
            <input
              id="description"
              type="text"
              bind:value={form.description}
              class="input w-full"
              placeholder="e.g., DB BR 218 diesel locomotive"
              required
            />
          </div>

          <!-- Category & Scale -->
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div>
              <label for="category" class="block space-y-1">
                <span class="text-sm font-medium text-surface-300">
                  {m.wishlist_field_category()}
                  <span class="text-error-500">*</span>
                </span>
              </label>
              <select id="category" bind:value={form.category} class="input w-full" required>
                <option value="">-- {m.wishlist_field_category()} --</option>
                {#each CATEGORIES as cat (cat)}
                  <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                  <option value={cat}>{(m as any)[getCategoryLabelKey(cat)]()}</option>
                {/each}
              </select>
            </div>

            <div>
              <label for="scale" class="block space-y-1">
                <span class="text-sm font-medium text-surface-300">
                  {m.wishlist_field_scale()}
                  <span class="text-error-500">*</span>
                </span>
              </label>
              <select id="scale" bind:value={form.scale} class="input w-full" required>
                <option value="">-- {m.wishlist_field_scale()} --</option>
                {#each SCALES as scale (scale)}
                  <option value={scale}>{scale}</option>
                {/each}
              </select>
            </div>
          </div>

          <!-- Power Method & Epoch -->
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div>
              <label for="power-method" class="block space-y-1">
                <span class="text-sm font-medium text-surface-300">
                  {m.wishlist_field_power_method()}
                  <span class="text-error-500">*</span>
                </span>
              </label>
              <select id="power-method" bind:value={form.powerMethod} class="input w-full" required>
                <option value="">-- {m.wishlist_field_power_method()} --</option>
                {#each POWER_METHODS as method (method)}
                  <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                  <option value={method}>{(m as any)[getPowerMethodLabelKey(method)]()}</option>
                {/each}
              </select>
            </div>

            <div>
              <label for="epoch" class="block space-y-1">
                <span class="text-sm font-medium text-surface-300">
                  {m.wishlist_field_epoch()}
                  <span class="text-error-500">*</span>
                </span>
              </label>
              <select id="epoch" bind:value={form.epoch} class="input w-full" required>
                <option value={null}>-- {m.wishlist_field_epoch()} --</option>
                {#each epochs as epoch (epoch.id)}
                  <option value={epoch.id}>{epoch.display}</option>
                {/each}
              </select>
            </div>
          </div>
        </div>

        <!-- Rolling Stocks Section -->
        <div class="border-surface-700 bg-surface-900 space-y-4 rounded-lg border p-4">
          <div class="flex items-center justify-between">
            <h3 class="text-lg font-semibold text-surface-100">
              {m.wishlist_rolling_stocks_title()}
            </h3>
            <button type="button" class="variant-soft btn btn-sm" onclick={addRollingStock}>
              <Plus size={16} />
              <span>{m.wishlist_rolling_stock_add()}</span>
            </button>
          </div>

          {#if form.rollingStocks.length === 0}
            <p class="text-surface-400 text-sm">No rolling stocks added yet.</p>
          {:else}
            <div class="space-y-4">
              {#each form.rollingStocks as entry, i (entry.id)}
                <RollingStockEntry
                  bind:entry={form.rollingStocks[i]}
                  {railwayCompanies}
                  canRemove={form.rollingStocks.length > 0}
                  onRemove={() => removeRollingStock(entry.id)}
                />
              {/each}
            </div>
          {/if}
        </div>

        <!-- Wishlist Item Details -->
        <div class="border-surface-700 bg-surface-900 space-y-4 rounded-lg border p-4">
          <h3 class="text-lg font-semibold text-surface-100">Wishlist Details</h3>

          <!-- Priority -->
          <div>
            <label for="priority" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300">{m.wishlist_field_priority()}</span
              >
            </label>
            <select id="priority" bind:value={form.priority} class="input w-full">
              {#each PRIORITIES as priority (priority)}
                <!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
                <option value={priority}>{(m as any)[getPriorityLabelKey(priority)]()}</option>
              {/each}
            </select>
          </div>

          <!-- Desired Price -->
          <div>
            <label for="desired-price" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300"
                >{m.wishlist_field_desired_price()}</span
              >
            </label>
            <input
              id="desired-price"
              type="number"
              step="0.01"
              min="0"
              bind:value={form.desiredPriceAmount}
              class="input w-full"
              placeholder="0.00"
            />
          </div>

          <!-- Notes -->
          <div>
            <label for="notes" class="block space-y-1">
              <span class="text-sm font-medium text-surface-300">{m.wishlist_field_notes()}</span>
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
  <div class="border-surface-700 flex items-center justify-end gap-2 border-t p-4">
    <button type="button" class="variant-ghost btn" onclick={handleCloseRequest}>
      {m.wishlist_drawer_cancel()}
    </button>
    <button
      type="button"
      class="variant-filled-primary btn"
      disabled={!isFormValid || isSubmitting}
      onclick={handleSubmit}
    >
      {#if isSubmitting}
        <span>{m.wishlist_toast_adding()}</span>
      {:else}
        <span>{m.wishlist_drawer_submit()}</span>
      {/if}
    </button>
  </div>
</div>

<style>
  .drawer {
    max-height: 100vh;
  }
</style>
