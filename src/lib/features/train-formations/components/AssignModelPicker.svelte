<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Skeleton } from '$lib/components/ui/skeleton';
  import { commands } from '$lib/bindings';
  import type { OwnedRollingStockView, CollectionItemView } from '$lib/bindings.js';

  let {
    open = $bindable(false),
    elementId,
    prototypeSeriesCode,
    onAssign,
    onUnassign
  }: {
    open?: boolean;
    elementId: string;
    prototypeSeriesCode: string;
    onAssign: (elementId: string, ownedRollingStockId: string) => void;
    onUnassign: (elementId: string) => void;
  } = $props();

  let matchingStocks = $state<OwnedRollingStockView[]>([]);
  let isLoading = $state(false);

  $effect(() => {
    if (open) {
      loadStocks();
    }
  });

  async function loadStocks() {
    isLoading = true;
    const result = await commands.getCollection();
    isLoading = false;
    if (result.status === 'ok') {
      const all: OwnedRollingStockView[] = result.data.items.flatMap((item: CollectionItemView) =>
        item.rollingStocks.filter(
          (rs: OwnedRollingStockView) =>
            rs.series?.toLowerCase().includes(prototypeSeriesCode.toLowerCase()) ?? false
        )
      );
      matchingStocks = all;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-sm">
    <Dialog.Header>
      <Dialog.Title>{m.formations_assign_model()}</Dialog.Title>
    </Dialog.Header>

    <div class="py-2">
      <p class="mb-3 text-xs text-muted-foreground">{prototypeSeriesCode}</p>
      {#if isLoading}
        <div class="space-y-2">
          {#each Array.from({ length: 3 }) as _, index (index)}
            <Skeleton class="h-10 w-full" />
          {/each}
        </div>
      {:else if matchingStocks.length === 0}
        <p class="py-4 text-center text-sm text-muted-foreground">No matching models found</p>
      {:else}
        <div class="space-y-1">
          {#each matchingStocks as stock (stock.id)}
            <Button
              variant="outline"
              class="h-auto w-full justify-start px-3 py-2 text-left"
              onclick={() => {
                onAssign(elementId, stock.id);
                open = false;
              }}
            >
              <div class="flex flex-col">
                <span class="text-sm">{stock.series ?? 'Unknown series'}</span>
                {#if stock.roadNumber}
                  <span class="text-xs text-muted-foreground">{stock.roadNumber}</span>
                {/if}
                {#if stock.livery}
                  <span class="text-xs text-muted-foreground">{stock.livery}</span>
                {/if}
              </div>
            </Button>
          {/each}
        </div>
      {/if}
    </div>

    <Dialog.Footer>
      <Button
        variant="ghost"
        onclick={() => {
          onUnassign(elementId);
          open = false;
        }}
      >
        {m.formations_unassign_model()}
      </Button>
      <Button variant="outline" onclick={() => (open = false)}>
        {m.formations_cancel()}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
