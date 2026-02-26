<script lang="ts">
  import { Search, X, Loader2 } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import { goto } from '$app/navigation';
  import { Button, Input } from '$lib/components';
  import { commands, type RailwayModelView } from '$lib/bindings';
  import { collectionStore } from '$lib/state/collection.svelte';

  let isExpanded = $state(false);
  let query = $state('');
  let results = $state<RailwayModelView[]>([]);
  let isSearching = $state(false);
  let showResults = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  function toggleSearch() {
    isExpanded = !isExpanded;
    if (!isExpanded) {
      query = '';
      results = [];
      showResults = false;
    }
  }

  function handleInput(value: string) {
    query = value;
    if (debounceTimer) clearTimeout(debounceTimer);

    if (value.length < 2) {
      results = [];
      showResults = false;
      return;
    }

    debounceTimer = setTimeout(() => void runSearch(value), 300);
  }

  async function runSearch(q: string) {
    isSearching = true;
    showResults = true;
    try {
      const searchResult = await commands.searchRailwayModels({ query: q });
      if (searchResult.status !== 'ok') {
        results = [];
        return;
      }
      const ids = searchResult.data;
      const lang = getLocale();
      const modelResults = await Promise.all(
        ids.slice(0, 10).map((id) => commands.getRailwayModelById(id, lang))
      );
      results = modelResults
        .filter((r) => r.status === 'ok' && r.data != null)
        .map((r) => (r as { status: 'ok'; data: RailwayModelView }).data);
    } finally {
      isSearching = false;
    }
  }

  function goToModel(railwayModelId: string) {
    const collectionItem = collectionStore.items.find(
      (item) => item.railwayModel.railwayModelId === railwayModelId
    );
    if (collectionItem) {
      window.location.assign(`/collection/${collectionItem.id}`);
    } else {
      window.location.assign('/collection');
    }
    showResults = false;
    query = '';
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && query.trim().length >= 2) {
      showResults = false;
      goto(`/search?q=${encodeURIComponent(query.trim())}`);
      return;
    }
    if (e.key === 'Escape') {
      showResults = false;
      query = '';
    }
  }
</script>

<!-- Desktop: Inline Input -->
<div class="relative hidden w-64 items-center lg:flex xl:w-96">
  {#if isSearching}
    <Loader2
      class="text-surface-400 pointer-events-none absolute left-3 z-10 animate-spin"
      size={18}
    />
  {:else}
    <Search class="text-surface-400 pointer-events-none absolute left-3 z-10" size={18} />
  {/if}
  <Input
    type="text"
    value={query}
    placeholder={m.app_search_placeholder()}
    class="focus:border-primary-500 rounded-full py-2 pl-10 text-sm transition-colors"
    oninput={(e) => handleInput((e.currentTarget as HTMLInputElement).value)}
    onkeydown={handleKeydown}
    onfocus={() => {
      if (results.length > 0) showResults = true;
    }}
  />

  {#if showResults && query.length >= 2}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="absolute top-full right-0 left-0 z-50 mt-1 max-h-80 overflow-y-auto rounded-xl border border-zinc-800 bg-[#0C0C0C] shadow-xl"
      onmousedown={(e) => e.preventDefault()}
    >
      {#if isSearching}
        <div class="px-4 py-3 text-xs text-zinc-500">{m.app_search_instruction()}</div>
      {:else if results.length === 0}
        <div class="px-4 py-3 text-xs text-zinc-500">No results found</div>
      {:else}
        {#each results as model (model.id)}
          <button
            type="button"
            class="flex w-full items-start gap-3 px-4 py-3 text-left hover:bg-zinc-800/60"
            onclick={() => goToModel(model.id)}
          >
            <div class="min-w-0 flex-1">
              <p class="truncate text-xs font-semibold text-zinc-200">{model.description}</p>
              <p class="truncate text-xs text-zinc-500">
                {model.manufacturer.display} · {model.productCode}
              </p>
            </div>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<!-- Mobile: Icon Trigger + Overlay -->
<div class="lg:hidden">
  <Button variant="ghost" size="icon" onclick={toggleSearch}>
    <Search size={20} />
  </Button>

  {#if isExpanded}
    <div
      class="animate-fade-in fixed inset-0 z-50 flex flex-col bg-background/95 p-4 pt-20 backdrop-blur-sm"
    >
      <Button variant="ghost" size="icon" class="absolute top-4 right-4" onclick={toggleSearch}>
        <X size={24} />
      </Button>
      <div class="relative w-full">
        <Search class="text-surface-400 absolute top-1/2 left-4 -translate-y-1/2" size={20} />
        <Input
          type="text"
          value={query}
          placeholder={m.app_search_mobile_placeholder()}
          class="border-primary-500 rounded-xl py-4 pl-12 text-lg shadow-xl"
          autofocus
          oninput={(e) => handleInput((e.currentTarget as HTMLInputElement).value)}
          onkeydown={handleKeydown}
        />
      </div>

      {#if isSearching}
        <div class="text-surface-400 mt-8 text-center text-sm tracking-widest uppercase">
          {m.app_search_instruction()}
        </div>
      {:else if results.length > 0}
        <div class="mt-4 space-y-2">
          {#each results as model (model.id)}
            <button
              type="button"
              class="flex w-full items-start gap-3 rounded-lg border border-zinc-800 bg-zinc-900/40 px-4 py-3 text-left"
              onclick={() => goToModel(model.id)}
            >
              <div class="min-w-0 flex-1">
                <p class="truncate text-sm font-semibold text-zinc-200">{model.description}</p>
                <p class="truncate text-xs text-zinc-500">
                  {model.manufacturer.display} · {model.productCode}
                </p>
              </div>
            </button>
          {/each}
        </div>
      {:else if query.length >= 2}
        <div class="text-surface-400 mt-8 text-center text-sm">No results found</div>
      {:else}
        <div class="text-surface-400 mt-8 text-center text-sm tracking-widest uppercase">
          {m.app_search_instruction()}
        </div>
      {/if}
    </div>
  {/if}
</div>
