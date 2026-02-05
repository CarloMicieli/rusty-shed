<script lang="ts">
  import { fade } from 'svelte/transition';
  import { PencilLine, Trash2 } from 'lucide-svelte';
  import { Badge } from '$lib/components';
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import type { CollectionItemView } from '$lib/bindings';

  const { item, onEdit, onDelete } = $props<{
    item: CollectionItemView;
    onEdit?: (item: CollectionItemView) => void;
    onDelete?: (id: string) => void;
  }>();

  const primaryTag = 'default';
  const PrimaryIcon = $derived(tagIcon(primaryTag));

  function handleEdit() {
    onEdit?.(item);
  }

  function handleDelete() {
    onDelete?.(item.id);
  }
</script>

<article
  class="group hover:border-accent-500/60 border-surface-700/60 bg-surface-900 shadow-surface-900/40 rounded-xl border p-4 shadow-lg transition hover:-translate-y-1"
  in:fade
>
  <div
    class={`relative mb-3 h-32 overflow-hidden rounded-lg ${resolveTagMeta(primaryTag).gradient}`}
  >
    <div class="from-surface-900/80 absolute inset-0 bg-gradient-to-t to-transparent"></div>
    <div class="bg-surface-900/60 absolute top-3 left-3 rounded-full p-2">
      {#if PrimaryIcon}
        <PrimaryIcon size={20} class="text-accent-300" />
      {/if}
    </div>
  </div>

  <div class="space-y-2">
    <div class="flex items-start justify-between gap-2">
      <div>
        <p class="text-surface-500 text-xs tracking-[0.18em] uppercase">
          {item.railwayModel.manufacturer} • {item.railwayModel.productCode}
        </p>
        <h3 class="text-lg leading-tight font-semibold">{item.railwayModel.description}</h3>
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

    <div class="text-surface-400 flex flex-wrap gap-2 text-xs">
      <Badge variant="outline">{item.railwayModel.scale}</Badge>
      <Badge variant="outline">{new Date(item.addedDate).toLocaleDateString()}</Badge>
    </div>

    {#if item.notes}
      <p class="text-surface-300 line-clamp-2 text-sm">{item.notes}</p>
    {/if}
  </div>
</article>
