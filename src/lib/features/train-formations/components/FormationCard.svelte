<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Trash2, Combine, Zap } from 'lucide-svelte';
  import type { TrainFormationSummary } from '$lib/bindings.js';

  let {
    summary,
    onclick,
    ondelete
  }: {
    summary: TrainFormationSummary;
    onclick?: () => void;
    ondelete?: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<article
  class="variant-steampunk-riveted relative overflow-hidden rounded-sm border border-border bg-card transition-colors hover:border-primary/50"
  class:cursor-pointer={!!onclick}
  {onclick}
  role={onclick ? 'button' : undefined}
  tabindex={onclick ? 0 : undefined}
  onkeydown={onclick
    ? (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onclick();
        }
      }
    : undefined}
>
  <!-- Header -->
  <div class="flex items-start justify-between gap-2 px-3 pt-3 pb-2">
    <h3 class="font-bebas text-lg leading-tight text-foreground">{summary.name}</h3>
    {#if ondelete}
      <button
        type="button"
        class="variant-steampunk-lever shrink-0 rounded-sm border border-border/50 p-1"
        onclick={(e) => {
          e.stopPropagation();
          ondelete();
        }}
        aria-label="Delete formation"
      >
        <Trash2 class="size-3.5 text-muted-foreground" />
      </button>
    {/if}
  </div>

  <!-- Visual section -->
  <div class="relative flex h-20 items-center justify-center border-b border-border bg-muted/20">
    {#if Number(summary.element_count) === 0}
      <div class="flex flex-col items-center gap-1 text-muted-foreground">
        <Combine class="size-8 stroke-2" />
        <span class="font-mono text-[10px] tracking-wider uppercase">{m.formations_no_units()}</span
        >
      </div>
    {:else}
      <div class="flex items-center gap-1">
        {#each Array.from({ length: Math.min(Number(summary.element_count), 7) }) as _, i (i)}
          <div class="h-7 w-9 rounded-sm border border-border/60 bg-card/80"></div>
        {/each}
        {#if Number(summary.element_count) > 7}
          <span class="font-mono text-[9px] text-muted-foreground"
            >+{Number(summary.element_count) - 7}</span
          >
        {/if}
      </div>
    {/if}

    {#if summary.category}
      <span
        class="absolute top-2 right-2 rounded-full border border-primary/30 bg-primary/20 px-2 py-0.5 font-mono text-[9px] tracking-wider text-primary uppercase"
      >
        {summary.category.name}
      </span>
    {/if}

    {#if summary.has_traction}
      <Zap class="absolute bottom-2 left-2 size-3 text-yellow-500" aria-label="Has traction" />
    {/if}
  </div>

  <!-- Three-column footer -->
  <div class="grid grid-cols-3 divide-x divide-border py-2">
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase">ERA</span
      >
      <span class="font-mono text-sm text-foreground">
        {summary.epoch ? `Ep. ${summary.epoch}` : '—'}
      </span>
    </div>
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
        >UNITS</span
      >
      <span class="font-mono text-sm text-foreground">{String(summary.element_count)}</span>
    </div>
    <div class="flex flex-col items-center gap-0.5 px-2">
      <span class="text-[10px] font-medium tracking-wider text-muted-foreground uppercase"
        >LENGTH</span
      >
      <span class="font-mono text-sm text-foreground">—</span>
    </div>
  </div>
</article>
