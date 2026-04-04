<script lang="ts">
  import { ChevronDown, ChevronUp, Settings } from 'lucide-svelte';
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
    editable = false,
    onToggle,
    onEditSpecs
  }: Props = $props();

  const editSpecsClass =
    'variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95';
</script>

{#snippet collectionIdentity()}
  <button
    type="button"
    class="group flex min-w-0 items-center gap-3 text-left transition-opacity hover:opacity-80"
    aria-expanded={isExpanded}
    onclick={onToggle}
  >
    <div class="flex items-center gap-2">
      <span class="text-lg leading-none" title={countryCode ?? ''}>
        {getFlag(countryCode)}
      </span>
      <div class="flex flex-col">
        <div class="flex items-center gap-2">
          <span class="font-bebas tracking-widest text-muted-foreground uppercase">
            {railwayName || '—'}
          </span>
          <span class="font-mono font-bold text-foreground">
            {roadNumber || '—'}
          </span>
        </div>
      </div>
    </div>

    <div
      class="flex h-5 w-5 items-center justify-center rounded-full border border-layout-border text-muted-foreground transition-colors group-hover:border-primary group-hover:text-primary"
    >
      {#if isExpanded}
        <ChevronUp class="h-3 w-3" />
      {:else}
        <ChevronDown class="h-3 w-3" />
      {/if}
    </div>
  </button>
{/snippet}

{#snippet collectionClassification()}
  <div class="flex items-center justify-center px-4">
    <div
      class="flex items-center gap-1.5 font-sans text-[10px] font-semibold tracking-wider text-muted-foreground/80 uppercase"
    >
      <span>{category || '—'}</span>
      <span aria-hidden="true" class="text-[10px] leading-none text-muted-foreground/40">•</span>
      <span>{subcategory || '—'}</span>
    </div>
  </div>
{/snippet}

{#snippet collectionActions()}
  <div class="flex justify-end">
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
  </div>
{/snippet}

<RollingStockCardHeaderShell
  identity={collectionIdentity}
  classification={collectionClassification}
  actions={collectionActions}
/>
