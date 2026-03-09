<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    /** Callback to confirm discarding changes */
    onConfirm: () => void;
    /** Callback to cancel and keep editing */
    onCancel: () => void;
  }

  let { onConfirm, onCancel }: Props = $props();
</script>

<div
  class="fixed inset-0 z-[60] flex items-center justify-center bg-background/80 backdrop-blur-sm"
>
  <div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl">
    <h3 class="mb-2 text-lg font-bold text-foreground">{m.add_model_discard_title()}</h3>
    <p class="mb-4 text-muted-foreground">{m.add_model_discard_message()}</p>
    <div class="flex justify-end gap-3">
      <Button type="button" variant="ghost" onclick={onCancel}>
        {m.add_model_discard_cancel()}
      </Button>
      <Button type="button" variant="destructive" onclick={onConfirm}>
        {m.add_model_discard_confirm()}
      </Button>
    </div>
  </div>
</div>
