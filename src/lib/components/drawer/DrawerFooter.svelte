<script lang="ts">
  import { Button } from '$lib/components/ui/button';

  interface Props {
    cancelLabel: string;
    submitLabel: string;
    onCancel: () => void;
    onSubmit: () => void;
    submitting?: boolean;
    isLoading?: boolean;
    disabled?: boolean;
  }

  let {
    cancelLabel,
    submitLabel,
    onCancel,
    onSubmit,
    submitting = false,
    isLoading = false,
    disabled = false
  }: Props = $props();
</script>

<div class="flex justify-end gap-3 border-t border-white/10 px-6 py-4">
  <Button type="button" variant="ghost" onclick={onCancel} disabled={submitting}>
    {cancelLabel}
  </Button>
  <Button
    type="button"
    onclick={onSubmit}
    disabled={submitting || isLoading || disabled}
    class="bg-[#D48A42] font-bold text-black hover:bg-[#D48A42]/90"
  >
    {#if submitting}
      <span
        class="mr-2 inline-block h-3 w-3 animate-spin rounded-full border-2 border-black border-t-transparent"
      ></span>
    {/if}
    {submitLabel}
  </Button>
</div>
