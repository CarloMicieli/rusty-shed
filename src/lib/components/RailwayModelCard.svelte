<script lang="ts">
  import { tick } from 'svelte';
  /**
   * RailwayModelCard Component
   *
   * A reusable component for displaying railway model information including:
   * - Product details (manufacturer, code, scale, era, power method, category)
   * - Image with optional upload/replace functionality (grid-pattern aesthetic)
   * - Rolling stock specifications (single-unit spec grid or multi-unit mini-cards)
   * - Tabbed navigation between model details and rolling stock
   *
   * @component
   * @example
   * ```svelte
   * <RailwayModelCard {model} editable={true} onImageUploaded={handleUpload} />
   * ```
   */
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Badge } from '$lib/components/ui/badge';
  import { Button } from '$lib/components/ui/button';
  import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
  import { Upload, Loader2, TrainFront, Box, Users, Layers } from 'lucide-svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import * as m from '$lib/paraglide/messages';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import RichTextEditor from '$lib/components/RichTextEditor.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import LanguageFallbackBadge from '$lib/components/LanguageFallbackBadge.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import { commands, type Scale, type Language, type RollingStockView } from '$lib/bindings';

  interface RailwayModelCardProps {
    /** The railway model to display */
    model: RailwayModel;

    /** Whether to enable editing features (image upload) */
    editable?: boolean;

    /** Optional class for styling customization */
    class?: string;

    /** Callback when image is uploaded (if editable=true) */
    onImageUploaded?: (imagePath: string) => void;

    /** Callback when errors occur */
    onError?: (error: string) => void;

    /** Callback after a successful model update */
    onModelUpdated?: () => Promise<void> | void;
  }

  let {
    model,
    editable = false,
    class: className = '',
    onImageUploaded,
    onError,
    onModelUpdated
  }: RailwayModelCardProps = $props();

  // Component-local state
  let activeTab = $state<'details' | 'rolling-stock'>('details');
  let isDragging = $state(false);
  let isUploading = $state(false);
  let _uploadProgress = $state(0);

  // Current locale as Language type
  const currentLocale = getLocale() as Language;

  // Local copies of editable text fields — updated optimistically on successful save
  let localDescription = $state('');
  let localDetails = $state('');
  let localScale = $state('');
  let localEra = $state('');
  let isEditingDescription = $state(false);
  let descriptionDraft = $state('');
  let isSavingDescription = $state(false);
  let descriptionInput = $state<HTMLInputElement | null>(null);

  $effect(() => {
    localDescription = model.description ?? '';
    localDetails = model.details ?? '';
    localScale = model.scale ?? '';
    localEra = model.era ?? '';
  });

  $effect(() => {
    if (!isEditingDescription) {
      descriptionDraft = localDescription;
    }
  });

  $effect(() => {
    if (isEditingDescription) {
      void tick().then(() => descriptionInput?.focus());
    }
  });

  // Load rolling stock specs when editable=true
  $effect(() => {
    if (editable && model.rolling_stock) {
      for (const unit of model.rolling_stock) {
        if (!rollingStockSpecLoaded.has(unit.id)) {
          void loadRollingStockSpec(unit.id);
        }
      }
    }
  });

  const scaleOptions = [
    { id: 'H0', label: 'H0 (1:87)' },
    { id: 'H0m', label: 'H0m (1:87)' },
    { id: 'H0e', label: 'H0e (1:87)' },
    { id: 'N', label: 'N (1:160)' },
    { id: 'TT', label: 'TT (1:120)' },
    { id: 'Z', label: 'Z (1:220)' },
    { id: 'G', label: 'G (1:22.5)' },
    { id: 'Scale1', label: '1 (1:32)' },
    { id: 'Scale0', label: '0 (1:43.5)' },
    { id: 'Scale00', label: '00 (1:76.2)' }
  ];

  const eraOptions = [
    'I',
    'II',
    'IIa',
    'IIb',
    'IIc',
    'III',
    'IIIa',
    'IIIb',
    'IV',
    'IVa',
    'IVb',
    'V',
    'Va',
    'Vb',
    'Vc',
    'VI'
  ].map((e) => ({ id: e, label: e }));

  // Derived values
  let isSingleUnit = $derived(model.rolling_stock?.length === 1);

  // ── Rolling Stock Edit State ────────────────────────────────────────────────────
  // Per-unit spec state: maps unit.id → FormState
  interface RsFormState {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    flywheelFitted: boolean | null;
    bodyShell: string;
    chassis: string;
    interiorLights: string;
    lights: string;
    dccInterface: string;
    control: string;
    couplingSocket: string;
    closeCouplers: boolean | null;
    digitalShunting: boolean | null;
  }

  let rollingStockFormState = $state<Map<string, RsFormState>>(new Map());
  let rollingStockSpecLoaded = $state<Set<string>>(new Set());

  // Option lists for rolling stock fields
  const controlOptions = [
    { id: '', label: '—' },
    { id: 'DCC_READY', label: 'DCC Ready' },
    { id: 'DCC_FITTED', label: 'DCC Fitted' },
    { id: 'DCC_SOUND', label: 'DCC Sound' },
    { id: 'NO_DCC', label: 'Analogue (No DCC)' }
  ];

  const dccInterfaceOptions = [
    { id: '', label: '—' },
    { id: 'NEM_651', label: 'NEM 651' },
    { id: 'NEM_652', label: 'NEM 652' },
    { id: 'NEM_654', label: 'NEM 654' },
    { id: 'PLUX_8', label: 'PLUX 8' },
    { id: 'PLUX_12', label: 'PLUX 12' },
    { id: 'PLUX_16', label: 'PLUX 16' },
    { id: 'PLUX_22', label: 'PLUX 22' },
    { id: 'NEXT_18', label: 'Next18' },
    { id: 'NEXT_18_S', label: 'Next18-S' },
    { id: 'MTC_21', label: 'MTC 21' }
  ];

  const couplingSockeOptions = [
    { id: '', label: '—' },
    { id: 'NONE', label: 'None' },
    { id: 'NEM_355', label: 'NEM 355' },
    { id: 'NEM_356', label: 'NEM 356' },
    { id: 'NEM_357', label: 'NEM 357' },
    { id: 'NEM_359', label: 'NEM 359' },
    { id: 'NEM_360', label: 'NEM 360' },
    { id: 'NEM_362', label: 'NEM 362' },
    { id: 'NEM_365', label: 'NEM 365' }
  ];

  function getEmptyRsForm(): RsFormState {
    return {
      seriesCode: '',
      roadNumber: '',
      livery: '',
      depot: '',
      flywheelFitted: null,
      bodyShell: '',
      chassis: '',
      interiorLights: '',
      lights: '',
      dccInterface: '',
      control: '',
      couplingSocket: '',
      closeCouplers: null,
      digitalShunting: null
    };
  }

  function extractRsDataFromView(view: RollingStockView): RsFormState {
    let rs;
    if ('locomotive' in view) rs = view.locomotive;
    else if ('electricMultipleUnit' in view) rs = view.electricMultipleUnit;
    else if ('freightCar' in view) rs = view.freightCar;
    else if ('passengerCar' in view) rs = view.passengerCar;
    else if ('railcar' in view) rs = view.railcar;
    else return getEmptyRsForm();

    const ts = rs.technical_specifications;
    return {
      seriesCode: rs.series_code,
      roadNumber: rs.road_number ?? '',
      livery: rs.livery ?? '',
      depot: 'depot' in rs ? (rs.depot ?? '') : '',
      flywheelFitted:
        ts?.flywheel_fitted === 'YES' ? true : ts?.flywheel_fitted === 'NO' ? false : null,
      bodyShell: ts?.body_shell ?? '',
      chassis: ts?.chassis ?? '',
      interiorLights: ts?.interior_lights ?? '',
      lights: ts?.lights ?? '',
      dccInterface: 'dcc_interface' in rs ? (rs.dcc_interface ?? '') : '',
      control: 'control' in rs ? (rs.control ?? '') : '',
      couplingSocket: ts?.coupling?.socket ?? '',
      closeCouplers:
        ts?.coupling?.close_couplers === 'YES'
          ? true
          : ts?.coupling?.close_couplers === 'NO'
            ? false
            : null,
      digitalShunting:
        ts?.coupling?.digital_shunting === 'YES'
          ? true
          : ts?.coupling?.digital_shunting === 'NO'
            ? false
            : null
    };
  }

  async function loadRollingStockSpec(unitId: string) {
    if (rollingStockSpecLoaded.has(unitId)) return;

    try {
      const result = await commands.getRailwayModelById(model.id, getLocale());
      if (result.status === 'error' || !result.data) {
        rollingStockFormState.set(unitId, getEmptyRsForm());
        rollingStockSpecLoaded.add(unitId);
        return;
      }

      const rsView = result.data.rollingStock.find((r) => {
        if ('locomotive' in r) return r.locomotive.id === unitId;
        if ('electricMultipleUnit' in r) return r.electricMultipleUnit.id === unitId;
        if ('freightCar' in r) return r.freightCar.id === unitId;
        if ('passengerCar' in r) return r.passengerCar.id === unitId;
        if ('railcar' in r) return r.railcar.id === unitId;
        return false;
      });

      if (!rsView) {
        rollingStockFormState.set(unitId, getEmptyRsForm());
      } else {
        rollingStockFormState.set(unitId, extractRsDataFromView(rsView));
      }
      rollingStockSpecLoaded.add(unitId);
    } catch {
      rollingStockFormState.set(unitId, getEmptyRsForm());
      rollingStockSpecLoaded.add(unitId);
    }
  }

  async function saveRollingStockIdentification(
    unitId: string,
    field: 'series' | 'roadNumber' | 'livery' | 'depot',
    value: string,
    unit: (typeof model.rolling_stock)[0]
  ) {
    // Build the full request with current values
    const currentForm = rollingStockFormState.get(unitId) || getEmptyRsForm();
    const seriesCode = field === 'series' ? value : currentForm.seriesCode || unit.series_code;
    const roadNumber =
      field === 'roadNumber' ? value || null : currentForm.roadNumber || unit.road_number || null;
    const livery = field === 'livery' ? value || null : currentForm.livery || unit.livery || null;
    const depot = field === 'depot' ? value || null : currentForm.depot || unit.depot || null;

    const result = await commands.updateRollingStockIdentification({
      railwayModelId: model.id,
      rollingStockId: unitId,
      seriesCode,
      roadNumber,
      livery,
      depot
    });

    if (result.status === 'error') {
      throw new Error('Failed to save');
    }

    // Update local form state
    if (!currentForm) {
      rollingStockFormState.set(unitId, getEmptyRsForm());
    }
    const form = rollingStockFormState.get(unitId)!;
    form.seriesCode = seriesCode;
    form.roadNumber = roadNumber ?? '';
    form.livery = livery ?? '';
    form.depot = depot ?? '';

    await notifyModelUpdated();
  }

  async function saveRollingStockSpec(unitId: string, field: string, value: string) {
    const form = rollingStockFormState.get(unitId);
    if (!form) return;

    // Update the field in form
    (form as unknown as Record<string, string | boolean | null>)[field] = value;

    const result = await commands.updateRollingStockSpecifications({
      railwayModelId: model.id,
      rollingStockId: unitId,
      seriesCode: form.seriesCode,
      roadNumber: form.roadNumber || null,
      livery: form.livery || null,
      depot: form.depot || null,
      flywheelFitted: form.flywheelFitted,
      bodyShell: form.bodyShell || null,
      chassis: form.chassis || null,
      interiorLights: form.interiorLights || null,
      lights: form.lights || null,
      dccInterface: (form.dccInterface || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['dccInterface'],
      control: (form.control || null) as Parameters<
        typeof commands.updateRollingStockSpecifications
      >[0]['control'],
      couplingSocket: form.couplingSocket || null,
      closeCouplers: form.closeCouplers,
      digitalShunting: form.digitalShunting
    });

    if (result.status === 'error') {
      throw new Error('Failed to save');
    }

    await notifyModelUpdated();
  }

  async function notifyModelUpdated() {
    await onModelUpdated?.();
  }

  // Power method display label (amber badge)
  let powerMethodLabel = $derived.by((): string => {
    if (!model.power_method) return '';
    const labels: Record<string, string> = {
      ac: 'AC',
      dc: 'DC',
      dcc: 'DCC',
      trix_express: 'TX'
    };
    return labels[model.power_method.toLowerCase()] ?? model.power_method.toUpperCase();
  });

  // Category-to-icon mapping for technical drawing placeholder
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const PlaceholderIcon: any = $derived.by(() => {
    const cat = model.category?.toLowerCase() ?? '';
    if (cat.includes('locomotive')) return TrainFront;
    if (cat.includes('freight')) return Box;
    if (cat.includes('passenger')) return Users;
    if (cat.includes('railcar') || cat.includes('train_set')) return Layers;
    return TrainFront;
  });

  async function saveDetails(value: string) {
    const result = await commands.updateRailwayModelText({
      railwayModelId: model.id,
      field: 'Details',
      value,
      lang: getLocale()
    });
    if (result.status === 'error') {
      onError?.(m.details_save_failed());
      throw new Error('Failed to save details');
    }
    localDetails = value;
    await notifyModelUpdated();
  }

  function startDescriptionEditing() {
    if (!editable || isSavingDescription) return;
    descriptionDraft = localDescription;
    isEditingDescription = true;
  }

  function cancelDescriptionEditing() {
    descriptionDraft = localDescription;
    isEditingDescription = false;
  }

  async function saveDescription() {
    if (!isEditingDescription || isSavingDescription) return;

    const value = descriptionDraft.trim();
    if (!value) {
      cancelDescriptionEditing();
      return;
    }

    if (value === localDescription) {
      isEditingDescription = false;
      return;
    }

    isSavingDescription = true;
    try {
      const result = await commands.updateRailwayModelText({
        railwayModelId: model.id,
        field: 'Description',
        value,
        lang: getLocale()
      });
      if (result.status === 'error') {
        onError?.(m.edit_save_error());
        throw new Error('Failed to save description');
      }
      localDescription = value;
      isEditingDescription = false;
      await notifyModelUpdated();
    } finally {
      isSavingDescription = false;
    }
  }

  async function saveScale(id: string) {
    const result = await commands.updateRailwayModelClassification({
      railwayModelId: model.id,
      scale: id as Scale,
      epoch: null
    });
    if (result.status === 'error') {
      throw new Error('Failed to save scale');
    }
    localScale = id;
    await notifyModelUpdated();
  }

  async function saveEra(id: string) {
    const result = await commands.updateRailwayModelClassification({
      railwayModelId: model.id,
      scale: null,
      epoch: id
    });
    if (result.status === 'error') {
      throw new Error('Failed to save era');
    }
    localEra = id;
    await notifyModelUpdated();
  }

  async function handleBrowseImage() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp'] }]
      });
      if (selected && typeof selected === 'string') {
        await uploadImage(selected);
      }
    } catch (err) {
      console.error('File selection error:', err);
      onError?.(m.upload_error_unknown());
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files[0];

      if (!file.type.startsWith('image/')) {
        onError?.(m.error_invalid_image_format());
        return;
      }

      if (file.size > 50 * 1024 * 1024) {
        onError?.('File too large. Maximum size is 50 MB.');
        return;
      }

      try {
        const arrayBuffer = await file.arrayBuffer();
        const bytes = Array.from(new Uint8Array(arrayBuffer));
        isUploading = true;
        await invoke('upload_model_image_bytes', {
          args: { modelId: model.id, fileName: file.name, fileData: bytes }
        });
        onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
        await notifyModelUpdated();
      } catch (err) {
        console.error('Upload error:', err);
        onError?.(m.upload_error_unknown());
      } finally {
        isUploading = false;
      }
    }
  }

  async function uploadImage(filePath: string) {
    try {
      isUploading = true;
      await invoke('upload_model_image', {
        args: { modelId: model.id, filePath: filePath }
      });
      onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
      await notifyModelUpdated();
    } catch (err) {
      console.error('Upload error:', err);
      onError?.(m.upload_error_unknown());
    } finally {
      isUploading = false;
    }
  }
</script>

<div class="railway-model-card rounded-2xl border border-zinc-800 bg-[#0C0C0C] {className}">
  <!-- ═══ Header ════════════════════════════════════════════════════════════ -->
  <div class="flex items-start justify-between p-4 pb-3">
    <div class="min-w-0 flex-1">
      <div class="flex flex-wrap items-center gap-1.5">
        <span class="text-xs font-semibold text-zinc-200">{model.manufacturer}</span>
        <span class="text-zinc-700" aria-hidden="true">·</span>
        <span class="font-mono text-xs text-zinc-500">{model.product_code}</span>
      </div>
      <div class="mt-0.5">
        {#if editable}
          {#if isEditingDescription}
            <input
              bind:this={descriptionInput}
              class="w-full rounded-sm border border-zinc-700 bg-zinc-900/70 px-1.5 py-0.5 text-sm text-zinc-200 outline-none focus:border-[#E2994F]"
              bind:value={descriptionDraft}
              onblur={saveDescription}
              onkeydown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  void saveDescription();
                }
                if (e.key === 'Escape') {
                  e.preventDefault();
                  cancelDescriptionEditing();
                }
              }}
              disabled={isSavingDescription}
            />
          {:else}
            <button
              type="button"
              class="line-clamp-1 cursor-text text-left text-sm text-zinc-400 transition-colors hover:text-zinc-300"
              onclick={startDescriptionEditing}
            >
              {localDescription || m.details_placeholder()}
            </button>
          {/if}
        {:else if localDescription}
          <p class="line-clamp-1 text-sm text-zinc-400">{localDescription}</p>
        {/if}

        {#if localDescription && model.descriptionLang !== currentLocale}
          <div class="mt-1">
            <LanguageFallbackBadge lang={model.descriptionLang} />
          </div>
        {/if}
      </div>
    </div>

    <!-- Amber power method badge -->
    {#if model.power_method}
      <Badge
        class="ml-2 shrink-0 border-transparent bg-[#E2994F] px-1.5 py-0.5 text-[10px] font-bold text-black"
      >
        {powerMethodLabel}
      </Badge>
    {/if}
  </div>

  <!-- ═══ Grid-pattern icon area ═══════════════════════════════════════════ -->
  <div
    class="hero-section relative mx-4 aspect-video overflow-hidden rounded-xl bg-zinc-900 {isDragging
      ? 'ring-2 ring-[#E2994F]'
      : ''}"
    style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
    role="img"
    aria-label="Railway model image"
    ondragover={editable ? handleDragOver : undefined}
    ondragleave={editable ? handleDragLeave : undefined}
    ondrop={editable ? handleDrop : undefined}
  >
    {#if model.image_path}
      <img
        src={convertFileSrc(model.image_path)}
        alt="{model.manufacturer} {model.product_code}"
        class="block h-full w-full object-cover object-center"
      />
      {#if editable}
        <div class="absolute top-2 right-2">
          <Button
            variant="secondary"
            size="sm"
            onclick={handleBrowseImage}
            disabled={isUploading}
            aria-label={m.replace_image()}
          >
            {#if isUploading}
              <Loader2 class="h-4 w-4 animate-spin" />
            {:else}
              <Upload class="h-4 w-4" />
            {/if}
            <span class="ml-2">{m.replace_image()}</span>
          </Button>
        </div>
      {/if}
    {:else}
      <div class="flex h-full w-full items-center justify-center">
        {#if editable && !isUploading}
          <div class="flex flex-col items-center gap-3 text-zinc-600">
            <PlaceholderIcon class="size-12" />
            <p class="text-xs text-zinc-500">
              {isDragging ? m.drag_drop_image_here() : 'No image available'}
            </p>
            <Button variant="secondary" size="sm" onclick={handleBrowseImage}>
              <Upload class="mr-2 h-4 w-4" />
              {m.upload_image()}
            </Button>
          </div>
        {:else if isUploading}
          <div class="flex flex-col items-center gap-2">
            <Loader2 class="size-10 animate-spin text-zinc-600" />
            <p class="text-xs text-zinc-500">Uploading…</p>
          </div>
        {:else}
          <div class="flex flex-col items-center gap-2 text-zinc-600/70">
            <PlaceholderIcon class="size-12" />
            <p class="text-xs text-zinc-600">No image available</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- ═══ Specs bar ════════════════════════════════════════════════════════ -->
  <div
    data-testid="specs"
    class="mx-4 mt-3 grid grid-cols-4 divide-x divide-zinc-800 rounded-lg border border-zinc-800 bg-zinc-900/40 py-2"
  >
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">Category</span>
      <span class="text-xs text-zinc-200">{model.category ?? '—'}</span>
    </div>
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase"
        >{m.railway_model_field_scale()}</span
      >
      {#if editable}
        <BadgePicker value={localScale || '—'} options={scaleOptions} onSelect={saveScale} />
      {:else}
        <span class="font-mono text-xs text-zinc-200">{model.scale ?? '—'}</span>
      {/if}
    </div>
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase"
        >{m.railway_model_field_era()}</span
      >
      {#if editable}
        <BadgePicker value={localEra || '—'} options={eraOptions} onSelect={saveEra} />
      {:else}
        <span class="font-mono text-xs text-zinc-200">{model.era ?? '—'}</span>
      {/if}
    </div>
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">Status</span>
      <span class="text-xs text-zinc-200">{model.status}</span>
    </div>
  </div>

  <!-- ═══ Tabs ══════════════════════════════════════════════════════════════ -->
  <div data-testid="tabs-box" class="mx-4 mt-3 mb-4">
    <Tabs bind:value={activeTab} class="w-full">
      <TabsList
        class="grid h-auto w-full grid-cols-2 rounded-lg border border-zinc-800 bg-zinc-900 p-1"
      >
        <TabsTrigger
          value="details"
          class="rounded-md text-xs text-zinc-400 transition-colors
            data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
            dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
        >
          {m.railway_model_details()}
        </TabsTrigger>
        <TabsTrigger
          value="rolling-stock"
          class="rounded-md text-xs text-zinc-400 transition-colors
            data-[state=active]:bg-[#E2994F]/10 data-[state=active]:text-[#E2994F] data-[state=active]:shadow-none
            dark:text-zinc-400 dark:data-[state=active]:border-input dark:data-[state=active]:bg-[#E2994F]/10 dark:data-[state=active]:text-[#E2994F]"
        >
          {m.rolling_stock_list()}
        </TabsTrigger>
      </TabsList>

      <!-- ── Tab 1: Model Details ─────────────────────────────────────── -->
      <TabsContent value="details" class="mt-2">
        <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
          {#if model.detailsLang && model.detailsLang !== currentLocale}
            <div class="mb-2 flex items-center gap-1 text-xs text-zinc-500">
              <span>{m.railway_model_field_details()}</span>
              <LanguageFallbackBadge lang={model.detailsLang} />
            </div>
          {/if}
          <RichTextEditor
            value={localDetails}
            {editable}
            placeholder={m.details_placeholder()}
            onSave={saveDetails}
          />
        </div>
      </TabsContent>

      <!-- ── Tab 2: Rolling Stock ─────────────────────────────────────── -->
      <TabsContent value="rolling-stock" class="mt-2">
        {#if model.rolling_stock && model.rolling_stock.length > 0}
          {#if isSingleUnit}
            <!-- Single unit: hero road number + 3-column spec grid -->
            {@const unit = model.rolling_stock[0]}
            <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
              <h3 class="sr-only">{m.rolling_stock_list()}</h3>

              <!-- Road number as primary identity element -->
              <div class="mb-4 flex items-center justify-between border-b border-zinc-800 pb-3">
                <div class="flex items-baseline gap-2">
                  <span class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.road_number()}
                  </span>
                  {#if editable}
                    <div class="font-mono text-base font-semibold text-zinc-100">
                      <InPlaceEdit
                        value={unit.road_number ?? ''}
                        placeholder={m.road_number()}
                        onSave={(v) =>
                          saveRollingStockIdentification(unit.id, 'roadNumber', v, unit)}
                      />
                    </div>
                  {:else}
                    <span class="font-mono text-base font-semibold text-zinc-100">
                      {unit.road_number ?? '—'}
                    </span>
                  {/if}
                </div>
                {#if unit.railway_company}
                  <span class="text-[10px] font-medium tracking-wider text-zinc-400">
                    {unit.railway_company}
                  </span>
                {/if}
              </div>

              <!-- 3-column spec grid -->
              <dl class="grid grid-cols-3 gap-x-4 gap-y-3">
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.series_code()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable}
                      <InPlaceEdit
                        value={unit.series_code}
                        placeholder={m.rolling_stock_field_series_code()}
                        onSave={(v) => saveRollingStockIdentification(unit.id, 'series', v, unit)}
                      />
                    {:else}
                      {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.depot()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable}
                      <InPlaceEdit
                        value={unit.depot ?? ''}
                        placeholder={m.depot()}
                        onSave={(v) => saveRollingStockIdentification(unit.id, 'depot', v, unit)}
                      />
                    {:else}
                      {unit.depot ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.livery()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable}
                      <InPlaceEdit
                        value={unit.livery ?? ''}
                        placeholder={m.livery()}
                        onSave={(v) => saveRollingStockIdentification(unit.id, 'livery', v, unit)}
                      />
                    {:else}
                      {unit.livery ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.control_type()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.control ??
                          unit.control_type ??
                          '—'}
                        options={controlOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'control', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">Loading…</span>
                    {:else}
                      {unit.control_type ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.dcc_interface()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.dccInterface ??
                          unit.dcc_interface ??
                          '—'}
                        options={dccInterfaceOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'dccInterface', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">Loading…</span>
                    {:else}
                      {unit.dcc_interface ?? '—'}
                    {/if}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    Length
                  </dt>
                  <dd class="font-mono text-xs text-zinc-200">
                    {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.coupling_type()}
                  </dt>
                  <dd class="text-xs text-zinc-200">
                    {#if editable && rollingStockSpecLoaded.has(unit.id)}
                      <BadgePicker
                        value={rollingStockFormState.get(unit.id)?.couplingSocket ??
                          unit.coupling_type ??
                          '—'}
                        options={couplingSockeOptions}
                        onSelect={(id) => saveRollingStockSpec(unit.id, 'couplingSocket', id)}
                      />
                    {:else if editable}
                      <span class="text-xs text-zinc-500 italic">Loading…</span>
                    {:else}
                      {unit.coupling_type ?? '—'}
                    {/if}
                  </dd>
                </div>
              </dl>
            </div>
          {:else}
            <!-- Multi-unit: mini-cards with consistent 3-column spec grid -->
            <div class="space-y-2">
              <h3 class="sr-only">{m.rolling_stock_list()}</h3>
              {#each model.rolling_stock as unit (unit.id)}
                <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-3">
                  <!-- Mini-card header: series name + railway company -->
                  <div
                    class="mb-2.5 flex items-center justify-between border-b border-zinc-800/60 pb-2"
                  >
                    <div class="flex min-w-0 items-baseline gap-2">
                      {#if editable}
                        <div class="truncate text-xs font-medium text-zinc-200">
                          <InPlaceEdit
                            value={unit.series_code}
                            placeholder={m.rolling_stock_field_series_code()}
                            onSave={(v) =>
                              saveRollingStockIdentification(unit.id, 'series', v, unit)}
                          />
                        </div>
                        <div class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
                          <InPlaceEdit
                            value={unit.road_number ?? ''}
                            placeholder={m.road_number()}
                            onSave={(v) =>
                              saveRollingStockIdentification(unit.id, 'roadNumber', v, unit)}
                          />
                        </div>
                      {:else}
                        <span class="truncate text-xs font-medium text-zinc-200">
                          {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                        </span>
                        <span class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
                          {unit.road_number ?? '—'}
                        </span>
                      {/if}
                    </div>
                    {#if unit.railway_company}
                      <span class="ml-2 shrink-0 text-[10px] text-zinc-500">
                        {unit.railway_company}
                      </span>
                    {/if}
                  </div>
                  <!-- 3-column spec grid (consistent, always all fields) -->
                  <dl class="grid grid-cols-3 gap-x-3 gap-y-2">
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.depot()}
                      </dt>
                      <dd class="text-xs text-zinc-200">
                        {#if editable}
                          <InPlaceEdit
                            value={unit.depot ?? ''}
                            placeholder={m.depot()}
                            onSave={(v) =>
                              saveRollingStockIdentification(unit.id, 'depot', v, unit)}
                          />
                        {:else}
                          {unit.depot ?? '—'}
                        {/if}
                      </dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.livery()}
                      </dt>
                      <dd class="truncate text-xs text-zinc-200">
                        {#if editable}
                          <InPlaceEdit
                            value={unit.livery ?? ''}
                            placeholder={m.livery()}
                            onSave={(v) =>
                              saveRollingStockIdentification(unit.id, 'livery', v, unit)}
                          />
                        {:else}
                          {unit.livery ?? '—'}
                        {/if}
                      </dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        Length
                      </dt>
                      <dd class="font-mono text-xs text-zinc-200">
                        {unit.length_mm != null ? `${unit.length_mm} mm` : '—'}
                      </dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.control_type()}
                      </dt>
                      <dd class="text-xs text-zinc-200">
                        {#if editable && rollingStockSpecLoaded.has(unit.id)}
                          <BadgePicker
                            value={rollingStockFormState.get(unit.id)?.control ??
                              unit.control_type ??
                              '—'}
                            options={controlOptions}
                            onSelect={(id) => saveRollingStockSpec(unit.id, 'control', id)}
                          />
                        {:else if editable}
                          <span class="text-xs text-zinc-500 italic">—</span>
                        {:else}
                          {unit.control_type ?? '—'}
                        {/if}
                      </dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.dcc_interface()}
                      </dt>
                      <dd class="text-xs text-zinc-200">
                        {#if editable && rollingStockSpecLoaded.has(unit.id)}
                          <BadgePicker
                            value={rollingStockFormState.get(unit.id)?.dccInterface ??
                              unit.dcc_interface ??
                              '—'}
                            options={dccInterfaceOptions}
                            onSelect={(id) => saveRollingStockSpec(unit.id, 'dccInterface', id)}
                          />
                        {:else if editable}
                          <span class="text-xs text-zinc-500 italic">—</span>
                        {:else}
                          {unit.dcc_interface ?? '—'}
                        {/if}
                      </dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.coupling_type()}
                      </dt>
                      <dd class="text-xs text-zinc-200">
                        {#if editable && rollingStockSpecLoaded.has(unit.id)}
                          <BadgePicker
                            value={rollingStockFormState.get(unit.id)?.couplingSocket ??
                              unit.coupling_type ??
                              '—'}
                            options={couplingSockeOptions}
                            onSelect={(id) => saveRollingStockSpec(unit.id, 'couplingSocket', id)}
                          />
                        {:else if editable}
                          <span class="text-xs text-zinc-500 italic">—</span>
                        {:else}
                          {unit.coupling_type ?? '—'}
                        {/if}
                      </dd>
                    </div>
                  </dl>
                </div>
              {/each}
            </div>
          {/if}
        {:else}
          <div class="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
            <p class="text-sm text-zinc-600 italic">{m.no_additional_details()}</p>
          </div>
        {/if}
      </TabsContent>
    </Tabs>
  </div>
</div>
