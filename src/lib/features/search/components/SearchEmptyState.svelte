<script lang="ts">
  import { SearchX } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    /** The query that returned no results. */
    query: string;
  }

  const { query }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center py-20 text-center">
  <SearchX class="mb-4 text-zinc-600" size={48} strokeWidth={1.5} />
  <h2 class="mb-2 text-lg font-semibold text-zinc-200">
    {m.search_no_results_title()}
  </h2>
  <p class="max-w-sm text-sm text-zinc-500">
    {m.search_no_results_body({ query })}
  </p>
</div>
