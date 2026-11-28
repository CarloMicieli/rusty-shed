<script lang="ts">
  interface Props {
    open: boolean;
    isSubmitting: boolean;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let { open, isSubmitting, onConfirm, onCancel }: Props = $props();
</script>

{#if open}
  <div data-testid="decoder-install-confirm-dialog">
    <button type="button" data-testid="confirm-replace" disabled={isSubmitting} onclick={onConfirm}>
      confirm
    </button>
    <button type="button" data-testid="cancel-replace" disabled={isSubmitting} onclick={onCancel}>
      cancel
    </button>
  </div>
{/if}
