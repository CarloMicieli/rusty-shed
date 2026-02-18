<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Loader2, Settings2, Ruler, Compass } from 'lucide-svelte';
  import type { ManufacturerId, TrackType, TrackCode } from '$lib/bindings';
  import { getTrackInventoryContext } from '../services/TrackInventoryService.svelte';
  import { Input, Button } from '$lib/components';

  interface Props {
    open?: boolean;
    manufacturers: Array<{ id: ManufacturerId; name: string }>;
    onClose?: () => void;
    onProductCreated?: (productId: string) => void;
  }

  let { open = $bindable(false), manufacturers, onClose, onProductCreated }: Props = $props();

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
    open = false;
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
      if (manufacturers.length > 0 && !manufacturerId) {
        manufacturerId = manufacturers[0].id;
      }
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) handleClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-[110] bg-black/90 backdrop-blur-xl transition-all duration-300"
    onclick={handleClose}
    aria-hidden="true"
  ></div>

  <!-- Dialog -->
  <div
    class="fixed top-1/2 left-1/2 z-[111] w-full max-w-2xl -translate-x-1/2 -translate-y-1/2 p-4"
    role="dialog"
    aria-modal="true"
    aria-labelledby="create-product-title"
  >
    <div
      class="flex flex-col overflow-hidden rounded-3xl border border-white/10 bg-[#0c0c0c] shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-white/5 bg-zinc-950/30 p-6">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-xl border border-white/5 bg-zinc-900 text-zinc-400"
          >
            <Settings2 size={20} />
          </div>
          <h2
            id="create-product-title"
            class="text-xl font-bold tracking-tight text-zinc-100 uppercase"
          >
            {m.track_product_dialog_title()}
          </h2>
        </div>
        <button
          onclick={handleClose}
          class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
          disabled={submitting}
        >
          <X size={20} />
        </button>
      </div>

      <div class="p-8">
        <form onsubmit={handleSubmit} class="space-y-8">
          <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <!-- Manufacturer -->
            <div class="space-y-2">
              <label
                for="manufacturer-select"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.track_product_field_manufacturer()}
              </label>
              <select
                id="manufacturer-select"
                bind:value={manufacturerId}
                disabled={submitting}
                required
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              >
                <option value="" disabled>Select brand...</option>
                {#each manufacturers as manufacturer (manufacturer.id)}
                  <option value={manufacturer.id}>{manufacturer.name}</option>
                {/each}
              </select>
            </div>

            <!-- Product Code -->
            <div class="space-y-2">
              <label
                for="product-code-input"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.track_product_field_product_code()}
              </label>
              <Input
                id="product-code-input"
                bind:value={productCode}
                disabled={submitting}
                required
                placeholder="e.g. 6210"
                class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-0"
              />
            </div>
          </div>

          <!-- Description -->
          <div class="space-y-2">
            <label
              for="track-description-input"
              class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
            >
              {m.track_product_field_description()}
            </label>
            <Input
              id="track-description-input"
              bind:value={description}
              disabled={submitting}
              required
              placeholder="e.g. Straight Track 111.06mm"
              class="h-12 rounded-xl border-white/10 bg-zinc-950 text-zinc-100 focus:border-white/20 focus:ring-0"
            />
          </div>

          <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <!-- Track Type -->
            <div class="space-y-2">
              <label
                for="track-type-select"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.track_product_field_track_type()}
              </label>
              <select
                id="track-type-select"
                bind:value={trackType}
                disabled={submitting}
                required
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              >
                {#each trackTypes as type (type)}
                  <option value={type}>{type.replace('_', ' ')}</option>
                {/each}
              </select>
            </div>

            <!-- Track Code -->
            <div class="space-y-2">
              <label
                for="track-code-select"
                class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                {m.track_product_field_track_code()}
              </label>
              <select
                id="track-code-select"
                bind:value={trackCode}
                disabled={submitting}
                required
                class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
              >
                {#each trackCodes as code (code)}
                  <option value={code}>{code.replace('CODE_', 'Code ')}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-6 sm:grid-cols-3">
            <!-- Length -->
            <div class="space-y-2">
              <label
                class="ml-1 flex items-center gap-2 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                <Ruler size={10} />
                Length (mm)
              </label>
              <Input
                type="number"
                step="0.01"
                bind:value={length}
                disabled={submitting}
                placeholder="0.00"
                class="h-12 rounded-xl border-white/10 bg-zinc-950 font-mono text-zinc-100 placeholder:text-zinc-700"
              />
            </div>

            <!-- Radius -->
            <div class="space-y-2">
              <label
                class="ml-1 flex items-center gap-2 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
              >
                <Compass size={10} />
                Radius (mm)
              </label>
              <Input
                type="number"
                step="0.01"
                bind:value={radius}
                disabled={submitting}
                placeholder="0.00"
                class="h-12 rounded-xl border-white/10 bg-zinc-950 font-mono text-zinc-100 placeholder:text-zinc-700"
              />
            </div>

            <!-- With Roadbed -->
            <div class="flex items-center justify-center">
              <label class="group flex cursor-pointer items-center gap-3">
                <div class="relative flex items-center justify-center">
                  <input
                    type="checkbox"
                    bind:checked={withRoadbed}
                    disabled={submitting}
                    class="peer absolute h-6 w-6 cursor-pointer rounded-lg opacity-0"
                  />
                  <div
                    class="h-6 w-6 rounded-lg border-2 border-white/10 bg-zinc-900 transition-all peer-checked:border-amber-500 peer-checked:bg-amber-500"
                  ></div>
                  <X
                    size={14}
                    class="absolute scale-150 rotate-45 text-black opacity-0 transition-all peer-checked:opacity-100"
                  />
                </div>
                <span
                  class="text-[10px] font-bold tracking-widest text-zinc-500 uppercase transition-colors group-hover:text-zinc-300"
                  >With Roadbed</span
                >
              </label>
            </div>
          </div>

          {#if error}
            <div
              class="rounded-xl border border-red-500/20 bg-red-500/10 p-4 text-xs font-bold tracking-widest text-red-500 uppercase"
            >
              Catalog Error: {error}
            </div>
          {/if}

          <div class="flex justify-end gap-3 pt-4">
            <Button
              type="button"
              onclick={handleClose}
              class="px-8 text-zinc-500 hover:bg-transparent hover:text-white"
              variant="ghost"
              disabled={submitting}
            >
              {m.track_product_cancel()}
            </Button>
            <Button
              type="submit"
              variant="rusty"
              class="h-12 min-w-[180px] px-8"
              disabled={submitting}
            >
              {#if submitting}
                <Loader2 size={18} class="mr-2 animate-spin" />
                <span>Publishing...</span>
              {:else}
                {m.track_product_submit()}
              {/if}
            </Button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
