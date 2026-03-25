<script lang="ts">
  import type { RailwayModel } from '$lib/types/railway-model';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { homeDir } from '@tauri-apps/api/path';
  import * as m from '$lib/paraglide/messages';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { commands, type Scale, type Category } from '$lib/bindings';
  import ImageCropDialog from '$lib/components/model-details/ImageCropDialog.svelte';
  import RailwayModelCardHeader from '$lib/components/model-details/RailwayModelCardHeader.svelte';
  import RailwayModelImagePanel from '$lib/components/model-details/RailwayModelImagePanel.svelte';
  import RailwayModelTabsContainer from '$lib/components/model-details/RailwayModelTabsContainer.svelte';

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

  // ─────────────────────────────────────────────────────────────────────────
  // State
  // ─────────────────────────────────────────────────────────────────────────

  // Crop dialog state — drop path
  let pendingDropFile = $state<File | null>(null);
  let dropBlobUrl = $state<string | null>(null);

  // Crop dialog state — browse path
  let pendingBrowsePath = $state<string | null>(null);

  // Incremented after each successful upload to bust the browser image cache
  let imageVersion = $state(0);

  // Local copies for editable fields
  let localScale = $state('');
  let localEra = $state('');
  let localCategory = $state('');
  let localDeliveryDate = $state('');

  $effect(() => {
    localScale = model.scale ?? '';
    localEra = model.era ?? '';
    localCategory = model.category ?? '';
    localDeliveryDate = model.delivery_date ?? '';
  });

  // ─────────────────────────────────────────────────────────────────────────
  // Derived
  // ─────────────────────────────────────────────────────────────────────────

  // Power method display label (amber badge)
  const powerMethodLabel = $derived.by((): string => {
    if (!model.power_method) return '';
    const labels: Record<string, string> = {
      ac: 'AC',
      dc: 'DC',
      dcc: 'DCC',
      trix_express: 'TX'
    };
    return labels[model.power_method.toLowerCase()] ?? model.power_method.toUpperCase();
  });

  // Category options
  const categoryOptions: { id: string; label: string }[] = [
    { id: 'LOCOMOTIVES', label: 'Locomotives' },
    { id: 'TRAIN_SETS', label: 'Train Sets' },
    { id: 'STARTER_SETS', label: 'Starter Sets' },
    { id: 'FREIGHT_CARS', label: 'Freight Cars' },
    { id: 'PASSENGER_CARS', label: 'Passenger Cars' },
    { id: 'ELECTRIC_MULTIPLE_UNITS', label: 'Electric Multiple Units' },
    { id: 'RAILCARS', label: 'Railcars' }
  ];

  // Scale options
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

  // Era options
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

  // ─────────────────────────────────────────────────────────────────────────
  // Image handling
  // ─────────────────────────────────────────────────────────────────────────

  async function handleBrowseImage() {
    try {
      const selected = await open({
        multiple: false,
        defaultPath: await homeDir().catch(() => undefined),
        filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp'] }]
      });
      if (selected && typeof selected === 'string') {
        pendingBrowsePath = selected;
      }
    } catch (err) {
      console.error('File selection error:', err);
      onError?.(m.upload_error_unknown());
    }
  }

  function handleImageDropped(file: File, blobUrl: string) {
    pendingDropFile = file;
    dropBlobUrl = blobUrl;
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Scale & Era
  // ─────────────────────────────────────────────────────────────────────────

  async function saveScale(id: string) {
    const result = await commands.updateRailwayModelClassification({
      railwayModelId: model.id,
      scale: id as Scale,
      epoch: null,
      category: null
    });
    if (result.status === 'error') {
      throw new Error('Failed to save scale');
    }
    localScale = id;
    await onModelUpdated?.();
  }

  async function saveEra(id: string) {
    const result = await commands.updateRailwayModelClassification({
      railwayModelId: model.id,
      scale: null,
      epoch: id,
      category: null
    });
    if (result.status === 'error') {
      throw new Error('Failed to save era');
    }
    localEra = id;
    await onModelUpdated?.();
  }

  async function saveCategory(id: string) {
    const result = await commands.updateRailwayModelClassification({
      railwayModelId: model.id,
      scale: null,
      epoch: null,
      category: id as Category
    });
    if (result.status === 'error') {
      throw new Error('Failed to save category');
    }
    localCategory = id;
    await onModelUpdated?.();
  }

  async function saveDeliveryDate(value: string) {
    const result = await commands.updateRailwayModelDeliveryDate({
      railwayModelId: model.id,
      deliveryDate: value || null
    });
    if (result.status === 'error') {
      throw new Error('Failed to save delivery date');
    }
    localDeliveryDate = value;
    await onModelUpdated?.();
  }

  // ─────────────────────────────────────────────────────────────────────────
  // Description save
  // ─────────────────────────────────────────────────────────────────────────

  async function saveDescription(value: string) {
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
    await onModelUpdated?.();
  }
</script>

<div class="railway-model-card rounded-2xl border border-border bg-card {className}">
  <!-- ═══ Header ════════════════════════════════════════════════════════════ -->
  <div class="p-4 pb-3">
    <RailwayModelCardHeader
      {model}
      {editable}
      {powerMethodLabel}
      onDescriptionSave={editable ? saveDescription : undefined}
    />
  </div>

  <!-- ═══ Grid-pattern image area ═══════════════════════════════════════════ -->
  <div class="mx-4">
    <RailwayModelImagePanel
      {model}
      {editable}
      {imageVersion}
      onBrowseImage={editable ? handleBrowseImage : undefined}
      onImageDropped={editable ? handleImageDropped : undefined}
      {onError}
    />
  </div>

  <!-- ═══ Specs bar ════════════════════════════════════════════════════════ -->
  <div
    data-testid="specs"
    class="mx-4 mt-3 grid grid-cols-5 divide-x divide-border rounded-lg border border-border bg-card/40 py-2"
  >
    <div class="flex flex-col items-center gap-1 px-2">
      <span
        class="h-3 text-[9px] leading-none font-medium tracking-wider text-muted-foreground uppercase"
        >{m.railway_model_field_category() || ' '}</span
      >
      <div class="flex h-4 items-center justify-center">
        {#if editable}
          <BadgePicker
            value={localCategory || '—'}
            options={categoryOptions}
            onSelect={saveCategory}
          />
        {:else}
          <span class="text-xs leading-none text-foreground">{model.category ?? '—'}</span>
        {/if}
      </div>
    </div>

    <div class="flex flex-col items-center gap-1 px-2">
      <span
        class="h-3 text-[9px] leading-none font-medium tracking-wider text-muted-foreground uppercase"
        >{m.railway_model_field_scale() || ' '}</span
      >
      <div class="flex h-4 items-center justify-center">
        {#if editable}
          <BadgePicker value={localScale || '—'} options={scaleOptions} onSelect={saveScale} />
        {:else}
          <span class="font-mono text-xs leading-none text-foreground">{model.scale ?? '—'}</span>
        {/if}
      </div>
    </div>

    <div class="flex flex-col items-center gap-1 px-2">
      <span
        class="h-3 text-[9px] leading-none font-medium tracking-wider text-muted-foreground uppercase"
        >{m.railway_model_field_era() || ' '}</span
      >
      <div class="flex h-4 items-center justify-center">
        {#if editable}
          <BadgePicker value={localEra || '—'} options={eraOptions} onSelect={saveEra} />
        {:else}
          <span class="font-mono text-xs leading-none text-foreground">{model.era ?? '—'}</span>
        {/if}
      </div>
    </div>

    <div class="flex flex-col items-center gap-1 px-2">
      <span
        class="h-3 text-[9px] leading-none font-medium tracking-wider text-muted-foreground uppercase"
        >{m.railway_model_field_delivery_date() || ' '}</span
      >
      <div class="flex h-4 items-center justify-center">
        {#if editable}
          <InPlaceEdit
            value={localDeliveryDate}
            placeholder="YYYY or YYYY/MM"
            onSave={saveDeliveryDate}
          />
        {:else}
          <span class="font-mono text-xs leading-none text-foreground"
            >{model.delivery_date ?? '—'}</span
          >
        {/if}
      </div>
    </div>

    <div class="flex flex-col items-center gap-1 px-2">
      <span
        class="h-3 text-[9px] leading-none font-medium tracking-wider text-muted-foreground uppercase"
        >{m.railway_model_field_status() || ' '}</span
      >
      <div class="flex h-4 items-center justify-center">
        <span class="text-xs leading-none text-foreground"
          >{model.status === 'InCollection' ? 'In Collection' : model.status}</span
        >
      </div>
    </div>
  </div>

  <!-- ═══ Tabs ══════════════════════════════════════════════════════════════ -->
  <div data-testid="tabs-box" class="mx-4 mt-3 mb-4">
    <RailwayModelTabsContainer {model} {editable} {onModelUpdated} {onError} />
  </div>

  <!-- ═══ Crop dialogs ════════════════════════════════════════════════════ -->
  <!-- Crop dialog — drop path -->
  <ImageCropDialog
    open={pendingDropFile !== null}
    imageSrc={dropBlobUrl ?? ''}
    fileName={pendingDropFile?.name ?? 'image.jpg'}
    modelId={model.id}
    onSaveSuccess={() => {
      pendingDropFile = null;
      dropBlobUrl = null;
      imageVersion++;
      onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
      void onModelUpdated?.();
    }}
    onCancel={() => {
      pendingDropFile = null;
      dropBlobUrl = null;
    }}
  />

  <!-- Crop dialog — browse path -->
  <ImageCropDialog
    open={pendingBrowsePath !== null}
    imageSrc={pendingBrowsePath ? convertFileSrc(pendingBrowsePath) : ''}
    fileName={pendingBrowsePath ? (pendingBrowsePath.split('/').pop() ?? 'image.jpg') : 'image.jpg'}
    modelId={model.id}
    onSaveSuccess={() => {
      pendingBrowsePath = null;
      imageVersion++;
      onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
      void onModelUpdated?.();
    }}
    onCancel={() => {
      pendingBrowsePath = null;
    }}
  />
</div>
