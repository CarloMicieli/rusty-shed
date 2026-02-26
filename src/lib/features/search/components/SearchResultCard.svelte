<script lang="ts">
  import { BookOpen, Heart } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { GlobalSearchResultView } from '$lib/bindings';

  interface Props {
    /** The search result to display. */
    result: GlobalSearchResultView;
    /** Called when the user clicks the card. */
    onclick?: () => void;
  }

  const { result, onclick }: Props = $props();

  const isCollection = $derived(result.source === 'collection');
  const sourceLabel = $derived(
    isCollection ? m.search_source_collection() : m.search_source_wishlist()
  );
</script>

<button
  type="button"
  class="group flex w-full cursor-pointer items-center gap-4 rounded-xl border border-zinc-800 bg-[#0F0F0F] px-4 py-3 text-left transition-colors hover:border-zinc-700 hover:bg-zinc-900/60"
  {onclick}
  aria-label="{result.displayName} — {sourceLabel}"
>
  <!-- Source icon -->
  <div
    class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg border border-zinc-700 bg-zinc-900"
  >
    {#if isCollection}
      <BookOpen class="text-amber-500" size={18} />
    {:else}
      <Heart class="text-rose-500" size={18} />
    {/if}
  </div>

  <!-- Text content -->
  <div class="min-w-0 flex-1">
    <p class="truncate text-sm font-semibold text-zinc-100 group-hover:text-white">
      {result.displayName || result.railwayModelId}
    </p>
    <p class="truncate text-xs text-zinc-500">
      {result.manufacturerName}
    </p>
  </div>

  <!-- Source badge -->
  <span
    class="shrink-0 rounded-full px-2.5 py-0.5 text-xs font-medium {isCollection
      ? 'bg-amber-500/10 text-amber-400'
      : 'bg-rose-500/10 text-rose-400'}"
  >
    {sourceLabel}
  </span>
</button>
