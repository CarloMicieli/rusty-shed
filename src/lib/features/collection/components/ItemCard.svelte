<script lang="ts">
  /** @deprecated Use RailwayModelPreviewCard with collectionItemToCardData() instead */
  import { fade } from 'svelte/transition';
  import { Trash2 } from 'lucide-svelte';
  import { Badge, Button } from '$lib/components';
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import type { CollectionItemView } from '$lib/bindings';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  const { item, onDelete, onClick } = $props<{
    item: CollectionItemView;
    onDelete?: (id: string) => void;
    onClick?: (item: CollectionItemView) => void;
  }>();

  const primaryTag = 'default';
  const PrimaryIcon = $derived(tagIcon(primaryTag));

  function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    onDelete?.(item.id);
  }

  function handleClick() {
    onClick?.(item);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleClick();
    }
  }
</script>

<div
  class="group hover:border-accent-500/60 shadow-surface-900/40 flex h-96 w-full cursor-pointer flex-col rounded-xl border border-border/60 bg-card p-4 shadow-lg transition hover:-translate-y-1"
  in:fade
  onclick={handleClick}
  onkeydown={handleKeyDown}
  role="button"
  tabindex={0}
>
  <div
    class={`relative mb-3 h-32 overflow-hidden rounded-lg ${resolveTagMeta(primaryTag).gradient}`}
  >
    <div class="absolute inset-0 bg-gradient-to-t from-background/80 to-transparent"></div>
    <div class="absolute top-3 left-3 rounded-full bg-card/60 p-2">
      {#if PrimaryIcon}
        <PrimaryIcon size={20} class="text-accent-300" />
      {/if}
    </div>
  </div>

  <div class="flex flex-1 flex-col space-y-2">
    <div class="flex items-start justify-between gap-2">
      <div>
        <p class="text-surface-500 text-xs tracking-[0.18em] uppercase">
          {item.railwayModel.manufacturer} • {item.railwayModel.productCode}
        </p>
        <h3 class="text-lg leading-tight font-semibold">{item.railwayModel.description}</h3>
      </div>
      <div class="flex gap-2 opacity-0 transition group-hover:opacity-100">
        <Button variant="destructive" size="icon" class="h-8 w-8" onclick={handleDelete}>
          <Trash2 size={16} />
        </Button>
      </div>
    </div>

    <div class="flex flex-wrap gap-2 text-xs text-muted-foreground">
      <Badge variant="outline">{item.railwayModel.scale}</Badge>
      <Badge variant="outline">{regionalManager.formatDate(item.addedDate)}</Badge>
    </div>

    {#if item.notes}
      <p class="line-clamp-3 flex-1 text-sm text-muted-foreground">{item.notes}</p>
    {/if}
  </div>
</div>
