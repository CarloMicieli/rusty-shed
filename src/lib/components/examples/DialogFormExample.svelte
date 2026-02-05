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
  import { Button, Dialog, Input, Textarea } from '$lib/components';
  
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
  <Button onclick={() => dialogOpen = true}>
    Create New Item
  </Button>
  
  <Dialog
    bind:open={dialogOpen}
    aria-labelledby="dialog-title"
    aria-describedby="dialog-description"
  >
    <div class="p-6 space-y-4">
      <div class="space-y-1">
        <h2 id="dialog-title" class="text-lg font-semibold">
          Create New Item
        </h2>
        <p id="dialog-description" class="text-sm text-surface-400">
          Fill in the details below to create a new item.
        </p>
      </div>
      
      <div class="space-y-4">
        <div class="space-y-1">
          <label for="title" class="block text-sm font-medium">
            Title
          </label>
          <Input
            id="title"
            bind:value={form.title}
            placeholder="Enter title"
            required
          />
        </div>
        
        <div class="space-y-1">
          <label for="description" class="block text-sm font-medium">
            Description
          </label>
          <Textarea
            id="description"
            bind:value={form.description}
            placeholder="Enter description"
            rows={4}
          />
        </div>
      </div>
      
      <div class="flex gap-2 justify-end">
        <Button variant="ghost" onclick={handleCancel}>
          Cancel
        </Button>
        <Button
          variant="default"
          onclick={handleSubmit}
          disabled={!form.title}
        >
          Create
        </Button>
      </div>
    </div>
  </Dialog>
</div>
