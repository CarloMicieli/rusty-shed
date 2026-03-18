<script lang="ts">
  import type { QuickAction } from '$lib/components/QuickActionButtons.svelte';

  // Use destructuring for props, but don't pull properties
  // out of the 'action' object into local variables.
  let { action, isPrimary = false } = $props<{
    action: QuickAction;
    isPrimary?: boolean;
  }>();
</script>

<button
  type="button"
  onclick={() => action.onClick()}
  class="group hover:border-accent-500/50 relative flex cursor-pointer flex-col items-center gap-2 rounded-lg border border-border/60 bg-card/50 p-4 text-center transition-all hover:bg-muted/70 {isPrimary
    ? 'border-accent-500/40 bg-accent-950/30'
    : ''}"
>
  <div
    class="flex h-10 w-10 items-center justify-center rounded-full transition-colors group-hover:scale-110 {isPrimary
      ? 'bg-accent-500/20 text-accent-400'
      : 'text-surface-300 bg-muted'}"
  >
    <action.icon class="h-5 w-5" />
  </div>

  <span class="text-xs font-semibold {isPrimary ? 'text-accent-400' : 'text-surface-200'}">
    {action.label}
  </span>
</button>
