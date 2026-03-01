<script lang="ts">
  import { X } from 'lucide-svelte';
  import { Button, Input, Textarea } from '$lib/components';
  import type { CollectionItemView } from '$lib/bindings';

  type CreateCollectionItemInput = {
    brand: string;
    catalogNumber: string;
    title: string;
    scale: string;
    powerSystem: string;
    description: string;
    tags: string[];
  };

  type ScaleOption = { id: string; display: string };

  const { open, editing, availableScales, onClose, onSubmit } = $props<{
    open: boolean;
    editing: CollectionItemView | null;
    availableScales: ScaleOption[];
    onClose?: () => void;
    onSubmit?: (detail: { form: CreateCollectionItemInput; editingId: string | null }) => void;
  }>();

  const blankForm: CreateCollectionItemInput = {
    brand: '',
    catalogNumber: '',
    title: '',
    scale: 'H0',
    powerSystem: 'DC',
    description: '',
    tags: []
  };

  let form = $state<CreateCollectionItemInput>({ ...blankForm });

  $effect(() => {
    if (!open) return;
    form = editing
      ? {
          brand: editing.railwayModel.manufacturer,
          catalogNumber: editing.railwayModel.productCode,
          title: editing.railwayModel.description,
          scale: editing.railwayModel.scale,
          powerSystem: 'DC', // Placeholder
          description: editing.railwayModel.description,
          tags: []
        }
      : { ...blankForm };
  });

  function handleClose() {
    onClose?.();
  }

  function handleSubmit() {
    onSubmit?.({
      form: { ...form, tags: form.tags ?? [] },
      editingId: editing?.id ?? null
    });
  }

  function handleTagsInput(value: string) {
    form.tags = value
      .split(',')
      .map((t) => t.trim())
      .filter(Boolean);
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex justify-end bg-black/40"
    role="presentation"
    tabindex="-1"
    onclick={handleClose}
    onkeydown={(event) => event.key === 'Escape' && handleClose()}
  >
    <div
      class="h-full w-full max-w-xl overflow-y-auto border-l border-border/60 bg-card p-6 shadow-2xl"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => {
        if (event.key === 'Escape') {
          event.stopPropagation();
          handleClose();
        }
      }}
    >
      <div class="mb-4 flex items-center justify-between">
        <div>
          <p class="text-xs tracking-[0.2em] text-muted-foreground uppercase">
            {editing ? 'Edit item' : 'Add item'}
          </p>
          <h3 class="text-xl font-semibold">
            {editing ? editing.railwayModel.description : 'New item'}
          </h3>
        </div>
        <Button variant="ghost" size="icon" class="h-8 w-8" onclick={handleClose}>
          <X size={16} />
        </Button>
      </div>

      <div class="space-y-4">
        <label class="block space-y-1">
          <span class="text-sm text-muted-foreground">Brand</span>
          <Input bind:value={form.brand} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-muted-foreground">Catalog Number</span>
          <Input bind:value={form.catalogNumber} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-muted-foreground">Title</span>
          <Input bind:value={form.title} class="w-full" />
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="block space-y-1">
            <span class="text-sm text-muted-foreground">Scale</span>
            <select
              class="h-9 w-full rounded-md border border-input bg-background px-3 py-2 text-sm transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/40"
              bind:value={form.scale}
            >
              {#each availableScales as scaleOpt (scaleOpt.id)}
                <option value={scaleOpt.id}>{scaleOpt.display}</option>
              {/each}
            </select>
          </label>
          <label class="block space-y-1">
            <span class="text-sm text-muted-foreground">Power</span>
            <Input bind:value={form.powerSystem} class="w-full" />
          </label>
        </div>
        <label class="block space-y-1">
          <span class="text-sm text-muted-foreground">Description</span>
          <Textarea rows={3} bind:value={form.description} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-muted-foreground">Tags (comma separated)</span>
          <Input
            value={form.tags.join(', ')}
            oninput={(e) => handleTagsInput((e.target as HTMLInputElement).value)}
            class="w-full"
          />
        </label>
      </div>

      <div class="mt-6 flex justify-end gap-3">
        <Button variant="ghost" onclick={handleClose}>Cancel</Button>
        <Button onclick={handleSubmit}>
          {editing ? 'Save changes' : 'Add item'}
        </Button>
      </div>
    </div>
  </div>
{/if}
