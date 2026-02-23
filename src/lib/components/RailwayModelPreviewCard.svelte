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
    /** Series designation (e.g., "Class 140", "BR 01") */
    series: string | null;
    /** Model category for classification */
    category: ModelCategory;
    /** Road number / identification marking (e.g., "50 80 26-81 517-7") */
    roadNumber: string | null;
    /** Model scale (e.g., "H0", "N", "TT", "Z") */
    scale: string | null;
    /** Power method (e.g., "DC", "AC", "DCC") */
    powerMethod: string | null;
    /** Historical era classification (e.g., "III", "IV", "V") */
    era: string | null;
    /** Purchase date (ISO 8601 format: "YYYY-MM-DD") */
    purchaseDate: string | null;
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
  import { Button } from '$lib/components/ui/button';
  import { Separator } from '$lib/components/ui/separator';
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import { Volume2, Zap, Trash2, TrainFront, Box, Users, Layers } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { commands } from '$lib/bindings';

  const displayManufacturer = $derived(model.manufacturer ?? m.components_unknownManufacturer());

  const displaySeries = $derived(
    model.series
      ? model.series.length > 30
        ? model.series.substring(0, 30) + '…'
        : model.series
      : 'Railway Model'
  );

  const displayPurchaseDate = $derived.by((): string | null => {
    if (!model.purchaseDate) return null;

    const parsedDate = new Date(model.purchaseDate);
    return Number.isNaN(parsedDate.getTime())
      ? model.purchaseDate
      : parsedDate.toLocaleDateString();
  });

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

  let showDeleteDialog = $state(false);
  let resolvedPhotoUrl = $state<string | null>(null);

  $effect(() => {
    if (model.photoUrl) {
      resolvedPhotoUrl =
        model.photoUrl.startsWith('/') || model.photoUrl.includes('\\')
          ? convertFileSrc(model.photoUrl)
          : model.photoUrl;
    } else {
      void fetchImage(model.id);
    }
  });

  async function fetchImage(id: string) {
    try {
      const result = await commands.getRailwayModelImage(id);
      const imageData = result.status === 'ok' ? result.data : null;

      if (imageData?.hasImage && imageData.imagePath) {
        resolvedPhotoUrl = convertFileSrc(imageData.imagePath);
      } else {
        resolvedPhotoUrl = null;
      }
    } catch (e) {
      console.warn(`Failed to fetch image for model ${id}:`, e);
      resolvedPhotoUrl = null;
    }
  }

  function handleDeleteConfirm() {
    onDelete?.(model.id);
    showDeleteDialog = false;
  }
</script>

<!--
  Card uses `group` so child elements can react to card-level hover via group-hover:*.
  Amber ring glow appears on hover; delete button fades in.
-->
<Card
  class="group card rounded-2xl border-2 border-[rgba(212,138,62,0.35)] shadow-xl transition-all duration-200 hover:border-[rgba(226,153,79,0.65)] hover:shadow-[0_0_20px_rgba(226,153,79,0.2)] {className ??
    ''}"
>
  <CardHeader class="p-3 pb-2">
    <div class="flex items-start justify-between gap-2">
      <!-- Manufacturer · product code -->
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-1">
          <span class="text-xs font-semibold text-zinc-200">{displayManufacturer}</span>
          {#if model.productCode}
            <span class="text-zinc-600" aria-hidden="true">·</span>
            <span class="font-mono text-xs text-zinc-500">{model.productCode}</span>
          {/if}
        </div>
        <!-- Locomotive class / series name -->
        <h3 class="mt-0.5 truncate text-sm leading-tight font-medium text-zinc-100">
          {displaySeries}
        </h3>
      </div>

      <!-- Delete button: hidden by default, revealed on card hover -->
      {#if onDelete}
        <Button
          variant="ghost"
          size="icon"
          class="-mt-1 -mr-1 h-7 w-7 shrink-0 opacity-0 transition-opacity duration-150 group-hover:opacity-100"
          aria-label={m.components_deleteButton()}
          onclick={() => (showDeleteDialog = true)}
        >
          <Trash2 class="h-3.5 w-3.5 text-zinc-500 hover:text-red-400" />
        </Button>
      {/if}
    </div>
  </CardHeader>

  <CardContent class="p-3 pt-0">
    <!--
      Centerpiece: "technical drawing" area.
      Subtle grid pattern via inline background-image; bg-zinc-900 as base.
    -->
    <div
      class="relative aspect-video w-full overflow-hidden rounded-xl bg-zinc-900"
      style="background-image: linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px); background-size: 20px 20px;"
    >
      {#if resolvedPhotoUrl}
        <img
          src={resolvedPhotoUrl}
          alt={model.series ?? 'Railway model'}
          class="h-full w-full object-cover"
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
        <div class="absolute top-2 left-2 z-10 flex gap-1" aria-label="Digital features">
          {#each model.digitalFeatures as feature (feature)}
            {#if feature === 'Sound'}
              <div class="rounded-full bg-black/70 p-1" title="Sound equipped">
                <Volume2 class="h-3.5 w-3.5 text-zinc-300" aria-hidden="true" />
              </div>
            {:else if feature === 'DCC'}
              <div class="rounded-full bg-black/70 p-1" title="DCC equipped">
                <Zap class="h-3.5 w-3.5 text-zinc-300" aria-hidden="true" />
              </div>
            {/if}
          {/each}
        </div>
      {/if}

      <!-- Power method badge (top-right): amber on black, prominent -->
      {#if model.powerMethod}
        <div class="absolute top-2 right-2 z-10">
          <Badge
            class="border-transparent bg-[#E2994F] px-1.5 py-0.5 text-[10px] font-bold text-black"
          >
            {powerMethodLabel}
          </Badge>
        </div>
      {/if}

      <!-- Unit count badge (bottom-right): shown only when > 1 -->
      {#if model.unitCount && model.unitCount > 1}
        <div class="absolute right-2 bottom-2 z-10">
          <Badge class="border-transparent bg-zinc-800/90 px-1.5 py-0.5 text-xs text-zinc-300">
            ×{model.unitCount}
          </Badge>
        </div>
      {/if}
    </div>

    <!-- Separator between image and technical specs -->
    <Separator class="my-2.5 bg-zinc-800" />

    <!-- Technical specs grid: Road Number · Scale · Era -->
    <div class="grid grid-cols-3 divide-x divide-zinc-800">
      <div class="flex flex-col items-center gap-0.5 pr-2">
        <span class="text-[10px] tracking-wider text-zinc-500 uppercase">
          {m.depot_road_number()}
        </span>
        <span class="font-mono text-xs text-zinc-300">{model.roadNumber ?? '—'}</span>
      </div>
      <div class="flex flex-col items-center gap-0.5 px-2">
        <span class="text-[10px] tracking-wider text-zinc-500 uppercase">
          {m.depot_scale()}
        </span>
        <span class="font-mono text-xs text-zinc-300">{model.scale ?? '—'}</span>
      </div>
      <div class="flex flex-col items-center gap-0.5 pl-2">
        <span class="text-[10px] tracking-wider text-zinc-500 uppercase">Era</span>
        <span class="font-mono text-xs text-zinc-300">{model.era ?? '—'}</span>
      </div>
    </div>

    {#if displayPurchaseDate}
      <div class="mt-2.5 text-center">
        <span class="text-[10px] tracking-wider text-zinc-500 uppercase">
          {m.components_purchaseDate()}
        </span>
        <div class="mt-0.5 font-mono text-xs text-zinc-300">{displayPurchaseDate}</div>
      </div>
    {/if}
  </CardContent>
</Card>

<!-- Delete confirmation dialog -->
{#if onDelete}
  <AlertDialog.Root bind:open={showDeleteDialog}>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>{m.components_deleteConfirmTitle()}</AlertDialog.Title>
        <AlertDialog.Description>
          {m.components_deleteConfirmMessage({ model: model.series ?? 'this model' })}
        </AlertDialog.Description>
      </AlertDialog.Header>
      <AlertDialog.Footer>
        <AlertDialog.Cancel>{m.common_cancel()}</AlertDialog.Cancel>
        <AlertDialog.Action onclick={handleDeleteConfirm}>
          {m.common_delete()}
        </AlertDialog.Action>
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{/if}
