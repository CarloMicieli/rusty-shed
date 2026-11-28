<script lang="ts">
  interface Props {
    open?: boolean;
    onClose?: () => void;
    onProductCreated?: (productId: string) => void;
  }

  let { open = $bindable(false), onClose, onProductCreated }: Props = $props();
</script>

{#if open}
  <div data-testid="create-product-dialog">
    <button
      type="button"
      data-testid="create-product-submit"
      onclick={() => {
        onProductCreated?.('track-created');
        open = false;
      }}
    >
      submit
    </button>
    <button
      type="button"
      data-testid="create-product-close"
      onclick={() => {
        open = false;
        onClose?.();
      }}
    >
      close
    </button>
  </div>
{/if}
