<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
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
  let showDiscardDialog = $state(false);
  let validationErrors = $state<AcquisitionValidationErrors>({});
  let scrollableEl = $state<HTMLDivElement | null>(null);

  // Form state
  let form = $state<AcquisitionFormState>(createDefaultFormState());

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

  // Reactive validation: re-run whenever form changes, but only after first submit attempt
  let _validationReactive = $derived.by(() => {
    if (!touched) return;
    validationErrors = validateForm(form);
  });

  // Watch for drawer open/close — lock scroll and load data
  $effect(() => {
    if (open) {
      handleOpen();

      const mainElement = document.querySelector('main');
      document.body.style.overflow = 'hidden';
      if (mainElement) mainElement.style.overflow = 'hidden';
    } else {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    }

    return () => {
      const mainElement = document.querySelector('main');
      document.body.style.overflow = '';
      if (mainElement) mainElement.style.overflow = '';
    };
  });

  // Auto-scroll to bottom when items are added
  $effect(() => {
    // Track items.length to trigger this effect
    const _len = form.items.length;
    if (scrollableEl && _len > 1) {
      scrollableEl.scrollTop = scrollableEl.scrollHeight;
    }
  });

  async function handleOpen() {
    form = createDefaultFormState();
    touched = false;
    validationErrors = {};
    showDiscardDialog = false;

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
    // Propagate to items that still have the old default value
    form.items = form.items.map((item) =>
      item[field] === oldValue ? { ...item, [field]: value } : item
    );
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

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm"
    onclick={handleCloseRequest}
    role="presentation"
  ></div>

  <!-- Drawer panel -->
  <div
    class="fixed inset-y-0 right-0 z-50 flex w-full max-w-2xl translate-x-0 flex-col bg-zinc-950 shadow-2xl transition-transform duration-300"
    role="dialog"
    aria-modal="true"
    aria-labelledby="acquisition-drawer-title"
  >
    <!-- Sticky header -->
    <div class="flex items-center justify-between border-b border-white/10 p-4">
      <div>
        <h2 id="acquisition-drawer-title" class="text-lg font-semibold text-zinc-100">
          {m.acquisition_drawer_title()}
        </h2>
        <p class="text-sm text-zinc-500">{m.acquisition_drawer_subtitle()}</p>
      </div>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        onclick={handleCloseRequest}
        aria-label={m.acquisition_cancel_button()}
      >
        <X size={16} />
      </Button>
    </div>

    <!-- Sticky session fields -->
    <div class="border-b border-white/10 px-4 py-3">
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

    <!-- Scrollable items area -->
    <div class="flex-1 space-y-3 overflow-y-auto p-4" bind:this={scrollableEl}>
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

    <!-- Sticky footer -->
    <AcquisitionFooter
      {isSubmitting}
      {isLoadingData}
      onAddItem={handleAddItem}
      onFinalize={handleFinalize}
    />
  </div>

  <!-- Discard changes confirmation dialog -->
  {#if showDiscardDialog}
    <div
      class="fixed inset-0 z-[60] flex items-center justify-center bg-background/80 backdrop-blur-sm"
    >
      <div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl">
        <h3 class="mb-2 text-lg font-bold text-foreground">{m.acquisition_discard_title()}</h3>
        <p class="mb-4 text-muted-foreground">{m.acquisition_discard_description()}</p>
        <div class="flex justify-end gap-3">
          <Button type="button" variant="ghost" onclick={handleDiscardCancel}>
            {m.acquisition_discard_cancel()}
          </Button>
          <Button type="button" variant="destructive" onclick={handleDiscardConfirm}>
            {m.acquisition_discard_confirm()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
