<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import * as m from '$lib/paraglide/messages';

  interface Props {
    onSave: () => void;
    onCancel: () => void;
    isSaving?: boolean;
    isLoading?: boolean;
    disabled?: boolean;
  }

  let { onSave, onCancel, isSaving = false, isLoading = false, disabled = false }: Props =
    $props();
</script>

<div class="flex items-center justify-end gap-3 border-t border-zinc-800 px-6 py-4">
  <Button variant="ghost" onclick={onCancel} disabled={isSaving}>
    {m.specs_drawer_cancel()}
  </Button>
  <Button
    onclick={onSave}
    disabled={isSaving || isLoading || disabled}
    class="bg-[#E2994F] text-black hover:bg-[#E2994F]/90"
  >
    {#if isSaving}
      <span
        class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-black border-t-transparent"
      ></span>
    {/if}
    {m.specs_drawer_save()}
  </Button>
</div>
