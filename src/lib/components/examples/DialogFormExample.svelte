<!--
  Example: Modal Dialog with Form
  Feature: 012-shadcn-migration
  
  This example demonstrates:
  - Dialog component usage
  - Form handling within dialogs
  - State management
  - Keyboard accessibility (ESC to close)
-->
<script lang="ts">
  import { Button, Input, Textarea } from '$lib/components';
  import * as Dialog from '$lib/components/ui/dialog';

  let dialogOpen = $state(false);
  let form = $state({
    title: '',
    description: ''
  });

  function handleSubmit() {
    console.log('Form submitted:', form);
    // Handle form submission

    // Close dialog and reset form
    dialogOpen = false;
    form = { title: '', description: '' };
  }

  function handleCancel() {
    dialogOpen = false;
    // Optionally reset form
    form = { title: '', description: '' };
  }
</script>

<div class="p-4">
  <Button onclick={() => (dialogOpen = true)}>Create New Item</Button>

  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Content aria-labelledby="dialog-title" aria-describedby="dialog-description">
      <div class="space-y-4 p-6">
        <div class="space-y-1">
          <h2 id="dialog-title" class="text-lg font-semibold">Create New Item</h2>
          <p id="dialog-description" class="text-surface-400 text-sm">
            Fill in the details below to create a new item.
          </p>
        </div>

        <div class="space-y-4">
          <div class="space-y-1">
            <label for="title" class="block text-sm font-medium"> Title </label>
            <Input id="title" bind:value={form.title} placeholder="Enter title" required />
          </div>

          <div class="space-y-1">
            <label for="description" class="block text-sm font-medium"> Description </label>
            <Textarea
              id="description"
              bind:value={form.description}
              placeholder="Enter description"
              rows={4}
            />
          </div>
        </div>

        <div class="flex justify-end gap-2">
          <Button variant="ghost" onclick={handleCancel}>Cancel</Button>
          <Button variant="default" onclick={handleSubmit} disabled={!form.title}>Create</Button>
        </div>
      </div>
    </Dialog.Content>
  </Dialog.Root>
</div>
