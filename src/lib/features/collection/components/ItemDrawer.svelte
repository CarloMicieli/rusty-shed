<script lang="ts">
  import { X } from 'lucide-svelte';
  // Local lightweight types matching expected shape used by this UI.
  type CollectionItemLite = {
    id: string;
    brand?: string | null;
    catalogNumber?: string | null;
    title?: string | null;
    scale?: string | null;
    powerSystem?: string | null;
    description?: string | null;
    tags?: string[] | null;
    createdAt?: string | null;
  };

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
    editing: CollectionItemLite | null;
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
          brand: editing.brand,
          catalogNumber: editing.catalogNumber,
          title: editing.title,
          scale: editing.scale,
          powerSystem: editing.powerSystem,
          description: editing.description ?? '',
          tags: editing.tags ?? []
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
      class="h-full w-full max-w-xl overflow-y-auto border-l border-surface-700/60 bg-surface-900 p-6 shadow-2xl"
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
          <p class="text-xs tracking-[0.2em] text-surface-500 uppercase">
            {editing ? 'Edit item' : 'Add item'}
          </p>
          <h3 class="text-xl font-semibold">{editing ? editing.title : 'New item'}</h3>
        </div>
        <button class="variant-ghost-surface btn-icon btn btn-icon-sm" onclick={handleClose}>
          <X size={16} />
        </button>
      </div>

      <div class="space-y-4">
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Brand</span>
          <input class="input w-full bg-surface-800" bind:value={form.brand} />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Catalog Number</span>
          <input class="input w-full bg-surface-800" bind:value={form.catalogNumber} />
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Title</span>
          <input class="input w-full bg-surface-800" bind:value={form.title} />
        </label>
        <div class="grid grid-cols-2 gap-3">
          <label class="block space-y-1">
            <span class="text-sm text-surface-300">Scale</span>
            <select class="input w-full bg-surface-800" bind:value={form.scale}>
              {#each availableScales as scaleOpt (scaleOpt.id)}
                <option value={scaleOpt.id}>{scaleOpt.display}</option>
              {/each}
            </select>
          </label>
          <label class="block space-y-1">
            <span class="text-sm text-surface-300">Power</span>
            <input class="input w-full bg-surface-800" bind:value={form.powerSystem} />
          </label>
        </div>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Description</span>
          <textarea class="input w-full bg-surface-800" rows="3" bind:value={form.description}
          ></textarea>
        </label>
        <label class="block space-y-1">
          <span class="text-sm text-surface-300">Tags (comma separated)</span>
          <input
            class="input w-full bg-surface-800"
            value={form.tags.join(', ')}
            oninput={(e) => handleTagsInput((e.target as HTMLInputElement).value)}
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
