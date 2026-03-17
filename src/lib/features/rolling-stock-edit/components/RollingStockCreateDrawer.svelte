<script lang="ts">
  import { Train } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { toaster } from '$lib/toaster';
  import { commands, type RailwayModelId, type RollingStockId } from '$lib/bindings';
  import { onMount } from 'svelte';
  import RollingStockCategoryFields from './RollingStockCategoryFields.svelte';
  import RollingStockBasicFields from './RollingStockBasicFields.svelte';
  import RollingStockControlField from './RollingStockControlField.svelte';
  import { DrawerShell, DrawerHeader, DrawerFooter } from '$lib/components/drawer';

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
</script>

<DrawerShell
  {open}
  {onClose}
  size="lg"
  hasChanges={isDirty}
  labelledby="rs-create-title"
  error={inlineError}
>
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="rs-create-title"
      title={m.rolling_stock_create_drawer_title()}
      icon={Train}
      onClose={requestClose}
    />
  {/snippet}

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

  {#snippet footer({ requestClose })}
    <DrawerFooter
      cancelLabel={m.specs_drawer_cancel()}
      submitLabel={m.specs_drawer_save()}
      onCancel={requestClose}
      onSubmit={handleSave}
      submitting={isSaving}
      disabled={!isValid}
    />
  {/snippet}
</DrawerShell>
