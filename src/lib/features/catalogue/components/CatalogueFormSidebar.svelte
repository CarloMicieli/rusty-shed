<script lang="ts">
  import { Button } from '$lib/components';
  import { resolveLabel } from '../../../../utils/resolveLabel';
  import { formLabels } from '../constants';
  import scalesData from '$lib/data/constants/scales.json';

  interface Props {
    productCode: string;
    scale: string;
    powerMethod: string;
    rollingStockCount: number;
    isSubmitting: boolean;
    onAddRollingStock: () => void;
    onSubmit: () => void;
    onCancel: () => void;
  }

  const {
    productCode,
    scale,
    powerMethod,
    rollingStockCount,
    isSubmitting,
    onAddRollingStock,
    onSubmit,
    onCancel
  }: Props = $props();
</script>

<aside class="sticky top-8 w-60 shrink-0">
  <div class="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
    <!-- Header -->
    <div class="mb-4 flex items-center gap-2">
      <div class="h-px flex-1 bg-zinc-800"></div>
      <span class="text-[10px] font-semibold tracking-widest text-zinc-500 uppercase"
        >Command Center</span
      >
      <div class="h-px flex-1 bg-zinc-800"></div>
    </div>

    <!-- Model summary -->
    <div class="mb-4 space-y-3">
      {#if productCode}
        <div>
          <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">
            {resolveLabel(formLabels.productCode)}
          </div>
          <div class="font-mono text-sm tracking-wider text-amber-400">
            {productCode}
          </div>
        </div>
      {/if}
      {#if scale}
        {@const scaleDisplay =
          scalesData.find((s) => s.id === scale)?.display ?? scale}
        <div>
          <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">
            {resolveLabel(formLabels.scale)}
          </div>
          <div class="font-mono text-sm text-zinc-200">{scaleDisplay}</div>
        </div>
      {/if}
      <div>
        <div class="mb-0.5 text-[10px] tracking-wider text-zinc-500 uppercase">
          {resolveLabel(formLabels.powerMethod)}
        </div>
        <span
          class="rounded-full border border-amber-500/30 bg-amber-500/10 px-2.5 py-0.5 text-xs font-semibold text-amber-400"
          >{powerMethod}</span
        >
      </div>
    </div>

    <div class="mb-4 h-px bg-zinc-800"></div>

    <!-- Rolling stock -->
    <div class="mb-4 space-y-2">
      <div class="flex items-center justify-between">
        <span class="text-[10px] tracking-wider text-zinc-500 uppercase"
          >{resolveLabel(formLabels.rollingStock)}</span
        >
        <span class="rounded-full bg-zinc-700 px-2 py-0.5 text-xs font-semibold text-zinc-200"
          >{rollingStockCount}</span
        >
      </div>
      <Button
        type="button"
        variant="outline"
        class="w-full border-zinc-700 text-xs hover:border-amber-500/50 hover:text-amber-400"
        onclick={onAddRollingStock}
      >
        + {resolveLabel(formLabels.addRollingStock)}
      </Button>
    </div>

    <div class="mb-4 h-px bg-zinc-800"></div>

    <!-- Actions -->
    <div class="space-y-2">
      <Button
        type="submit"
        class="w-full"
        disabled={isSubmitting}
        onclick={onSubmit}
      >
        {isSubmitting
          ? `${resolveLabel(formLabels.create)}...`
          : resolveLabel(formLabels.create)}
      </Button>
      <Button
        type="button"
        variant="ghost"
        class="w-full text-zinc-400 hover:text-zinc-200"
        onclick={onCancel}
      >
        {resolveLabel(formLabels.cancel)}
      </Button>
    </div>
  </div>
</aside>
