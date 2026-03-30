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
    class="flex h-24 items-center justify-center rounded-lg border-2 border-dashed
      border-muted-foreground/30 text-sm text-muted-foreground"
  >
    {m.formations_empty_composition()}
  </div>
{:else}
  <div
    class="scrollbar-thin flex gap-2 overflow-x-auto pb-2"
    use:dndzone={{ items, flipDurationMs: 200, type: 'formation-cell' }}
    onconsider={handleConsider}
    onfinalize={handleFinalize}
  >
    {#each items as element (element.id)}
      <div animate:flip={{ duration: 200 }} class="group shrink-0">
        <FormationCell
          {element}
          onRemove={handleRemove}
          {onOpenPicker}
          onTractionToggle={handleTractionToggle}
        />
      </div>
    {/each}
  </div>
{/if}
