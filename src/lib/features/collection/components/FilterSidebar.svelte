<script lang="ts">
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import * as m from '$lib/paraglide/messages.js';
  import type { FilterState } from '$lib/features/collection/CollectionState.svelte';
  import { Input, Badge } from '$lib/components';

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
  // eslint-disable-next-line svelte/prefer-writable-derived
  let query = $state('');

  $effect(() => {
    query = filters.query ?? '';
  });

  function handleSearch(value: string) {
    query = value;
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

<aside class="border-surface-700/60 bg-surface-900 space-y-4 rounded-xl border p-4">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold tracking-wide text-surface-300 uppercase">
      {m.collection_filters_title()}
    </h3>
    <button class="text-accent-400 hover:text-accent-300 text-xs" onclick={handleClear}>
      {m.collection_clear_filters()}
    </button>
  </div>

  <div class="space-y-2">
    <p class="text-surface-400 text-xs font-medium tracking-wide uppercase">
      {m.collection_search_placeholder()}
    </p>
    <Input
      class="input-md bg-surface-900 w-full"
      placeholder={m.collection_search_placeholder()}
      value={query}
      oninput={(e) => handleSearch((e.target as HTMLInputElement).value)}
    />
  </div>

  <div class="space-y-2">
    <p class="text-surface-400 text-xs font-medium tracking-wide uppercase">
      {m.collection_filter_scales()}
    </p>
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

  <div class="space-y-2">
    <p class="text-surface-400 text-xs font-medium tracking-wide uppercase">
      {m.collection_filter_tags()}
    </p>
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
</aside>
