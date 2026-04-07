<script lang="ts">
  import type { CollectionItemView, Category } from '$lib/bindings';
  import { Badge } from '$lib/components/ui/badge';
  import DepotThumbnail from '$lib/features/depot/components/DepotThumbnail.svelte';

  interface Props {
    items: CollectionItemView[];
    onRowClick: (item: CollectionItemView) => void;
  }

  const { items, onRowClick }: Props = $props();

  function categoryLabel(category: Category | null): string {
    switch (category) {
      case 'LOCOMOTIVES':
        return 'Locomotive';
      case 'FREIGHT_CARS':
        return 'Freight Car';
      case 'PASSENGER_CARS':
        return 'Passenger Car';
      case 'TRAIN_SETS':
        return 'Train Set';
      case 'STARTER_SETS':
        return 'Starter Set';
      case 'ELECTRIC_MULTIPLE_UNITS':
        return 'EMU';
      case 'RAILCARS':
        return 'Railcar';
      default:
        return '—';
    }
  }

  function powerMethodLabel(pm: string | null): string {
    if (!pm) return '—';
    const labels: Record<string, string> = {
      ac: 'AC',
      dc: 'DC',
      dcc: 'DCC',
      trix_express: 'TX'
    };
    return labels[pm.toLowerCase()] ?? pm.toUpperCase();
  }
</script>

<div class="w-full overflow-x-auto">
  <table class="w-full border-separate border-spacing-y-1">
    <thead>
      <tr>
        <th
          class="w-20 px-2 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >IMG</th
        >
        <th
          class="px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >BRAND & MODEL</th
        >
        <th
          class="hidden min-w-[160px] px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase sm:table-cell"
          >ROAD NO.</th
        >
        <th
          class="hidden px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase md:table-cell"
          >SCALE</th
        >
        <th
          class="hidden px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase lg:table-cell"
          >ERA</th
        >
        <th
          class="hidden px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase lg:table-cell"
          >TYPE</th
        >
        <th
          class="px-4 py-2 text-left text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
          >STATUS</th
        >
      </tr>
    </thead>
    <tbody>
      {#each items as item (item.id)}
        {@const rm = item.railwayModel}
        {@const roadNumber = item.rollingStocks[0]?.roadNumber ?? '—'}
        <tr
          class="group cursor-pointer transition-all duration-200"
          role="button"
          tabindex="0"
          onclick={() => onRowClick(item)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              onRowClick(item);
            }
          }}
        >
          <!-- Thumbnail -->
          <td
            class="relative rounded-l-lg border-y border-l border-white/5 bg-white/5 px-2 py-2.5 group-hover:border-primary/30"
          >
            <!-- Left amber indicator -->
            <div
              class="absolute top-1/4 bottom-1/4 left-0 w-0.5 rounded-full bg-transparent transition-all duration-200 group-hover:bg-primary"
            ></div>
            <DepotThumbnail railwayModelId={rm.railwayModelId} productCode={rm.productCode} />
          </td>

          <!-- Brand & Model -->
          <td class="border-y border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30">
            <div class="flex flex-col gap-0.5">
              <span
                class="line-clamp-2 text-xs leading-tight font-semibold text-zinc-200 transition-colors group-hover:text-primary"
                >{rm.description}</span
              >
              <div class="flex items-center gap-1">
                <span class="text-[10px] font-bold tracking-wider text-zinc-500 uppercase"
                  >{rm.manufacturer}</span
                >
                <span class="h-1 w-1 rounded-full bg-zinc-700" aria-hidden="true"></span>
                <span class="font-mono text-[10px] text-zinc-500">{rm.productCode}</span>
              </div>
            </div>
          </td>

          <!-- Road Number -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30 sm:table-cell"
          >
            <span class="font-mono text-xs whitespace-nowrap text-zinc-300">{roadNumber}</span>
          </td>

          <!-- Scale -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30 md:table-cell"
          >
            <span class="font-mono text-xs text-zinc-400">{rm.scale ?? '—'}</span>
          </td>

          <!-- Era -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30 lg:table-cell"
          >
            <span class="font-mono text-xs text-zinc-400">{rm.epoch ?? '—'}</span>
          </td>

          <!-- Type -->
          <td
            class="hidden border-y border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30 lg:table-cell"
          >
            <span class="text-[10px] text-zinc-500">{categoryLabel(rm.category)}</span>
          </td>

          <!-- Status (Power Method) -->
          <td
            class="rounded-r-lg border-y border-r border-white/5 bg-white/5 px-4 py-2.5 group-hover:border-primary/30"
          >
            {#if rm.powerMethod}
              <Badge
                class="border-transparent bg-primary px-1.5 py-0.5 text-[10px] font-bold text-primary-foreground"
              >
                {powerMethodLabel(rm.powerMethod)}
              </Badge>
            {:else}
              <span class="font-mono text-xs text-zinc-700">—</span>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
