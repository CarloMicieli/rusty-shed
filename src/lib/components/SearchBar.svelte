<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Input } from '$lib/components';

  let isExpanded = $state(false);

  function toggleSearch() {
    isExpanded = !isExpanded;
  }
</script>

<!-- Desktop: Inline Input -->
<div class="relative hidden w-64 items-center lg:flex xl:w-96">
  <Search class="text-surface-400 pointer-events-none absolute left-3" size={18} />
  <Input
    type="text"
    placeholder={m.app_search_placeholder()}
    class="focus:border-primary-500 rounded-full py-2 pl-10 text-sm transition-colors"
  />
</div>

<!-- Mobile: Icon Trigger + Overlay -->
<div class="lg:hidden">
  <Button variant="ghost" size="icon" onclick={toggleSearch}>
    <Search size={20} />
  </Button>

  {#if isExpanded}
    <div
      class="bg-background/95 animate-fade-in fixed inset-0 z-50 flex flex-col p-4 pt-20 backdrop-blur-sm"
    >
      <Button variant="ghost" size="icon" class="absolute top-4 right-4" onclick={toggleSearch}>
        <X size={24} />
      </Button>
      <div class="relative w-full">
        <Search class="text-surface-400 absolute top-1/2 left-4 -translate-y-1/2" size={20} />
        <Input
          type="text"
          placeholder={m.app_search_mobile_placeholder()}
          class="border-primary-500 rounded-xl py-4 pl-12 text-lg shadow-xl"
          autofocus
        />
      </div>
      <div class="text-surface-400 mt-8 text-center text-sm tracking-widest uppercase">
        {m.app_search_instruction()}
      </div>
    </div>
  {/if}
</div>
