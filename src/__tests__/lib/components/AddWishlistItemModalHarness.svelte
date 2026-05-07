<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';

  let {
    open = false,
    preselectedWishlistId = undefined,
    onClose = undefined,
    onSaved = undefined
  }: {
    open?: boolean;
    preselectedWishlistId?: string;
    onClose?: () => void;
    onSaved?: () => void;
  } = $props();

  // Provide default no-op functions for optional callbacks
  const handleClose = $derived(onClose ?? (() => {}));
  const handleSaved = $derived(onSaved ?? (() => {}));
</script>

<Tooltip.Provider>
  <AddWishlistItemDrawer
    {open}
    {preselectedWishlistId}
    onClose={handleClose}
    onSaved={handleSaved}
  />
</Tooltip.Provider>
