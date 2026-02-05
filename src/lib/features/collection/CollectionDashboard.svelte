<script lang="ts">
  import { Plus, Tag, X, Filter } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { getCollectionContext, availableScales } from './CollectionState.svelte';
  import { Button, Input, Badge } from '$lib/components';

  const collectionService = getCollectionContext();

  import type {
    CollectionSummary as CollectionSummaryType,
    CollectionItemView
  } from '$lib/bindings';

  import ItemCard from './components/ItemCard.svelte';
  import FilterPanel from './components/FilterPanel.svelte';
  import AddModelDrawer from './components/AddModelDrawer.svelte';
  import DeleteModal from './components/DeleteModal.svelte';

  function useCollectionUI() {
    let showDrawer = $state(false);
    let showFilterSidebar = $state(true);
    let editing = $state<CollectionItemView | null>(null);
    let confirmDeleteId = $state<string | null>(null);

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

    const requestDelete = (id: string | null) => {
      confirmDeleteId = id;
    };

    const clearDelete = () => {
      confirmDeleteId = null;
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
      get editing() {
        return editing;
      },
      get confirmDeleteId() {
        return confirmDeleteId;
      },
      startCreate,
      edit,
      closeDrawer,
      toggleFilterSidebar,
      requestDelete,
      clearDelete
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
  const totalValue = $derived(
    collectionService.collection?.totalValue
      ? new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: collectionService.collection.totalValue.currency
        }).format(Number(collectionService.collection.totalValue.amount) / 100)
      : '--'
  );

  const totalUnits = $derived(
    summaryData.locomotivesCount +
      summaryData.passengerCarsCount +
      summaryData.freightCarsCount +
      summaryData.trainSetsCount +
      summaryData.railcarsCount +
      summaryData.electricMultipleUnitsCount
  );

  const rawItems = $derived(collectionService.rawItems);
  const filteredItems = $derived(collectionService.filteredItems);
  const filters = $derived(collectionService.filters);
  const availableTags = $derived(collectionService.availableTags);
  const isLoading = $derived(collectionService.isLoading);

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

  function handleTag(tag: string) {
    collectionService.toggleTag(tag);
  }

  function handleClear() {
    collectionService.clearFilters();
    void collectionService.fetchCollection('');
  }

  async function handleDeleteConfirm() {
    if (!ui.confirmDeleteId) return;
    await collectionService.deleteItem(ui.confirmDeleteId);
    ui.clearDelete();
  }
</script>

{#snippet LoadingSkeleton()}
  <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
    {#each Array.from({ length: 6 }, (_, i) => i) as i (i)}
      <div
        class="bg-surface-800/80 h-56 animate-pulse rounded-xl"
        aria-label={`loading-card-${i}`}
      ></div>
    {/each}
  </div>
{/snippet}

{#snippet EmptyState()}
  <div
    class="border-surface-700/60 bg-surface-900 flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed p-10 text-center"
  >
    <Tag class="text-surface-500" size={32} />
    <h3 class="text-lg font-semibold">{m.collection_add_first()}</h3>
    <p class="text-surface-400 text-sm">{m.collection_empty_caption()}</p>
    <button class="variant-filled-primary btn" onclick={ui.startCreate}>
      {m.collection_add_item()}
    </button>
  </div>
{/snippet}

{#snippet NoResults()}
  <div
    class="border-warning-500/40 bg-surface-900 flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed p-8 text-center"
  >
    <X class="text-warning-400" size={28} />
    <h3 class="text-lg font-semibold">{m.collection_no_results()}</h3>
    <button class="variant-soft-warning btn" onclick={handleClear}>
      {m.collection_clear_filters()}
    </button>
  </div>
{/snippet}

<svelte:head>
  <title>{m.collection_title()}</title>
</svelte:head>

<div class="flex h-screen overflow-hidden bg-surface-950">
  <!-- Main Content -->
  <div class="flex flex-col flex-1 overflow-hidden">
    <!-- Sticky Header -->
    <header class="sticky top-0 z-30 border-b border-surface-700/60 bg-surface-900/95 backdrop-blur-sm flex-shrink-0">
      <div class="px-4 py-3 sm:px-6">
        <div class="flex items-center justify-between gap-4 mb-3">
          <div>
            <p class="text-xs tracking-[0.2em] text-surface-400 uppercase">{m.app_collection()}</p>
            <h1 class="text-lg font-bold text-surface-50">{m.collection_title()}</h1>
          </div>
          <div class="flex items-center gap-2">
            <div class="text-right">
              <p class="text-xs text-surface-400 uppercase tracking-widest">Collection Value</p>
              <p class="text-xl font-bold text-primary-200">{totalValue}</p>
            </div>
            <div class="w-px h-12 bg-surface-700/60"></div>
            <div class="text-right">
              <p class="text-xs text-surface-400 uppercase tracking-widest">Total Units</p>
              <p class="text-xl font-bold text-primary-200">{totalUnits}</p>
            </div>
            <Button onclick={ui.startCreate} size="sm">
              <Plus size={18} />
              {m.collection_add_model()}
            </Button>
          </div>
        </div>

        <!-- Horizontal Stat Chips -->
        <div class="flex items-center gap-2 overflow-x-auto pb-1 -mx-4 px-4 scrollbar-hide">
          {@render StatChip('Locomotives', summaryData.locomotivesCount)}
          {@render StatChip('Passenger Cars', summaryData.passengerCarsCount)}
          {@render StatChip('Freight Cars', summaryData.freightCarsCount)}
          {@render StatChip('Train Sets', summaryData.trainSetsCount)}
          {@render StatChip('Railcars', summaryData.railcarsCount)}
          {@render StatChip('EMU', summaryData.electricMultipleUnitsCount)}
        </div>
      </div>
    </header>

    <!-- Main Content Area -->
    <main class="flex-1 overflow-y-auto">
      <div class="px-4 py-6 sm:px-6">
        {#if isLoading && rawItems.length === 0}
          {@render LoadingSkeleton()}
        {:else if !isLoading && rawItems.length === 0}
          {@render EmptyState()}
        {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
          {@render NoResults()}
        {:else}
          <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each filteredItems as item (item.id)}
              <ItemCard {item} onEdit={ui.edit} onDelete={ui.requestDelete} />
            {/each}
          </div>
        {/if}
      </div>
    </main>
  </div>

  <!-- Sidebar (Right) -->
  {#if ui.showFilterSidebar}
    <aside class="w-80 flex-shrink-0 border-l border-surface-700/60 bg-surface-900 overflow-y-auto">
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
</div>

<AddModelDrawer
  open={ui.showDrawer}
  onClose={ui.closeDrawer}
  onSuccess={() => {
    ui.closeDrawer();
  }}
/>

<DeleteModal
  open={Boolean(ui.confirmDeleteId)}
  title={m.collection_delete_item()}
  message={m.collection_confirm_delete()}
  onClose={ui.clearDelete}
  onConfirm={handleDeleteConfirm}
/>

{#snippet StatChip(label: string, count: number)}
  <div
    class="flex-shrink-0 px-3 py-1.5 rounded-full bg-surface-800/60 border border-surface-700/80 hover:border-primary-500/40 transition-colors whitespace-nowrap"
  >
    <p class="text-xs text-surface-300 font-medium">{label}</p>
    <p class="text-sm font-bold text-primary-200">{count}</p>
  </div>
{/snippet}
