<script lang="ts">
  import { Plus, Tag, X, Filter } from 'lucide-svelte';
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
  import AddCollectionItemDrawer from './components/AddCollectionItemDrawer.svelte';
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
  <div
    class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-border bg-card p-10 text-center"
  >
    <Tag class="text-muted-foreground" size={32} />
    <h3 class="text-lg font-semibold">{m.collection_add_first()}</h3>
    <p class="text-sm text-muted-foreground">{m.collection_empty_caption()}</p>
    <Button variant="rusty" onclick={ui.startCreate} class="shadow-lg shadow-amber-500/10">
      {m.collection_add_item()}
    </Button>
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
        <Button
          variant="rusty"
          onclick={ui.startCreate}
          size="sm"
          class="shadow-lg shadow-amber-500/10"
        >
          <Plus size={18} />
          {m.collection_add_model()}
        </Button>
        <Button
          onclick={ui.toggleFilterSidebar}
          variant="outline"
          size="sm"
          title={m.collection_toggle_filters_title()}
        >
          <Filter size={18} />
        </Button>
      {/snippet}
      <div
        class="grid grid-cols-2 gap-3 rounded-2xl border border-border/50 bg-muted/30 p-4 sm:grid-cols-3 lg:grid-cols-6"
      >
        {@render StatChip('Locomotives', summaryData.locomotivesCount)}
        {@render StatChip('Passenger Cars', summaryData.passengerCarsCount)}
        {@render StatChip('Freight Cars', summaryData.freightCarsCount)}
        {@render StatChip('Train Sets', summaryData.trainSetsCount)}
        {@render StatChip('Railcars', summaryData.railcarsCount)}
        {@render StatChip('EMU', summaryData.electricMultipleUnitsCount)}
      </div>
    </PageHeader>
  </div>

  <!-- Content Area with Sidebar -->
  <div class="relative -mx-4 flex flex-1 flex-col md:flex-row lg:-mx-8">
    <!-- Main Content -->
    <div class="flex-1">
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
                  onDelete={() => ui.requestDelete(item.id)}
                />
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Sidebar (Right) -->
    {#if ui.showFilterSidebar}
      <aside
        class="w-full flex-shrink-0 border-t border-border bg-card md:w-80 md:border-t-0 md:border-l"
      >
        <div class="sticky top-4">
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
    class="flex flex-col justify-between rounded-xl border border-border/50 bg-muted/20 px-4 py-3 transition-colors hover:bg-muted/40"
  >
    <p class="text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">{label}</p>
    <p class="mt-1 text-lg font-bold text-primary">{count}</p>
  </div>
{/snippet}
