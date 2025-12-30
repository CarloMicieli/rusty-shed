<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { Tag, Trash2, PencilLine, Plus, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { availableScales, collectionStore } from '$lib/stores/collectionStore';
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import type { CollectionItemLite, CreateCollectionItemInput } from '$lib/bindings';

  const {
    rawItems,
    filteredItems,
    filters,
    availableTags,
    isLoading,
    fetchCollection,
    createItem,
    updateItem,
    deleteItem,
    setQuery,
    toggleTag,
    setScale,
    clearFilters
  } = collectionStore;

  let search = '';
  let showDrawer = false;
  let editing: CollectionItemLite | null = null;
  let confirmDeleteId: string | null = null;
  let form: CreateCollectionItemInput = {
    brand: '',
    catalogNumber: '',
    title: '',
    scale: 'H0',
    powerSystem: 'DC',
    description: '',
    tags: []
  };

  let debounceTimer: ReturnType<typeof setTimeout>;
  const debounceMs = 300;

  onMount(() => {
    void fetchCollection();
  });

  $: if (!showDrawer && editing) {
    editing = null;
  }

  function startCreate() {
    editing = null;
    form = {
      brand: '',
      catalogNumber: '',
      title: '',
      scale: 'H0',
      powerSystem: 'DC',
      description: '',
      tags: []
    };
    showDrawer = true;
  }

  function startEdit(item: CollectionItemLite) {
    editing = item;
    form = {
      brand: item.brand,
      catalogNumber: item.catalogNumber,
      title: item.title,
      scale: item.scale,
      powerSystem: item.powerSystem,
      description: item.description ?? '',
      tags: item.tags
    };
    showDrawer = true;
  }

  function onSearchChange(value: string) {
    search = value;
    setQuery(value);
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      void fetchCollection(value);
    }, debounceMs);
  }

  async function submitForm() {
    if (editing) {
      await updateItem({ id: editing.id, ...form });
    } else {
      await createItem(form);
    }
    showDrawer = false;
  }

  function resetFilters() {
    clearFilters();
    search = '';
    void fetchCollection('');
  }

  function renderTags(item: CollectionItemLite) {
    if (!item.tags || !item.tags.length) return [] as string[];
    return item.tags;
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
      <div class="flex items-center gap-2">
        <input
          class="input-md input w-64 bg-surface-900"
          placeholder={m.collection_search_placeholder()}
          value={search}
          on:input={(e) => onSearchChange((e.target as HTMLInputElement).value)}
        />
      </div>
      <button class="variant-filled-primary btn gap-2" on:click={startCreate}>
        <Plus size={18} />
        {m.collection_add_item()}
      </button>
    </div>
  </div>

  <div class="grid gap-4 lg:grid-cols-[280px,1fr]">
    <aside class="space-y-4 rounded-xl border border-surface-700/60 bg-surface-900 p-4">
      <div class="flex items-center justify-between">
        <h3 class="text-sm font-semibold tracking-wide text-surface-300 uppercase">
          {m.collection_filters_title()}
        </h3>
        <button class="text-accent-400 hover:text-accent-300 text-xs" on:click={resetFilters}>
          {m.collection_clear_filters()}
        </button>
      </div>

      <div class="space-y-2">
        <p class="text-xs font-medium tracking-wide text-surface-400 uppercase">
          {m.collection_filter_scales()}
        </p>
        <div class="flex flex-wrap gap-2">
          <button
            class="variant-soft-surface badge"
            class:variant-filled-primary={$filters.scale === null}
            on:click={() => setScale(null)}
          >
            All
          </button>
          {#each availableScales as scaleOpt (scaleOpt.id)}
            <button
              class="variant-soft-surface badge"
              class:variant-filled-primary={$filters.scale === scaleOpt.id}
              on:click={() => setScale(scaleOpt.id)}
            >
              {scaleOpt.display}
            </button>
          {/each}
        </div>
      </div>

      <div class="space-y-2">
        <p class="text-xs font-medium tracking-wide text-surface-400 uppercase">
          {m.collection_filter_tags()}
        </p>
        <div class="flex flex-wrap gap-2">
          {#each $availableTags as tag (tag)}
            {#if tag}
              <button
                class={`badge ${resolveTagMeta(tag).variant}`}
                class:variant-filled-primary={$filters.tags.has(tag)}
                on:click={() => toggleTag(tag)}
              >
                {#if tagIcon(tag)}
                  <svelte:component this={tagIcon(tag)} size={14} />
                {/if}
                <span>{resolveTagMeta(tag).label()}</span>
              </button>
            {/if}
          {/each}
        </div>
      </div>
    </aside>

    <section class="space-y-4">
      {#if $isLoading && $rawItems.length === 0}
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each Array(6) as i (i)}
            <div
              class="h-56 animate-pulse rounded-xl bg-surface-800/80"
              aria-label={`loading-card-${i}`}
            ></div>
          {/each}
        </div>
      {:else if !$isLoading && $rawItems.length === 0}
        <div
          class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-surface-700/60 bg-surface-900 p-10 text-center"
        >
          <Tag class="text-surface-500" size={32} />
          <h3 class="text-lg font-semibold">{m.collection_add_first()}</h3>
          <p class="text-sm text-surface-400">{m.collection_empty_caption()}</p>
          <button class="variant-filled-primary btn" on:click={startCreate}>
            {m.collection_add_item()}
          </button>
        </div>
      {:else if !$isLoading && $rawItems.length > 0 && $filteredItems.length === 0}
        <div
          class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-warning-500/40 bg-surface-900 p-8 text-center"
        >
          <X class="text-warning-400" size={28} />
          <h3 class="text-lg font-semibold">{m.collection_no_results()}</h3>
          <button class="variant-soft-warning btn" on:click={resetFilters}>
            {m.collection_clear_filters()}
          </button>
        </div>
      {:else}
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {#each $filteredItems as item (item.id)}
            <article
              class="group hover:border-accent-500/60 rounded-xl border border-surface-700/60 bg-surface-900 p-4 shadow-lg shadow-surface-900/40 transition hover:-translate-y-1"
              in:fade
            >
              <div
                class={`relative mb-3 h-32 overflow-hidden rounded-lg ${resolveTagMeta(renderTags(item)[0] ?? 'default').gradient}`}
              >
                <div
                  class="absolute inset-0 bg-gradient-to-t from-surface-900/80 to-transparent"
                ></div>
                <div class="absolute top-3 left-3 rounded-full bg-surface-900/60 p-2">
                  {#if tagIcon(renderTags(item)[0] ?? 'default')}
                    <svelte:component
                      this={tagIcon(renderTags(item)[0] ?? 'default')}
                      size={20}
                      class="text-accent-300"
                    />
                  {/if}
                </div>
              </div>
              <div class="space-y-2">
                <div class="flex items-start justify-between gap-2">
                  <div>
                    <p class="text-xs tracking-[0.18em] text-surface-500 uppercase">
                      {item.brand} • {item.catalogNumber}
                    </p>
                    <h3 class="text-lg leading-tight font-semibold">{item.title}</h3>
                  </div>
                  <div class="flex gap-2 opacity-0 transition group-hover:opacity-100">
                    <button
                      class="variant-soft-surface btn-icon btn btn-icon-sm"
                      on:click={() => startEdit(item)}
                    >
                      <PencilLine size={16} />
                    </button>
                    <button
                      class="variant-soft-error btn-icon btn btn-icon-sm"
                      on:click={() => (confirmDeleteId = item.id)}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>
                <div class="flex flex-wrap gap-2 text-xs text-surface-400">
                  <span class="variant-soft-surface badge">{item.scale}</span>
                  <span class="variant-soft-surface badge">{item.powerSystem}</span>
                  <span class="variant-soft-surface badge">
                    {new Date(item.createdAt).toLocaleDateString()}
                  </span>
                </div>
                {#if item.description}
                  <p class="line-clamp-2 text-sm text-surface-300">{item.description}</p>
                {/if}
                {#if renderTags(item).length}
                  <div class="flex flex-wrap gap-2">
                    {#each renderTags(item) as tag (tag)}
                      <span class={`badge ${resolveTagMeta(tag).variant}`}>
                        {#if tagIcon(tag)}
                          <svelte:component this={tagIcon(tag)} size={12} />
                        {/if}
                        {resolveTagMeta(tag).label() ?? tag}
                      </span>
                    {/each}
                  </div>
                {/if}
              </div>
            </article>
          {/each}
        </div>
      {/if}
    </section>
  </div>
</div>

{#if showDrawer}
  <div
    class="fixed inset-0 z-50 flex justify-end bg-black/40"
    role="presentation"
    tabindex="-1"
    on:click={() => (showDrawer = false)}
    on:keydown={(event) => {
      if (event.key === 'Escape') showDrawer = false;
    }}
  >
    <div
      class="h-full w-full max-w-xl overflow-y-auto border-l border-surface-700/60 bg-surface-900 p-6 shadow-2xl"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown={(event) => {
        if (event.key === 'Escape') {
          event.stopPropagation();
          showDrawer = false;
        }
      }}
    >
      <div class="mb-4 flex items-center justify-between">
        <div>
          <p class="text-xs tracking-[0.2em] text-surface-500 uppercase">
            {editing ? m.collection_edit_item() : m.collection_add_item()}
          </p>
          <h3 class="text-xl font-semibold">{editing ? editing.title : m.collection_add_item()}</h3>
        </div>
        <button
          class="variant-ghost-surface btn-icon btn btn-icon-sm"
          on:click={() => (showDrawer = false)}
        >
          <X size={16} />
        </button>
      </div>

      <div class="space-y-4">
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Brand</span>
          <input class="input w-full bg-surface-800" bind:value={form.brand} />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Catalog Number</span>
          <input class="input w-full bg-surface-800" bind:value={form.catalogNumber} />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Title</span>
          <input class="input w-full bg-surface-800" bind:value={form.title} />
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="block space-y-1">
            <span class="text-sm text-surface-300">Scale</span>
            <select class="input w-full bg-surface-800" bind:value={form.scale}>
              {#each availableScales as scaleOpt (scaleOpt.id)}
                <option value={scaleOpt.id}>{scaleOpt.display}</option>
              {/each}
            </select>
          </label>
          <label class="block space-y-1">
            <span class="text-sm text-surface-300">Power</span>
            <input class="input w-full bg-surface-800" bind:value={form.powerSystem} />
          </label>
        </div>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Description</span>
          <textarea class="input w-full bg-surface-800" rows="3" bind:value={form.description}
          ></textarea>
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Tags (comma separated)</span>
          <input
            class="input w-full bg-surface-800"
            value={form.tags.join(', ')}
            on:input={(e) =>
              (form.tags = (e.target as HTMLInputElement).value
                .split(',')
                .map((t) => t.trim())
                .filter(Boolean))}
          />
        </label>
      </div>

      <div class="mt-6 flex justify-end gap-3">
        <button class="variant-ghost-surface btn" on:click={() => (showDrawer = false)}>
          Cancel
        </button>
        <button class="variant-filled-primary btn" on:click={submitForm}>
          {editing ? m.collection_edit_item() : m.collection_add_item()}
        </button>
      </div>
    </div>
  </div>
{/if}

{#if confirmDeleteId}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
    role="presentation"
    tabindex="-1"
    on:click={() => (confirmDeleteId = null)}
    on:keydown={(event) => {
      if (event.key === 'Escape') confirmDeleteId = null;
    }}
  >
    <div
      class="w-full max-w-md rounded-xl border border-surface-700/70 bg-surface-900 p-6"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown={(event) => {
        if (event.key === 'Escape') {
          event.stopPropagation();
          confirmDeleteId = null;
        }
      }}
    >
      <h3 class="text-lg font-semibold">{m.collection_delete_item()}</h3>
      <p class="mt-2 text-sm text-surface-400">{m.collection_confirm_delete()}</p>
      <div class="mt-5 flex justify-end gap-3">
        <button class="variant-ghost-surface btn" on:click={() => (confirmDeleteId = null)}>
          Cancel
        </button>
        <button
          class="variant-filled-error btn"
          on:click={async () => {
            await deleteItem(confirmDeleteId!);
            confirmDeleteId = null;
          }}
        >
          {m.collection_delete_item()}
        </button>
      </div>
    </div>
  </div>
{/if}
