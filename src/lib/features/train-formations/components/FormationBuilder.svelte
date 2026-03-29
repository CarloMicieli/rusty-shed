<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Plus } from 'lucide-svelte';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import IdentityCard from './IdentityCard.svelte';
  import FormationTrack from './FormationTrack.svelte';
  import AddStockDrawer from './AddStockDrawer.svelte';
  import AssignModelPicker from './AssignModelPicker.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

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
</script>

<Tooltip.Provider>
  <div class="flex min-h-0 flex-1 flex-col gap-4 p-4 md:p-6">
    <!-- Header row -->
    <div class="flex items-start gap-4">
      <!-- Sticky identity card -->
      <IdentityCard state={ctx} />

      <!-- Track area -->
      <div class="flex flex-1 flex-col gap-3 overflow-hidden">
        <div class="flex items-center justify-between">
          <h3 class="text-sm font-medium text-muted-foreground">
            {m.formations_element_count({
              n: Number(ctx.detail?.elements?.length ?? 0)
            })}
          </h3>
          <Button size="sm" onclick={() => (drawerOpen = true)}>
            <Plus class="mr-1.5 size-3.5" />
            {m.formations_add_stock()}
          </Button>
        </div>

        <FormationTrack state={ctx} {formationId} onOpenPicker={openPicker} />
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
