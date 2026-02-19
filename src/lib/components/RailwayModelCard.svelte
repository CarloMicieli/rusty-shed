<script lang="ts">
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
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import BadgePicker from '$lib/components/BadgePicker.svelte';
  import { commands, type Scale } from '$lib/bindings';

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
  }

  let {
    model,
    editable = false,
    class: className = '',
    onImageUploaded,
    onError
  }: RailwayModelCardProps = $props();

  // Component-local state
  let activeTab = $state<'details' | 'rolling-stock'>('details');
  let isDragging = $state(false);
  let isUploading = $state(false);
  let _uploadProgress = $state(0);

  // Local copies of editable text fields — updated optimistically on successful save
  let localDescription = $state('');
  let localDetails = $state('');
  let localScale = $state('');
  let localEra = $state('');
  $effect(() => {
    localDescription = model.description ?? '';
    localDetails = model.details ?? '';
    localScale = model.scale ?? '';
    localEra = model.era ?? '';
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

  async function saveDescription(value: string) {
    const result = await commands.updateRailwayModelText({
      railwayModelId: model.id,
      field: 'Description',
      value
    });
    if (result.status === 'error') {
      throw new Error('Failed to save');
    }
    localDescription = value;
  }

  async function saveDetails(value: string) {
    const result = await commands.updateRailwayModelText({
      railwayModelId: model.id,
      field: 'Details',
      value
    });
    if (result.status === 'error') {
      throw new Error('Failed to save');
    }
    localDetails = value;
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
      {#if editable}
        <div class="mt-0.5">
          <InPlaceEdit
            value={localDescription}
            placeholder={m.railway_model_field_description()}
            onSave={saveDescription}
          />
        </div>
      {:else if model.description}
        <p class="mt-0.5 line-clamp-1 text-sm text-zinc-400">{model.description}</p>
      {/if}
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
          {#if editable}
            <InPlaceEdit
              value={localDetails}
              placeholder={m.railway_model_field_details()}
              multiline={true}
              onSave={saveDetails}
            />
          {:else if model.details}
            <p class="line-clamp-6 text-sm leading-relaxed text-zinc-400">{model.details}</p>
          {:else}
            <p class="text-sm text-zinc-600 italic">{m.no_additional_details()}</p>
          {/if}
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
                  <span class="font-mono text-base font-semibold text-zinc-100">
                    {unit.road_number ?? '—'}
                  </span>
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
                    {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                  </dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.depot()}
                  </dt>
                  <dd class="text-xs text-zinc-200">{unit.depot ?? '—'}</dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.livery()}
                  </dt>
                  <dd class="text-xs text-zinc-200">{unit.livery ?? '—'}</dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.control_type()}
                  </dt>
                  <dd class="text-xs text-zinc-200">{unit.control_type ?? '—'}</dd>
                </div>
                <div class="flex flex-col gap-0.5">
                  <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                    {m.dcc_interface()}
                  </dt>
                  <dd class="text-xs text-zinc-200">{unit.dcc_interface ?? '—'}</dd>
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
                  <dd class="text-xs text-zinc-200">{unit.coupling_type ?? '—'}</dd>
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
                      <span class="truncate text-xs font-medium text-zinc-200">
                        {unit.series_code}{unit.series_name ? ` — ${unit.series_name}` : ''}
                      </span>
                      <span class="shrink-0 font-mono text-sm font-semibold text-zinc-100">
                        {unit.road_number ?? '—'}
                      </span>
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
                      <dd class="text-xs text-zinc-200">{unit.depot ?? '—'}</dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.livery()}
                      </dt>
                      <dd class="truncate text-xs text-zinc-200">{unit.livery ?? '—'}</dd>
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
                      <dd class="text-xs text-zinc-200">{unit.control_type ?? '—'}</dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.dcc_interface()}
                      </dt>
                      <dd class="text-xs text-zinc-200">{unit.dcc_interface ?? '—'}</dd>
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <dt class="text-[9px] font-medium tracking-wider text-zinc-500 uppercase">
                        {m.coupling_type()}
                      </dt>
                      <dd class="text-xs text-zinc-200">{unit.coupling_type ?? '—'}</dd>
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
