<script lang="ts">
  import { Plus, Tag, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';
  import { collectionStore, availableScales } from '$lib/stores/collectionStore';
  import type { CollectionItemLite, CreateCollectionItemInput } from '$lib/bindings';
  import ItemCard from './components/ItemCard.svelte';
  import FilterSidebar from './components/FilterSidebar.svelte';
  import ItemDrawer from './components/ItemDrawer.svelte';
  import DeleteModal from './components/DeleteModal.svelte';

  type SubmitDetail = { form: CreateCollectionItemInput; editingId: string | null };

  let showDrawer = $state(false);
  let editing = $state<CollectionItemLite | null>(null);
  let confirmDeleteId = $state<string | null>(null);

  const rawItemsStore = collectionStore.rawItems;
  const filteredItemsStore = collectionStore.filteredItems;
  const filtersStore = collectionStore.filters;
  const availableTagsStore = collectionStore.availableTags;
  const isLoadingStore = collectionStore.isLoading;

  const rawItems = $derived($rawItemsStore);
  const filteredItems = $derived($filteredItemsStore);
  const filters = $derived($filtersStore);
  const availableTags = $derived($availableTagsStore);
  const isLoading = $derived($isLoadingStore);

  onMount(() => {
    void collectionStore.fetchCollection();
  });

  function handleStartCreate() {
    editing = null;
    showDrawer = true;
  }

  function handleEdit(item: CollectionItemLite) {
    editing = item;
    showDrawer = true;
  }

  async function handleSubmit(detail: SubmitDetail) {
    const { form, editingId } = detail;
    if (editingId) {
      await collectionStore.updateItem({ id: editingId, ...form });
    } else {
      await collectionStore.createItem(form);
    }
    showDrawer = false;
    editing = null;
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
    if (!confirmDeleteId) return;
    await collectionStore.deleteItem(confirmDeleteId);
    confirmDeleteId = null;
  }
</script>

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
      <button class="variant-filled-primary btn gap-2" onclick={handleStartCreate}>
        <Plus size={18} />
        {m.collection_add_item()}
      </button>
    </div>
  </div>

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
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each Array(6) as i (i)}
            <div
              class="h-56 animate-pulse rounded-xl bg-surface-800/80"
              aria-label={`loading-card-${i}`}
            ></div>
          {/each}
        </div>
      {:else if !isLoading && rawItems.length === 0}
        <div
          class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-surface-700/60 bg-surface-900 p-10 text-center"
        >
          <Tag class="text-surface-500" size={32} />
          <h3 class="text-lg font-semibold">{m.collection_add_first()}</h3>
          <p class="text-sm text-surface-400">{m.collection_empty_caption()}</p>
          <button class="variant-filled-primary btn" onclick={handleStartCreate}>
            {m.collection_add_item()}
          </button>
        </div>
      {:else if !isLoading && rawItems.length > 0 && filteredItems.length === 0}
        <div
          class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-warning-500/40 bg-surface-900 p-8 text-center"
        >
          <X class="text-warning-400" size={28} />
          <h3 class="text-lg font-semibold">{m.collection_no_results()}</h3>
          <button class="variant-soft-warning btn" onclick={handleClear}>
            {m.collection_clear_filters()}
          </button>
        </div>
      {:else}
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each filteredItems as item (item.id)}
            <ItemCard {item} onEdit={handleEdit} onDelete={(id) => (confirmDeleteId = id)} />
          {/each}
        </div>
      {/if}
    </section>
  </div>
</div>

<ItemDrawer
  open={showDrawer}
  {editing}
  {availableScales}
  onClose={() => (showDrawer = false)}
  onSubmit={handleSubmit}
/>

<DeleteModal
  open={Boolean(confirmDeleteId)}
  title={m.collection_delete_item()}
  message={m.collection_confirm_delete()}
  onClose={() => (confirmDeleteId = null)}
  onConfirm={handleDeleteConfirm}
/>
