<script lang="ts">
  const {
    open,
    title = 'Delete item',
    message = 'Are you sure?',
    onClose,
    onConfirm
  } = $props<{
    open: boolean;
    title?: string;
    message?: string;
    onClose?: () => void;
    onConfirm?: () => void;
  }>();

  function handleClose() {
    onClose?.();
  }

  function handleConfirm() {
    onConfirm?.();
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
    role="presentation"
    tabindex="-1"
    onclick={handleClose}
    onkeydown={(event) => event.key === 'Escape' && handleClose()}
  >
    <div
      class="w-full max-w-md rounded-xl border border-surface-700/70 bg-surface-900 p-6"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => {
        if (event.key === 'Escape') {
          event.stopPropagation();
          handleClose();
        }
      }}
    >
      <h3 class="text-lg font-semibold">{title}</h3>
      <p class="mt-2 text-sm text-surface-400">{message}</p>
      <div class="mt-5 flex justify-end gap-3">
        <button class="variant-ghost-surface btn" onclick={handleClose}>Cancel</button>
        <button class="variant-filled-error btn" onclick={handleConfirm}>Confirm</button>
      </div>
    </div>
  </div>
{/if}
