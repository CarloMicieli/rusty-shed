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
  <Search class="pointer-events-none absolute left-3 text-muted-foreground" size={18} />
  <Input
    type="text"
    placeholder={m.app_search_placeholder()}
    class="focus:border-primary-500 rounded-full border-border bg-background py-2 pl-10 text-sm transition-colors"
  />
</div>

<!-- Mobile: Icon Trigger + Overlay -->
<div class="lg:hidden">
  <Button variant="ghost" size="icon" onclick={toggleSearch}>
    <Search size={20} />
  </Button>

  {#if isExpanded}
    <div
      class="animate-fade-in fixed inset-0 z-50 flex flex-col bg-background/95 p-4 pt-20 backdrop-blur-sm"
    >
      <Button variant="ghost" size="icon" class="absolute top-4 right-4" onclick={toggleSearch}>
        <X size={24} />
      </Button>
      <div class="relative w-full">
        <Search class="absolute top-1/2 left-4 -translate-y-1/2 text-muted-foreground" size={20} />

        <Input
          type="text"
          placeholder={m.app_search_mobile_placeholder()}
          class="border-primary-500 rounded-xl bg-card py-4 pl-12 text-lg shadow-xl"
          autoFocus
        />
      </div>
      <div class="mt-8 text-center text-sm tracking-widest text-muted-foreground uppercase">
        {m.app_search_instruction()}
      </div>
    </div>
  {/if}
</div>
