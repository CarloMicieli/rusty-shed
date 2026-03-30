<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Plus, ChevronLeft, Zap } from 'lucide-svelte';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import IdentityCard from './IdentityCard.svelte';
  import FormationTrack from './FormationTrack.svelte';
  import AddStockDrawer from './AddStockDrawer.svelte';
  import AssignModelPicker from './AssignModelPicker.svelte';
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
</script>

<Tooltip.Provider>
  <div class="grid min-h-0 flex-1 grid-cols-[15rem_1fr_13rem] gap-4 p-4 md:p-6">
    <!-- Col 1: Sidebar (Train Info) -->
    <aside class="flex flex-col gap-3 rounded-sm border border-border bg-card p-4">
      <!-- Back button -->
      <a
        href={resolve('/train-formations')}
        class="variant-steampunk-lever flex items-center gap-1.5 self-start rounded-sm border-2 border-copper px-2 py-1 text-[11px] tracking-wider text-muted-foreground uppercase transition-colors hover:text-foreground"
        aria-label={m.formations_back()}
      >
        <ChevronLeft class="size-3" />
        {m.formations_back()}
      </a>

      <div class="h-px bg-border"></div>

      <!-- Formation identity content -->
      <IdentityCard state={ctx} />
    </aside>

    <!-- Col 2: Main Content (Formation Grid) -->
    <div class="flex min-w-0 flex-col gap-3">
      <div class="flex items-center gap-2">
        <span class="text-[10px] tracking-tighter text-muted-foreground uppercase">
          {m.formations_element_count({ n: Number(elementCount) })}
        </span>
      </div>

      <FormationTrack state={ctx} {formationId} onOpenPicker={openPicker} />
    </div>

    <!-- Col 3: Command Center (Stats / Actions) -->
    <aside class="flex flex-col gap-4 rounded-sm border border-border bg-card p-4">
      <!-- Stats section -->
      <div class="flex flex-col gap-2">
        <span class="text-[10px] tracking-tighter text-muted-foreground uppercase">
          {m.formations_element_count({ n: Number(elementCount) })}
        </span>

        <div class="flex flex-col gap-1">
          <div class="flex items-center justify-between">
            <span class="text-[10px] tracking-tighter text-muted-foreground uppercase">Units</span>
            <span class="font-mono text-sm text-foreground">{elementCount}</span>
          </div>

          {#if ctx.hasTraction}
            <div class="flex items-center gap-1 text-[10px] text-primary">
              <Zap class="size-3" />
              <span class="tracking-tighter uppercase">Traction</span>
            </div>
          {:else if elementCount > 0}
            <div class="text-[10px] tracking-tighter text-muted-foreground/60 uppercase">
              No traction
            </div>
          {/if}
        </div>
      </div>

      <div class="h-px bg-border"></div>

      <!-- Add Stock action -->
      <Button size="sm" class="variant-steampunk-lever w-full" onclick={() => (drawerOpen = true)}>
        <Plus class="mr-1.5 size-3.5" />
        {m.formations_add_stock()}
      </Button>
    </aside>
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
