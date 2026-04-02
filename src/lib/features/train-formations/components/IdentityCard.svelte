<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Settings2, Trash2, ZapOff } from 'lucide-svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';
  import FormationForm from './FormationForm.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';

  let { state: ctx }: { state: TrainFormationState } = $props();

  let showEditDialog = $state(false);
  let showDeleteDialog = $state(false);

  let isEditingName = $state(false);
  let editName = $state('');
  let isSavingName = $state(false);

  const formation = $derived(ctx.detail);

  function startEditingName() {
    if (!formation) return;
    editName = formation.name;
    isEditingName = true;
  }

  function cancelEditName() {
    isEditingName = false;
    editName = '';
  }

  async function saveName() {
    if (!formation || isSavingName) return;
    const trimmed = editName.trim();
    if (!trimmed) {
      cancelEditName();
      return;
    }
    isSavingName = true;
    try {
      await ctx.update(formation.id, {
        name: trimmed,
        category_id: formation.category?.id ?? null,
        start_year: formation.start_year ?? null,
        end_year: formation.end_year ?? null,
        epoch: formation.epoch ?? null,
        notes: formation.notes ?? null
      });
    } finally {
      isSavingName = false;
      isEditingName = false;
    }
  }

  function handleNameKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      void saveName();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancelEditName();
    }
  }

  async function saveEpoch(value: string) {
    if (!formation) return;
    await ctx.update(formation.id, {
      name: null,
      category_id: formation.category?.id ?? null,
      start_year: formation.start_year ?? null,
      end_year: formation.end_year ?? null,
      epoch: value || null,
      notes: formation.notes ?? null
    });
  }

  async function saveStartYear(value: string) {
    if (!formation) return;
    const year = value ? parseInt(value, 10) : null;
    if (year !== null && isNaN(year)) return;
    await ctx.update(formation.id, {
      name: null,
      category_id: formation.category?.id ?? null,
      start_year: year,
      end_year: formation.end_year ?? null,
      epoch: formation.epoch ?? null,
      notes: formation.notes ?? null
    });
  }

  async function saveEndYear(value: string) {
    if (!formation) return;
    const year = value ? parseInt(value, 10) : null;
    if (year !== null && isNaN(year)) return;
    await ctx.update(formation.id, {
      name: null,
      category_id: formation.category?.id ?? null,
      start_year: formation.start_year ?? null,
      end_year: year,
      epoch: formation.epoch ?? null,
      notes: formation.notes ?? null
    });
  }
</script>

{#if formation}
  <div class="flex flex-col gap-1.5">
    <!-- Title row -->
    <div class="flex items-center gap-2">
      {#if isEditingName}
        <input
          class="rounded border border-primary bg-card px-1 font-bebas text-xl leading-none tracking-wider text-foreground ring-1 ring-primary/30 transition-all duration-150 ease-out outline-none"
          bind:value={editName}
          onblur={saveName}
          onkeydown={handleNameKeydown}
          disabled={isSavingName}
          aria-label={m.formations_edit_formation()}
        />
      {:else}
        <button
          type="button"
          class="group cursor-pointer rounded border-0 bg-transparent p-0 text-left transition-all duration-150 ease-out hover:opacity-80"
          onclick={startEditingName}
          title={m.formations_edit_formation()}
        >
          <h2 class="font-bebas text-xl leading-none tracking-wider text-foreground">
            {formation.name}
          </h2>
        </button>
      {/if}

      {#if !ctx.hasTraction}
        <Tooltip.Root>
          <Tooltip.Trigger>
            <ZapOff class="text-warning size-3.5 shrink-0" />
          </Tooltip.Trigger>
          <Tooltip.Content>{m.formations_traction_warning_tooltip()}</Tooltip.Content>
        </Tooltip.Root>
      {/if}

      <button
        type="button"
        class="text-muted-foreground transition-colors hover:text-primary"
        aria-label={m.formations_edit_formation()}
        onclick={() => (showEditDialog = true)}
      >
        <Settings2 class="size-3.5" />
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

    <!-- Two-column horizontal metadata: ERA | YEARS -->
    <div class="flex gap-6 pt-0.5">
      <div class="flex flex-col gap-0.5">
        <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_meta_era()}
        </span>
        <InPlaceEdit value={formation.epoch ?? ''} placeholder="—" onSave={saveEpoch} />
      </div>
      <div class="flex flex-col gap-0.5">
        <span class="text-[10px] tracking-wider text-muted-foreground uppercase">
          {m.formations_meta_years()}
        </span>
        <div class="flex items-center gap-1 font-mono text-sm text-foreground">
          <div class="w-12">
            <InPlaceEdit
              value={String(formation.start_year ?? '')}
              placeholder="?"
              onSave={saveStartYear}
            />
          </div>
          <span class="text-muted-foreground">–</span>
          <div class="w-12">
            <InPlaceEdit
              value={String(formation.end_year ?? '')}
              placeholder="…"
              onSave={saveEndYear}
            />
          </div>
        </div>
      </div>
    </div>
  </div>
{:else}
  <div class="h-16 animate-pulse rounded bg-muted"></div>
{/if}

<!-- Settings dialog (name + category) -->
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
