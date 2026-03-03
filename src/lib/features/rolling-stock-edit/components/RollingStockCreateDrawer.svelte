<script lang="ts">
  import { X } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import { commands, type RailwayModelId, type RollingStockId } from '$lib/bindings';
  import { onMount } from 'svelte';

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
    { value: '', label: '—' },
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

      <div class="space-y-4">
        <!-- Railway Company -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-company">
            {m.model_rolling_stock_field_company()} <span class="text-red-400">*</span>
          </label>
          {#if companyOptions.length > 0}
            <BadgePicker
              value={form.railwayCompanyName || '—'}
              options={companyOptions}
              onSelect={async (id) => {
                form.railwayCompanyId = id;
                form.railwayCompanyName = companyOptions.find((c) => c.id === id)?.label ?? id;
              }}
            />
          {:else}
            <p class="text-xs text-zinc-500">Loading…</p>
          {/if}
        </div>

        <!-- Category -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-category">
            {m.rolling_stock_field_category()} <span class="text-red-400">*</span>
          </label>
          <select
            id="create-category"
            bind:value={form.category}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          >
            {#each categoryOptions as opt (opt.value)}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>

        <!-- Series Code -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-series-code">
            {m.rolling_stock_field_series_code()} <span class="text-red-400">*</span>
          </label>
          <input
            id="create-series-code"
            type="text"
            bind:value={form.seriesCode}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          />
        </div>

        <!-- Road Number -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-road-number">
            {m.rolling_stock_field_road_number()}
          </label>
          <input
            id="create-road-number"
            type="text"
            bind:value={form.roadNumber}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          />
        </div>

        <!-- Livery -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-livery">
            {m.rolling_stock_field_livery()}
          </label>
          <input
            id="create-livery"
            type="text"
            bind:value={form.livery}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          />
        </div>

        <!-- Depot -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-depot">
            {m.rolling_stock_field_depot()}
          </label>
          <input
            id="create-depot"
            type="text"
            bind:value={form.depot}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          />
        </div>

        <!-- Control Type -->
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-control">
            {m.model_rolling_stock_field_control()}
          </label>
          <select
            id="create-control"
            bind:value={form.control}
            class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
          >
            {#each controlOptions as opt (opt.value)}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end gap-3 border-t border-zinc-800 px-6 py-4">
      <Button variant="ghost" onclick={requestClose} disabled={isSaving}>
        {m.specs_drawer_cancel()}
      </Button>
      <Button
        onclick={handleSave}
        disabled={isSaving || !isValid}
        class="bg-[#E2994F] text-black hover:bg-[#E2994F]/90"
      >
        {#if isSaving}
          <span
            class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-black border-t-transparent"
          ></span>
        {/if}
        {m.specs_drawer_save()}
      </Button>
    </div>
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
