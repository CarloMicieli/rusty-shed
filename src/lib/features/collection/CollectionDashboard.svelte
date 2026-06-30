<script lang="ts">
  import {
    TrainFront,
    X,
    Filter,
    LayoutGrid,
    Rows3,
    SlidersHorizontal,
    Search
  } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { collectionState, availableScales } from './CollectionState.svelte';
  import type { StatusFilter } from './CollectionState.svelte';
  import { Button, Badge, Input } from '$lib/components';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { commands } from '$lib/bindings';

  const collectionService = collectionState;

  import type {
    CollectionStats,
    CollectionSummary as CollectionSummaryType,
    CollectionItemView
  } from '$lib/bindings';

  import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';
  import VirtualGrid from '$lib/components/VirtualGrid.svelte';
  import { collectionItemToCardData } from './utils/cardDataMapper';
  import FilterPanel from './components/FilterPanel.svelte';
  import ControlPanel from './components/ControlPanel.svelte';
  import CollectionTableView from './components/CollectionTableView.svelte';
  import AddCollectionItemDrawer from './components/AddCollectionItemDrawer.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import { createMobileMatchMediaState } from '$lib/state/match-media.svelte';

  const mobileMedia = createMobileMatchMediaState();
  let isMobileViewport = $state(false);

  $effect(() => {
    const unsubscribe = mobileMedia.subscribe((matches) => {
      isMobileViewport = matches;
      if (matches && ui.viewMode === 'table') {
        ui.setViewMode('grid');
      }
    });

    return () => {
      unsubscribe();
    };
  });

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
  let mobileSearchDebounce: ReturnType<typeof setTimeout> | null = null;

  const defaultSummary = $state<CollectionSummaryType>({
    locomotivesCount: 0,
    passengerCarsCount: 0,
    freightCarsCount: 0,
    trainSetsCount: 0,
    railcarsCount: 0,
    electricMultipleUnitsCount: 0,
    starterSetsCount: 0
  });
  const defaultCollectionStats: CollectionStats = {
    preorderedCount: 0,
    activeCount: 0,
    soldCount: 0,
    investmentAtRiskAmount: 0,
    investmentAtRiskCurrency: null,
    realizedProfitAmount: 0,
    realizedProfitCurrency: null
  };
  const summaryData = $derived(collectionService.summary ?? defaultSummary);
  let collectionStats = $state<CollectionStats>(defaultCollectionStats);

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
  let mobileSearchQuery = $derived(filters.query);

  onMount(() => {
    void collectionService.fetchCollection();
    void loadCollectionStats();
  });

  async function loadCollectionStats() {
    const result = await commands.getCollectionStats();
    if (result.status === 'ok') {
      collectionStats = result.data;
    }
  }

  function handleSearch(query: string) {
    collectionService.setQuery(query);
  }

  function handleMobileSearchInput(value: string) {
    mobileSearchQuery = value;
    if (mobileSearchDebounce) {
      clearTimeout(mobileSearchDebounce);
    }

    mobileSearchDebounce = setTimeout(() => {
      handleSearch(value);
    }, 220);
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

  function handleSetStatus(status: StatusFilter) {
    collectionService.setStatus(status);
  }

  function handleLifecycleChipClick(status: StatusFilter) {
    collectionService.setStatus(status);
  }

  function handleTag(tag: string) {
    collectionService.toggleTag(tag);
  }

  function handleClear() {
    collectionService.clearFilters();
  }

  function handleCardClick(item: CollectionItemView) {
    const itemId = item.id.split(':').pop();
    if (!itemId) {
      return;
    }

    goto(resolve(`/collection/${itemId}`));
  }
</script>

{#snippet LoadingSkeleton()}
  <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
    {#each Array.from({ length: 6 }, (_, i) => i) as i (i)}
      <div class="h-56 animate-pulse rounded-xl bg-muted" aria-label={`loading-card-${i}`}></div>
    {/each}
  </div>
{/snippet}

{#snippet CollectionEmptyState()}
  <EmptyState
    icon={TrainFront}
    title={m.collection_add_first()}
    description={m.collection_empty_caption()}
    ctaLabel={m.collection_add_item()}
    onCta={ui.startCreate}
  />
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
            class="hidden min-h-11 shadow-lg shadow-amber-500/10 md:inline-flex"
          >
            <TrainFront size={18} />
            {m.collection_add_model()}
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
          <div class="mb-4 grid grid-cols-[1fr_auto] items-center gap-2 md:hidden">
            <label class="relative flex min-h-11 items-center" for="collection-mobile-search">
              <Search class="pointer-events-none absolute left-3 h-4 w-4 text-muted-foreground" />
              <Input
                id="collection-mobile-search"
                type="search"
                class="h-11 w-full rounded-full border-border/70 bg-card pr-3 pl-9 text-sm"
                placeholder={m.collection_search_placeholder()}
                value={mobileSearchQuery}
                oninput={(e) => handleMobileSearchInput((e.target as HTMLInputElement).value)}
              />
            </label>
            <Button
              onclick={ui.toggleFilterSidebar}
              variant="outline"
              size="sm"
              class="min-h-11 min-w-11 rounded-full transition-all active:scale-[0.98] active:bg-muted/50"
              title={m.collection_toggle_filters_title()}
              aria-expanded={ui.showFilterSidebar}
              aria-controls="collection-mobile-filter-panel"
            >
              <Filter size={18} />
            </Button>
          </div>
          <div
            class="mb-4 flex snap-x snap-mandatory [scrollbar-width:none] gap-2 overflow-x-auto pb-1 whitespace-nowrap [-ms-overflow-style:none] md:hidden"
          >
            {@render StatPill(
              m.category_value_locomotives(),
              summaryData.locomotivesCount,
              'text-primary'
            )}
            {@render StatPill(
              m.category_value_passenger_cars(),
              summaryData.passengerCarsCount,
              'text-primary'
            )}
            {@render StatPill(
              m.category_value_freight_cars(),
              summaryData.freightCarsCount,
              'text-primary'
            )}
            {@render StatPill(
              m.category_value_train_sets(),
              summaryData.trainSetsCount,
              'text-primary'
            )}
            {@render StatPill(
              m.category_value_railcars(),
              summaryData.railcarsCount,
              'text-primary'
            )}
            {@render StatPill(
              m.category_value_electric_multiple_units(),
              summaryData.electricMultipleUnitsCount,
              'text-primary'
            )}
            <button
              type="button"
              class="min-h-11 shrink-0 snap-start rounded-full border border-amber-500/30 bg-amber-500/8 px-3 text-xs font-medium text-amber-300 transition-all active:scale-[0.98] active:bg-amber-500/18"
              onclick={() => handleLifecycleChipClick('preordered')}
            >
              {collectionStats.preorderedCount}
              {m.collection_stats_preordered()}
            </button>
            <button
              type="button"
              class="min-h-11 shrink-0 snap-start rounded-full border border-emerald-500/30 bg-emerald-500/8 px-3 text-xs font-medium text-emerald-300 transition-all active:scale-[0.98] active:bg-emerald-500/18"
              onclick={() => handleLifecycleChipClick('active')}
            >
              {collectionStats.activeCount}
              {m.collection_stats_active()}
            </button>
            <button
              type="button"
              class="min-h-11 shrink-0 snap-start rounded-full border border-rose-500/30 bg-rose-500/8 px-3 text-xs font-medium text-rose-300 transition-all active:scale-[0.98] active:bg-rose-500/18"
              onclick={() => handleLifecycleChipClick('sold')}
            >
              {collectionStats.soldCount}
              {m.collection_stats_sold()}
            </button>
          </div>
          <div
            class="mb-6 hidden gap-3 rounded-2xl border border-border/50 bg-muted/30 p-4 md:grid md:grid-cols-3 lg:grid-cols-6"
          >
            {@render StatChip(m.category_value_locomotives(), summaryData.locomotivesCount)}
            {@render StatChip(m.category_value_passenger_cars(), summaryData.passengerCarsCount)}
            {@render StatChip(m.category_value_freight_cars(), summaryData.freightCarsCount)}
            {@render StatChip(m.category_value_train_sets(), summaryData.trainSetsCount)}
            {@render StatChip(m.category_value_railcars(), summaryData.railcarsCount)}
            {@render StatChip(
              m.category_value_electric_multiple_units(),
              summaryData.electricMultipleUnitsCount
            )}
          </div>
          <div class="mb-4 hidden flex-wrap gap-2 md:flex">
            <button
              type="button"
              class="rounded-full border border-amber-500/30 bg-amber-500/8 px-3 py-1.5 text-xs font-medium text-amber-300 transition-colors hover:bg-amber-500/14"
              onclick={() => handleLifecycleChipClick('preordered')}
            >
              {collectionStats.preorderedCount}
              {m.collection_stats_preordered()}
            </button>
            <button
              type="button"
              class="rounded-full border border-emerald-500/30 bg-emerald-500/8 px-3 py-1.5 text-xs font-medium text-emerald-300 transition-colors hover:bg-emerald-500/14"
              onclick={() => handleLifecycleChipClick('active')}
            >
              {collectionStats.activeCount}
              {m.collection_stats_active()}
            </button>
            <button
              type="button"
              class="rounded-full border border-rose-500/30 bg-rose-500/8 px-3 py-1.5 text-xs font-medium text-rose-300 transition-colors hover:bg-rose-500/14"
              onclick={() => handleLifecycleChipClick('sold')}
            >
              {collectionStats.soldCount}
              {m.collection_stats_sold()}
            </button>
          </div>
        {/if}
        {#if isLoading && rawItems.length === 0}
          {@render LoadingSkeleton()}
        {:else if !isLoading && rawItems.length === 0}
          {@render CollectionEmptyState()}
        {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
          {@render NoResults()}
        {:else}
          <!-- Filter Chips Row -->
          {#if hasActiveFilters}
            <div class="mb-4 flex flex-wrap gap-2">
              <!-- Scale chips -->
              {#if filters.scales.size > 0}
                {#each Array.from(filters.scales) as scale (scale)}
                  <Badge variant="default" class="flex items-center gap-1.5 pr-1 pl-2.5">
                    <span class="text-xs"
                      >{availableScales.find((s) => s.id === scale)?.display || scale}</span
                    >
                    <button
                      type="button"
                      onclick={() => collectionService.toggleScale(scale)}
                      class="h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20"
                      aria-label={`Remove scale filter: ${scale}`}
                    >
                      <X size={14} />
                    </button>
                  </Badge>
                {/each}
              {/if}

              <!-- Epoch chips -->
              {#if filters.epochs.size > 0}
                {#each Array.from(filters.epochs) as epoch (epoch)}
                  <Badge variant="default" class="flex items-center gap-1.5 pr-1 pl-2.5">
                    <span class="text-xs">Epoch {epoch}</span>
                    <button
                      type="button"
                      onclick={() => collectionService.toggleEpoch(epoch)}
                      class="h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20"
                      aria-label={`Remove epoch filter: ${epoch}`}
                    >
                      <X size={14} />
                    </button>
                  </Badge>
                {/each}
              {/if}

              <!-- Category chips -->
              {#if filters.categories.size > 0}
                {#each Array.from(filters.categories) as category (category)}
                  <Badge variant="default" class="flex items-center gap-1.5 pr-1 pl-2.5">
                    <span class="text-xs">{category}</span>
                    <button
                      type="button"
                      onclick={() => collectionService.toggleCategory(category)}
                      class="h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20"
                      aria-label={`Remove category filter: ${category}`}
                    >
                      <X size={14} />
                    </button>
                  </Badge>
                {/each}
              {/if}

              <!-- Company chips -->
              {#if filters.companies.size > 0}
                {#each Array.from(filters.companies) as company (company)}
                  <Badge variant="default" class="flex items-center gap-1.5 pr-1 pl-2.5">
                    <span class="text-xs">{company}</span>
                    <button
                      type="button"
                      onclick={() => collectionService.toggleCompany(company)}
                      class="h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20"
                      aria-label={`Remove company filter: ${company}`}
                    >
                      <X size={14} />
                    </button>
                  </Badge>
                {/each}
              {/if}

              <!-- Tag chips -->
              {#if filters.tags.size > 0}
                {#each Array.from(filters.tags) as tag (tag)}
                  <Badge variant="default" class="flex items-center gap-1.5 pr-1 pl-2.5">
                    <span class="text-xs">{tag}</span>
                    <button
                      type="button"
                      onclick={() => collectionService.toggleTag(tag)}
                      class="h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20"
                      aria-label={`Remove tag filter: ${tag}`}
                    >
                      <X size={14} />
                    </button>
                  </Badge>
                {/each}
              {/if}
            </div>
          {/if}

          <!-- View mode toolbar -->
          <div class="mb-4 hidden items-center justify-end md:flex">
            <div class="flex items-center gap-1 rounded-lg border border-border/60 p-0.5">
              <button
                type="button"
                class="rounded p-1.5 transition-colors {ui.viewMode === 'grid'
                  ? 'bg-muted text-foreground'
                  : 'text-muted-foreground hover:text-foreground'}"
                onclick={() => ui.setViewMode('grid')}
                title={m.view_grid()}
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
                title={m.view_table()}
                aria-pressed={ui.viewMode === 'table'}
              >
                <Rows3 size={14} />
              </button>
            </div>
          </div>

          {#if ui.viewMode === 'grid'}
            <!--
              VirtualGrid renders only the rows currently visible in the scroll
              viewport. itemHeight is an estimate of the card's rendered height;
              overscan keeps a few extra rows mounted to prevent blank flashes.
            -->
            <VirtualGrid
              items={filteredItems}
              itemHeight={340}
              itemMinWidth={isMobileViewport ? 320 : 240}
              gap={16}
              overscan={3}
            >
              {#snippet children(item, _idx)}
                <div
                  role="button"
                  tabindex={0}
                  class="min-h-11 cursor-pointer transition-transform active:scale-[0.99]"
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
              {/snippet}
            </VirtualGrid>
          {:else}
            <CollectionTableView items={filteredItems} onRowClick={handleCardClick} />
          {/if}
        {/if}
      </div>
    </div>

    <!-- Sidebar (Right) — persistent on desktop, toggled on mobile -->
    {#if ui.showFilterSidebar}
      <!-- Mobile: full-width panel below content -->
      <aside
        id="collection-mobile-filter-panel"
        class="w-full flex-shrink-0 border-t border-border bg-card md:hidden"
      >
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
    {#if !isCollectionEmpty}
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
            onSetStatus={handleSetStatus}
            onClear={handleClear}
            onToggleSidebar={ui.toggleFilterSidebar}
          />
        </div>
      </aside>
    {/if}
  </div>
</div>

<AddCollectionItemDrawer
  open={ui.showDrawer}
  onClose={ui.closeDrawer}
  onSuccess={() => {
    ui.closeDrawer();
    void loadCollectionStats();
  }}
/>

{#if isMobileViewport && !isCollectionEmpty}
  <Button
    class="safe-area-inset-bottom fixed right-4 bottom-4 z-40 min-h-11 rounded-full px-5 font-semibold shadow-xl md:hidden"
    variant="rusty"
    onclick={ui.startCreate}
    aria-label={m.collection_mobile_add_fab()}
  >
    <TrainFront size={18} />
    {m.collection_mobile_add_fab()}
  </Button>
{/if}

{#snippet StatChip(label: string, count: number)}
  <div
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors md:hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">{label}</p>
    <p class="mt-1 text-lg font-bold text-primary">{count}</p>
  </div>
{/snippet}

{#snippet StatPill(label: string, count: number, countColorClass = 'text-primary')}
  <div
    class="min-h-11 shrink-0 snap-start rounded-full border border-border/60 bg-muted/30 px-3 py-2 leading-tight"
  >
    <p class="text-[10px] font-semibold tracking-wide text-muted-foreground uppercase">{label}</p>
    <p class={`text-sm font-bold ${countColorClass}`}>{count}</p>
  </div>
{/snippet}
