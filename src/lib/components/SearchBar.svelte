<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let isExpanded = $state(false);

  function toggleSearch() {
    isExpanded = !isExpanded;
  }
</script>

<!-- Desktop: Inline Input -->
<div class="relative hidden w-64 items-center lg:flex xl:w-96">
  <Search class="text-surface-400 pointer-events-none absolute left-3" size={18} />
  <input
    type="text"
    placeholder={m.app_search_placeholder()}
    class="input border-surface-600 bg-surface-800 focus:border-primary-500 rounded-full py-2 pl-10 text-sm transition-colors"
  />
</div>

<!-- Mobile: Icon Trigger + Overlay -->
<div class="lg:hidden">
  <button class="variant-ghost-surface btn-icon" onclick={toggleSearch}>
    <Search size={20} />
  </button>

  {#if isExpanded}
    <div
      class="bg-background/95 animate-fade-in fixed inset-0 z-50 flex flex-col p-4 pt-20 backdrop-blur-sm"
    >
      <button class="btn-icon absolute top-4 right-4" onclick={toggleSearch}>
        <X size={24} />
      </button>
      <div class="relative w-full">
        <Search class="text-surface-400 absolute top-1/2 left-4 -translate-y-1/2" size={20} />
        <!-- svelte-ignore a11y_autofocus -->
        <input
          type="text"
          placeholder={m.app_search_mobile_placeholder()}
          class="input border-primary-500 bg-surface-900 rounded-xl py-4 pl-12 text-lg shadow-xl"
          autoFocus
        />
      </div>
      <div class="text-surface-400 mt-8 text-center text-sm tracking-widest uppercase">
        {m.app_search_instruction()}
      </div>
    </div>
  {/if}
</div>
