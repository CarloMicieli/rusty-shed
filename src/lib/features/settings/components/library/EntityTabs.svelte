<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { LibraryTab } from '$lib/features/settings/library-types';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';
  import EntityTable from './EntityTable.svelte';

  interface Props {
    activeTab: LibraryTab;
    onTabChange: (tab: LibraryTab) => void;
    onEdit: (row: LibraryEntityRow) => void;
    manufacturers: LibraryEntityRow[];
    sellers: LibraryEntityRow[];
    buyers: LibraryEntityRow[];
  }

  let { activeTab, onTabChange, onEdit, manufacturers, sellers, buyers }: Props = $props();

  const visibleRows = $derived.by(() => {
    if (activeTab === 'manufacturers') return manufacturers;
    if (activeTab === 'sellers') return sellers;
    return buyers;
  });
</script>

<div class="space-y-4">
  <div class="flex gap-2" role="tablist" aria-label={m.settings_library_title()}>
    <button
      type="button"
      role="tab"
      class="rounded-sm border px-3 py-1 text-sm"
      aria-selected={activeTab === 'manufacturers'}
      onclick={() => onTabChange('manufacturers')}
    >
      {m.settings_library_tab_manufacturers()}
    </button>
    <button
      type="button"
      role="tab"
      class="rounded-sm border px-3 py-1 text-sm"
      aria-selected={activeTab === 'sellers'}
      onclick={() => onTabChange('sellers')}
    >
      {m.settings_library_tab_sellers()}
    </button>
    <button
      type="button"
      role="tab"
      class="rounded-sm border px-3 py-1 text-sm"
      aria-selected={activeTab === 'buyers'}
      onclick={() => onTabChange('buyers')}
    >
      {m.settings_library_tab_buyers()}
    </button>
  </div>

  {#if visibleRows.length === 0}
    <p class="text-sm text-muted-foreground">{m.settings_library_empty_state()}</p>
  {:else}
    <EntityTable rows={visibleRows} onEdit={onEdit} />
  {/if}
</div>
