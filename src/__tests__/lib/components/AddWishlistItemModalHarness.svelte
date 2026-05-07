<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';

  let { open = false, onClose = undefined, onSaved = undefined }: {
    open?: boolean;
    onClose?: () => void;
    onSaved?: () => void;
  } = $props();
</script>

<Tooltip.Provider>
  <AddWishlistItemDrawer {open} {onClose} {onSaved} />
</Tooltip.Provider>
