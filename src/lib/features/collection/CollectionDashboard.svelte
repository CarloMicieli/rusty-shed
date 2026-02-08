<script lang="ts">
  import { Plus, Tag, X, Filter } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { getCollectionContext, availableScales } from './CollectionState.svelte';
  import { Button } from '$lib/components';

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
    let showFilterSidebar = $state(false);
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

  function handleCardClick(item: CollectionItemView) {
    goto(`/models/${item.railwayModel.railwayModelId}`);
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

<div class="bg-surface-950 flex h-screen flex-col overflow-hidden">
  <!-- Sticky Header (Full Width) -->
  <header
    class="border-surface-700/60 bg-surface-900/95 sticky top-0 z-30 flex-shrink-0 border-b backdrop-blur-sm"
  >
    <div class="px-4 py-3 sm:px-6">
      <div class="mb-3 flex items-center justify-between gap-4">
        <div>
          <p class="text-surface-400 text-xs tracking-[0.2em] uppercase">{m.app_collection()}</p>
          <h1 class="text-surface-50 text-lg font-bold">{m.collection_title()}</h1>
        </div>
        <div class="flex items-center gap-2">
          <div class="text-right">
            <p class="text-surface-400 text-xs tracking-widest uppercase">Collection Value</p>
            <p class="text-primary-200 text-xl font-bold">{totalValue}</p>
          </div>
          <div class="bg-surface-700/60 h-12 w-px"></div>
          <div class="text-right">
            <p class="text-surface-400 text-xs tracking-widest uppercase">Total Units</p>
            <p class="text-primary-200 text-xl font-bold">{totalUnits}</p>
          </div>
        </div>
      </div>

      <!-- Horizontal Stat Chips -->
      <div class="scrollbar-hide -mx-4 flex items-center gap-2 overflow-x-auto px-4 pb-1">
        {@render StatChip('Locomotives', summaryData.locomotivesCount)}
        {@render StatChip('Passenger Cars', summaryData.passengerCarsCount)}
        {@render StatChip('Freight Cars', summaryData.freightCarsCount)}
        {@render StatChip('Train Sets', summaryData.trainSetsCount)}
        {@render StatChip('Railcars', summaryData.railcarsCount)}
        {@render StatChip('EMU', summaryData.electricMultipleUnitsCount)}
      </div>
    </div>
  </header>

  <!-- Content Area with Sidebar -->
  <div class="flex flex-1 overflow-hidden">
    <!-- Main Content -->
    <main class="flex-1 overflow-y-auto">
      <div class="px-4 py-6 sm:px-6">
        <!-- Add Button and Filter Toggle -->
        <div class="mb-6 flex items-center justify-end gap-3">
          <Button onclick={ui.startCreate} size="sm">
            <Plus size={18} />
            {m.collection_add_model()}
          </Button>
          <Button
            onclick={ui.toggleFilterSidebar}
            variant="outline"
            size="sm"
            title="Toggle filters"
          >
            <Filter size={18} />
          </Button>
        </div>

        {#if isLoading && rawItems.length === 0}
          {@render LoadingSkeleton()}
        {:else if !isLoading && rawItems.length === 0}
          {@render EmptyState()}
        {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
          {@render NoResults()}
        {:else}
          <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {#each filteredItems as item (item.id)}
              <ItemCard
                {item}
                onEdit={ui.edit}
                onDelete={ui.requestDelete}
                onClick={handleCardClick}
              />
            {/each}
          </div>
        {/if}
      </div>
    </main>

    <!-- Sidebar (Right) -->
    {#if ui.showFilterSidebar}
      <aside
        class="border-surface-700/60 bg-surface-900 w-80 flex-shrink-0 overflow-y-auto border-l"
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
  </div>
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
    class="bg-surface-800/60 border-surface-700/80 hover:border-primary-500/40 w-36 flex-shrink-0 rounded-full border px-3 py-1.5 transition-colors"
  >
    <p class="text-surface-300 truncate text-xs font-medium">{label}</p>
    <p class="text-primary-200 text-sm font-bold">{count}</p>
  </div>
{/snippet}
