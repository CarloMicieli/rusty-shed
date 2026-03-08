<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { Loader2 } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    onCancel: () => void;
    onSubmit: (e: Event) => void;
    submitting?: boolean;
  }

  let { onCancel, onSubmit, submitting = false }: Props = $props();
</script>

<div class="flex justify-end gap-3 pt-4">
  <Button
    type="button"
    onclick={onCancel}
    class="px-8 text-zinc-500 hover:bg-transparent hover:text-white"
    variant="ghost"
    disabled={submitting}
  >
    {m.track_product_cancel()}
  </Button>
  <Button
    type="submit"
    variant="rusty"
    class="h-12 min-w-[180px] px-8"
    disabled={submitting}
    onclick={onSubmit}
  >
    {#if submitting}
      <Loader2 size={18} class="mr-2 animate-spin" />
      <span>Publishing...</span>
    {:else}
      {m.track_product_submit()}
    {/if}
  </Button>
</div>
