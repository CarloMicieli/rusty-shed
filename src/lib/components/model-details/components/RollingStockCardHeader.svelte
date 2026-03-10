<script lang="ts">
  import { ChevronDown, ChevronUp } from 'lucide-svelte';

  interface Props {
    seriesRoadNumber: string;
    railwayCompanyName?: string | null;
    isExpanded: boolean;
    onToggle: () => void;
  }

  const { seriesRoadNumber, railwayCompanyName, isExpanded, onToggle }: Props = $props();
</script>

<button
  type="button"
  class="flex w-full items-center justify-between p-4 text-left transition-colors hover:bg-[#1F1F1F]/50"
  onclick={onToggle}
  aria-expanded={isExpanded}
>
  <div class="flex flex-col gap-1">
    <span class="text-[10px] font-medium tracking-wider text-[#808080] uppercase">
      Rolling Stock
    </span>
    <h3 class="font-mono text-lg font-semibold text-white">
      {seriesRoadNumber}
    </h3>
  </div>
  <div class="ml-4 flex flex-shrink-0 items-center gap-3">
    {#if railwayCompanyName}
      <span
        class="rounded-full bg-[#D48A42] px-2.5 py-0.5 text-[10px] font-bold tracking-tight text-[#050505] uppercase"
      >
        {railwayCompanyName}
      </span>
    {/if}
    <div
      class="flex h-8 w-8 items-center justify-center rounded-full border border-[#1F1F1F] text-[#808080]"
    >
      {#if isExpanded}
        <ChevronUp class="h-4 w-4" />
      {:else}
        <ChevronDown class="h-4 w-4" />
      {/if}
    </div>
  </div>
</button>
