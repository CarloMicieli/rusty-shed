<script lang="ts">
  import { X } from 'lucide-svelte';
  import { Input, Textarea } from '$lib/components';
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
          <p class="text-surface-500 text-xs tracking-[0.2em] uppercase">
            {editing ? 'Edit item' : 'Add item'}
          </p>
          <h3 class="text-xl font-semibold">
            {editing ? editing.railwayModel.description : 'New item'}
          </h3>
        </div>
        <button class="variant-ghost-surface btn-icon btn btn-icon-sm" onclick={handleClose}>
          <X size={16} />
        </button>
      </div>

      <div class="space-y-4">
        <label class="block space-y-1">
          <span class="text-surface-300 text-sm">Brand</span>
          <Input bind:value={form.brand} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-surface-300 text-sm">Catalog Number</span>
          <Input bind:value={form.catalogNumber} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-surface-300 text-sm">Title</span>
          <Input bind:value={form.title} class="w-full" />
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="block space-y-1">
            <span class="text-surface-300 text-sm">Scale</span>
            <select class="input w-full bg-background" bind:value={form.scale}>
              {#each availableScales as scaleOpt (scaleOpt.id)}
                <option value={scaleOpt.id}>{scaleOpt.display}</option>
              {/each}
            </select>
          </label>
          <label class="block space-y-1">
            <span class="text-surface-300 text-sm">Power</span>
            <Input bind:value={form.powerSystem} class="w-full" />
          </label>
        </div>
        <label class="block space-y-1">
          <span class="text-surface-300 text-sm">Description</span>
          <Textarea rows={3} bind:value={form.description} class="w-full" />
        </label>
        <label class="block space-y-1">
          <span class="text-surface-300 text-sm">Tags (comma separated)</span>
          <Input
            value={form.tags.join(', ')}
            oninput={(e) => handleTagsInput((e.target as HTMLInputElement).value)}
            class="w-full"
          />
        </label>
      </div>

      <div class="mt-6 flex justify-end gap-3">
        <button class="variant-ghost-surface btn" onclick={handleClose}>Cancel</button>
        <button class="variant-filled-primary btn" onclick={handleSubmit}>
          {editing ? 'Save changes' : 'Add item'}
        </button>
      </div>
    </div>
  </div>
{/if}
