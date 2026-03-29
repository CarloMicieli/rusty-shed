<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Textarea } from '$lib/components/ui/textarea';
  import * as Select from '$lib/components/ui/select';
  import type {
    CreateTrainFormationArgs,
    FormationCategoryView,
    TrainFormationDetail
  } from '$lib/bindings.js';

  type FormArgs = CreateTrainFormationArgs;

  let {
    categories,
    initial,
    onsubmit,
    oncancel
  }: {
    categories: FormationCategoryView[];
    initial?: TrainFormationDetail | null;
    onsubmit: (args: FormArgs) => Promise<void>;
    oncancel: () => void;
  } = $props();

  let name = $state('');
  let categoryId = $state<string | null>(null);
  let epoch = $state('');
  let startYear = $state('');
  let endYear = $state('');
  let notes = $state('');
  let submitting = $state(false);

  let errors = $state<Record<string, string>>({});
  let hydratedInitialId = $state<string | null>(null);

  $effect(() => {
    const nextInitialId = initial?.id ?? null;
    if (hydratedInitialId === nextInitialId) {
      return;
    }

    hydratedInitialId = nextInitialId;
    name = initial?.name ?? '';
    categoryId = initial?.category?.id ?? null;
    epoch = initial?.epoch ?? '';
    startYear = initial?.start_year?.toString() ?? '';
    endYear = initial?.end_year?.toString() ?? '';
    notes = initial?.notes ?? '';
    errors = {};
  });

  function validate(): boolean {
    const errs: Record<string, string> = {};
    if (!name.trim()) errs.name = 'Name is required';
    const start = startYear ? parseInt(startYear) : null;
    const end = endYear ? parseInt(endYear) : null;
    if (start !== null && end !== null && start > end) {
      errs.endYear = 'End year must be after start year';
    }
    errors = errs;
    return Object.keys(errs).length === 0;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!validate()) return;
    submitting = true;
    try {
      await onsubmit({
        name: name.trim(),
        category_id: categoryId,
        epoch: epoch || null,
        start_year: startYear ? parseInt(startYear) : null,
        end_year: endYear ? parseInt(endYear) : null,
        notes: notes.trim() || null
      });
    } finally {
      submitting = false;
    }
  }

  const selectedCategoryLabel = $derived(
    categoryId ? (categories.find((c) => c.id === categoryId)?.name ?? '') : ''
  );
</script>

<form onsubmit={handleSubmit} class="space-y-4">
  <div class="space-y-1.5">
    <Label for="formation-name">{m.formations_form_name_label()}</Label>
    <Input
      id="formation-name"
      bind:value={name}
      placeholder={m.formations_form_name_placeholder()}
      required
      autocomplete="off"
    />
    {#if errors.name}
      <p class="text-xs text-destructive">{errors.name}</p>
    {/if}
  </div>

  <div class="space-y-1.5">
    <Label for="formation-category">{m.formations_form_category_label()}</Label>
    <Select.Root
      type="single"
      value={categoryId ?? ''}
      onValueChange={(v) => (categoryId = v || null)}
    >
      <Select.Trigger id="formation-category" class="w-full">
        {selectedCategoryLabel || 'None'}
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="">None</Select.Item>
        {#each categories as cat (cat.id)}
          <Select.Item value={cat.id}>{cat.name}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <div class="grid grid-cols-2 gap-4">
    <div class="space-y-1.5">
      <Label for="formation-start-year">{m.formations_form_start_year_label()}</Label>
      <Input
        id="formation-start-year"
        type="number"
        min="1800"
        max="2100"
        bind:value={startYear}
        placeholder="Year"
      />
    </div>
    <div class="space-y-1.5">
      <Label for="formation-end-year">{m.formations_form_end_year_label()}</Label>
      <Input
        id="formation-end-year"
        type="number"
        min="1800"
        max="2100"
        bind:value={endYear}
        placeholder="Year"
      />
      {#if errors.endYear}
        <p class="text-xs text-destructive">{errors.endYear}</p>
      {/if}
    </div>
  </div>

  <div class="space-y-1.5">
    <Label for="formation-epoch">{m.formations_form_epoch_label()}</Label>
    <Input id="formation-epoch" bind:value={epoch} placeholder="e.g. IV" autocomplete="off" />
  </div>

  <div class="space-y-1.5">
    <Label for="formation-notes">{m.formations_form_notes_label()}</Label>
    <Textarea
      id="formation-notes"
      bind:value={notes}
      placeholder={m.formations_form_notes_placeholder()}
      rows={3}
    />
  </div>

  <div class="flex justify-end gap-2 pt-2">
    <Button type="button" variant="outline" onclick={oncancel} disabled={submitting}>
      {m.formations_cancel()}
    </Button>
    <Button type="submit" disabled={submitting}>
      {m.formations_save()}
    </Button>
  </div>
</form>
