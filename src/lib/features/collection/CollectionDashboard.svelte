<script lang="ts">
  import { Plus, Tag, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { collectionStore, availableScales } from '$lib/stores/collectionStore.svelte';
  import type {
    CollectionItemLite,
    CollectionSummary as CollectionSummaryType,
    CreateCollectionItemInput
  } from '$lib/bindings';
  import ItemCard from './components/ItemCard.svelte';
  import FilterSidebar from './components/FilterSidebar.svelte';
  import ItemDrawer from './components/ItemDrawer.svelte';
  import DeleteModal from './components/DeleteModal.svelte';
  import CollectionSummary from './components/CollectionSummary.svelte';

  type SubmitDetail = { form: CreateCollectionItemInput; editingId: string | null };

  function useCollectionUI() {
    let showDrawer = $state(false);
    let editing = $state<CollectionItemLite | null>(null);
    let confirmDeleteId = $state<string | null>(null);

    const startCreate = () => {
      editing = null;
      showDrawer = true;
    };

    const edit = (item: CollectionItemLite) => {
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
    locomotives_count: 0,
    passenger_cars_count: 0,
    freight_cars_count: 0,
    train_sets_count: 0,
    railcars_count: 0,
    electric_multiple_units_count: 0
  });
  const summaryData = $derived(defaultSummary);
  const totalValue = $state('--');

  const rawItems = $derived(collectionStore.rawItems);
  const filteredItems = $derived(collectionStore.filteredItems);
  const filters = $derived(collectionStore.filters);
  const availableTags = $derived(collectionStore.availableTags);
  const isLoading = $derived(collectionStore.isLoading);

  onMount(() => {
    void collectionStore.fetchCollection();
  });

  async function handleSubmit(detail: SubmitDetail) {
    const { form, editingId } = detail;
    if (editingId) {
      await collectionStore.updateItem({ id: editingId, ...form });
    } else {
      await collectionStore.createItem(form);
    }
    ui.closeDrawer();
  }

  function handleSearch(query: string) {
    collectionStore.setQuery(query);
    void collectionStore.fetchCollection(query);
  }

  function handleScale(scale: string | null) {
    collectionStore.setScale(scale);
  }

  function handleTag(tag: string) {
    collectionStore.toggleTag(tag);
  }

  function handleClear() {
    collectionStore.clearFilters();
    void collectionStore.fetchCollection('');
  }

  async function handleDeleteConfirm() {
    if (!ui.confirmDeleteId) return;
    await collectionStore.deleteItem(ui.confirmDeleteId);
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
        {m.collection_add_item()}
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

<ItemDrawer
  open={ui.showDrawer}
  editing={ui.editing}
  {availableScales}
  onClose={ui.closeDrawer}
  onSubmit={handleSubmit}
/>

<DeleteModal
  open={Boolean(ui.confirmDeleteId)}
  title={m.collection_delete_item()}
  message={m.collection_confirm_delete()}
  onClose={ui.clearDelete}
  onConfirm={handleDeleteConfirm}
/>
