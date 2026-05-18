<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
  import type { LibraryTab } from '$lib/features/settings/library-types';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';
  import EntityTable from './EntityTable.svelte';
  import EntityCards from './EntityCards.svelte';

  interface Props {
    activeTab: LibraryTab;
    onTabChange: (tab: LibraryTab) => void;
    onEdit: (row: LibraryEntityRow) => void;
    onDelete: (row: LibraryEntityRow) => void;
    onMerge: (row: LibraryEntityRow) => void;
    manufacturers: LibraryEntityRow[];
    sellers: LibraryEntityRow[];
    buyers: LibraryEntityRow[];
  }

  let { activeTab, onTabChange, onEdit, onDelete, onMerge, manufacturers, sellers, buyers }: Props =
    $props();

  const visibleRows = $derived.by(() => {
    if (activeTab === 'manufacturers') return manufacturers;
    if (activeTab === 'sellers') return sellers;
    return buyers;
  });
</script>

<Tabs
  value={activeTab}
  onValueChange={(value) => onTabChange(value as LibraryTab)}
  class="space-y-4"
>
  <TabsList
    class="grid h-auto w-full grid-cols-3 rounded-sm border border-border bg-background/50 p-1"
  >
    <TabsTrigger
      value="manufacturers"
      class="rounded-sm text-xs text-muted-foreground transition-colors data-[state=active]:bg-card data-[state=active]:text-foreground"
    >
      {m.settings_library_tab_manufacturers()}
    </TabsTrigger>
    <TabsTrigger
      value="sellers"
      class="rounded-sm text-xs text-muted-foreground transition-colors data-[state=active]:bg-card data-[state=active]:text-foreground"
    >
      {m.settings_library_tab_sellers()}
    </TabsTrigger>
    <TabsTrigger
      value="buyers"
      class="rounded-sm text-xs text-muted-foreground transition-colors data-[state=active]:bg-card data-[state=active]:text-foreground"
    >
      {m.settings_library_tab_buyers()}
    </TabsTrigger>
  </TabsList>

  {#if visibleRows.length === 0}
    <p class="text-sm text-muted-foreground">{m.settings_library_empty_state()}</p>
  {:else}
    <TabsContent value={activeTab} class="mt-0">
      <!-- Desktop: compact table layout (hidden on mobile) -->
      <div class="hidden md:block" data-layout="desktop">
        <EntityTable rows={visibleRows} {onEdit} {onDelete} {onMerge} />
      </div>
      <!-- Mobile: stacked card layout (visible below md breakpoint) -->
      <div class="block md:hidden" data-layout="mobile">
        <EntityCards rows={visibleRows} {onEdit} {onDelete} {onMerge} />
      </div>
    </TabsContent>
  {/if}
</Tabs>
