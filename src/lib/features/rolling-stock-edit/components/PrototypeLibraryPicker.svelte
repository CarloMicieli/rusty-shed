<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { Search, SearchX, X, BookOpen, Check } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import * as Popover from '$lib/components/ui/popover';
  import { commands, type PrototypeGroupView, type PrototypeView } from '$lib/bindings';

  interface Props {
    /** Filters the list to only show prototypes matching this specification_type. */
    category: string;
    /** The currently selected prototype id (controlled). */
    selectedId: string;
    /** Called when the user picks a prototype from the list. */
    onSelect: (p: PrototypeView) => void;
    /** Called when the user clears the current selection. */
    onClear: () => void;
  }

  let { category, selectedId, onSelect, onClear }: Props = $props();

  let groups = $state<PrototypeGroupView[]>([]);
  let query = $state('');
  let open = $state(false);
  let searchInputEl = $state<HTMLInputElement | null>(null);
  let triggerRef = $state<HTMLButtonElement | null>(null);
  let contentWidth = $state(280);

  onMount(async () => {
    const result = await commands.getPrototypes(null);
    if (result.status === 'ok') {
      groups = result.data;
    }
  });

  // Measure trigger width so popover matches it
  $effect(() => {
    if (triggerRef) {
      contentWidth = triggerRef.offsetWidth;
    }
  });

  // Focus search input and clear query when popover opens
  $effect(() => {
    if (open) {
      tick().then(() => searchInputEl?.focus());
    } else {
      query = '';
    }
  });

  const allPrototypes: PrototypeView[] = $derived(groups.flatMap((g) => g.prototypes));

  const filtered: PrototypeView[] = $derived(
    allPrototypes
      .filter((p) => !category || p.specification_type === category)
      .filter(
        (p) =>
          !query ||
          p.series_code.toLowerCase().includes(query.toLowerCase()) ||
          (p.friendly_name ?? '').toLowerCase().includes(query.toLowerCase())
      )
  );

  const selectedPrototype: PrototypeView | undefined = $derived(
    allPrototypes.find((p) => p.id === selectedId)
  );

  function handleSelect(p: PrototypeView) {
    onSelect(p);
    open = false;
  }
</script>

{#snippet prototypeItem(p: PrototypeView)}
  <button
    type="button"
    onclick={() => handleSelect(p)}
    class="group flex w-full items-start gap-2 rounded-sm px-3 py-2 text-left transition-all duration-150
      hover:bg-primary/10
      {selectedId === p.id ? 'border-l-2 border-primary bg-primary/15 pl-[10px]' : ''}"
  >
    <span class="mt-0.5 w-[13px] shrink-0">
      {#if selectedId === p.id}
        <Check size={13} class="text-primary" />
      {/if}
    </span>
    <span class="min-w-0 flex-1">
      <span class="flex items-baseline gap-2">
        <span class="font-mono text-sm text-foreground">{p.series_code}</span>
        {#if p.friendly_name}
          <span class="truncate text-xs text-muted-foreground">{p.friendly_name}</span>
        {/if}
      </span>
      <span class="text-[10px] tracking-tighter text-muted-foreground uppercase">
        {p.company_name}
      </span>
    </span>
  </button>
{/snippet}

<div class="rounded-sm border border-border bg-card p-2">
  <p class="mb-1.5 text-[10px] tracking-tighter text-muted-foreground uppercase">
    {m.rolling_stock_create_section_prototype()} Library
  </p>

  <div class="flex items-center gap-1">
    <Popover.Root bind:open>
      <Popover.Trigger
        bind:ref={triggerRef}
        class="variant-steampunk-lever flex h-9 min-w-0 flex-1 items-center gap-2 rounded-sm border border-border bg-background px-3 text-left text-sm transition-all duration-150
          hover:border-primary/50 focus-visible:ring-1 focus-visible:ring-primary focus-visible:outline-none
          {open ? 'border-primary/50' : ''}"
      >
        <BookOpen size={14} class="shrink-0 text-muted-foreground" />
        {#if selectedPrototype}
          <span class="flex min-w-0 flex-1 items-baseline gap-2 overflow-hidden">
            <span class="font-mono text-xs text-foreground">{selectedPrototype.series_code}</span>
            {#if selectedPrototype.friendly_name}
              <span class="truncate text-xs text-muted-foreground"
                >{selectedPrototype.friendly_name}</span
              >
            {/if}
          </span>
        {:else}
          <span class="flex-1 text-muted-foreground">
            {m.rolling_stock_prototype_search_placeholder()}
          </span>
        {/if}
      </Popover.Trigger>

      <Popover.Content
        align="start"
        sideOffset={4}
        style="width: {contentWidth}px;"
        class="rounded-sm border border-border bg-card p-0 shadow-lg"
      >
        <!-- Search input -->
        <div class="flex items-center border-b border-border">
          <Search size={13} class="ml-3 shrink-0 text-muted-foreground" />
          <input
            bind:this={searchInputEl}
            type="text"
            bind:value={query}
            placeholder={m.rolling_stock_prototype_search_placeholder()}
            class="h-9 flex-1 bg-transparent px-2 font-mono text-sm text-foreground placeholder:text-muted-foreground focus:outline-none"
            onkeydown={(e) => {
              if (e.key === 'Escape') open = false;
            }}
          />
          {#if query}
            <button
              type="button"
              onclick={() => (query = '')}
              class="mr-2 text-muted-foreground hover:text-foreground"
            >
              <X size={13} />
            </button>
          {/if}
        </div>

        <!-- Results list -->
        <div class="max-h-48 overflow-y-auto py-1">
          {#if filtered.length === 0}
            <div class="flex flex-col items-center gap-1.5 py-6">
              <SearchX size={20} class="text-muted-foreground" />
              <p class="text-xs text-muted-foreground">{m.rolling_stock_prototype_no_results()}</p>
            </div>
          {:else}
            {#each filtered as p (p.id)}
              {@render prototypeItem(p)}
            {/each}
          {/if}
        </div>
      </Popover.Content>
    </Popover.Root>

    {#if selectedId}
      <button
        type="button"
        onclick={onClear}
        title={m.rolling_stock_prototype_clear()}
        class="flex h-9 w-9 shrink-0 items-center justify-center rounded-sm border border-border bg-background text-muted-foreground transition-all duration-150 hover:border-destructive/50 hover:text-destructive"
      >
        <X size={14} />
      </button>
    {/if}
  </div>
</div>
