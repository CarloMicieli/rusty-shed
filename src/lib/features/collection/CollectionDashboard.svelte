<script lang="ts">
  import { Plus, Tag, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { getCollectionContext, availableScales } from './CollectionState.svelte';

  const collectionService = getCollectionContext();

  import type {
    CollectionSummary as CollectionSummaryType,
    CollectionItemView
  } from '$lib/bindings';

  import ItemCard from './components/ItemCard.svelte';
  import FilterSidebar from './components/FilterSidebar.svelte';
  import AddModelDrawer from './components/AddModelDrawer.svelte';
  import DeleteModal from './components/DeleteModal.svelte';
  import CollectionSummary from './components/CollectionSummary.svelte';

  function useCollectionUI() {
    let showDrawer = $state(false);
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
      get editing() {
        return editing;
      },
      get confirmDeleteId() {
        return confirmDeleteId;
      },
      startCreate,
      edit,
      closeDrawer,
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
        class="h-56 animate-pulse rounded-xl bg-surface-800/80"
        aria-label={`loading-card-${i}`}
      ></div>
    {/each}
  </div>
{/snippet}

{#snippet EmptyState()}
  <div
    class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-surface-700/60 bg-surface-900 p-10 text-center"
  >
    <Tag class="text-surface-500" size={32} />
    <h3 class="text-lg font-semibold">{m.collection_add_first()}</h3>
    <p class="text-sm text-surface-400">{m.collection_empty_caption()}</p>
    <button class="variant-filled-primary btn" onclick={ui.startCreate}>
      {m.collection_add_item()}
    </button>
  </div>
{/snippet}

{#snippet NoResults()}
  <div
    class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-warning-500/40 bg-surface-900 p-8 text-center"
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

<div class="space-y-6">
  <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
    <div>
      <p class="text-sm tracking-[0.2em] text-surface-400 uppercase">{m.app_collection()}</p>
      <h1 class="h2 font-bold">{m.collection_title()}</h1>
      <p class="text-sm text-surface-400">{m.collection_empty_caption()}</p>
    </div>
    <div class="flex flex-col gap-3 md:flex-row md:items-center">
      <button class="variant-filled-primary btn gap-2" onclick={ui.startCreate}>
        <Plus size={18} />
        {m.collection_add_model()}
      </button>
    </div>
  </div>

  <CollectionSummary summary={summaryData} {totalValue} />

  <div class="grid gap-4 lg:grid-cols-[280px,1fr]">
    <FilterSidebar
      {filters}
      {availableTags}
      {availableScales}
      onSearch={handleSearch}
      onSetScale={handleScale}
      onToggleTag={handleTag}
      onClear={handleClear}
    />

    <section class="space-y-4">
      {#if isLoading && rawItems.length === 0}
        {@render LoadingSkeleton()}
      {:else if !isLoading && rawItems.length === 0}
        {@render EmptyState()}
      {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
        {@render NoResults()}
      {:else}
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each filteredItems as item (item.id)}
            <ItemCard {item} onEdit={ui.edit} onDelete={ui.requestDelete} />
          {/each}
        </div>
      {/if}
    </section>
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
