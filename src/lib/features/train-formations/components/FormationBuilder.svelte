<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Plus, ChevronLeft, ChevronDown } from 'lucide-svelte';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import IdentityCard from './IdentityCard.svelte';
  import FormationTrack from './FormationTrack.svelte';
  import AddStockDrawer from './AddStockDrawer.svelte';
  import AssignModelPicker from './AssignModelPicker.svelte';
  import RichTextEditor from '$lib/components/RichTextEditor.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';
  import { resolve } from '$app/paths';

  let {
    state: ctx,
    formationId
  }: {
    state: TrainFormationState;
    formationId: string;
  } = $props();

  let drawerOpen = $state(false);
  let notesExpanded = $state(false);

  let pickerOpen = $state(false);
  let activeElementId = $state('');
  let activeSeriesCode = $state('');

  function openPicker(elementId: string) {
    const element = ctx.detail?.elements.find((e) => e.id === elementId);
    if (!element) return;
    activeElementId = elementId;
    activeSeriesCode = element.prototype.series_code;
    pickerOpen = true;
  }

  const elementCount = $derived(ctx.detail?.elements?.length ?? 0);

  async function saveNotes(value: string) {
    const f = ctx.detail;
    if (!f) return;
    await ctx.update(f.id, {
      name: null,
      category_id: f.category?.id ?? null,
      start_year: f.start_year ?? null,
      end_year: f.end_year ?? null,
      epoch: f.epoch ?? null,
      notes: value
    });
  }
</script>

<Tooltip.Provider>
  <div class="flex flex-col gap-4">
    <!-- ① Control Strip -->
    <header
      class="variant-steampunk-riveted flex items-center gap-3 rounded-sm border border-border bg-card px-4 py-3"
    >
      <!-- Back button: circular icon-only -->
      <a
        href={resolve('/train-formations')}
        class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-white/5 bg-zinc-900/50 text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-white"
        aria-label={m.formations_back()}
      >
        <ChevronLeft size={20} />
      </a>

      <!-- Vertical divider -->
      <div class="h-8 w-px shrink-0 bg-border"></div>

      <!-- Identity section -->
      <div class="min-w-0 flex-1">
        <IdentityCard state={ctx} />
      </div>

      <!-- Vertical divider -->
      <div class="h-8 w-px shrink-0 bg-border"></div>

      <!-- Right: unit count + Add Stock -->
      <div class="flex shrink-0 items-center gap-4">
        <div class="flex flex-col items-end gap-0.5">
          <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
            {m.formations_meta_units()}
          </span>
          <span class="font-mono text-sm text-foreground">{elementCount}</span>
        </div>
        <Button
          size="sm"
          class="variant-steampunk-lever bg-primary text-primary-foreground hover:bg-primary/90"
          onclick={() => (drawerOpen = true)}
        >
          <Plus class="mr-1.5 size-3.5" />
          {m.formations_add_stock()}
        </Button>
      </div>
    </header>

    <!-- ② Main Yard -->
    <div class="rounded-sm border border-border bg-background/50 p-3">
      <FormationTrack state={ctx} {formationId} onOpenPicker={openPicker} />
    </div>

    <!-- ③ Logbook Notes (Accordion) -->
    <div class="rounded-sm border border-border bg-card">
      <!-- Accordion header — click to expand/collapse -->
      <button
        type="button"
        class="flex w-full items-center justify-between px-4 py-2.5 text-left transition-colors hover:bg-white/5"
        onclick={() => (notesExpanded = !notesExpanded)}
      >
        <span class="font-mono text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_form_notes_label()}
        </span>
        <ChevronDown
          size={14}
          class="text-muted-foreground transition-transform duration-300 {notesExpanded
            ? 'rotate-180'
            : ''}"
        />
      </button>

      <!-- Separator -->
      <div class="h-px bg-border/40"></div>

      <!-- Sliding panel: max-height animates between preview and full -->
      <div
        class="relative overflow-hidden transition-[max-height] duration-300 ease-in-out"
        style="max-height: {notesExpanded ? '600px' : '5.5rem'}"
      >
        <div class="px-4 py-3">
          <RichTextEditor
            value={ctx.detail?.notes ?? null}
            editable={true}
            placeholder={m.formations_notes_placeholder()}
            onSave={saveNotes}
          />
        </div>

        <!-- Collapsed overlay: gradient fade + Read More button -->
        {#if !notesExpanded && ctx.detail?.notes}
          <div
            class="pointer-events-none absolute right-0 bottom-0 left-0 flex items-end justify-end bg-gradient-to-t from-card/95 to-transparent px-4 pt-8 pb-2"
          >
            <button
              type="button"
              class="pointer-events-auto font-mono text-[9px] tracking-widest text-muted-foreground uppercase transition-colors hover:text-foreground"
              onclick={(e) => {
                e.stopPropagation();
                notesExpanded = true;
              }}
            >
              {m.formations_notes_read_more()}
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <AddStockDrawer state={ctx} {formationId} bind:open={drawerOpen} />

  <AssignModelPicker
    bind:open={pickerOpen}
    elementId={activeElementId}
    prototypeSeriesCode={activeSeriesCode}
    onAssign={async (elementId, ownedRollingStockId) => {
      await ctx.assignRollingStock(elementId, { owned_rolling_stock_id: ownedRollingStockId });
    }}
    onUnassign={async (elementId) => {
      await ctx.assignRollingStock(elementId, { owned_rolling_stock_id: null });
    }}
  />
</Tooltip.Provider>
