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
   */

  /**
   * Component props interface
   */
  interface Props {
    /** The railway model data to display in the card */
    model: RailwayModelCardData;
    /**
     * Optional callback invoked when user confirms deletion
     * If not provided, delete button will not be rendered
     */
    onDelete?: (modelId: string) => void;
    /** Optional CSS class to apply to the card root element */
    class?: string;
  }

  // eslint-disable-next-line svelte/no-unused-props
  let { model, onDelete, class: className }: Props = $props();

  import { Card, CardContent } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import { Volume2, Zap, Trash2, Train, Box, Users, Layers } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  // Derived state for display values
  const displayManufacturer = $derived(model.manufacturer || m.components_unknownManufacturer());

  // Series truncated to max 30 characters
  const displaySeries = $derived(
    model.series
      ? model.series.length > 30
        ? model.series.substring(0, 30) + '...'
        : model.series
      : 'Railway Model'
  );

  // Category-to-icon mapping for placeholders
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let PlaceholderIcon: any = $derived.by(() => {
    switch (model.category) {
      case 'SteamLocomotive':
        return Train;
      case 'ElectricLocomotive':
      case 'DieselLocomotive':
        return Zap;
      case 'Wagon':
      case 'FreightCar':
        return Box;
      case 'PassengerCar':
        return Users;
      case 'Railcar':
      case 'TrainSet':
        return Layers;
      default:
        return Train; // Generic fallback
    }
  });

  // Delete dialog state
  let showDeleteDialog = $state(false);

  function handleDeleteConfirm() {
    onDelete?.(model.id);
    showDeleteDialog = false;
  }
</script>

<Card
  class="card gauge-frame hover:ring-primary-500 ring-1 ring-border/40 transition-all duration-200 hover:scale-[1.02] hover:ring-2 {className ||
    ''}"
>
  <CardContent class="p-4">
    <div class="flex flex-col gap-3">
      <!-- Row 1: manufacturer · productCode + trash icon -->
      <div class="flex items-start justify-between gap-2">
        <div class="text-surface-600 min-w-0 text-sm">
          <span class="font-medium">{displayManufacturer}</span>
          {#if model.productCode}
            <span class="text-surface-400 mx-1">·</span>
            <span>{model.productCode}</span>
          {/if}
        </div>
        {#if onDelete}
          <Button
            variant="ghost"
            size="icon"
            class="-mr-2 -mt-2 h-8 w-8 shrink-0"
            aria-label={m.components_deleteButton()}
            onclick={() => (showDeleteDialog = true)}
          >
            <Trash2 class="h-4 w-4" />
          </Button>
        {/if}
      </div>

      <!-- Row 2: Description (series, max 30 chars) -->
      <h3 class="text-base font-semibold leading-snug">
        {displaySeries}
      </h3>

      <!-- Image -->
      <div class="bg-surface-200 relative aspect-video w-full overflow-hidden rounded-lg">
        {#if model.photoUrl}
          <img
            src={model.photoUrl}
            alt={model.series || 'Railway model'}
            class="h-full w-full object-cover"
            loading="lazy"
            decoding="async"
          />
        {:else}
          <!-- Category-specific placeholder icon -->
          <div class="flex h-full w-full items-center justify-center">
            <PlaceholderIcon class="text-surface-500 size-16" />
          </div>
        {/if}

        <!-- Digital Features Overlay (top-left) -->
        {#if model.digitalFeatures.length > 0}
          <div class="absolute top-2 left-2 z-10 flex gap-1" aria-label="Digital features">
            {#each model.digitalFeatures as feature (feature)}
              {#if feature === 'Sound'}
                <div class="rounded-full bg-black/60 p-1" title="Sound equipped">
                  <Volume2 class="h-4 w-4 text-white" aria-hidden="true" />
                </div>
              {:else if feature === 'DCC'}
                <div class="rounded-full bg-black/60 p-1" title="DCC equipped">
                  <Zap class="h-4 w-4 text-white" aria-hidden="true" />
                </div>
              {/if}
            {/each}
          </div>
        {/if}

        <!-- Unit Count Overlay (bottom-right) -->
        {#if model.unitCount && model.unitCount > 1}
          <div
            class="absolute right-2 bottom-2 z-10 rounded-full bg-black/60 px-2 py-1 text-xs font-medium text-white"
            aria-label="{model.unitCount} units"
          >
            ×{model.unitCount}
          </div>
        {/if}
      </div>

      <!-- Row 3: # roadNumber · scale · era -->
      <div class="text-surface-600 flex flex-wrap items-center gap-3 font-mono text-sm">
        {#if model.roadNumber}
          <span># {model.roadNumber}</span>
        {/if}
        {#if model.scale}
          <span>{model.scale}</span>
        {/if}
        {#if model.era}
          <span>Era {model.era}</span>
        {/if}
      </div>
    </div>
  </CardContent>
</Card>

<!-- Delete Confirmation Dialog -->
{#if onDelete}
  <AlertDialog.Root bind:open={showDeleteDialog}>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>{m.components_deleteConfirmTitle()}</AlertDialog.Title>
        <AlertDialog.Description>
          {m.components_deleteConfirmMessage({ model: model.series || 'this model' })}
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
