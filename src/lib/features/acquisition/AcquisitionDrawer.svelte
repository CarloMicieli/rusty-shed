<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ShoppingBag, X } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import { commands, type Manufacturer, type SellerView } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import type {
    AcquisitionFormState,
    AcquisitionItemEntry,
    AcquisitionValidationErrors
  } from './types.js';
  import {
    createDefaultFormState,
    createDefaultItem,
    validateForm,
    hasErrors,
    toRecordAcquisitionArgs
  } from './types.js';
  import AcquisitionHeader from './components/AcquisitionHeader.svelte';
  import AcquisitionItemCard from './components/AcquisitionItemCard.svelte';
  import AcquisitionFooter from './components/AcquisitionFooter.svelte';
  import { DrawerShell } from '$lib/components/drawer';

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
  let touched = $state(false);
  let validationErrors = $state<AcquisitionValidationErrors>({});
  // Form state
  let form = $state<AcquisitionFormState>(
    createDefaultFormState({
      scale: settingsState.settings?.favouriteScale || null,
      powerMethod: settingsState.settings?.powerMethod || null
    })
  );

  // Derived values
  let hasChanges = $derived(
    form.sellerId !== null ||
      form.items.length > 1 ||
      form.items.some(
        (i) =>
          i.manufacturerId !== null || i.productCode.trim() !== '' || i.description.trim() !== ''
      )
  );

  let currency = $derived(settingsState.settings.currency ?? 'EUR');

  // Reactive validation
  let _validationReactive = $derived.by(() => {
    if (!touched) return;
    validationErrors = validateForm(form);
  });

  // Watch for drawer open — load data
  $effect(() => {
    if (open) {
      handleOpen();
    }
  });

  async function handleOpen() {
    form = createDefaultFormState({
      scale: settingsState.settings?.favouriteScale || null,
      powerMethod: settingsState.settings?.powerMethod || null
    });
    touched = false;
    validationErrors = {};

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
    form.items = [...form.items, createDefaultItem(form.batchDefaults)];
  }

  function handleDuplicate(uid: string) {
    const source = form.items.find((i) => i.uid === uid);
    if (!source) return;
    const clone: AcquisitionItemEntry = {
      ...source,
      uid: crypto.randomUUID(),
      productCode: ''
    };
    const idx = form.items.findIndex((i) => i.uid === uid);
    const next = [...form.items];
    next.splice(idx + 1, 0, clone);
    form.items = next;
  }

  function handleRemove(uid: string) {
    if (form.items.length <= 1) return;
    form.items = form.items.filter((i) => i.uid !== uid);
  }

  function handleUpdateItem(uid: string, patch: Partial<AcquisitionItemEntry>) {
    form.items = form.items.map((i) => (i.uid === uid ? { ...i, ...patch } : i));
  }

  function handleBatchDefaultChange(field: 'scale' | 'powerMethod', value: string | null) {
    const oldValue = form.batchDefaults[field];
    form.batchDefaults = { ...form.batchDefaults, [field]: value };
    form.items = form.items.map((item) =>
      item[field] === oldValue ? { ...item, [field]: value } : item
    );
  }

  async function handleFinalize() {
    touched = true;
    const errors = validateForm(form);
    validationErrors = errors;

    if (hasErrors(errors)) return;

    isSubmitting = true;
    try {
      const args = toRecordAcquisitionArgs(form, currency);
      const result = await commands.recordAcquisition(args);
      if (result.status === 'ok') {
        toaster.success(m.acquisition_toast_success());
        onSuccess();
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
</script>

<DrawerShell
  {open}
  {onClose}
  size="lg"
  {hasChanges}
  labelledby="acquisition-drawer-title"
  discardTitle={m.acquisition_discard_title()}
  discardDescription={m.acquisition_discard_description()}
  discardConfirm={m.acquisition_discard_confirm()}
  discardCancel={m.acquisition_discard_cancel()}
>
  {#snippet header({ requestClose })}
    <div class="flex items-center justify-between p-4">
      <div class="flex items-center gap-3">
        <div class="rounded-lg bg-emerald-500/10 p-2">
          <ShoppingBag class="h-5 w-5 text-emerald-500" />
        </div>
        <div>
          <h2 id="acquisition-drawer-title" class="text-lg font-semibold text-zinc-100">
            {m.acquisition_drawer_title()}
          </h2>
          <p class="text-sm text-zinc-500">{m.acquisition_drawer_subtitle()}</p>
        </div>
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        onclick={requestClose}
        aria-label={m.acquisition_cancel_button()}
      >
        <X size={16} />
      </Button>
    </div>
  {/snippet}

  {#snippet stickyBand()}
    <div class="px-4 py-3">
      <AcquisitionHeader
        sellerId={form.sellerId}
        onSellerChange={(id) => (form.sellerId = id)}
        purchaseDate={form.purchaseDate}
        onDateChange={(date) => (form.purchaseDate = date)}
        batchDefaults={form.batchDefaults}
        onBatchDefaultChange={handleBatchDefaultChange}
        {sellers}
      />
    </div>
  {/snippet}

  <div class="space-y-3">
    {#if validationErrors.general}
      <div
        class="rounded-lg border border-destructive/40 bg-destructive/10 p-3 text-sm text-destructive"
      >
        {validationErrors.general}
      </div>
    {/if}

    {#if isLoadingData}
      <div class="flex items-center justify-center py-12">
        <p class="text-sm text-zinc-500">Loading…</p>
      </div>
    {:else}
      {#each form.items as item (item.uid)}
        <AcquisitionItemCard
          {item}
          index={form.items.indexOf(item)}
          {manufacturers}
          {currency}
          errors={validationErrors.items?.[form.items.indexOf(item)] ?? {}}
          canRemove={form.items.length > 1}
          onUpdate={handleUpdateItem}
          onDuplicate={handleDuplicate}
          onRemove={handleRemove}
        />
      {/each}
    {/if}
  </div>

  {#snippet footer({ requestClose: _requestClose })}
    <AcquisitionFooter
      {isSubmitting}
      {isLoadingData}
      onAddItem={handleAddItem}
      onFinalize={handleFinalize}
    />
  {/snippet}
</DrawerShell>
