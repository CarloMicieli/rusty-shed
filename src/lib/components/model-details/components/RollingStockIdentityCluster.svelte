<script lang="ts">
  import type { RollingStock } from '$lib/types/railway-model';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import { getFlag } from '$lib/utils/flags';
  import * as m from '$lib/paraglide/messages';

  interface Props {
    unit: RollingStock;
    editable: boolean;
    onSaveRoadNumber: (value: string) => Promise<void>;
  }

  const { unit, editable, onSaveRoadNumber }: Props = $props();
</script>

<div class="flex min-w-0 items-center gap-2">
  <span class="text-lg leading-none" title={unit.country_code ?? ''}>
    {getFlag(unit.country_code)}
  </span>
  {#if unit.railway_company}
    <span class="font-bebas tracking-widest text-muted-foreground uppercase">
      {unit.railway_company}
    </span>
  {/if}
  {#if editable}
    <div class="min-w-0 font-mono text-sm font-bold whitespace-nowrap text-foreground">
      <InPlaceEdit
        value={unit.road_number ?? ''}
        placeholder={m.road_number()}
        onSave={onSaveRoadNumber}
      />
    </div>
  {:else}
    <span class="font-mono text-sm font-bold whitespace-nowrap text-foreground normal-case">
      {unit.road_number ?? '—'}
    </span>
  {/if}
</div>
