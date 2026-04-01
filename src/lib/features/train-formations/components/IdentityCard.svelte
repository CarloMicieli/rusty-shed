<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Pencil, Trash2 } from 'lucide-svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import FormationForm from './FormationForm.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';

  let { state: ctx }: { state: TrainFormationState } = $props();

  let showEditDialog = $state(false);
  let showDeleteDialog = $state(false);

  const formation = $derived(ctx.detail);

  async function saveNotes(value: string) {
    if (!formation) return;
    await ctx.update(formation.id, {
      name: null,
      category_id: null,
      start_year: null,
      end_year: null,
      epoch: null,
      notes: value
    });
  }
</script>

{#if formation}
  <div class="flex flex-col gap-1.5">
    <!-- Title row -->
    <div class="flex items-center gap-2">
      <h2 class="font-bebas text-xl leading-none tracking-wider text-foreground">
        {formation.name}
      </h2>
      <button
        type="button"
        class="text-muted-foreground transition-colors hover:text-primary"
        aria-label={m.formations_edit_formation()}
        onclick={() => (showEditDialog = true)}
      >
        <Pencil class="size-3.5" />
      </button>
      <button
        type="button"
        class="text-muted-foreground transition-colors hover:text-destructive"
        aria-label={m.formations_delete_formation()}
        onclick={() => (showDeleteDialog = true)}
      >
        <Trash2 class="size-3.5" />
      </button>
    </div>

    <!-- Inline notes editor -->
    <InPlaceEdit
      value={formation.notes ?? ''}
      placeholder={m.formations_notes_placeholder()}
      multiline={false}
      onSave={saveNotes}
    />

    <!-- Three-column horizontal metadata -->
    <div class="flex gap-6 pt-0.5">
      <div class="flex flex-col gap-0.5">
        <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_meta_era()}
        </span>
        <span class="font-mono text-sm text-foreground">{formation.epoch ?? '—'}</span>
      </div>
      <div class="flex flex-col gap-0.5">
        <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_meta_traction()}
        </span>
        <span class="font-mono text-sm text-foreground">
          {ctx.hasTraction ? m.formations_meta_traction_yes() : m.formations_meta_traction_no()}
        </span>
      </div>
      <div class="flex flex-col gap-0.5">
        <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_meta_years()}
        </span>
        <span class="font-mono text-sm text-foreground">
          {formation.start_year ?? '?'} – {formation.end_year ?? '…'}
        </span>
      </div>
    </div>
  </div>
{:else}
  <div class="h-16 animate-pulse rounded bg-muted"></div>
{/if}

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
