<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ShoppingBag } from 'lucide-svelte';
  import {
    commands,
    type Manufacturer,
    type PowerMethod,
    type Seller,
    type Scale,
    type SellerView
  } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { Button } from '$lib/components';
  import type { AcquisitionFormState, AcquisitionItemEntry, BatchDefaults } from './types.js';
  import { createDefaultFormState, createDefaultItem, toRecordAcquisitionArgs } from './types.js';
  import AcquisitionBatchFields from './components/AcquisitionBatchFields.svelte';
  import AcquisitionItemCard from './components/AcquisitionItemCard.svelte';
  import { DrawerShell, DrawerHeader, DrawerFooter, QuickAddShell } from '$lib/components/drawer';
  import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';
  import type { QuickAddTarget } from '$lib/features/quick-add/types';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 as zod } from 'sveltekit-superforms/adapters';
  import { acquisitionSchema } from '$lib/schemas/acquisition-form';

  interface Props {
    open: boolean;
    onClose: () => void;
    onSuccess: () => void;
  }

  let { open, onClose, onSuccess }: Props = $props();

  // Reference data
  let sellers = $state<SellerView[]>([]);
  let manufacturers = $state<Manufacturer[]>([]);

  // UI state
  let isSubmitting = $state(false);
  let isLoadingData = $state(false);
  let formEl: HTMLFormElement | undefined = $state();
  let quickAddTarget = $state<QuickAddTarget | null>(null);
  let quickAddItemUid = $state<string | null>(null);
  let quickAddDirty = $state(false);

  function getDefaults() {
    return {
      scale: (settingsState.settings?.favouriteScale as Scale) || null,
      powerMethod: (settingsState.settings?.powerMethod as PowerMethod) || null
    };
  }

  const { form, errors, tainted, enhance, reset, isTainted } = superForm(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    createDefaultFormState(getDefaults()) as any,
    {
      SPA: true,
      dataType: 'json',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      validators: zod(acquisitionSchema as any),
      onUpdate: async ({ form: fd }) => {
        if (!fd.valid) return;
        isSubmitting = true;
        try {
          const args = toRecordAcquisitionArgs($form as AcquisitionFormState, currency);
          const result = await commands.recordAcquisition(args);
          if (result.status === 'ok') {
            toaster.success(m.acquisition_toast_success());
            onSuccess();
            onClose();
          } else {
            toaster.error(m.acquisition_error_finalize());
          }
        } catch (e) {
          console.error('Error saving acquisition:', e);
          toaster.error(m.acquisition_error_finalize());
        } finally {
          isSubmitting = false;
        }
      }
    }
  );

  let currency = $derived(settingsState.settings.currency ?? 'EUR');
  let hasChanges = $derived(isTainted($tainted));

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const itemsErrors = $derived($errors.items as any);
  const formItems = $derived($form.items as AcquisitionItemEntry[]);

  $effect.pre(() => {
    if (open) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      reset({ data: createDefaultFormState(getDefaults()) as any });
    }
  });

  $effect(() => {
    if (open) {
      void loadReferenceData();
    }
  });

  async function loadReferenceData() {
    isLoadingData = true;
    try {
      const [mfgResult, sellerResult] = await Promise.all([
        commands.getManufacturers(),
        commands.getSellers()
      ]);
      manufacturers = mfgResult.status === 'ok' ? mfgResult.data : [];
      sellers = sellerResult.status === 'ok' ? sellerResult.data : [];
    } catch (e) {
      console.error('Error loading reference data:', e);
    } finally {
      isLoadingData = false;
    }
  }

  function handleAddItem() {
    $form.items = [...formItems, createDefaultItem()];
  }

  function handleDuplicate(uid: string) {
    const source = formItems.find((i) => i.uid === uid);
    if (!source) return;
    const clone: AcquisitionItemEntry = { ...source, uid: crypto.randomUUID(), productCode: '' };
    const idx = formItems.findIndex((i) => i.uid === uid);
    const next = [...formItems];
    next.splice(idx + 1, 0, clone);
    $form.items = next;
  }

  function handleRemove(uid: string) {
    if (formItems.length <= 1) return;
    $form.items = formItems.filter((i) => i.uid !== uid);
  }

  function handleUpdateItem(uid: string, patch: Partial<AcquisitionItemEntry>) {
    $form.items = formItems.map((i) => (i.uid === uid ? { ...i, ...patch } : i));
  }

  function handleBatchDefaultChange(field: 'scale' | 'powerMethod', value: string | null) {
    $form.batchDefaults = { ...($form.batchDefaults as BatchDefaults), [field]: value };
  }

  function handleSubmit() {
    formEl?.requestSubmit();
  }

  function openManufacturerQuickAdd(itemUid: string) {
    if (quickAddTarget) return;
    quickAddTarget = 'manufacturer';
    quickAddItemUid = itemUid;
  }

  function openSellerQuickAdd() {
    if (quickAddTarget) return;
    quickAddTarget = 'seller';
    quickAddItemUid = null;
  }

  function closeQuickAdd() {
    if (quickAddDirty && !window.confirm(m.quick_add_dirty_discard_confirm())) {
      return;
    }
    quickAddTarget = null;
    quickAddItemUid = null;
    quickAddDirty = false;
  }

  function handleQuickAddSuccess(entity: Manufacturer | Seller) {
    if (quickAddTarget === 'manufacturer' && quickAddItemUid) {
      const created = entity as Manufacturer;
      manufacturers = [...manufacturers, created];
      handleUpdateItem(quickAddItemUid, { manufacturerId: created.id });
      toaster.success(m.quick_add_manufacturer_success({ name: created.name }));
    } else {
      const created = entity as Seller;
      const nextSeller: SellerView = {
        id: created.id,
        name: created.name,
        sellerType: created.sellerType,
        email: created.email,
        phone: created.phone,
        websiteUrl: created.websiteUrl,
        address: created.address,
        isSystemSeeded: false,
        usageCount: 0
      };
      sellers = [...sellers, nextSeller];
      $form.sellerId = created.id;
      toaster.success(m.quick_add_seller_success({ name: created.name }));
    }

    quickAddTarget = null;
    quickAddItemUid = null;
    quickAddDirty = false;
  }

  const quickAddTitle = $derived.by(() => {
    switch (quickAddTarget) {
      case 'manufacturer':
        return m.quick_add_drawer_title_manufacturer();
      case 'seller':
        return m.quick_add_drawer_title_seller();
      default:
        return '';
    }
  });

  const quickAddNames = $derived.by(() => {
    if (quickAddTarget === 'manufacturer') {
      return manufacturers.map((entry) => entry.name);
    }
    return sellers.map((entry) => entry.name);
  });
</script>

<DrawerShell
  {open}
  {onClose}
  size="xl"
  {hasChanges}
  dimmed={quickAddTarget !== null}
  labelledby="acquisition-drawer-title"
  discardTitle={m.acquisition_discard_title()}
  discardDescription={m.acquisition_discard_description()}
  discardConfirm={m.acquisition_discard_confirm()}
  discardCancel={m.acquisition_discard_cancel()}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="acquisition-drawer-title"
      title={m.acquisition_drawer_title()}
      subtitle={m.acquisition_drawer_subtitle()}
      icon={ShoppingBag}
      onClose={requestClose}
    />
  {/snippet}

  {#snippet stickyBand()}
    <div class="px-4 py-3">
      <AcquisitionBatchFields
        sellerId={$form.sellerId as string | null}
        onSellerChange={(id) => ($form.sellerId = id)}
        onQuickAddSeller={openSellerQuickAdd}
        purchaseDate={$form.purchaseDate as string}
        onDateChange={(date) => ($form.purchaseDate = date)}
        batchDefaults={$form.batchDefaults as BatchDefaults}
        onBatchDefaultChange={handleBatchDefaultChange}
        {sellers}
      />
    </div>
  {/snippet}

  <form bind:this={formEl} use:enhance class="space-y-3">
    <!-- Hidden submit button to enable Enter-to-submit keyboard navigability -->
    <button type="submit" class="hidden" aria-hidden="true" tabindex="-1"></button>
    {#if itemsErrors?._errors?.[0]}
      <div
        class="rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
      >
        {itemsErrors._errors[0]}
      </div>
    {/if}

    {#if isLoadingData}
      <div class="flex items-center justify-center py-12">
        <p class="text-sm text-zinc-500">Loading…</p>
      </div>
    {:else}
      {#each formItems as item, idx (item.uid)}
        <AcquisitionItemCard
          {item}
          index={idx}
          {manufacturers}
          {currency}
          errors={{
            manufacturerId: itemsErrors?.[idx]?.manufacturerId?.[0],
            productCode: itemsErrors?.[idx]?.productCode?.[0],
            category: itemsErrors?.[idx]?.category?.[0]
          }}
          canRemove={formItems.length > 1}
          onUpdate={handleUpdateItem}
          onDuplicate={handleDuplicate}
          onRemove={handleRemove}
          onQuickAddManufacturer={openManufacturerQuickAdd}
        />
      {/each}
    {/if}
  </form>

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.acquisition_cancel_button()}
      submitLabel={isSubmitting
        ? m.acquisition_finalizing_button()
        : m.acquisition_finalize_button()}
      onCancel={requestClose}
      onSubmit={handleSubmit}
      submitting={isSubmitting}
      isLoading={isLoadingData}
    >
      {#snippet leading()}
        <Button
          type="button"
          variant="rusty"
          onclick={handleAddItem}
          disabled={isSubmitting || isLoadingData}
          class="shadow-lg shadow-amber-500/10"
        >
          {m.acquisition_add_item_button()}
        </Button>
      {/snippet}
    </DrawerFooter>
  {/snippet}
</DrawerShell>

<QuickAddShell open={quickAddTarget !== null} title={quickAddTitle} onDismiss={closeQuickAdd}>
  {#if quickAddTarget}
    <QuickAddEntityForm
      target={quickAddTarget}
      existingNames={quickAddNames}
      onSuccess={handleQuickAddSuccess}
      onCancel={closeQuickAdd}
      onDirtyChange={(dirty) => (quickAddDirty = dirty)}
    />
  {/if}

  {#snippet footer()}
    <div class="text-xs text-muted-foreground">{m.quick_add_footer_hint()}</div>
  {/snippet}
</QuickAddShell>
