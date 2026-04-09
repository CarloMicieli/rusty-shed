<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Accordion from '$lib/components/ui/accordion';
  import { Badge } from '$lib/components';
  import DepotTable from './DepotTable.svelte';
  import type { IconComponent } from '$lib/config/icons';

  let {
    value,
    title,
    icon: Icon,
    items,
    categoryId
  } = $props<{
    value: string;
    title: string;
    icon: IconComponent;
    items: { id: string }[];
    categoryId: string;
  }>();
</script>

{#if items.length > 0}
  <Accordion.Item {value} class="overflow-hidden rounded-xl border border-white/10 bg-white/2">
    <Accordion.Trigger
      class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
    >
      <div class="flex w-full items-center gap-4">
        <div
          class="flex h-10 w-10 items-center justify-center rounded-lg bg-white/5 text-amber-500 transition-all group-hover:bg-amber-500/20 group-hover:shadow-[0_0_15px_rgba(245,158,11,0.2)]"
        >
          <Icon size={20} />
        </div>

        <div class="flex flex-col items-start gap-0.5">
          <span class="font-mono text-[10px] tracking-widest text-zinc-500 uppercase">
            {categoryId}
          </span>
          <h3 class="text-lg font-bold tracking-tight text-white">
            {title}
          </h3>
        </div>

        <div class="ml-auto">
          <Badge
            variant="outline"
            class="border-amber-500/30 bg-amber-500/10 px-3 py-1 font-mono text-amber-500"
          >
            {items.length}
            {m.depot_units()}
          </Badge>
        </div>
      </div>
    </Accordion.Trigger>

    <Accordion.Content class="border-t border-white/5 px-0 pt-0">
      <DepotTable {items} />
    </Accordion.Content>
  </Accordion.Item>
{/if}
