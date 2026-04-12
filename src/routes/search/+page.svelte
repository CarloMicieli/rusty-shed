<script lang="ts">
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import * as m from '$lib/paraglide/messages.js';
  import { Loader2 } from 'lucide-svelte';
  import { setSearchContext } from '$lib/features/search';
  import SearchResultCard from '$lib/features/search/components/SearchResultCard.svelte';
  import SearchEmptyState from '$lib/features/search/components/SearchEmptyState.svelte';
  import type { GlobalSearchResultView } from '$lib/bindings';

  const service = setSearchContext();

  /** Current query from URL */
  const query = $derived($page.url.searchParams.get('q') ?? '');

  $effect(() => {
    if (query.length >= 2) {
      void service.search(query, getLocale());
    } else {
      service.reset();
    }
  });

  function navigate(result: GlobalSearchResultView) {
    if (result.source === 'collection') {
      goto(resolve(`/collection/${result.itemId.split(':').pop()}`));
    } else {
      const wishlistId = result.parentId ?? '';
      const wishlistItemId = result.itemId.split(':').pop() ?? result.itemId;
      goto(resolve(`/wishlists/${wishlistId}/items/${wishlistItemId}`));
    }
  }

  const isNavigating = $derived(!!$navigating);
</script>

<svelte:head>
  <title>{m.search_page_title()}</title>
</svelte:head>

<div class="w-full max-w-full px-4 py-8">
  <!-- Page header -->
  <header class="mb-6">
    {#if query}
      <h1 class="text-xl font-semibold text-foreground">
        {m.search_results_for({ query })}
      </h1>
      {#if !service.isLoading && !isNavigating && service.results.length > 0}
        <p class="mt-1 text-sm text-muted-foreground">
          {m.search_result_count({ count: service.results.length })}
        </p>
      {/if}
    {:else}
      <h1 class="text-xl font-semibold text-foreground">{m.search_page_title()}</h1>
      <p class="mt-1 text-sm text-muted-foreground">{m.search_min_length_hint()}</p>
    {/if}
  </header>

  <!-- Loading state -->
  {#if service.isLoading || isNavigating}
    <div class="flex items-center justify-center py-16">
      <Loader2 class="animate-spin text-primary" size={32} />
      <span class="ml-3 text-sm text-muted-foreground">{m.search_loading()}</span>
    </div>

    <!-- Error state -->
  {:else if service.error}
    <p class="py-8 text-center text-sm text-destructive">{service.error}</p>

    <!-- Results -->
  {:else if service.results.length > 0}
    <ol class="space-y-2">
      {#each service.results as result (result.itemId)}
        <li>
          <SearchResultCard {result} onclick={() => navigate(result)} />
        </li>
      {/each}
    </ol>

    <!-- Empty state (only shown after a real search) -->
  {:else if query.length >= 2}
    <SearchEmptyState {query} />
  {/if}
</div>
