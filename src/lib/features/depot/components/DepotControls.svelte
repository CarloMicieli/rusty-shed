<script lang="ts">
  import { Search, X, LayoutGrid, List } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let {
    searchInput = $bindable(),
    viewMode = $bindable(),
    onClear
  } = $props<{
    searchInput: string;
    viewMode: 'grid' | 'table';
    onClear: () => void;
  }>();
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-4">
  <div class="lg:col-span-3">
    <div
      class="group relative flex items-center rounded-lg border border-white/10 bg-white/5 p-1 transition-all focus-within:border-[#f59e0b]/50 focus-within:bg-white/10"
    >
      <div
        class="flex h-10 w-10 items-center justify-center text-zinc-500 group-focus-within:text-[#f59e0b]"
      >
        <Search size={18} />
      </div>
      <input
        class="h-10 flex-1 bg-transparent px-2 font-mono text-sm outline-none placeholder:text-zinc-600"
        placeholder={m.depot_search_placeholder()}
        bind:value={searchInput}
      />
      {#if searchInput}
        <button
          class="flex h-8 w-8 items-center justify-center text-zinc-400 hover:text-white"
          onclick={onClear}
        >
          <X size={16} />
        </button>
      {/if}
      <div
        class="flex h-10 items-center border-l border-white/10 px-3 font-mono text-[10px] tracking-widest text-zinc-500 uppercase"
      >
        Query_Search
      </div>
    </div>
  </div>

  <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 p-1">
    <button
      class="flex h-10 flex-1 items-center justify-center rounded-md transition-all"
      class:bg-white-5={viewMode === 'grid'}
      onclick={() => (viewMode = 'grid')}
    >
      <LayoutGrid size={18} class={viewMode === 'grid' ? 'text-[#f59e0b]' : 'text-zinc-500'} />
    </button>
    <button
      class="flex h-10 flex-1 items-center justify-center rounded-md transition-all"
      class:bg-white-10={viewMode === 'table'}
      onclick={() => (viewMode = 'table')}
    >
      <List size={18} class={viewMode === 'table' ? 'text-[#f59e0b]' : 'text-zinc-500'} />
    </button>
  </div>
</div>
