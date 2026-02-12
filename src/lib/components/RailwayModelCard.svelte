<script lang="ts">
  /**
   * RailwayModelCard Component
   *
   * A reusable component for displaying railway model information including:
   * - Product details (manufacturer, code, scale, era, power method, category)
   * - Image with optional upload/replace functionality
   * - Rolling stock specifications (single-unit or multi-unit display)
   * - Tabbed navigation for multi-unit models
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
  import { ImageIcon, Upload, Loader2, ChevronDown } from 'lucide-svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { SvelteSet } from 'svelte/reactivity';
  import * as m from '$lib/paraglide/messages';

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
  let activeTab = $state<'details' | 'rolling-stock'>('details'); // T073
  let expandedRows = new SvelteSet<string>();
  let isDragging = $state(false);
  let isUploading = $state(false);
  let _uploadProgress = $state(0);

  // Derived/computed values (T058, T074)
  let isSingleUnit = $derived(model.rolling_stock?.length === 1);

  /**
   * Toggles the expansion state of a rolling stock row.
   * Used in multi-unit display mode to expand/collapse individual units.
   *
   * @param rowId - The unique ID of the rolling stock unit to toggle
   */
  function toggleRow(rowId: string) {
    if (expandedRows.has(rowId)) {
      expandedRows.delete(rowId);
    } else {
      expandedRows.add(rowId);
    }
  }

  // StatusBadge derived state
  let statusBadge = $derived({
    variant: model.status === 'InCollection' ? ('default' as const) : ('secondary' as const),
    badgeClass:
      model.status === 'InCollection'
        ? 'bg-green-600 hover:bg-green-700'
        : 'bg-blue-600 hover:bg-blue-700',
    text: model.status
  });

  /**
   * Opens a file browser dialog to select an image for upload.
   * Filters for common image formats (JPEG, PNG, WebP).
   *
   * @async
   * @throws {Error} If file selection fails or upload command errors
   */
  async function handleBrowseImage() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Images',
            extensions: ['jpg', 'jpeg', 'png', 'webp']
          }
        ]
      });

      if (selected && typeof selected === 'string') {
        await uploadImage(selected);
      }
    } catch (err) {
      console.error('File selection error:', err);
      onError?.(m.upload_error_unknown());
    }
  }

  /**
   * Handles drag over event to provide visual feedback for file drop.
   *
   * @param e - The drag event
   */
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  /**
   * Handles drag leave event to remove visual feedback.
   */
  function handleDragLeave() {
    isDragging = false;
  }

  /**
   * Handles file drop event for drag-and-drop image upload.
   * Validates file type and size on client-side before uploading.
   *
   * @async
   * @param e - The drop event containing the dragged files
   */
  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files[0];

      // Client-side pre-validation for immediate feedback
      if (!file.type.startsWith('image/')) {
        onError?.(m.error_invalid_image_format());
        return;
      }

      if (file.size > 50 * 1024 * 1024) {
        // 50 MB limit (matches backend)
        onError?.('File too large. Maximum size is 50 MB.');
        return;
      }

      // Read file as bytes and upload
      try {
        const arrayBuffer = await file.arrayBuffer();
        const bytes = Array.from(new Uint8Array(arrayBuffer));

        isUploading = true;

        await invoke('upload_model_image_bytes', {
          args: {
            modelId: model.id,
            fileName: file.name,
            fileData: bytes
          }
        });

        // Success - notify parent
        onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
      } catch (err) {
        console.error('Upload error:', err);
        onError?.(m.upload_error_unknown());
      } finally {
        isUploading = false;
      }
    }
  }

  /**
   * Uploads an image file to the backend via Tauri IPC command.
   * Calls the onImageUploaded callback on success.
   *
   * @async
   * @param filePath - Absolute path to the image file to upload
   */
  async function uploadImage(filePath: string) {
    try {
      isUploading = true;

      await invoke('upload_model_image', {
        args: {
          modelId: model.id,
          filePath: filePath
        }
      });

      // Success - notify parent
      onImageUploaded?.(`models/${model.manufacturer}_${model.product_code}`);
    } catch (err) {
      console.error('Upload error:', err);
      onError?.(m.upload_error_unknown());
    } finally {
      isUploading = false;
    }
  }
</script>

<!-- Card container following MEMORY.md conventions -->
<div class="railway-model-card card {className}">
  <!-- Header Section (T012) -->
  <header
    class="flex flex-col gap-2 border-b border-white/10 pb-4 md:flex-row md:items-center md:justify-between"
  >
    <div class="flex flex-col gap-1">
      <h2 class="text-lg font-semibold md:text-xl">{model.manufacturer} {model.product_code}</h2>

      {#if model.description}
        <p class="mt-1 text-sm text-muted-foreground">{model.description}</p>
      {:else}
        <p class="mt-1 text-sm text-muted-foreground/50 italic">{m.no_additional_details()}</p>
      {/if}
    </div>

    <!-- Status Badge (T013 partial) -->
    <Badge variant={statusBadge.variant} class={statusBadge.badgeClass}>
      {statusBadge.text}
    </Badge>
  </header>

  <!-- Hero Section with Image/Upload (T013, T044-T052) -->
  <div
    class="hero-section relative mx-auto mt-4 flex aspect-video w-full items-center justify-center overflow-hidden rounded-lg border bg-black/20 md:max-w-2xl {isDragging
      ? 'border-primary bg-primary/10'
      : 'border-white/10'}"
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

      <!-- Replace image button (when editable) -->
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
      <!-- Placeholder with upload controls (when no image) -->
      <div class="flex h-full w-full items-center justify-center">
        {#if editable && !isUploading}
          <div class="flex flex-col items-center gap-4 text-muted-foreground">
            <ImageIcon class="h-16 w-16 opacity-50" />
            <p class="text-sm">{isDragging ? m.drag_drop_image_here() : 'No image available'}</p>
            <Button
              variant="secondary"
              size="sm"
              onclick={handleBrowseImage}
              aria-label={m.upload_image()}
            >
              <Upload class="mr-2 h-4 w-4" />
              {m.upload_image()}
            </Button>
          </div>
        {:else if isUploading}
          <div class="flex flex-col items-center gap-2 text-muted-foreground">
            <Loader2 class="h-16 w-16 animate-spin" />
            <p class="text-sm">Uploading...</p>
          </div>
        {:else}
          <div class="flex flex-col items-center gap-2 text-muted-foreground">
            <ImageIcon class="h-16 w-16 opacity-50" />
            <p class="text-sm">No image available</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Content Section: Specs + Tabs (always shown) -->
  <div data-testid="specs" class="mt-6 rounded-lg border border-white/10 bg-black/20 p-4">
    <dl class="grid grid-cols-2 gap-2 text-sm font-medium md:grid-cols-4">
      {#if model.category}
        <div class="flex flex-col gap-1">
          <dt class="text-xs text-muted-foreground">Category</dt>
          <dd class="text-sm">{model.category}</dd>
        </div>
      {/if}

      {#if model.scale}
        <div class="flex flex-col gap-1">
          <dt class="text-xs text-muted-foreground">Scale</dt>
          <dd class="text-sm">{model.scale}</dd>
        </div>
      {/if}

      {#if model.era}
        <div class="flex flex-col gap-1">
          <dt class="text-xs text-muted-foreground">Era</dt>
          <dd class="text-sm">{model.era}</dd>
        </div>
      {/if}

      {#if model.power_method}
        <div class="flex flex-col gap-1">
          <dt class="text-xs text-muted-foreground">Power Method</dt>
          <dd class="text-sm">{model.power_method}</dd>
        </div>
      {/if}
    </dl>
  </div>

  <div data-testid="tabs-box" class="mt-4 rounded-lg border border-white/10 bg-black/10 p-3">
    <Tabs bind:value={activeTab} class="w-full">
      <TabsList class="grid w-full grid-cols-2">
        <TabsTrigger value="details">{m.railway_model_details()}</TabsTrigger>
        <TabsTrigger value="rolling-stock">{m.rolling_stock_list()}</TabsTrigger>
      </TabsList>

      <!-- Tab 1: Railway Model Details -->
      <TabsContent value="details" class="mt-2">
        {#if model.details}
          <div class="rounded-lg bg-black/20 p-4">
            <div class="prose mt-0 text-sm leading-relaxed text-muted-foreground">
              {model.details}
            </div>
          </div>
        {:else}
          <div class="rounded-lg bg-black/10 p-4">
            <p class="text-sm text-muted-foreground italic">{m.no_additional_details()}</p>
          </div>
        {/if}
      </TabsContent>

      <!-- Tab 2: Rolling Stock -->
      <TabsContent value="rolling-stock" class="mt-2">
        {#if model.rolling_stock && model.rolling_stock.length > 0}
          {#if isSingleUnit}
            <!-- Single unit: inline display, no expand/collapse -->
            {@const unit = model.rolling_stock[0]}
            <div class="rounded-lg bg-black/20 p-4">
              <h3 class="sr-only">{m.rolling_stock_list()}</h3>
              <div class="space-y-3">
                <div class="flex flex-col gap-1">
                  <dt class="text-xs text-muted-foreground">{m.series_code()}</dt>
                  <dd class="text-sm font-medium">
                    {unit.series_code}{unit.series_name ? ` - ${unit.series_name}` : ''}
                  </dd>
                </div>
                <dl class="grid grid-cols-1 gap-3 md:grid-cols-2">
                  {#if unit.category}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">Category</dt>
                      <dd class="text-sm">{unit.category}</dd>
                    </div>
                  {/if}
                  {#if unit.subcategory}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">Subcategory</dt>
                      <dd class="text-sm">{unit.subcategory}</dd>
                    </div>
                  {/if}
                  {#if unit.road_number}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.road_number()}</dt>
                      <dd class="text-sm">{unit.road_number}</dd>
                    </div>
                  {/if}
                  {#if unit.depot}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.depot()}</dt>
                      <dd class="text-sm">{unit.depot}</dd>
                    </div>
                  {/if}
                  {#if unit.livery}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.livery()}</dt>
                      <dd class="text-sm">{unit.livery}</dd>
                    </div>
                  {/if}
                  {#if unit.control_type}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.control_type()}</dt>
                      <dd class="text-sm">{unit.control_type}</dd>
                    </div>
                  {/if}
                  {#if unit.dcc_interface}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.dcc_interface()}</dt>
                      <dd class="text-sm">{unit.dcc_interface}</dd>
                    </div>
                  {/if}
                  {#if unit.coupling_type}
                    <div class="flex flex-col gap-1">
                      <dt class="text-xs text-muted-foreground">{m.coupling_type()}</dt>
                      <dd class="text-sm">{unit.coupling_type}</dd>
                    </div>
                  {/if}
                </dl>
              </div>
            </div>
          {:else}
            <!-- Multi-unit: expandable rows -->
            <div class="rounded-lg border border-white/10 bg-black/20 p-4">
              <h3 class="sr-only">{m.rolling_stock_list()}</h3>
              <div class="space-y-2">
                {#each model.rolling_stock as unit (unit.id)}
                  {@const isExpanded = expandedRows.has(unit.id)}
                  <div class="rounded-lg border border-white/10 bg-black/10">
                    <button
                      class="flex w-full items-center justify-between p-3 text-left transition-colors hover:bg-white/5"
                      onclick={() => toggleRow(unit.id)}
                      aria-expanded={isExpanded}
                      aria-label="Toggle rolling stock details for {unit.series_code}"
                    >
                      <div class="flex-1">
                        <span class="text-sm font-medium">
                          {unit.series_code}{unit.series_name ? ` - ${unit.series_name}` : ''}
                        </span>
                      </div>
                      <ChevronDown
                        class="h-4 w-4 transition-transform {isExpanded ? 'rotate-180' : ''}"
                      />
                    </button>
                    {#if isExpanded}
                      <div class="border-t border-white/10 p-3">
                        <dl class="grid grid-cols-1 gap-3 md:grid-cols-2">
                          {#if unit.category}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">Category</dt>
                              <dd class="text-sm">{unit.category}</dd>
                            </div>
                          {/if}
                          {#if unit.subcategory}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">Subcategory</dt>
                              <dd class="text-sm">{unit.subcategory}</dd>
                            </div>
                          {/if}
                          {#if unit.road_number}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.road_number()}</dt>
                              <dd class="text-sm">{unit.road_number}</dd>
                            </div>
                          {/if}
                          {#if unit.depot}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.depot()}</dt>
                              <dd class="text-sm">{unit.depot}</dd>
                            </div>
                          {/if}
                          {#if unit.livery}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.livery()}</dt>
                              <dd class="text-sm">{unit.livery}</dd>
                            </div>
                          {/if}
                          {#if unit.control_type}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.control_type()}</dt>
                              <dd class="text-sm">{unit.control_type}</dd>
                            </div>
                          {/if}
                          {#if unit.dcc_interface}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.dcc_interface()}</dt>
                              <dd class="text-sm">{unit.dcc_interface}</dd>
                            </div>
                          {/if}
                          {#if unit.coupling_type}
                            <div class="flex flex-col gap-1">
                              <dt class="text-xs text-muted-foreground">{m.coupling_type()}</dt>
                              <dd class="text-sm">{unit.coupling_type}</dd>
                            </div>
                          {/if}
                        </dl>
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/if}
      </TabsContent>
    </Tabs>
  </div>
</div>
