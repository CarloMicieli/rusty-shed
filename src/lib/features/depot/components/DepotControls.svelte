<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let { searchInput = $bindable(), onClear } = $props<{
    searchInput: string;
    onClear: () => void;
  }>();
</script>

<div
  class="group relative flex items-center rounded-lg border border-white/10 bg-white/5 p-1 transition-all focus-within:border-amber-500/50 focus-within:bg-white/10"
>
  <div
    class="flex h-10 w-10 items-center justify-center text-zinc-500 group-focus-within:text-amber-500"
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
    {m.depot_search_label()}
  </div>
</div>
