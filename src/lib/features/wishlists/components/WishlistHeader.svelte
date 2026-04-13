<script lang="ts">
  import { Plus } from 'lucide-svelte';
  import type { WishlistPreview } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  const { wishlist, onAddModel } = $props<{
    wishlist: WishlistPreview | null;
    onAddModel?: () => void;
  }>();

  function formatDate(value: string): string {
    const d = new Date(value);
    return isNaN(d.getTime()) ? '-' : regionalManager.formatDate(d);
  }
</script>

{#if wishlist}
  <div class="flex items-center justify-between border-b border-border/20 pb-6">
    <p class="font-mono text-xs tracking-wider text-muted-foreground uppercase">
      {m.wishlist_header_last_updated()}
      {formatDate(wishlist.updatedAt)}
    </p>

    <Button onclick={onAddModel} variant="rusty" class="shadow-lg shadow-amber-500/10">
      <Plus size={18} class="mr-2" />
      {m.wishlist_add_model_button()}
    </Button>
  </div>
{:else}
  <!-- Empty selection handled by Dashboard -->
{/if}
