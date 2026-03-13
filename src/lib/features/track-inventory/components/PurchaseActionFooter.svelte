<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Loader2 } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    onCancel: () => void;
    submitting?: boolean;
  }

  let { onCancel, submitting = false }: Props = $props();
</script>

<div class="flex items-center justify-between border-t border-white/5 bg-[#0c0c0c] px-6 py-4">
  <Button
    type="button"
    variant="ghost"
    onclick={onCancel}
    class="px-6 text-zinc-500 hover:bg-transparent hover:text-white"
    disabled={submitting}
  >
    {m.track_purchase_cancel()}
  </Button>
  <Button type="submit" variant="rusty" class="h-12 min-w-[160px] px-8" disabled={submitting}>
    {#if submitting}
      <Loader2 size={18} class="mr-2 animate-spin" />
      <span>Recording…</span>
    {:else}
      {m.track_purchase_submit()}
    {/if}
  </Button>
</div>
