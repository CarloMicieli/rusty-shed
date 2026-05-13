<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { TrainFront } from 'lucide-svelte';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { addCollectionSchema } from '$lib/schemas/collection-form';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';
  import { collectionState } from '$lib/features/collection/CollectionState.svelte';
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

  const collectionService = collectionState;

  // Reference data (loaded from backend to ensure IDs match database)
  let manufacturers = $state<Manufacturer[]>([]);
  let railwayCompanies = $state<RailwayCompany[]>([]);
  let sellers = $state<SellerView[]>([]);

  // UI state
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);
  let showPurchaseSection = $state(false);
  let isRollingStockExpanded = $state(false);
  let formEl: HTMLFormElement | undefined = $state();

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
      priceCurrency: settingsState.settings.currency ?? 'EUR',
      purchaseCondition: null,
      modelCondition: null,
      boxCondition: null,
      notes: '',
      purchaseDate: new Date().toISOString().split('T')[0],
      purchaseType: 'STANDARD',
      depositAmount: null,
      depositCurrency: null,
      preorderTotalAmount: null,
      preorderTotalCurrency: null,
      expectedDate: null
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
      rollingStocks: [],
      purchase: createDefaultPurchaseState()
    };
  }

  const { form, errors, tainted, enhance, reset, isTainted } = superForm(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    createDefaultFormState() as any,
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zod(addCollectionSchema as any),
      onUpdate: async ({ form: fd }) => {
        if (fd.valid) {
          isSubmitting = true;
          try {
            const args = toAddRailwayModelArgs($form as AddModelFormState);
            const success = await collectionService.addRailwayModel(args);
            if (success) {
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              reset({ data: createDefaultFormState() as any });
              onSuccess();
            }
          } catch (err) {
            console.error('Error submitting railway model:', err);
          } finally {
            isSubmitting = false;
          }
        }
      }
    }
  );

  const hasChanges = $derived(isTainted($tainted));

  // Watch for drawer open/close
  $effect(() => {
    if (open) {
      handleOpen();
    }
  });

  async function handleOpen() {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    reset({ data: createDefaultFormState() as any });
    showPurchaseSection = false;
    isRollingStockExpanded = false;

    // Load all reference data from backend to ensure IDs match database
    isLoadingData = true;
    try {
      const [mfgResult, rcResult, sellerResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getRailwayCompanies(),
        commands.getSellers()
      ]);

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
    $form.rollingStocks = [
      ...($form.rollingStocks as RollingStockFormEntry[]),
      createDefaultRollingStock()
    ];
  }

  function handleRemoveRollingStock(uid: string) {
    const stocks = $form.rollingStocks as RollingStockFormEntry[];
    $form.rollingStocks = stocks.filter((rs) => rs.uid !== uid);
  }

  function toAddRailwayModelArgs(formState: AddModelFormState): AddRailwayModelToCollectionArgs {
    const today = new Date().toISOString().split('T')[0];
    const isPreorder = formState.purchase.purchaseType === 'PREORDER';

    // priceAmount is stored as integer cents in the form state
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
      priceAmount: priceInCents as number,
      priceCurrency: formState.purchase.priceCurrency,
      sellerId: formState.purchase.sellerId,
      addedDate: today,
      purchaseDate: formState.purchase.purchaseDate || today,
      purchaseCondition: formState.purchase.purchaseCondition,
      modelCondition: formState.purchase.modelCondition,
      boxCondition: formState.purchase.boxCondition,
      notes: formState.purchase.notes || null,
      purchaseType: formState.purchase.purchaseType,
      depositAmount: isPreorder ? (formState.purchase.depositAmount ?? null) : null,
      depositCurrency: isPreorder
        ? (formState.purchase.depositCurrency ?? formState.purchase.priceCurrency)
        : null,
      preorderTotalAmount: isPreorder ? (formState.purchase.preorderTotalAmount ?? null) : null,
      preorderTotalCurrency: isPreorder
        ? (formState.purchase.preorderTotalCurrency ?? formState.purchase.priceCurrency)
        : null,
      expectedDate: isPreorder ? (formState.purchase.expectedDate ?? null) : null
    };
  }

  // Flatten Superforms array errors to string | undefined for child components
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const rsErrors = $derived($errors.rollingStocks as any);
  const mappedErrors = $derived({
    manufacturerId: $errors.manufacturerId?.[0] as string | undefined,
    productCode: $errors.productCode?.[0] as string | undefined,
    description: $errors.description?.[0] as string | undefined,
    category: $errors.category?.[0] as string | undefined,
    scale: $errors.scale?.[0] as string | undefined,
    powerMethod: $errors.powerMethod?.[0] as string | undefined,
    epoch: $errors.epoch?.[0] as string | undefined,
    rollingStocks: rsErrors?._errors?.[0] as string | undefined,
    rollingStockErrors: ($form.rollingStocks as RollingStockFormEntry[]).map((_, i) => ({
      railwayCompanyId: rsErrors?.[i]?.railwayCompanyId?.[0] as string | undefined,
      seriesCode: rsErrors?.[i]?.seriesCode?.[0] as string | undefined,
      category: rsErrors?.[i]?.category?.[0] as string | undefined
    }))
  });

  function handleSubmit() {
    formEl?.requestSubmit();
  }
</script>

<DrawerShell {open} {onClose} size="xl" {hasChanges} labelledby="drawer-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="drawer-title"
      title={m.add_model_title()}
      subtitle={m.add_model_subtitle()}
      icon={TrainFront}
      onClose={requestClose}
    />
  {/snippet}

  <form bind:this={formEl} use:enhance class="space-y-6">
    <!-- Hidden submit button to enable Enter-to-submit keyboard navigability -->
    <button type="submit" class="hidden" aria-hidden="true" tabindex="-1"></button>
    <ModelSearchSection
      bind:form={$form as AddModelFormState}
      {manufacturers}
      {railwayCompanies}
      {sellers}
      bind:showPurchaseSection
      bind:isRollingStockExpanded
      validationErrors={mappedErrors}
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
