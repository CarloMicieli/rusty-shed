<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components/ui/badge';
  import { Button } from '$lib/components/ui/button';
  import { Pencil, Trash2 } from 'lucide-svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import FormationForm from './FormationForm.svelte';
  import TractionWarning from './TractionWarning.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';

  let { state: ctx }: { state: TrainFormationState } = $props();

  let showEditDialog = $state(false);
  let showDeleteDialog = $state(false);

  const formation = $derived(ctx.detail);
</script>

<div
  class="sticky left-0 z-10 flex min-w-[14rem] flex-col gap-3 rounded-lg border bg-card p-4 shadow-sm"
>
  {#if formation}
    <div class="flex items-start justify-between gap-2">
      <h2 class="text-base leading-tight font-semibold">{formation.name}</h2>
      <div class="flex shrink-0 gap-1">
        <Button variant="ghost" size="icon" class="size-7" onclick={() => (showEditDialog = true)}>
          <Pencil class="size-3.5" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          class="size-7 text-destructive"
          onclick={() => (showDeleteDialog = true)}
        >
          <Trash2 class="size-3.5" />
        </Button>
      </div>
    </div>

    <div class="flex flex-wrap gap-1">
      {#if formation.category}
        <Badge variant="secondary" class="text-xs">{formation.category.name}</Badge>
      {/if}
      {#if formation.epoch}
        <Badge variant="outline" class="text-xs">{formation.epoch}</Badge>
      {/if}
      {#if formation.start_year || formation.end_year}
        <Badge variant="outline" class="text-xs">
          {formation.start_year ?? '?'} – {formation.end_year ?? '…'}
        </Badge>
      {/if}
    </div>

    {#if formation.notes}
      <p class="text-xs text-muted-foreground">{formation.notes}</p>
    {/if}

    <TractionWarning hasTraction={ctx.hasTraction} />
  {:else}
    <div class="h-24 animate-pulse rounded bg-muted"></div>
  {/if}
</div>

<!-- Edit dialog -->
<Dialog.Root bind:open={showEditDialog}>
  <Dialog.Content class="sm:max-w-lg">
    <Dialog.Header>
      <Dialog.Title>{m.formations_edit_formation()}</Dialog.Title>
    </Dialog.Header>
    {#if formation}
      <FormationForm
        categories={ctx.categories}
        initial={formation}
        onsubmit={async (args) => {
          await ctx.update(formation.id, {
            name: args.name,
            category_id: args.category_id,
            start_year: args.start_year,
            end_year: args.end_year,
            epoch: args.epoch,
            notes: args.notes
          });
          showEditDialog = false;
        }}
        oncancel={() => (showEditDialog = false)}
      />
    {/if}
  </Dialog.Content>
</Dialog.Root>

<!-- Delete confirm dialog -->
<Dialog.Root bind:open={showDeleteDialog}>
  <Dialog.Content class="sm:max-w-sm">
    <Dialog.Header>
      <Dialog.Title>{m.formations_delete_formation()}</Dialog.Title>
    </Dialog.Header>
    <p class="text-sm text-muted-foreground">
      {m.components_deleteConfirmMessage({ model: formation?.name ?? '' })}
    </p>
    <Dialog.Footer class="mt-4">
      <Button variant="outline" onclick={() => (showDeleteDialog = false)}>
        {m.formations_cancel()}
      </Button>
      <Button
        variant="destructive"
        onclick={async () => {
          if (formation) {
            await ctx.delete(formation.id);
            goto(resolve('/train-formations'));
          }
        }}
      >
        {m.formations_delete_formation()}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
