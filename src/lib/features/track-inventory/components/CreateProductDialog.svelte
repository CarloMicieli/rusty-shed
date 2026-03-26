<script lang="ts">
  import type { ManufacturerId, TrackType, TrackCode } from '$lib/bindings';
  import { getTrackInventoryContext } from '../services/TrackInventoryService.svelte';
  import ProductFormFields from './ProductFormFields.svelte';
  import ProductDialogHeader from './ProductDialogHeader.svelte';
  import ProductActionFooter from './ProductActionFooter.svelte';

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
      class="flex flex-col overflow-hidden rounded-3xl border border-white/10 bg-layout-surface shadow-2xl"
    >
      <ProductDialogHeader onClose={handleClose} disabled={submitting} />

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

          <ProductActionFooter onCancel={handleClose} onSubmit={handleSubmit} {submitting} />
        </form>
      </div>
    </div>
  </div>
{/if}
