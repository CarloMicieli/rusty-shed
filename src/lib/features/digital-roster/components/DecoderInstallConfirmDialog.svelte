<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    open: boolean;
    isSubmitting: boolean;
    onConfirm: () => void;
    onCancel: () => void;
  }

  const { open, isSubmitting, onConfirm, onCancel }: Props = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50">
    <div class="mx-4 max-w-md space-y-4 rounded-lg border border-border bg-card p-6">
      <h3 class="text-lg font-semibold">{m.digital_roster_confirm_replace_title()}</h3>
      <p class="text-sm text-muted-foreground">
        {m.digital_roster_confirm_replace()}
      </p>
      <div class="flex justify-end gap-3">
        <Button type="button" variant="ghost" onclick={onCancel} disabled={isSubmitting}>
          {m.add_model_cancel()}
        </Button>
        <Button type="button" variant="default" onclick={onConfirm} disabled={isSubmitting}>
          {m.digital_roster_replace()}
        </Button>
      </div>
    </div>
  </div>
{/if}
