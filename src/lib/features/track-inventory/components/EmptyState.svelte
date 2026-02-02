<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Package } from 'lucide-svelte';

  interface Props {
    onCreateClick?: () => void;
  }

  const { onCreateClick }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center gap-6 py-16">
  <div class="variant-soft-surface flex h-24 w-24 items-center justify-center rounded-full">
    <Package size={48} class="text-surface-400" />
  </div>

  <div class="flex flex-col items-center gap-2 text-center">
    <h3 class="h3 font-bold">{m.track_inventories_empty_title()}</h3>
    <p class="text-surface-300">{m.track_inventories_empty_message()}</p>
  </div>

  {#if onCreateClick}
    <button onclick={onCreateClick} class="variant-filled-primary btn gap-2">
      {m.track_inventories_create_button()}
    </button>
  {/if}
</div>
