<script lang="ts">
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import * as m from '$lib/paraglide/messages.js';
  import type { FilterState } from '$lib/stores/collectionStore';

  type ScaleOption = { id: string; display: string };

  const { filters, availableTags, availableScales, onSearch, onSetScale, onToggleTag, onClear } =
    $props<{
      filters: FilterState;
      availableTags: string[];
      availableScales: ScaleOption[];
      onSearch?: (query: string) => void;
      onSetScale?: (scale: string | null) => void;
      onToggleTag?: (tag: string) => void;
      onClear?: () => void;
    }>();

  const debounceMs = 300;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let query = $state('');

  $effect(() => {
    query = filters.query;
  });

  function handleSearch(value: string) {
    query = value;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      onSearch?.(query);
    }, debounceMs);
  }

  function handleScaleChange(scale: string | null) {
    onSetScale?.(scale);
  }

  function handleTagToggle(tag: string) {
    onToggleTag?.(tag);
  }

  function handleClear() {
    query = '';
    onClear?.();
  }
</script>

<aside class="space-y-4 rounded-xl border border-surface-700/60 bg-surface-900 p-4">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold tracking-wide text-surface-300 uppercase">
      {m.collection_filters_title()}
    </h3>
    <button class="text-accent-400 hover:text-accent-300 text-xs" onclick={handleClear}>
      {m.collection_clear_filters()}
    </button>
  </div>

  <div class="space-y-2">
    <p class="text-xs font-medium tracking-wide text-surface-400 uppercase">
      {m.collection_search_placeholder()}
    </p>
    <input
      class="input-md input w-full bg-surface-900"
      placeholder={m.collection_search_placeholder()}
      bind:value={query}
      oninput={(e) => handleSearch((e.target as HTMLInputElement).value)}
    />
  </div>

  <div class="space-y-2">
    <p class="text-xs font-medium tracking-wide text-surface-400 uppercase">
      {m.collection_filter_scales()}
    </p>
    <div class="flex flex-wrap gap-2">
      <button
        class="variant-soft-surface badge"
        class:variant-filled-primary={filters.scale === null}
        onclick={() => handleScaleChange(null)}
      >
        All
      </button>
      {#each availableScales as scaleOpt (scaleOpt.id)}
        <button
          class="variant-soft-surface badge"
          class:variant-filled-primary={filters.scale === scaleOpt.id}
          onclick={() => handleScaleChange(scaleOpt.id)}
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
      {#each availableTags as tag (tag)}
        {#if tag}
          {@const Icon = tagIcon(tag)}
          <button
            class={`badge ${resolveTagMeta(tag).variant}`}
            class:variant-filled-primary={filters.tags.has(tag)}
            onclick={() => handleTagToggle(tag)}
          >
            {#if Icon}
              <Icon size={14} />
            {/if}
            <span>{resolveTagMeta(tag).label()}</span>
          </button>
        {/if}
      {/each}
    </div>
  </div>
</aside>
