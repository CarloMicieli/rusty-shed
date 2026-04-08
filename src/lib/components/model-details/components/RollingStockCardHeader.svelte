<script lang="ts">
  import { ChevronDown, Settings } from 'lucide-svelte';
  import { getFlag } from '$lib/utils/flags';
  import * as m from '$lib/paraglide/messages.js';
  import RollingStockCardHeaderShell from './RollingStockCardHeaderShell.svelte';

  interface Props {
    countryCode?: string | null;
    /** Railway / operator name (Bebas line). */
    railwayName?: string | null;
    roadNumber?: string | null;
    category?: string | null;
    subcategory?: string | null;
    isExpanded: boolean;
    isCollapsible?: boolean;
    editable?: boolean;
    onToggle: () => void;
    onEditSpecs?: () => void;
  }

  const {
    countryCode,
    railwayName,
    roadNumber,
    category,
    subcategory,
    isExpanded,
    isCollapsible = false,
    editable = false,
    onToggle,
    onEditSpecs
  }: Props = $props();

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
</script>

{#snippet collectionIdentity()}
  {#if isCollapsible}
    <button
      type="button"
      class="flex min-w-0 items-center gap-2 text-left transition-opacity hover:opacity-80"
      aria-expanded={isExpanded}
      onclick={onToggle}
    >
      <span class="text-lg leading-none" title={countryCode ?? ''}>
        {getFlag(countryCode)}
      </span>
      <span class="font-bebas tracking-widest text-muted-foreground uppercase">
        {railwayName || '—'}
      </span>
      <span class="font-mono font-bold text-foreground">
        {roadNumber || '—'}
      </span>
    </button>
  {:else}
    <div class="flex min-w-0 items-center gap-2">
      <span class="text-lg leading-none" title={countryCode ?? ''}>
        {getFlag(countryCode)}
      </span>
      <span class="font-bebas tracking-widest text-muted-foreground uppercase">
        {railwayName || '—'}
      </span>
      <span class="font-mono font-bold text-foreground">
        {roadNumber || '—'}
      </span>
    </div>
  {/if}
{/snippet}

{#snippet collectionClassification()}
  <div class="grid grid-cols-[1fr_auto_1fr] items-start px-4">
    <span
      class="text-right text-[10px] font-semibold tracking-wider text-muted-foreground/80 uppercase"
    >
      {category || '—'}
    </span>
    <span aria-hidden="true" class="px-1.5 text-[10px] leading-none text-muted-foreground/40"
      >•</span
    >
    <span
      class="text-left text-[10px] font-semibold tracking-wider text-muted-foreground/80 uppercase"
    >
      {subcategory || '—'}
    </span>
  </div>
{/snippet}

{#snippet collectionActions()}
  <div class="flex items-center justify-end gap-2">
    {#if editable}
      <button
        type="button"
        class={editSpecsClass}
        onclick={(e) => {
          e.stopPropagation();
          onEditSpecs?.();
        }}
      >
        <Settings size={12} />
        {m.rolling_stock_edit_specs_button()}
      </button>
    {/if}
    {#if isCollapsible}
      <button
        type="button"
        class="flex items-center justify-center rounded-sm border border-border p-0.5 text-muted-foreground transition-colors hover:border-primary hover:text-primary"
        aria-expanded={isExpanded}
        onclick={(e) => {
          e.stopPropagation();
          onToggle();
        }}
        aria-label={isExpanded ? 'Collapse' : 'Expand'}
      >
        <ChevronDown
          class="h-4 w-4 transition-transform duration-300 {isExpanded ? 'rotate-180' : ''}"
        />
      </button>
    {/if}
  </div>
{/snippet}

<RollingStockCardHeaderShell
  identity={collectionIdentity}
  classification={collectionClassification}
  actions={collectionActions}
/>
