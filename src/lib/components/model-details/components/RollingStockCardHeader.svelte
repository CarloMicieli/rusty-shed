<script lang="ts">
  import { ChevronDown, ChevronUp, Settings } from 'lucide-svelte';
  import { getFlag } from '$lib/utils/flags';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    countryCode?: string | null;
    series?: string | null;
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
    series,
    roadNumber,
    category,
    subcategory,
    isExpanded,
    editable = false,
    onToggle,
    onEditSpecs
  }: Props = $props();
</script>

<div class="grid grid-cols-[1fr_auto_1fr] items-center border-b border-border bg-card px-4 py-2">
  <!-- Left: Identity -->
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
            {series || '—'}
          </span>
          <span class="font-mono font-bold text-foreground">
            {roadNumber || '—'}
          </span>
        </div>
      </div>
    </div>

    <!-- Toggle Indicator -->
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

  <!-- Center: Classification -->
  <div class="flex items-center justify-center px-4">
    <div
      class="flex items-center gap-1.5 font-sans text-[10px] font-semibold tracking-wider text-muted-foreground/80 uppercase"
    >
      <span>{category || '—'}</span>
      <span aria-hidden="true" class="text-[10px] leading-none text-muted-foreground/40">•</span>
      <span>{subcategory || '—'}</span>
    </div>
  </div>

  <!-- Right: Actions -->
  <div class="flex justify-end">
    {#if editable}
      <button
        type="button"
        class="variant-steampunk-lever flex items-center gap-2 rounded-sm border border-border bg-background px-3 py-1 text-[10px] font-bold tracking-widest text-muted-foreground uppercase transition-all hover:border-primary/50 hover:bg-primary/5 hover:text-primary active:scale-95"
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
</div>
