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

  let { model, onDelete, class: className }: Props = $props();

  import { Card, CardContent, CardHeader } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import { Button } from '$lib/components/ui/button';
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import { Volume2, Zap, Trash2, Train, Box, Users, Layers } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  // Derived state for display values
  const displayManufacturer = $derived(model.manufacturer || m.components_unknownManufacturer());

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

  // Road number truncation logic
  const shouldTruncateRoadNumber = $derived(
    model.roadNumber !== null && model.roadNumber.length > 25
  );

  const displayRoadNumber = $derived(
    model.roadNumber
      ? shouldTruncateRoadNumber
        ? model.roadNumber.substring(0, 22) + '...'
        : model.roadNumber
      : m.components_noRoadNumber()
  );

  // State for expanded road number
  let roadNumberExpanded = $state(false);

  function toggleRoadNumber() {
    if (shouldTruncateRoadNumber) {
      roadNumberExpanded = !roadNumberExpanded;
    }
  }

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
  {#if onDelete}
    <CardHeader class="flex flex-row items-center justify-between p-4 pb-0">
      <div></div>
      <Button
        variant="ghost"
        size="icon"
        aria-label={m.components_deleteButton()}
        onclick={() => (showDeleteDialog = true)}
      >
        <Trash2 class="h-4 w-4" />
      </Button>
    </CardHeader>
  {/if}

  <CardContent class="p-4 {onDelete ? 'pt-2' : ''}">
    <div class="grid gap-4 sm:grid-cols-1 lg:grid-cols-[auto_1fr]">
      <!-- Thumbnail (16:9 aspect ratio) -->
      <div class="bg-surface-200 relative aspect-video w-full overflow-hidden rounded-lg lg:w-48">
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

      <!-- Content -->
      <div class="flex flex-col gap-2">
        <!-- Manufacturer and Product Code -->
        <div class="text-surface-600 text-sm">
          <span class="font-medium">{displayManufacturer}</span>
          {#if model.productCode}
            <span class="text-surface-400">·</span>
            <span>{model.productCode}</span>
          {/if}
        </div>

        <!-- Series and Category Title -->
        <h3 class="text-base font-semibold">
          {model.series || 'Railway Model'}
        </h3>

        <!-- Road Number Identification Plate -->
        <div class="mt-1">
          {#if shouldTruncateRoadNumber}
            <button
              type="button"
              onclick={toggleRoadNumber}
              class="bg-surface-100 hover:bg-surface-200 rounded px-2 py-1 font-mono text-sm"
            >
              # {roadNumberExpanded ? model.roadNumber : displayRoadNumber}
            </button>
          {:else}
            <span class="bg-surface-100 inline-block rounded px-2 py-1 font-mono text-sm">
              # {displayRoadNumber}
            </span>
          {/if}
        </div>

        <!-- Metadata Badges -->
        <div class="mt-2 flex flex-wrap gap-2">
          {#if model.scale}
            <Badge variant="secondary">{model.scale}</Badge>
          {/if}

          {#if model.powerMethod}
            <Badge variant="secondary">{model.powerMethod}</Badge>
          {/if}

          {#if model.era}
            <Badge variant="secondary">Era {model.era}</Badge>
          {/if}

          {#if model.purchaseDate}
            <Badge variant="secondary">
              {m.components_purchaseDate()}: {model.purchaseDate}
            </Badge>
          {/if}
        </div>
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
