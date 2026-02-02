<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X } from 'lucide-svelte';
  import type { ManufacturerId, TrackType, TrackCode } from '$lib/bindings';
  import { getTrackInventoryContext } from '../services/TrackInventoryService.svelte';

  interface Props {
    open?: boolean;
    manufacturers: Array<{ id: ManufacturerId; name: string }>;
    onClose?: () => void;
    onProductCreated?: (productId: string) => void;
  }

  const { open = $bindable(false), manufacturers, onClose, onProductCreated }: Props = $props();

  const service = getTrackInventoryContext();

  let manufacturerId = $state<string>('');
  let productCode = $state('');
  let description = $state('');
  let trackType = $state<TrackType>('STRAIGHT');
  let trackCode = $state<TrackCode>('CODE_83');
  let withRoadbed = $state(false);
  let length = $state('');
  let radius = $state('');
  let submitting = $state(false);
  let error = $state<string | null>(null);

  const trackTypes: TrackType[] = ['STRAIGHT', 'CURVE', 'TURNOUT', 'FLEX_TRACK'];
  const trackCodes: TrackCode[] = ['CODE_70', 'CODE_75', 'CODE_83', 'CODE_100'];

  function handleClose() {
    if (submitting) return;
    resetForm();
    onClose?.();
  }

  function resetForm() {
    manufacturerId = '';
    productCode = '';
    description = '';
    trackType = 'STRAIGHT';
    trackCode = 'CODE_83';
    withRoadbed = false;
    length = '';
    radius = '';
    error = null;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();

    // Validation
    if (!manufacturerId) {
      error = 'Manufacturer is required';
      return;
    }
    if (!productCode.trim()) {
      error = 'Product code is required';
      return;
    }
    if (!description.trim()) {
      error = 'Description is required';
      return;
    }

    try {
      submitting = true;
      error = null;

      const productId = await service.createProduct({
        manufacturerId: manufacturerId as ManufacturerId,
        productCode: productCode.trim(),
        description: description.trim(),
        trackType,
        trackCode,
        withRoadbed,
        length: length ? { Millimeters: length } : null,
        radius: radius ? { Millimeters: radius } : null
      });

      resetForm();
      onProductCreated?.(productId);
    } catch (err) {
      console.error('Failed to create product:', err);
      error = err instanceof Error ? err.message : 'Failed to create product';
    } finally {
      submitting = false;
    }
  }

  $effect(() => {
    if (open) {
      // Auto-select first manufacturer if available
      if (manufacturers.length > 0 && !manufacturerId) {
        manufacturerId = manufacturers[0].id;
      }
    }
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    role="dialog"
    aria-modal="true"
    aria-labelledby="create-product-title"
  >
    <div class="w-full max-w-2xl rounded-lg bg-surface-100-900 p-6 shadow-xl">
      <div class="mb-4 flex items-center justify-between">
        <h2 id="create-product-title" class="text-xl font-semibold">
          {m.track_product_dialog_title()}
        </h2>
        <button
          type="button"
          onclick={handleClose}
          disabled={submitting}
          class="rounded p-1 hover:bg-surface-200-800"
          aria-label="Close"
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="space-y-4">
        {#if error}
          <div class="rounded bg-error-500/10 p-3 text-error-500">
            {error}
          </div>
        {/if}

        <div class="grid grid-cols-2 gap-4">
          <!-- Manufacturer -->
          <div>
            <label for="manufacturer" class="mb-1 block text-sm font-medium">
              {m.track_product_field_manufacturer()}
            </label>
            <select
              id="manufacturer"
              bind:value={manufacturerId}
              disabled={submitting}
              required
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            >
              <option value="" disabled>Select manufacturer</option>
              {#each manufacturers as manufacturer (manufacturer.id)}
                <option value={manufacturer.id}>{manufacturer.name}</option>
              {/each}
            </select>
          </div>

          <!-- Product Code -->
          <div>
            <label for="product-code" class="mb-1 block text-sm font-medium">
              {m.track_product_field_product_code()}
            </label>
            <input
              id="product-code"
              type="text"
              bind:value={productCode}
              disabled={submitting}
              required
              placeholder="e.g. 6210"
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            />
          </div>
        </div>

        <!-- Description -->
        <div>
          <label for="description" class="mb-1 block text-sm font-medium">
            {m.track_product_field_description()}
          </label>
          <input
            id="description"
            type="text"
            bind:value={description}
            disabled={submitting}
            required
            placeholder="e.g. Straight Track 111.06mm"
            class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <!-- Track Type -->
          <div>
            <label for="track-type" class="mb-1 block text-sm font-medium">
              {m.track_product_field_track_type()}
            </label>
            <select
              id="track-type"
              bind:value={trackType}
              disabled={submitting}
              required
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            >
              {#each trackTypes as type (type)}
                <option value={type}>{type.replace('_', ' ')}</option>
              {/each}
            </select>
          </div>

          <!-- Track Code -->
          <div>
            <label for="track-code" class="mb-1 block text-sm font-medium">
              {m.track_product_field_track_code()}
            </label>
            <select
              id="track-code"
              bind:value={trackCode}
              disabled={submitting}
              required
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            >
              {#each trackCodes as code (code)}
                <option value={code}>{code.replace('CODE_', 'Code ')}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-4">
          <!-- Length -->
          <div>
            <label for="length" class="mb-1 block text-sm font-medium"> Length (mm) </label>
            <input
              id="length"
              type="number"
              step="0.01"
              bind:value={length}
              disabled={submitting}
              placeholder="111.06"
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            />
          </div>

          <!-- Radius -->
          <div>
            <label for="radius" class="mb-1 block text-sm font-medium"> Radius (mm) </label>
            <input
              id="radius"
              type="number"
              step="0.01"
              bind:value={radius}
              disabled={submitting}
              placeholder="360"
              class="w-full rounded border border-surface-300-700 bg-surface-50-950 p-2"
            />
          </div>

          <!-- With Roadbed -->
          <div class="flex items-end">
            <label class="flex items-center space-x-2">
              <input
                type="checkbox"
                bind:checked={withRoadbed}
                disabled={submitting}
                class="h-5 w-5 rounded border-surface-300-700"
              />
              <span class="text-sm">With Roadbed</span>
            </label>
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-4">
          <button
            type="button"
            onclick={handleClose}
            disabled={submitting}
            class="rounded bg-surface-200-800 px-4 py-2 hover:bg-surface-300-700"
          >
            {m.track_product_cancel()}
          </button>
          <button
            type="submit"
            disabled={submitting}
            class="rounded bg-primary-500 px-4 py-2 text-white hover:bg-primary-600 disabled:opacity-50"
          >
            {submitting ? 'Creating...' : m.track_product_submit()}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
