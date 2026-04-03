<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Button } from '$lib/components/ui/button';

  interface Props {
    cancelLabel: string;
    submitLabel: string;
    onCancel: () => void;
    onSubmit: () => void;
    submitting?: boolean;
    isLoading?: boolean;
    disabled?: boolean;
    leading?: Snippet;
  }

  let {
    cancelLabel,
    submitLabel,
    onCancel,
    onSubmit,
    submitting = false,
    isLoading = false,
    disabled = false,
    leading
  }: Props = $props();
</script>

<div
  class="flex {leading
    ? 'justify-between'
    : 'justify-end'} items-center gap-3 border-t border-border px-6 py-4"
>
  {#if leading}
    <div>{@render leading()}</div>
  {/if}
  <div class="flex gap-3">
    <Button type="button" variant="ghost" onclick={onCancel} disabled={submitting}>
      {cancelLabel}
    </Button>
    <Button
      type="button"
      onclick={onSubmit}
      disabled={submitting || isLoading || disabled}
      class="variant-steampunk-lever rounded-sm bg-primary font-bebas font-bold text-primary-foreground shadow-[2px_2px_0px_0px_rgba(0,0,0,0.2)] hover:bg-primary/90"
    >
      {#if submitting}
        <span
          class="mr-2 inline-flex items-center gap-1 rounded-sm border border-primary/50 bg-background px-1.5 font-mono text-[10px]"
        >
          <span class="inline-block h-1.5 w-1.5 animate-pulse rounded-full bg-primary"></span>
        </span>
      {/if}
      {submitLabel}
    </Button>
  </div>
</div>
