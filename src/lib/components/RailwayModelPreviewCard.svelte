<script lang="ts" module>
  /**
   * Model categories for placeholder icon selection
   */
  export type ModelCategory =
    | 'SteamLocomotive'
    | 'ElectricLocomotive'
    | 'DieselLocomotive'
    | 'Wagon'
    | 'PassengerCar'
    | 'FreightCar'
    | 'Railcar'
    | 'TrainSet'
    | 'Unknown';

  /**
   * Digital features for overlay badges
   */
  export type DigitalFeature =
    | 'Sound' // Speaker icon overlay
    | 'DCC' // Digital Command Control (bolt icon)
    | 'Smoke' // Smoke generator
    | 'Light'; // Interior/exterior lighting

  /**
   * Railway model data for display in the card
   */
  export interface RailwayModelCardData {
    /** Unique identifier for the model */
    id: string;
    /** Manufacturer name (e.g., "A.C.M.E.", "Märklin") */
    manufacturer: string | null;
    /** Manufacturer's product code (e.g., "1236", "H0-2047") */
    productCode: string | null;
    /** Short description or display title for the model */
    description?: string | null;
    /** Optional series designation (e.g., "Class 140", "BR 01") */
    series?: string | null;
    /** Model category for classification */
    category?: ModelCategory | null;
    /** Model scale (e.g., "H0", "N", "TT", "Z") */
    scale: string | null;
    /** Power method (e.g., "DC", "AC", "DCC") */
    powerMethod: string | null;
    /** Purchase condition (NEW / PRE_OWNED) */
    condition?: import('$lib/bindings').PurchaseCondition | null;
    /** Historical era classification (e.g., "III", "IV", "V") */
    era: string | null;
    /** Purchase date (ISO 8601 format: "YYYY-MM-DD") */
    purchaseDate: string | null;
    /** Purchase price */
    price?: import('$lib/bindings').MonetaryAmount | null;
    /** Whether the collection item is sold */
    isSold?: boolean;
    /** URL to model photo/image */
    photoUrl: string | null;
    /** Number of units in the set (e.g., 3 for a 3-car train set) */
    unitCount: number | null;
    /** Digital features available on the model */
    digitalFeatures: DigitalFeature[];
  }
</script>

<script lang="ts">
  /**
   * Railway Model Preview Card Component
   * Displays a compact visual summary of a railway model with thumbnail,
   * metadata badges, and optional delete functionality.
   * Premium technical aesthetic matching the Rusty Shed dark theme.
   */

  interface Props {
    /** The railway model data to display in the card */
    model: RailwayModelCardData;
    /**
     * Optional callback invoked when user confirms deletion.
     * If not provided, delete button will not be rendered.
     */
    onDelete?: (modelId: string) => void;
    /** Optional CSS class to apply to the card root element */
    class?: string;
  }

  let { model, onDelete, class: className }: Props = $props();

  import { Card, CardHeader, CardContent } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Separator } from '$lib/components/ui/separator';
  import { Volume2, Zap, TrainFront, Box, Users, Layers } from 'lucide-svelte';
  import { format, isValid } from 'date-fns';
  import * as m from '$lib/paraglide/messages.js';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { commands } from '$lib/bindings';
  import PreviewCardActions from '$lib/components/model-details/components/PreviewCardActions.svelte';
  import { getCachedImage, setCachedImage } from '$lib/state/image-cache';

  const displayManufacturer = $derived(model.manufacturer ?? m.components_unknownManufacturer());

  const displayDescription = $derived(model.description ?? model.series ?? '');

  const displayCategory = $derived.by((): string => {
    if (!model.category || model.category === 'Unknown') return '—';

    switch (model.category) {
      case 'SteamLocomotive':
      case 'ElectricLocomotive':
      case 'DieselLocomotive':
        return m.enum_category_locomotives();
      case 'FreightCar':
        return m.enum_category_freight_cars();
      case 'PassengerCar':
        return m.enum_category_passenger_cars();
      case 'Railcar':
        return m.enum_category_railcars();
      case 'TrainSet':
        return m.enum_category_train_sets();
      default:
        return '—';
    }
  });

  const displayPurchaseDate = $derived.by((): string | null => {
    if (!model.purchaseDate) return null;

    const parsedDate = new Date(model.purchaseDate);
    return isValid(parsedDate) ? format(parsedDate, 'MMM d, yyyy') : model.purchaseDate;
  });

  const isSold = $derived(model.isSold ?? false);

  // Compact display label for power method badge
  const powerMethodLabel = $derived.by((): string => {
    if (!model.powerMethod) return '';
    const labels: Record<string, string> = {
      ac: 'AC',
      dc: 'DC',
      dcc: 'DCC',
      trix_express: 'TX'
    };
    return labels[model.powerMethod.toLowerCase()] ?? model.powerMethod.toUpperCase();
  });

  // Category-to-icon mapping for technical drawing placeholder

  const displayCondition = $derived.by((): string => {
    switch (model.condition) {
      case 'NEW':
        return m.dashboard_condition_new();
      case 'PRE_OWNED':
        return m.dashboard_condition_preowned();
      default:
        return '';
    }
  });

  const displayPrice = $derived.by((): string | null => {
    if (!model.price) return null;

    return regionalManager.formatCurrencyWith(model.price.amount, model.price.currency);
  });
  const PlaceholderIcon = $derived.by(() => {
    switch (model.category) {
      case 'SteamLocomotive':
      case 'ElectricLocomotive':
      case 'DieselLocomotive':
        return TrainFront;
      case 'Wagon':
      case 'FreightCar':
        return Box;
      case 'PassengerCar':
        return Users;
      case 'Railcar':
      case 'TrainSet':
        return Layers;
      default:
        return TrainFront;
    }
  });

  const mobileStatusLabel = $derived.by((): string => {
    if (isSold) {
      return m.collection_stats_sold();
    }

    if (displayCondition) {
      return displayCondition;
    }

    return m.collection_stats_active();
  });

  let resolvedPhotoUrl = $state<string | null>(null);

  function mimeFromPath(path: string): string {
    const ext = path.split('.').pop()?.toLowerCase() ?? 'jpg';
    const mimes: Record<string, string> = {
      jpg: 'image/jpeg',
      jpeg: 'image/jpeg',
      png: 'image/png',
      webp: 'image/webp'
    };
    return mimes[ext] ?? 'image/jpeg';
  }

  async function pathToBlobUrl(filePath: string): Promise<string> {
    const bytes = await readFile(filePath);
    return URL.createObjectURL(new Blob([bytes], { type: mimeFromPath(filePath) }));
  }

  $effect(() => {
    const photoUrl = model.photoUrl;
    const modelId = model.id;
    let stale = false;

    async function load() {
      // Cache hit — no IPC or file-system read needed (covers virtual scroll remounts).
      const cached = getCachedImage(modelId);
      if (cached) {
        if (!stale) resolvedPhotoUrl = cached;
        return;
      }

      let newUrl: string | null = null;
      try {
        if (photoUrl) {
          newUrl =
            photoUrl.startsWith('/') || photoUrl.includes('\\')
              ? await pathToBlobUrl(photoUrl)
              : photoUrl;
        } else {
          const result = await commands.getRailwayModelImage(modelId);
          const imageData = result.status === 'ok' ? result.data : null;
          if (imageData?.hasImage && imageData.imagePath) {
            newUrl = await pathToBlobUrl(imageData.imagePath);
          }
        }
      } catch (e) {
        console.warn(`Failed to load image for model ${modelId}:`, e);
      }

      if (stale) {
        // Effect was superseded — discard the URL we just created.
        if (newUrl?.startsWith('blob:')) URL.revokeObjectURL(newUrl);
        return;
      }

      // Store blob URLs in the cache so remounts are instant.
      if (newUrl?.startsWith('blob:')) {
        setCachedImage(modelId, newUrl);
      }

      const prev = resolvedPhotoUrl;
      resolvedPhotoUrl = newUrl;
      // Only revoke the previous URL if it is not the shared cached value.
      if (prev?.startsWith('blob:') && getCachedImage(modelId) !== prev) {
        URL.revokeObjectURL(prev);
      }
    }

    void load();
    return () => {
      stale = true;
      // Do not revoke resolvedPhotoUrl here — it lives in the cache.
    };
  });
</script>

<!--
  Card uses `group` so child elements can react to card-level hover via group-hover:*.
  Amber border becomes solid on hover; delete button fades in.
-->
<Card
  class="group card relative rounded-2xl border-2 border-primary/35 shadow-xl transition-all duration-200 active:scale-[0.98] active:bg-muted/50 md:hover:border-primary md:hover:shadow-[0_0_10px_rgba(212,138,66,0.15)] {isSold
    ? 'opacity-70 grayscale'
    : ''} {className ?? ''}"
>
  <CardHeader class="p-3 pb-2">
    <div class="flex items-start justify-between gap-2">
      <div class="min-w-0 flex-1 space-y-1">
        <div class="flex min-w-0 items-center gap-1 whitespace-nowrap">
          <span class="truncate text-xs font-semibold text-zinc-200">{displayManufacturer}</span>
          {#if model.productCode}
            <span class="shrink-0 text-zinc-600" aria-hidden="true">·</span>
            <span class="shrink-0 font-mono text-xs text-zinc-500">{model.productCode}</span>
          {/if}
          {#if powerMethodLabel}
            <Badge
              class="ml-auto border-transparent bg-primary/15 px-1.5 py-0.5 text-[10px] font-bold text-primary md:hidden"
            >
              {powerMethodLabel}
            </Badge>
          {/if}
        </div>
        {#if displayDescription}
          <h3 class="line-clamp-2 text-sm leading-tight font-medium text-zinc-100">
            {displayDescription}
          </h3>
        {/if}
      </div>

      <!-- Delete button + confirmation dialog: hidden by default, revealed on card hover -->
      {#if onDelete}
        <PreviewCardActions
          modelId={model.id}
          modelSeries={displayDescription || model.series || null}
          {onDelete}
        />
      {/if}
    </div>
  </CardHeader>

  <CardContent class="p-3 pt-0">
    <!--
      Centerpiece: "technical drawing" area.
      Subtle grid pattern via inline background-image; bg-zinc-900 as base.
    -->
    <div
      class="relative aspect-video w-full overflow-hidden rounded-xl bg-card saturate-[0.9] transition-all duration-200 group-hover:saturate-100"
      style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
    >
      {#if resolvedPhotoUrl}
        <img
          src={resolvedPhotoUrl}
          alt=""
          aria-hidden="true"
          class="absolute inset-0 h-full w-full scale-110 object-cover opacity-60 blur-xl"
        />
        <img
          src={resolvedPhotoUrl}
          alt={model.series ?? m.railway_model_image_alt()}
          class="absolute inset-0 z-10 h-full w-full object-contain"
          loading="lazy"
          decoding="async"
        />
      {:else}
        <!-- Category placeholder icon: dimmed for the "blueprint" feel -->
        <div class="flex h-full w-full items-center justify-center">
          <PlaceholderIcon class="size-12 text-zinc-600/70" />
        </div>
      {/if}

      <!-- Digital features overlay (top-left) -->
      {#if model.digitalFeatures.length > 0}
        <div
          class="absolute top-12 left-2 z-20 flex gap-1 md:top-2"
          aria-label={m.railway_model_digital_features_label()}
        >
          {#each model.digitalFeatures as feature (feature)}
            {#if feature === 'Sound'}
              <div
                class="rounded-full bg-background/70 p-1"
                title={m.railway_model_sound_equipped_title()}
              >
                <Volume2 class="h-3.5 w-3.5 text-zinc-300" aria-hidden="true" />
              </div>
            {:else if feature === 'DCC'}
              <div
                class="rounded-full bg-background/70 p-1"
                title={m.railway_model_dcc_equipped_title()}
              >
                <Zap class="h-3.5 w-3.5 text-zinc-300" aria-hidden="true" />
              </div>
            {/if}
          {/each}
        </div>
      {/if}

      <div class="absolute top-2 left-2 z-20 md:hidden">
        <Badge
          class="max-w-[11rem] border-transparent bg-background/85 px-2 py-1 text-[10px] font-semibold"
        >
          <span class="block truncate">{displayCategory}</span>
        </Badge>
      </div>

      <div class="absolute top-2 right-2 z-20 md:hidden">
        <Badge
          class="border-transparent bg-primary px-2 py-1 text-[10px] font-bold text-primary-foreground"
        >
          {mobileStatusLabel}
        </Badge>
      </div>

      <!-- Power method badge (top-right): amber on black, prominent -->
      {#if model.powerMethod}
        <div class="absolute top-2 right-2 z-20 hidden md:block">
          <Badge
            class="border-transparent bg-primary px-1.5 py-0.5 text-[10px] font-bold text-primary-foreground"
          >
            {powerMethodLabel}
          </Badge>
        </div>
      {/if}

      {#if model.unitCount && model.unitCount > 1}
        <div class="absolute bottom-2 left-2 z-20">
          <Badge class="border-transparent bg-zinc-800/90 px-1.5 py-0.5 text-xs text-zinc-300">
            ×{model.unitCount}
          </Badge>
        </div>
      {/if}

      {#if displayCondition}
        <div class="absolute right-2 bottom-2 z-20 hidden md:block">
          <Badge
            variant={model.condition === 'NEW' ? 'default' : 'secondary'}
            class="px-1.5 py-0.5 text-[10px] font-bold"
          >
            {displayCondition}
          </Badge>
        </div>
      {/if}
    </div>

    <Separator class="my-2.5 bg-border" />

    <div class="space-y-3">
      <div class="grid grid-cols-2 gap-3 md:hidden">
        <div class="flex min-w-0 flex-col gap-0.5">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
            {m.depot_scale()}
          </span>
          <span class="truncate font-mono text-xs text-foreground">{model.scale ?? '—'}</span>
        </div>
        <div class="flex min-w-0 flex-col gap-0.5">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase"
            >{m.depot_era()}</span
          >
          <span class="truncate font-mono text-xs text-foreground">{model.era ?? '—'}</span>
        </div>
      </div>

      {#if displayPurchaseDate || displayPrice}
        <div class="grid grid-cols-2 gap-3 md:hidden">
          <div class="flex min-w-0 flex-col gap-0.5">
            <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
              {m.components_purchaseDate()}
            </span>
            <span class="truncate font-mono text-xs text-foreground">
              {displayPurchaseDate ?? '—'}
            </span>
          </div>
          <div class="flex min-w-0 flex-col gap-0.5">
            <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
              {m.dashboard_card_price()}
            </span>
            <span class="truncate font-mono text-xs text-foreground">{displayPrice ?? '—'}</span>
          </div>
        </div>
      {/if}

      <div class="hidden gap-3 md:grid md:grid-cols-[2fr_1fr_1fr]">
        <div class="flex min-w-0 flex-col items-center gap-0.5 text-center">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
            {m.depot_category()}
          </span>
          <Badge variant="secondary" class="w-full justify-center truncate font-mono text-xs">
            {displayCategory}
          </Badge>
        </div>
        <div class="flex min-w-0 flex-col items-center gap-0.5 text-center">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
            {m.depot_scale()}
          </span>
          <span class="w-full truncate font-mono text-xs text-foreground">{model.scale ?? '—'}</span
          >
        </div>
        <div class="flex min-w-0 flex-col items-center gap-0.5 text-center">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase"
            >{m.depot_era()}</span
          >
          <span class="w-full truncate font-mono text-xs text-foreground">{model.era ?? '—'}</span>
        </div>
      </div>

      {#if displayPurchaseDate || displayPrice}
        <div class="hidden gap-3 md:grid md:grid-cols-2">
          <div class="flex min-w-0 flex-col items-center gap-0.5 text-center">
            <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
              {m.components_purchaseDate()}
            </span>
            <span class="w-full truncate font-mono text-xs text-foreground">
              {displayPurchaseDate ?? '—'}
            </span>
          </div>
          <div class="flex min-w-0 flex-col items-center gap-0.5 text-center">
            <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
              {m.dashboard_card_price()}
            </span>
            <span class="w-full truncate font-mono text-xs text-foreground"
              >{displayPrice ?? '—'}</span
            >
          </div>
        </div>
      {/if}
    </div>
  </CardContent>
</Card>
