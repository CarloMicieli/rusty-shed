<script lang="ts">
  import { fade } from 'svelte/transition';
  import { PencilLine, Trash2 } from 'lucide-svelte';
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import type { CollectionItemLite } from '$lib/bindings';

  const { item, onEdit, onDelete } = $props<{
    item: CollectionItemLite;
    onEdit?: (item: CollectionItemLite) => void;
    onDelete?: (id: string) => void;
  }>();

  const primaryTag = $derived(item.tags?.[0] ?? 'default');
  const PrimaryIcon = $derived(tagIcon(primaryTag));

  function handleEdit() {
    onEdit?.(item);
  }

  function handleDelete() {
    onDelete?.(item.id);
  }
</script>

{#snippet TagBadge(tag: string)}
  {@const Icon = tagIcon(tag)}
  <span class={`badge ${resolveTagMeta(tag).variant}`}>
    {#if Icon}
      <Icon size={12} />
    {/if}
    {resolveTagMeta(tag).label() ?? tag}
  </span>
{/snippet}

<article
  class="group hover:border-accent-500/60 rounded-xl border border-surface-700/60 bg-surface-900 p-4 shadow-lg shadow-surface-900/40 transition hover:-translate-y-1"
  in:fade
>
  <div
    class={`relative mb-3 h-32 overflow-hidden rounded-lg ${resolveTagMeta(primaryTag).gradient}`}
  >
    <div class="absolute inset-0 bg-gradient-to-t from-surface-900/80 to-transparent"></div>
    <div class="absolute top-3 left-3 rounded-full bg-surface-900/60 p-2">
      {#if PrimaryIcon}
        <PrimaryIcon size={20} class="text-accent-300" />
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
        <button class="variant-soft-surface btn-icon btn btn-icon-sm" onclick={handleEdit}>
          <PencilLine size={16} />
        </button>
        <button class="variant-soft-error btn-icon btn btn-icon-sm" onclick={handleDelete}>
          <Trash2 size={16} />
        </button>
      </div>
    </div>

    <div class="flex flex-wrap gap-2 text-xs text-surface-400">
      <span class="variant-soft-surface badge">{item.scale}</span>
      <span class="variant-soft-surface badge">{item.powerSystem}</span>
      <span class="variant-soft-surface badge">{new Date(item.createdAt).toLocaleDateString()}</span
      >
    </div>

    {#if item.description}
      <p class="line-clamp-2 text-sm text-surface-300">{item.description}</p>
    {/if}

    {#if item.tags?.length}
      <div class="flex flex-wrap gap-2">
        {#each item.tags as tag (tag)}
          {@render TagBadge(tag)}
        {/each}
      </div>
    {/if}
  </div>
</article>
