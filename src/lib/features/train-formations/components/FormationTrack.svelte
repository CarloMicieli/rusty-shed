<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { dndzone, type DndEvent } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  import FormationCell from './FormationCell.svelte';
  import type { FormationElementView } from '$lib/bindings.js';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  let {
    state: ctx,
    formationId,
    onOpenPicker
  }: {
    state: TrainFormationState;
    formationId: string;
    onOpenPicker?: (elementId: string) => void;
  } = $props();

  // Writable derived state keeps local DnD order in sync with detail changes.
  let items = $derived.by<FormationElementView[]>(() => [...(ctx.detail?.elements ?? [])]);

  function handleConsider(e: CustomEvent<DndEvent<FormationElementView>>) {
    items = e.detail.items;
  }

  async function handleFinalize(e: CustomEvent<DndEvent<FormationElementView>>) {
    items = e.detail.items;
    await ctx.reorderElements(formationId, items);
  }

  async function handleRemove(elementId: string) {
    await ctx.removeElement(elementId);
  }

  async function handleTractionToggle(elementId: string, override: number) {
    await ctx.setTractionOverride(elementId, override);
  }
</script>

{#if items.length === 0}
  <div
    class="variant-steampunk-riveted flex h-32 items-center justify-center rounded-sm border border-border bg-card"
  >
    <span class="font-bebas text-2xl tracking-widest text-muted-foreground">
      {m.formations_no_units()}
    </span>
  </div>
{:else}
  <div class="scrollbar-thin overflow-x-auto pb-2">
    <div
      class="flex gap-2 pt-3 pr-2"
      use:dndzone={{ items, flipDurationMs: 200, type: 'formation-cell' }}
      onconsider={handleConsider}
      onfinalize={handleFinalize}
    >
      {#each items as element (element.id)}
        <div animate:flip={{ duration: 200 }} class="group shrink-0 overflow-visible">
          <FormationCell
            {element}
            onRemove={handleRemove}
            {onOpenPicker}
            onTractionToggle={handleTractionToggle}
          />
        </div>
      {/each}
    </div>
  </div>
{/if}
