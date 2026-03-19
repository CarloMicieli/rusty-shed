<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { TrainFront } from 'lucide-svelte';
  import {
    DrawerShell,
    DrawerHeader,
    DrawerFooter,
    createDrawerForm
  } from '$lib/components/drawer';
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
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import ModelSearchSection from './ModelSearchSection.svelte';

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

  function createDefaultRollingStock(): RollingStockFormEntry {
    return {
      uid: crypto.randomUUID(),
      railwayCompanyId: null,
      seriesCode: '',
      category: null,
      roadNumber: '',
      subcategory: null
    };
  }

  function createDefaultPurchaseState(): PurchaseFormState {
    return {
      sellerId: null,
      priceAmount: null,
      priceCurrency: settingsState.settings.currency,
      purchaseCondition: null,
      modelCondition: null,
      boxCondition: null,
      notes: '',
      purchaseDate: new Date().toISOString().split('T')[0]
    };
  }

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

  const f = createDrawerForm({
    initial: createDefaultFormState
  });

  // Watch for drawer open/close
  $effect(() => {
    if (open) {
      handleOpen();
    }
  });

  async function handleOpen() {
    f.reset(createDefaultFormState());
    validationErrors = {};
    showPurchaseSection = false;

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
    f.values.rollingStocks = [...f.values.rollingStocks, createDefaultRollingStock()];
  }

  function handleRemoveRollingStock(uid: string) {
    if (f.values.rollingStocks.length <= 1) return;
    f.values.rollingStocks = f.values.rollingStocks.filter((rs) => rs.uid !== uid);
  }

  function toAddRailwayModelArgs(formState: AddModelFormState): AddRailwayModelToCollectionArgs {
    const today = new Date().toISOString().split('T')[0];

    // priceAmount is now stored as integer cents in the form state
    const priceInCents = formState.purchase.priceAmount ?? 0;

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
          subcategory: rs.subcategory || null,
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

  async function handleSubmit() {
    f.touch();

    const errors = validateForm(f.values);
    validationErrors = errors;

    if (Object.keys(errors).length > 0) {
      console.error('Form validation errors:', errors);
      return;
    }

    isSubmitting = true;
    try {
      const args = toAddRailwayModelArgs(f.values);
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
</script>

<DrawerShell {open} {onClose} size="xl" hasChanges={f.isDirty} labelledby="drawer-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="drawer-title"
      title={m.add_model_title()}
      subtitle={m.add_model_subtitle()}
      icon={TrainFront}
      onClose={requestClose}
    />
  {/snippet}

  <form id="add-model-form" class="space-y-6">
    <ModelSearchSection
      bind:form={f.values}
      {manufacturers}
      {railwayCompanies}
      {sellers}
      bind:showPurchaseSection
      {validationErrors}
      isLoading={isLoadingData}
      onAddRollingStock={handleAddRollingStock}
      onRemoveRollingStock={handleRemoveRollingStock}
      onTogglePurchaseSection={() => (showPurchaseSection = !showPurchaseSection)}
    />
  </form>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.add_model_cancel()}
      submitLabel={isSubmitting ? m.add_model_submitting() : m.add_model_submit()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
      isLoading={isLoadingData}
    />
  {/snippet}
</DrawerShell>
