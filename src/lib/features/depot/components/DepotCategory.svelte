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
  <Accordion.Item {value} class="overflow-hidden rounded-sm border border-border bg-card">
    <Accordion.Trigger
      class="group w-full px-6 py-4 transition-all duration-300 hover:no-underline"
    >
      <div class="flex w-full items-center gap-4">
        <div
          class="flex h-10 w-10 items-center justify-center rounded-sm border border-border bg-background/50 text-primary transition-all group-hover:bg-primary/10"
        >
          <Icon size={20} />
        </div>

        <div class="flex flex-col items-start gap-0.5">
          <span class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
            {categoryId}
          </span>
          <h3 class="font-bebas text-lg tracking-widest text-foreground uppercase">
            {title}
          </h3>
        </div>

        <div class="ml-auto">
          <Badge
            variant="outline"
            class="border-primary/30 bg-primary/10 px-3 py-1 font-mono text-primary"
          >
            {items.length}
            {m.depot_units()}
          </Badge>
        </div>
      </div>
    </Accordion.Trigger>

    <Accordion.Content class="border-t border-border px-0 pt-0">
      <DepotTable {items} />
    </Accordion.Content>
  </Accordion.Item>
{/if}
