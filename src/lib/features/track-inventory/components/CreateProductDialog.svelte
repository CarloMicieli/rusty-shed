<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Loader2, Settings2 } from 'lucide-svelte';
  import type { ManufacturerId, TrackType, TrackCode } from '$lib/bindings';
  import { getTrackInventoryContext } from '../services/TrackInventoryService.svelte';
  import ProductFormFields from './ProductFormFields.svelte';
  import { Button } from '$lib/components';

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
          <ProductFormFields
            bind:manufacturerId
            bind:productCode
            bind:description
            bind:trackType
            bind:trackCode
            bind:withRoadbed
            bind:length
            bind:radius
            {submitting}
            {error}
            {manufacturers}
          />

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
