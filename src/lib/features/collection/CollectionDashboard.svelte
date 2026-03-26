<script lang="ts">
  import { TrainFront, X, Filter, LayoutGrid, Rows3, SlidersHorizontal } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { getCollectionContext, availableScales } from './CollectionState.svelte';
  import { Button } from '$lib/components';
  import PageHeader from '$lib/components/PageHeader.svelte';

  const collectionService = getCollectionContext();

  import type {
    CollectionSummary as CollectionSummaryType,
    CollectionItemView
  } from '$lib/bindings';

  import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';
  import { collectionItemToCardData } from './utils/cardDataMapper';
  import FilterPanel from './components/FilterPanel.svelte';
  import ControlPanel from './components/ControlPanel.svelte';
  import CollectionTableView from './components/CollectionTableView.svelte';
  import AddCollectionItemDrawer from './components/AddCollectionItemDrawer.svelte';

  function useCollectionUI() {
    let showDrawer = $state(false);
    let showFilterSidebar = $state(true);
    let viewMode = $state<'grid' | 'table'>('grid');
    let editing = $state<CollectionItemView | null>(null);

    const startCreate = () => {
      editing = null;
      showDrawer = true;
    };

    const edit = (item: CollectionItemView) => {
      editing = item;
      showDrawer = true;
    };

    const closeDrawer = () => {
      showDrawer = false;
      editing = null;
    };

    const toggleFilterSidebar = () => {
      showFilterSidebar = !showFilterSidebar;
    };

    const setViewMode = (mode: 'grid' | 'table') => {
      viewMode = mode;
    };

    return {
      get showDrawer() {
        return showDrawer;
      },
      set showDrawer(value: boolean) {
        showDrawer = value;
      },
      get showFilterSidebar() {
        return showFilterSidebar;
      },
      get viewMode() {
        return viewMode;
      },
      get editing() {
        return editing;
      },
      startCreate,
      edit,
      closeDrawer,
      toggleFilterSidebar,
      setViewMode
    };
  }

  const ui = useCollectionUI();

  const defaultSummary = $state<CollectionSummaryType>({
    locomotivesCount: 0,
    passengerCarsCount: 0,
    freightCarsCount: 0,
    trainSetsCount: 0,
    railcarsCount: 0,
    electricMultipleUnitsCount: 0,
    starterSetsCount: 0
  });
  const summaryData = $derived(collectionService.summary ?? defaultSummary);

  const rawItems = $derived(collectionService.rawItems);
  const filteredItems = $derived(collectionService.filteredItems);
  const filters = $derived(collectionService.filters);
  const availableTags = $derived(collectionService.availableTags);
  const isLoading = $derived(collectionService.isLoading);
  const isCollectionEmpty = $derived(rawItems.length === 0);

  // Control Panel derived values — dynamic options from live collection
  const availableScaleOptions = $derived.by(() => {
    const ids = collectionService.availableScaleIds;
    return availableScales.filter((s) => ids.includes(s.id));
  });
  const availableCompanies = $derived(collectionService.availableCompanies);
  const availableCategories = $derived(collectionService.availableCategories);
  const availableEpochs = $derived(collectionService.availableEpochs);
  const hasActiveFilters = $derived(collectionService.hasActiveFilters);

  onMount(() => {
    void collectionService.fetchCollection();
  });

  function handleSearch(query: string) {
    collectionService.setQuery(query);
    void collectionService.fetchCollection(query);
  }

  function handleScale(scale: string | null) {
    collectionService.setScale(scale);
  }

  function handleToggleScale(scale: string) {
    collectionService.toggleScale(scale);
  }

  function handleToggleCompany(company: string) {
    collectionService.toggleCompany(company);
  }

  function handleToggleCategory(category: string) {
    collectionService.toggleCategory(category);
  }

  function handleToggleEpoch(epoch: string) {
    collectionService.toggleEpoch(epoch);
  }

  function handleTag(tag: string) {
    collectionService.toggleTag(tag);
  }

  function handleClear() {
    collectionService.clearFilters();
    void collectionService.fetchCollection('');
  }

  function handleCardClick(item: CollectionItemView) {
    goto(`/collection/${item.id.split(':').pop()}`);
  }
</script>

{#snippet LoadingSkeleton()}
  <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
    {#each Array.from({ length: 6 }, (_, i) => i) as i (i)}
      <div class="h-56 animate-pulse rounded-xl bg-muted" aria-label={`loading-card-${i}`}></div>
    {/each}
  </div>
{/snippet}

{#snippet EmptyState()}
  <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
    <div
      class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-layout-surface/50 px-4 py-24 text-center"
    >
      <!-- Thematic Iconography -->
      <div class="relative">
        <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
        <div
          class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
        >
          <TrainFront size={56} class="text-zinc-600 opacity-50" />
        </div>
      </div>

      <!-- Branded Call-to-Action -->
      <div class="flex max-w-sm flex-col items-center gap-3 text-center">
        <h3 class="text-2xl font-bold text-zinc-200">
          {m.collection_add_first()}
        </h3>
        <p class="text-sm leading-relaxed text-zinc-500">
          {m.collection_empty_caption()}
        </p>
      </div>

      <!-- Central Action Button -->
      <button
        type="button"
        class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
        onclick={ui.startCreate}
      >
        <div
          class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
        ></div>
        <TrainFront class="h-5 w-5" />
        <span>{m.collection_add_item()}</span>
      </button>
    </div>
  </div>
{/snippet}

{#snippet NoResults()}
  <div
    class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-border bg-card p-8 text-center"
  >
    <X class="text-muted-foreground" size={28} />
    <h3 class="text-lg font-semibold">{m.collection_no_results()}</h3>
    <Button variant="outline" onclick={handleClear}>
      {m.collection_clear_filters()}
    </Button>
  </div>
{/snippet}

<svelte:head>
  <title>{m.collection_title()}</title>
</svelte:head>

<div class="mb-10 flex flex-col">
  <!-- Page Header -->
  <div class="-mx-4 -mt-4 border-b border-border bg-card/50 px-6 py-4 sm:px-6 lg:-mx-8 lg:-mt-8">
    <PageHeader
      title={m.collection_title()}
      subtitle={m.collection_subtitle()}
      description={m.collection_description()}
    >
      {#snippet actions()}
        {#if !isCollectionEmpty}
          <Button
            variant="rusty"
            onclick={ui.startCreate}
            size="sm"
            class="shadow-lg shadow-amber-500/10"
          >
            <TrainFront size={18} />
            {m.collection_add_model()}
          </Button>
          <!-- Mobile-only filter toggle -->
          <Button
            onclick={ui.toggleFilterSidebar}
            variant="outline"
            size="sm"
            class="md:hidden"
            title={m.collection_toggle_filters_title()}
          >
            <Filter size={18} />
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  <!-- Content Area with Sidebar -->
  <div class="relative -mx-4 flex flex-1 flex-col md:flex-row lg:-mx-8">
    <!-- Main Content -->
    <div class="flex-1">
      <div class="px-4 py-6 sm:px-6">
        {#if !isCollectionEmpty && !isLoading}
          <div
            class="mb-6 grid grid-cols-2 gap-3 rounded-2xl border border-border/50 bg-muted/30 p-4 sm:grid-cols-3 lg:grid-cols-6"
          >
            {@render StatChip('Locomotives', summaryData.locomotivesCount)}
            {@render StatChip('Passenger Cars', summaryData.passengerCarsCount)}
            {@render StatChip('Freight Cars', summaryData.freightCarsCount)}
            {@render StatChip('Train Sets', summaryData.trainSetsCount)}
            {@render StatChip('Railcars', summaryData.railcarsCount)}
            {@render StatChip('EMU', summaryData.electricMultipleUnitsCount)}
          </div>
        {/if}
        {#if isLoading && rawItems.length === 0}
          {@render LoadingSkeleton()}
        {:else if !isLoading && rawItems.length === 0}
          {@render EmptyState()}
        {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
          {@render NoResults()}
        {:else}
          <!-- View mode toolbar -->
          <div class="mb-4 flex items-center justify-between">
            <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
              {filteredItems.length} item{filteredItems.length === 1 ? '' : 's'}
            </span>
            <div class="flex items-center gap-1 rounded-lg border border-border/60 p-0.5">
              <button
                type="button"
                class="rounded p-1.5 transition-colors {ui.viewMode === 'grid'
                  ? 'bg-muted text-foreground'
                  : 'text-muted-foreground hover:text-foreground'}"
                onclick={() => ui.setViewMode('grid')}
                title="Grid view"
                aria-pressed={ui.viewMode === 'grid'}
              >
                <LayoutGrid size={14} />
              </button>
              <button
                type="button"
                class="rounded p-1.5 transition-colors {ui.viewMode === 'table'
                  ? 'bg-muted text-foreground'
                  : 'text-muted-foreground hover:text-foreground'}"
                onclick={() => ui.setViewMode('table')}
                title="Table view"
                aria-pressed={ui.viewMode === 'table'}
              >
                <Rows3 size={14} />
              </button>
            </div>
          </div>

          {#if ui.viewMode === 'grid'}
            <div
              class="grid gap-4"
              style="grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));"
            >
              {#each filteredItems as item (item.id)}
                <div
                  role="button"
                  tabindex={0}
                  class="cursor-pointer"
                  onclick={() => handleCardClick(item)}
                  onkeydown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      handleCardClick(item);
                    }
                  }}
                >
                  <RailwayModelPreviewCard
                    model={collectionItemToCardData(item)}
                    onDelete={() => void collectionService.deleteItem(item.id)}
                  />
                </div>
              {/each}
            </div>
          {:else}
            <CollectionTableView items={filteredItems} onRowClick={handleCardClick} />
          {/if}
        {/if}
      </div>
    </div>

    <!-- Sidebar (Right) — persistent on desktop, toggled on mobile -->
    {#if ui.showFilterSidebar}
      <!-- Mobile: full-width panel below content -->
      <aside class="w-full flex-shrink-0 border-t border-border bg-card md:hidden">
        <FilterPanel
          {filters}
          {availableTags}
          {availableScales}
          onSearch={handleSearch}
          onSetScale={handleScale}
          onToggleTag={handleTag}
          onClear={handleClear}
          onToggleSidebar={ui.toggleFilterSidebar}
        />
      </aside>
    {/if}

    <!-- Desktop: always-visible command center sidebar (narrows to icon rail when collapsed) -->
    <aside
      class="sticky top-0 hidden h-dvh flex-shrink-0 flex-col overflow-hidden border-l border-layout-border bg-card md:flex"
      style="width: {ui.showFilterSidebar
        ? '280px'
        : '60px'}; transition: width 280ms cubic-bezier(0.4, 0, 0.2, 1);"
    >
      <!-- Sidebar header — always visible -->
      <div
        class="flex flex-shrink-0 items-center border-b border-layout-border px-2 py-3"
        class:justify-between={ui.showFilterSidebar}
        class:justify-center={!ui.showFilterSidebar}
      >
        <span
          class="text-[10px] font-semibold tracking-widest whitespace-nowrap text-muted-foreground uppercase transition-[opacity,width] duration-200"
          style="opacity: {ui.showFilterSidebar ? '1' : '0'}; width: {ui.showFilterSidebar
            ? 'auto'
            : '0'}; overflow: hidden;"
        >
          Filters
        </span>
        <button
          type="button"
          class="rounded p-1 text-muted-foreground transition-colors hover:text-primary"
          onclick={ui.toggleFilterSidebar}
          title={m.collection_toggle_filters_title()}
          aria-expanded={ui.showFilterSidebar}
        >
          <SlidersHorizontal size={14} />
        </button>
      </div>

      <!-- Sidebar content — fades in after sidebar expands -->
      <div
        class="flex-1 overflow-y-auto transition-opacity duration-200"
        style="opacity: {ui.showFilterSidebar ? '1' : '0'}; pointer-events: {ui.showFilterSidebar
          ? 'auto'
          : 'none'};"
      >
        <ControlPanel
          {filters}
          availableScales={availableScaleOptions}
          {availableCompanies}
          {availableCategories}
          {availableEpochs}
          {hasActiveFilters}
          onToggleScale={handleToggleScale}
          onToggleCompany={handleToggleCompany}
          onToggleCategory={handleToggleCategory}
          onToggleEpoch={handleToggleEpoch}
          onClear={handleClear}
          onToggleSidebar={ui.toggleFilterSidebar}
        />
      </div>
    </aside>
  </div>
</div>

<AddCollectionItemDrawer
  open={ui.showDrawer}
  onClose={ui.closeDrawer}
  onSuccess={() => {
    ui.closeDrawer();
  }}
/>

{#snippet StatChip(label: string, count: number)}
  <div
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">{label}</p>
    <p class="mt-1 text-lg font-bold text-primary">{count}</p>
  </div>
{/snippet}
