<script lang="ts">
  import { ChevronDown, ChevronRight } from 'lucide-svelte';

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
  class="flex w-full items-center justify-between border border-white/10 px-3 py-2 text-left transition-colors hover:bg-white/5"
>
  <span class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
    {label}
  </span>
  {#if expanded}
    <ChevronDown size={14} class="text-zinc-500" />
  {:else}
    <ChevronRight size={14} class="text-zinc-500" />
  {/if}
</button>
