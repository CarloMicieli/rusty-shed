<script lang="ts">
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';

  interface Props {
    label: string;
    expanded: boolean;
    onToggle: () => void;
  }

  let { label, expanded, onToggle }: Props = $props();
</script>

<button
  type="button"
  onclick={onToggle}
  class="flex w-full items-center justify-between rounded-sm border border-border px-3 py-2 text-left transition-colors hover:bg-muted"
>
  <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase">
    {label}
  </span>
  {#if expanded}
    <ChevronDown size={14} class="text-muted-foreground" />
  {:else}
    <ChevronRight size={14} class="text-muted-foreground" />
  {/if}
</button>
