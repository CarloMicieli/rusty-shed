<script lang="ts">
  import DrawerShell from '$lib/components/drawer/DrawerShell.svelte';

  interface Props {
    open: boolean;
    onClose: () => void;
    hasChanges?: boolean;
    size?: 'md' | 'lg' | 'xl';
    error?: string | null;
    discardTitle?: string;
    discardDescription?: string;
    discardConfirm?: string;
    discardCancel?: string;
  }

  let {
    open,
    onClose,
    hasChanges = false,
    size = 'lg',
    error = null,
    discardTitle,
    discardDescription,
    discardConfirm,
    discardCancel
  }: Props = $props();
</script>

<DrawerShell
  {open}
  {onClose}
  {hasChanges}
  {size}
  {error}
  {discardTitle}
  {discardDescription}
  {discardConfirm}
  {discardCancel}
  labelledby="test-drawer-title"
>
  {#snippet header({ requestClose })}
    <div>
      <h2 id="test-drawer-title">Test Drawer</h2>
      <button type="button" onclick={() => requestClose()} aria-label="close drawer">
        Close
      </button>
    </div>
  {/snippet}

  <p>Drawer body content</p>

  {#snippet footer({ requestClose: _rc })}
    <div>Footer</div>
  {/snippet}
</DrawerShell>
