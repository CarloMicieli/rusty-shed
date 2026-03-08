<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    onCancel: () => void;
    onSubmit: () => void;
    isFormValid: boolean;
    isSubmitting: boolean;
  }

  let { onCancel, onSubmit, isFormValid, isSubmitting }: Props = $props();
</script>

<!-- Actions -->
<div class="flex items-center justify-end gap-4 border-t border-white/5 pt-5">
  <Button variant="ghost" onclick={onCancel} class="text-zinc-500 hover:text-zinc-100">
    {m.maintenance_add_event_cancel()}
  </Button>
  <Button
    type="submit"
    variant="rusty"
    disabled={!isFormValid || isSubmitting}
    class="min-w-[120px]"
    onclick={onSubmit}
  >
    {#if isSubmitting}
      <span
        class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-black border-t-transparent"
      ></span>
      {m.app_loading()}
    {:else}
      {m.maintenance_add_event_submit()}
    {/if}
  </Button>
</div>
