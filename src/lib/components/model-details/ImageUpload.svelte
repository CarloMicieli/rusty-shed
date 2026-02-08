<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { commands } from '$lib/bindings';
  import { Button } from '$lib/components/ui/button';
  import { Alert, AlertDescription } from '$lib/components/ui/alert';
  import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
  } from '$lib/components/ui/dialog';
  import { Upload, Loader2, Trash2 } from 'lucide-svelte';
  import {
    upload_image,
    upload_image_filter_name,
    uploading,
    upload_success,
    replace_image,
    upload_error_model_not_found,
    upload_error_unknown,
    delete_image,
    deleting,
    confirm_delete_image_title,
    confirm_delete_image_description,
    image_deleted
  } from '$lib/paraglide/messages.js';
  import type { RailwayModelId } from '$lib/bindings';

  interface Props {
    modelId: RailwayModelId;
    hasExistingImage?: boolean;
    onUploadSuccess?: () => void;
  }

  let { modelId, hasExistingImage = false, onUploadSuccess }: Props = $props();

  // State
  let isUploading = $state(false);
  let isDeleting = $state(false);
  let error = $state<string | null>(null);
  let showSuccess = $state(false);
  let showDeleteSuccess = $state(false);
  let isDeleteDialogOpen = $state(false);

  async function handleUpload() {
    // Reset state
    error = null;
    showSuccess = false;

    console.log('[ImageUpload] Starting upload for model:', modelId);

    try {
      // Open file dialog with image filters
      const file = await open({
        multiple: false,
        filters: [
          {
            name: upload_image_filter_name(),
            extensions: ['jpg', 'jpeg', 'png', 'webp']
          }
        ]
      });

      console.log('[ImageUpload] File selected:', file);

      // User cancelled
      if (!file) {
        console.log('[ImageUpload] User cancelled file selection');
        return;
      }

      isUploading = true;
      console.log('[ImageUpload] Calling upload command...');

      // Call upload command
      const result = await commands.uploadModelImage({
        modelId,
        filePath: file
      });

      console.log('[ImageUpload] Upload result:', result);

      if (result.status === 'error') {
        // Map error types
        const err = result.error;
        if (typeof err === 'object') {
          if ('BusinessRule' in err) {
            error = err.BusinessRule;
          } else if ('NotFound' in err) {
            error = upload_error_model_not_found();
          } else if ('Unknown' in err) {
            error = err.Unknown;
          } else {
            error = upload_error_unknown();
          }
        } else {
          error = upload_error_unknown();
        }
        return;
      }

      // Success!
      showSuccess = true;

      // Hide success message after 3 seconds
      setTimeout(() => {
        showSuccess = false;
      }, 3000);

      // Trigger parent refresh
      if (onUploadSuccess) {
        onUploadSuccess();
      }
    } catch (e) {
      console.error('Upload failed:', e);
      error = e instanceof Error ? e.message : upload_error_unknown();
    } finally {
      isUploading = false;
    }
  }

  async function handleDelete() {
    // Reset state
    error = null;
    showSuccess = false;
    showDeleteSuccess = false;

    try {
      isDeleting = true;

      // Call delete command
      const result = await commands.deleteModelImage({
        modelId
      });

      if (result.status === 'error') {
        // Map error types
        const err = result.error;
        if (typeof err === 'object') {
          if ('BusinessRule' in err) {
            error = err.BusinessRule;
          } else if ('NotFound' in err) {
            error = upload_error_model_not_found();
          } else if ('Unknown' in err) {
            error = err.Unknown;
          } else {
            error = upload_error_unknown();
          }
        } else {
          error = upload_error_unknown();
        }
        return;
      }

      // Success!
      showDeleteSuccess = true;

      // Close the dialog
      isDeleteDialogOpen = false;

      // Hide success message after 3 seconds
      setTimeout(() => {
        showDeleteSuccess = false;
      }, 3000);

      // Trigger parent refresh
      if (onUploadSuccess) {
        onUploadSuccess();
      }
    } catch (e) {
      console.error('Delete failed:', e);
      error = e instanceof Error ? e.message : upload_error_unknown();
    } finally {
      isDeleting = false;
    }
  }
</script>

<div class="space-y-4">
  <!-- Upload Button -->
  <div class="flex gap-2">
    <Button
      onclick={handleUpload}
      disabled={isUploading || isDeleting}
      variant={hasExistingImage ? 'outline' : 'default'}
      class="flex-1 sm:flex-initial"
    >
      {#if isUploading}
        <Loader2 class="mr-2 h-4 w-4 animate-spin" />
        {uploading()}
      {:else}
        <Upload class="mr-2 h-4 w-4" />
        {hasExistingImage ? replace_image() : upload_image()}
      {/if}
    </Button>

    <!-- Delete Button (only show if image exists) -->
    {#if hasExistingImage}
      <Dialog bind:open={isDeleteDialogOpen}>
        <DialogTrigger>
          <Button variant="outline" disabled={isUploading || isDeleting} class="px-3">
            {#if isDeleting}
              <Loader2 class="h-4 w-4 animate-spin" />
            {:else}
              <Trash2 class="h-4 w-4 text-destructive" />
            {/if}
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{confirm_delete_image_title()}</DialogTitle>
            <DialogDescription>
              {confirm_delete_image_description()}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              variant="outline"
              onclick={() => {
                isDeleteDialogOpen = false;
              }}
            >
              Cancel
            </Button>
            <Button variant="destructive" onclick={handleDelete} disabled={isDeleting}>
              {#if isDeleting}
                <Loader2 class="mr-2 h-4 w-4 animate-spin" />
                {deleting()}
              {:else}
                <Trash2 class="mr-2 h-4 w-4" />
                {delete_image()}
              {/if}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    {/if}
  </div>

  <!-- Success Message -->
  {#if showSuccess}
    <Alert
      class="border-green-500 bg-green-50 text-green-900 dark:bg-green-950 dark:text-green-100"
    >
      <AlertDescription>
        {upload_success()}
      </AlertDescription>
    </Alert>
  {/if}

  <!-- Delete Success Message -->
  {#if showDeleteSuccess}
    <Alert
      class="border-green-500 bg-green-50 text-green-900 dark:bg-green-950 dark:text-green-100"
    >
      <AlertDescription>
        {image_deleted()}
      </AlertDescription>
    </Alert>
  {/if}

  <!-- Error Message -->
  {#if error}
    <Alert variant="destructive">
      <AlertDescription>
        {error}
      </AlertDescription>
    </Alert>
  {/if}
</div>
