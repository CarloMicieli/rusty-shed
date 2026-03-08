<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import { getCollectionContext } from '$lib/features/collection/CollectionState.svelte';
  import type {
    AddModelFormState,
    RollingStockFormEntry,
    PurchaseFormState
  } from '$lib/features/collection/types/AddModelFormTypes';
  import type {
    Manufacturer,
    RailwayCompany,
    SellerView,
    AddRailwayModelToCollectionArgs
  } from '$lib/bindings';
  import { commands } from '$lib/bindings';
  import RollingStockEntry from './RollingStockEntry.svelte';
  import PurchaseSection from './PurchaseSection.svelte';
  import RailwayModelBaseForm from '$lib/shared/components/RailwayModelBaseForm.svelte';
  import scales from '$lib/data/constants/scales.json';
  import categories from '$lib/data/constants/categories.json';
  import powerMethods from '$lib/data/constants/powerMethods.json';

  interface Props {
    /** Controls drawer visibility */
    open: boolean;
    /** Callback when drawer requests close */
    onClose: () => void;
    /** Callback when model is successfully added */
    onSuccess: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  const collectionService = getCollectionContext();

  // Reference data (loaded from backend to ensure IDs match database)
  let manufacturers = $state<Manufacturer[]>([]);
  let railwayCompanies = $state<RailwayCompany[]>([]);
  let sellers = $state<SellerView[]>([]);

  // UI state
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);
  let showPurchaseSection = $state(false);
  let showDiscardDialog = $state(false);

  // Form state
  let form = $state<AddModelFormState>(createDefaultFormState());

  // Validation
  interface ValidationErrors {
    manufacturerId?: string;
    productCode?: string;
    description?: string;
    category?: string;
    scale?: string;
    powerMethod?: string;
    epoch?: string;
    rollingStocks?: string;
    rollingStockErrors?: Array<{
      railwayCompanyId?: string;
      seriesCode?: string;
      category?: string;
    }>;
  }

  let validationErrors = $state<ValidationErrors>({});
  let touched = $state(false);

  // Check if form has unsaved changes
  let hasChanges = $derived(
    form.productCode.trim() !== '' ||
      form.description.trim() !== '' ||
      form.manufacturerId !== null ||
      form.category !== null ||
      form.scale !== null ||
      form.powerMethod !== null ||
      form.epoch !== null ||
      form.rollingStocks.length > 1 ||
      form.rollingStocks[0].seriesCode.trim() !== ''
  );

  // Validate form
  let _isFormValid = $derived.by(() => {
    if (!touched) return true;

    const errors = validateForm(form);
    validationErrors = errors;
    return Object.keys(errors).length === 0;
  });

  // Watch for drawer open/close
  $effect(() => {
    if (open) {
      handleOpen();

      // Lock scroll on both body and main content area
      const mainElement = document.querySelector('main');

      document.body.style.overflow = 'hidden';
      if (mainElement) {
        mainElement.style.overflow = 'hidden';
      }
    } else {
      // Restore scroll when drawer closes
      const mainElement = document.querySelector('main');

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

  function createDefaultFormState(): AddModelFormState {
    return {
      manufacturerId: null,
      productCode: '',
      description: '',
      category: null,
      scale: null,
      powerMethod: null,
      epoch: null,
      rollingStocks: [createDefaultRollingStock()],
      purchase: createDefaultPurchaseState()
    };
  }

  function createDefaultRollingStock(): RollingStockFormEntry {
    return {
      uid: crypto.randomUUID(),
      railwayCompanyId: null,
      seriesCode: '',
      category: null,
      roadNumber: '',
      locomotiveType: null
    };
  }

  function createDefaultPurchaseState(): PurchaseFormState {
    return {
      sellerId: null,
      priceAmount: '',
      priceCurrency: 'EUR',
      purchaseCondition: null,
      modelCondition: null,
      boxCondition: null,
      notes: '',
      purchaseDate: new Date().toISOString().split('T')[0]
    };
  }

  function validateForm(formState: AddModelFormState): ValidationErrors {
    const errors: ValidationErrors = {};

    // Railway model validation
    if (!formState.manufacturerId) errors.manufacturerId = m.add_model_validation_manufacturer();
    if (!formState.productCode.trim()) errors.productCode = m.add_model_validation_product_code();
    if (!formState.description.trim()) errors.description = m.add_model_validation_description();
    if (!formState.category) errors.category = m.add_model_validation_category();
    if (!formState.scale) errors.scale = m.add_model_validation_scale();
    if (!formState.powerMethod) errors.powerMethod = m.add_model_validation_power();
    if (!formState.epoch) errors.epoch = m.add_model_validation_epoch();

    // Rolling stocks validation
    if (formState.rollingStocks.length === 0) {
      errors.rollingStocks = m.add_model_validation_rs_required();
    } else {
      const rsErrors = formState.rollingStocks.map((rs) => {
        const err: { railwayCompanyId?: string; seriesCode?: string; category?: string } = {};
        if (!rs.railwayCompanyId) err.railwayCompanyId = m.add_model_validation_rs_company();
        if (!rs.seriesCode.trim()) err.seriesCode = m.add_model_validation_rs_series();
        if (!rs.category) err.category = m.add_model_validation_rs_category();
        return err;
      });

      if (rsErrors.some((e) => Object.keys(e).length > 0)) {
        errors.rollingStockErrors = rsErrors;
      }
    }

    return errors;
  }

  function toAddRailwayModelArgs(formState: AddModelFormState): AddRailwayModelToCollectionArgs {
    const today = new Date().toISOString().split('T')[0];

    // Parse price amount from decimal to cents
    // Use number type for JSON serialization (safe for typical price values)
    const priceInCents = formState.purchase.priceAmount
      ? Math.round(parseFloat(formState.purchase.priceAmount) * 100)
      : 0;

    return {
      railwayModel: {
        manufacturerId: formState.manufacturerId!,
        productCode: formState.productCode,
        description: formState.description,
        category: formState.category!,
        scale: formState.scale!,
        epoch: formState.epoch!,
        powerMethod: formState.powerMethod!,
        rollingStocks: formState.rollingStocks.map((rs) => ({
          railwayCompanyId: rs.railwayCompanyId!,
          seriesCode: rs.seriesCode,
          roadNumber: rs.roadNumber || null,
          locomotiveType: rs.locomotiveType || null,
          category: rs.category!
        }))
      },
      priceAmount: priceInCents as unknown as bigint,
      priceCurrency: formState.purchase.priceCurrency,
      sellerId: formState.purchase.sellerId,
      addedDate: today,
      purchaseDate: formState.purchase.purchaseDate || today,
      purchaseCondition: formState.purchase.purchaseCondition,
      modelCondition: formState.purchase.modelCondition,
      boxCondition: formState.purchase.boxCondition,
      notes: formState.purchase.notes || null
    };
  }

  async function handleOpen() {
    // Reset form
    form = createDefaultFormState();
    touched = false;
    validationErrors = {};
    showPurchaseSection = false;
    showDiscardDialog = false;

    // Load all reference data from backend to ensure IDs match database
    isLoadingData = true;
    try {
      const [mfgResult, rcResult, sellerResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getRailwayCompanies(),
        commands.getSellers()
      ]);

      console.debug('getManufacturers result:', mfgResult);
      console.debug('getRailwayCompanies result:', rcResult);
      console.debug('getSellers result:', sellerResult);

      manufacturers = mfgResult.status === 'ok' ? mfgResult.data : [];
      railwayCompanies = rcResult.status === 'ok' ? rcResult.data : [];
      sellers = sellerResult.status === 'ok' ? sellerResult.data : [];
    } catch (e) {
      console.error('Error loading reference data:', e);
    } finally {
      isLoadingData = false;
    }
  }

  function handleAddRollingStock() {
    form.rollingStocks = [...form.rollingStocks, createDefaultRollingStock()];
  }

  function handleRemoveRollingStock(uid: string) {
    if (form.rollingStocks.length <= 1) return;
    form.rollingStocks = form.rollingStocks.filter((rs) => rs.uid !== uid);
  }

  async function handleSubmit() {
    touched = true;

    const errors = validateForm(form);
    validationErrors = errors;

    if (Object.keys(errors).length > 0) {
      console.error('Form validation errors:', errors);
      return;
    }

    isSubmitting = true;
    try {
      const args = toAddRailwayModelArgs(form);
      console.log('Submitting railway model to collection:', args);
      const success = await collectionService.addRailwayModel(args);
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
    if (hasChanges) {
      showDiscardDialog = true;
    } else {
      onClose();
    }
  }

  function handleDiscardConfirm() {
    showDiscardDialog = false;
    onClose();
  }

  function handleDiscardCancel() {
    showDiscardDialog = false;
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

<!-- Drawer Container -->
<div
  class="fixed top-0 right-0 z-50 h-full w-full max-w-3xl transform transition-transform duration-300 ease-in-out"
  class:translate-x-0={open}
  class:translate-x-full={!open}
  role="dialog"
  aria-modal="true"
  aria-labelledby="drawer-title"
>
  <div class="flex h-full flex-col overflow-y-auto border-l border-border bg-background shadow-2xl">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-border p-6">
      <div>
        <p class="using-track text-xs text-muted-foreground uppercase">
          {m.add_model_subtitle()}
        </p>
        <h2 id="drawer-title" class="text-xl font-semibold">{m.add_model_title()}</h2>
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        onclick={handleCloseRequest}
        aria-label={m.add_model_cancel()}
      >
        <X size={16} />
      </Button>
    </div>

    <!-- Content (scrollable) -->
    <div class="flex-1 overflow-y-auto p-6">
      {#if isLoadingData}
        <div class="flex items-center justify-center py-8">
          <p class="text-muted-foreground">Loading...</p>
        </div>
      {:else}
        <form id="add-model-form" class="space-y-6">
          <!-- Base Railway Model Form (shared component) -->
          <RailwayModelBaseForm
            {manufacturers}
            categoryOptions={categories}
            scaleOptions={scales}
            powerMethodOptions={powerMethods}
            {form}
            validationErrors={validationErrors as Record<string, string | undefined>}
            onAddRollingStock={handleAddRollingStock}
            onRemoveRollingStock={(id) => handleRemoveRollingStock(id as string)}
          >
            {#each form.rollingStocks as entry, index (entry.uid)}
              <RollingStockEntry
                bind:entry={form.rollingStocks[index]}
                {railwayCompanies}
                canRemove={form.rollingStocks.length > 1}
                onRemove={() => handleRemoveRollingStock(entry.uid)}
                errors={validationErrors.rollingStockErrors?.[index]}
              />
            {/each}
          </RailwayModelBaseForm>

          <!-- Purchase Section -->
          <PurchaseSection
            bind:purchase={form.purchase}
            {sellers}
            bind:expanded={showPurchaseSection}
            onToggle={() => (showPurchaseSection = !showPurchaseSection)}
          />
        </form>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-3 border-t border-border p-6">
      <Button type="button" variant="ghost" onclick={handleCloseRequest} disabled={isSubmitting}>
        {m.add_model_cancel()}
      </Button>
      <Button
        type="submit"
        form="add-model-form"
        variant="default"
        onclick={handleSubmit}
        disabled={isSubmitting || isLoadingData}
      >
        {isSubmitting ? m.add_model_submitting() : m.add_model_submit()}
      </Button>
    </div>
  </div>
</div>

<!-- Discard Changes Dialog -->
{#if showDiscardDialog}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-background/80 backdrop-blur-sm"
  >
    <div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl">
      <h3 class="mb-2 text-lg font-bold text-foreground">{m.add_model_discard_title()}</h3>
      <p class="mb-4 text-muted-foreground">{m.add_model_discard_message()}</p>
      <div class="flex justify-end gap-3">
        <Button type="button" variant="ghost" onclick={handleDiscardCancel}>
          {m.add_model_discard_cancel()}
        </Button>
        <Button type="button" variant="destructive" onclick={handleDiscardConfirm}>
          {m.add_model_discard_confirm()}
        </Button>
      </div>
    </div>
  </div>
{/if}
