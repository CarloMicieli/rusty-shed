<script lang="ts">
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import * as m from '$lib/paraglide/messages.js';
  import type { FilterState } from '$lib/features/collection/CollectionState.svelte';
  import { Input, Badge } from '$lib/components';
  import { X } from 'lucide-svelte';

  type ScaleOption = { id: string; display: string };

  const {
    filters,
    availableTags,
    availableScales,
    onSearch,
    onSetScale,
    onToggleTag,
    onClear,
    onToggleSidebar
  } = $props<{
    filters: FilterState;
    availableTags: string[];
    availableScales: ScaleOption[];
    onSearch?: (query: string) => void;
    onSetScale?: (scale: string | null) => void;
    onToggleTag?: (tag: string) => void;
    onClear?: () => void;
    onToggleSidebar?: () => void;
  }>();

  const debounceMs = 300;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let query = $derived.by(() => filters.query ?? '');

  function handleSearch(value: string) {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      onSearch?.(value);
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

<div class="flex h-full w-full flex-col">
  <div
    class="border-surface-700/60 bg-surface-800 flex flex-shrink-0 items-center justify-between border-b p-4 sm:p-6"
  >
    <h3 class="text-surface-100 text-sm font-semibold tracking-wide uppercase">
      {m.collection_filters_title()}
    </h3>
    <div class="flex items-center gap-2">
      <button
        class="text-surface-400 hover:text-surface-300 p-1 transition-colors"
        onclick={onToggleSidebar}
        title="Close filters"
      >
        <X size={18} />
      </button>
      <button
        class="text-accent-400 hover:text-accent-300 text-xs font-medium"
        onclick={handleClear}
      >
        {m.collection_clear_filters()}
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto">
    <div class="space-y-6 p-4 sm:p-6">
      <!-- Search -->
      <div class="space-y-2">
        <label
          for="search-input"
          class="text-surface-200 block text-xs font-bold tracking-widest uppercase"
        >
          {m.collection_search_placeholder()}
        </label>
        <Input
          id="search-input"
          type="text"
          class="input-md bg-surface-800 border-surface-700 text-surface-100 placeholder-surface-500 w-full border"
          placeholder={m.collection_search_placeholder()}
          value={query}
          oninput={(e) => handleSearch((e.target as HTMLInputElement).value)}
        />
      </div>

      <!-- Scales -->
      <div class="space-y-3">
        <h4 class="text-surface-200 text-xs font-bold tracking-widest uppercase">
          {m.collection_filter_scales()}
        </h4>
        <div class="flex flex-wrap gap-2">
          <Badge
            variant={filters.scale === null ? 'default' : 'outline'}
            onclick={() => handleScaleChange(null)}
            class="hover:bg-primary-600 cursor-pointer transition-colors"
          >
            All
          </Badge>
          {#each availableScales as scaleOpt (scaleOpt.id)}
            <Badge
              variant={filters.scale === scaleOpt.id ? 'default' : 'outline'}
              onclick={() => handleScaleChange(scaleOpt.id)}
              class="hover:bg-primary-600 cursor-pointer transition-colors"
            >
              {scaleOpt.display}
            </Badge>
          {/each}
        </div>
      </div>

      <!-- Tags -->
      <div class="space-y-3">
        <h4 class="text-surface-200 text-xs font-bold tracking-widest uppercase">
          {m.collection_filter_tags()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableTags as tag (tag)}
            {#if tag}
              {@const Icon = tagIcon(tag)}
              <Badge
                variant={filters.tags.has(tag) ? 'default' : 'outline'}
                onclick={() => handleTagToggle(tag)}
                class="hover:bg-primary-600 cursor-pointer transition-colors"
              >
                {#if Icon}
                  <Icon size={14} />
                {/if}
                <span>{resolveTagMeta(tag).label()}</span>
              </Badge>
            {/if}
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
