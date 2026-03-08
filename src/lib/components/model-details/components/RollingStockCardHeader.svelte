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
  class="flex w-full items-center justify-between p-4 text-left transition-colors hover:bg-muted/50"
  onclick={onToggle}
  aria-expanded={isExpanded}
>
  <h3 class="text-lg font-semibold">
    {seriesRoadNumber}
  </h3>
  <div class="ml-4 flex flex-shrink-0 items-center gap-2">
    {#if railwayCompanyName}
      <span class="rounded-md bg-zinc-800 px-2 py-0.5 text-xs font-medium text-zinc-300"
        >{railwayCompanyName}</span
      >
    {/if}
    {#if isExpanded}
      <ChevronUp class="h-5 w-5 text-muted-foreground" />
    {:else}
      <ChevronDown class="h-5 w-5 text-muted-foreground" />
    {/if}
  </div>
</button>
