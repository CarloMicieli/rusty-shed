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
    rows: LibraryEntityRow[];
    totalItems: number;
    totalPages: number;
    pageStart: number;
    pageEnd: number;
    currentPage: number;
    onPageChange: (page: number) => void;
  }

  let {
    activeTab,
    onTabChange,
    onEdit,
    onDelete,
    onMerge,
    rows,
    totalItems,
    totalPages,
    pageStart,
    pageEnd,
    currentPage,
    onPageChange
  }: Props = $props();

  const pageNumbers = $derived.by(() =>
    Array.from({ length: totalPages }, (_, index) => index + 1)
  );

  const footerHidden = $derived(totalItems === 0);
</script>

<Tabs
  value={activeTab}
  onValueChange={(value) => onTabChange(value as LibraryTab)}
  class="space-y-4"
>
  <TabsList
    class="grid h-auto w-full grid-cols-2 rounded-sm border border-border bg-background/50 p-1"
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
  </TabsList>

  {#if rows.length === 0}
    <p class="text-sm text-muted-foreground">{m.settings_library_empty_state()}</p>
  {:else}
    <TabsContent value={activeTab} class="mt-0">
      <!-- Desktop: compact table layout (hidden on mobile) -->
      <div class="hidden md:block" data-layout="desktop">
        <EntityTable {rows} {onEdit} {onDelete} {onMerge} />
      </div>
      <!-- Mobile: stacked card layout (visible below md breakpoint) -->
      <div class="block md:hidden" data-layout="mobile">
        <EntityCards {rows} {onEdit} {onDelete} {onMerge} />
      </div>

      {#if !footerHidden}
        <div
          class="mt-4 flex items-center justify-between gap-4 border-t border-border bg-card/30 px-4 py-3"
          aria-label={m.settings_library_pagination_aria()}
        >
          <p class="min-w-0 text-sm text-muted-foreground">
            {m.settings_library_pagination_showing()}
            <span class="font-mono text-foreground">{pageStart}</span>
            {m.settings_library_pagination_to()}
            <span class="font-mono text-foreground">{pageEnd}</span>
            {m.settings_library_pagination_of()}
            <span class="font-mono text-foreground">{totalItems}</span>
            {m.settings_library_pagination_entries()}
          </p>

          <div class="flex items-center gap-1.5">
            <button
              type="button"
              class="inline-flex h-8 items-center justify-center rounded-sm border border-border bg-card px-3 text-sm text-muted-foreground transition-all duration-150 ease-out hover:border-primary hover:bg-primary/10 hover:text-primary disabled:pointer-events-none disabled:opacity-50"
              aria-label={m.settings_library_pagination_previous_aria()}
              disabled={currentPage === 1}
              onclick={() => onPageChange(currentPage - 1)}
            >
              {m.settings_library_pagination_previous()}
            </button>

            <div class="flex items-center gap-1">
              {#each pageNumbers as page (page)}
                <button
                  type="button"
                  class={`inline-flex h-8 min-w-8 items-center justify-center rounded-sm border px-2 text-sm transition-all duration-150 ease-out ${
                    page === currentPage
                      ? 'border-primary bg-primary/15 text-primary'
                      : 'border-border bg-card text-muted-foreground hover:border-primary hover:bg-primary/10 hover:text-primary'
                  }`}
                  aria-label={m.settings_library_pagination_page_aria({ page })}
                  aria-current={page === currentPage ? 'page' : undefined}
                  onclick={() => onPageChange(page)}
                >
                  {page}
                </button>
              {/each}
            </div>

            <button
              type="button"
              class="inline-flex h-8 items-center justify-center rounded-sm border border-border bg-card px-3 text-sm text-muted-foreground transition-all duration-150 ease-out hover:border-primary hover:bg-primary/10 hover:text-primary disabled:pointer-events-none disabled:opacity-50"
              aria-label={m.settings_library_pagination_next_aria()}
              disabled={currentPage === totalPages}
              onclick={() => onPageChange(currentPage + 1)}
            >
              {m.settings_library_pagination_next()}
            </button>
          </div>
        </div>
      {/if}
    </TabsContent>
  {/if}
</Tabs>
