<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';

  let {
    open = false,
    onClose = undefined,
    onSaved = undefined
  }: {
    open?: boolean;
    onClose?: () => void;
    onSaved?: () => void;
  } = $props();

  // Provide default no-op functions for optional callbacks
  const handleClose = $derived(onClose ?? (() => {}));
  const handleSaved = $derived(onSaved ?? (() => {}));
</script>

<Tooltip.Provider>
  <AddWishlistItemDrawer {open} onClose={handleClose} onSaved={handleSaved} />
</Tooltip.Provider>
