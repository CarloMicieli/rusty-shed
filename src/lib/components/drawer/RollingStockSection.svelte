<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { DrawerSectionBar } from '$lib/components/drawer';
  import type { RailwayCompany } from '$lib/bindings';
  import type { RollingStockFormEntry } from '$lib/features/wishlists/types';
  import RollingStockEntry from '$lib/features/wishlists/components/RollingStockEntry.svelte';

  interface Props {
    entries: RollingStockFormEntry[];
    railwayCompanies: RailwayCompany[];
    errors?: Record<string, { railwayCompanyId?: string; seriesCode?: string }>;
    onadd?: () => void;
    onremove?: (id: string) => void;
    disabled?: boolean;
    expanded?: boolean;
  }

  let {
    entries = $bindable(),
    railwayCompanies,
    errors: _errors = {},
    onadd,
    onremove,
    disabled = false,
    expanded = $bindable(true)
  }: Props = $props();
</script>

<div class="space-y-3">
  <DrawerSectionBar
    label={m.drawer_section_rolling_stocks()}
    {expanded}
    onToggle={() => (expanded = !expanded)}
  />

  {#if expanded}
    {#if entries.length === 0}
      <div class="rounded-lg border border-dashed border-white/10 p-4">
        <p class="text-sm text-zinc-500">No rolling stocks added yet.</p>
      </div>
    {:else}
      {#each entries as entry, i (entry.id)}
        <RollingStockEntry
          bind:entry={entries[i]}
          {railwayCompanies}
          canRemove={entries.length > 0}
          onRemove={() => onremove?.(entry.id)}
        />
      {/each}
    {/if}

    <Button
      type="button"
      variant="ghost"
      size="sm"
      class="w-full border border-amber-500/30 text-amber-500 hover:bg-amber-500/10"
      onclick={() => onadd?.()}
      {disabled}
    >
      + {m.add_model_add_rolling_stock()}
    </Button>
  {/if}
</div>
