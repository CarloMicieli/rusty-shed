<script lang="ts">
  import { X } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import { commands, type RailwayModelId, type RollingStockId } from '$lib/bindings';
  import { onMount } from 'svelte';
  import RollingStockCategoryFields from './RollingStockCategoryFields.svelte';
  import RollingStockBasicFields from './RollingStockBasicFields.svelte';
  import RollingStockControlField from './RollingStockControlField.svelte';
  import DrawerActionFooter from './DrawerActionFooter.svelte';

  interface Props {
    /** Controls drawer visibility. */
    open: boolean;
    /** The parent railway model. */
    railwayModelId: RailwayModelId;
    /** Called after a successful creation with the new rolling stock id. */
    onCreated?: (id: RollingStockId) => void;
    /** Called when the drawer requests to close. */
    onClose: () => void;
  }

  let { open, railwayModelId, onCreated, onClose }: Props = $props();

  // ── Form state ──────────────────────────────────────────────────────────────
  interface FormState {
    railwayCompanyId: string;
    railwayCompanyName: string;
    category: string;
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    control: string;
  }

  const emptyForm: FormState = {
    railwayCompanyId: '',
    railwayCompanyName: '',
    category: '',
    seriesCode: '',
    roadNumber: '',
    livery: '',
    depot: '',
    control: ''
  };

  let form = $state<FormState>({ ...emptyForm });
  let originalForm = $state<FormState>({ ...emptyForm });
  let isSaving = $state(false);
  let inlineError = $state<string | null>(null);
  let showDiscardDialog = $state(false);

  // ── Derived ─────────────────────────────────────────────────────────────────
  const isDirty = $derived(JSON.stringify(form) !== JSON.stringify(originalForm));
  const isValid = $derived(
    form.seriesCode.trim().length > 0 &&
      form.railwayCompanyId.length > 0 &&
      form.category.length > 0
  );

  // ── Company options ──────────────────────────────────────────────────────────
  let companyOptions = $state<{ id: string; label: string }[]>([]);

  onMount(async () => {
    const result = await commands.getRailwayCompanies();
    if (result.status === 'ok') {
      companyOptions = result.data.map((c) => ({ id: c.id, label: c.name }));
    }
  });

  // ── Reset form when drawer opens ────────────────────────────────────────────
  $effect(() => {
    if (open) {
      form = { ...emptyForm };
      originalForm = { ...emptyForm };
      inlineError = null;
    }
  });

  // ── Category options ─────────────────────────────────────────────────────────
  const categoryOptions = [
    { value: '', label: '—' },
    { value: 'LOCOMOTIVE', label: 'Locomotive' },
    { value: 'ELECTRIC_MULTIPLE_UNIT', label: 'Electric Multiple Unit' },
    { value: 'PASSENGER_CAR', label: 'Passenger Car' },
    { value: 'FREIGHT_CAR', label: 'Freight Car' },
    { value: 'RAILCAR', label: 'Railcar' }
  ];

  // ── Control options ──────────────────────────────────────────────────────────
  const controlOptions = [
    { value: 'DCC_READY', label: 'DCC Ready' },
    { value: 'DCC_FITTED', label: 'DCC Fitted' },
    { value: 'DCC_SOUND', label: 'DCC Sound' },
    { value: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  // ── Save ─────────────────────────────────────────────────────────────────────
  async function handleSave() {
    if (!isValid) return;
    isSaving = true;
    inlineError = null;
    try {
      const result = await commands.addRollingStockToModel({
        railwayModelId,
        railwayCompanyId: form.railwayCompanyId,
        category: form.category,
        seriesCode: form.seriesCode.trim(),
        roadNumber: form.roadNumber || null,
        livery: form.livery || null,
        depot: form.depot || null,
        control: form.control || null
      });

      if (result.status === 'error') {
        inlineError = m.rolling_stock_create_error();
        return;
      }

      toaster.success(m.rolling_stock_create_success());
      onCreated?.(result.data);
      onClose();
    } finally {
      isSaving = false;
    }
  }

  // ── Close handling ───────────────────────────────────────────────────────────
  function requestClose() {
    if (isDirty) {
      showDiscardDialog = true;
    } else {
      onClose();
    }
  }

  function confirmDiscard() {
    showDiscardDialog = false;
    form = { ...emptyForm };
    onClose();
  }

  function cancelDiscard() {
    showDiscardDialog = false;
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && !showDiscardDialog && requestClose()} />

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-40 bg-black/40"
    role="presentation"
    tabindex="-1"
    onclick={requestClose}
    onkeydown={(e) => e.key === 'Escape' && requestClose()}
  ></div>

  <!-- Drawer panel -->
  <div
    class="fixed inset-y-0 right-0 z-50 flex w-full max-w-xl flex-col overflow-hidden border-l border-zinc-800 bg-[#0C0C0C] shadow-2xl"
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-label={m.rolling_stock_create_drawer_title()}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key !== 'Escape' && e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-zinc-800 px-6 py-4">
      <p class="text-[10px] font-medium tracking-widest text-zinc-500 uppercase">
        {m.rolling_stock_create_drawer_title()}
      </p>
      <button
        type="button"
        class="rounded p-1 text-zinc-500 transition-colors hover:bg-zinc-800 hover:text-zinc-200"
        onclick={requestClose}
        aria-label={m.specs_drawer_cancel()}
      >
        <X size={16} />
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      {#if inlineError}
        <div
          class="mb-4 rounded-lg border border-red-800/50 bg-red-950/40 px-4 py-3 text-sm text-red-400"
        >
          {inlineError}
        </div>
      {/if}

      <div class="space-y-6">
        <RollingStockCategoryFields
          bind:railwayCompanyId={form.railwayCompanyId}
          bind:railwayCompanyName={form.railwayCompanyName}
          bind:category={form.category}
          {companyOptions}
          {categoryOptions}
        />

        <RollingStockBasicFields
          bind:seriesCode={form.seriesCode}
          bind:roadNumber={form.roadNumber}
          bind:livery={form.livery}
          bind:depot={form.depot}
        />

        <RollingStockControlField bind:control={form.control} {controlOptions} />
      </div>
    </div>

    <!-- Footer -->
    <DrawerActionFooter
      {isSaving}
      disabled={!isValid}
      onSave={handleSave}
      onCancel={requestClose}
    />
  </div>

  <!-- Discard confirmation dialog -->
  {#if showDiscardDialog}
    <div
      class="fixed inset-0 z-60 flex items-center justify-center bg-black/60"
      role="presentation"
      tabindex="-1"
    >
      <div
        class="mx-4 w-full max-w-sm rounded-xl border border-zinc-700 bg-zinc-900 p-6 shadow-2xl"
        role="dialog"
        aria-modal="true"
      >
        <h2 class="mb-2 text-base font-semibold text-zinc-100">
          {m.specs_drawer_unsaved_title()}
        </h2>
        <p class="mb-6 text-sm text-zinc-400">{m.specs_drawer_unsaved_message()}</p>
        <div class="flex justify-end gap-3">
          <Button variant="ghost" onclick={cancelDiscard}>
            {m.specs_drawer_cancel()}
          </Button>
          <Button variant="destructive" onclick={confirmDiscard}>
            {m.specs_drawer_unsaved_confirm()}
          </Button>
        </div>
      </div>
    </div>
  {/if}
{/if}
