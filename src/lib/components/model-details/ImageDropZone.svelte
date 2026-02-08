<script lang="ts">
  import { commands } from '$lib/bindings';
  import { Alert, AlertDescription } from '$lib/components/ui/alert';
  import { Loader2, Upload } from 'lucide-svelte';
  import {
    drag_and_drop_hint,
    drop_image_here,
    uploading,
    upload_success,
    upload_error_model_not_found,
    upload_error_unknown,
    upload_error_multiple_files
  } from '$lib/paraglide/messages.js';
  import type { RailwayModelId } from '$lib/bindings';

  interface Props {
    modelId: RailwayModelId;
    onUploadSuccess?: () => void;
  }

  let { modelId, onUploadSuccess }: Props = $props();

  // State
  let isDragging = $state(false);
  let isUploading = $state(false);
  let error = $state<string | null>(null);
  let showSuccess = $state(false);

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    // Only set isDragging to false if we're leaving the drop zone itself
    const target = event.currentTarget as HTMLElement;
    const relatedTarget = event.relatedTarget as HTMLElement;
    if (!target.contains(relatedTarget)) {
      isDragging = false;
    }
  }

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDragging = false;

    // Reset state
    error = null;
    showSuccess = false;

    try {
      const files = event.dataTransfer?.files;
      if (!files || files.length === 0) {
        return;
      }

      // Validate single file
      if (files.length > 1) {
        error = upload_error_multiple_files();
        return;
      }

      const file = files[0];

      // Check MIME type hint (not foolproof, backend will validate)
      const validMimeTypes = ['image/jpeg', 'image/png', 'image/webp'];
      if (file.type && !validMimeTypes.includes(file.type)) {
        error = `Unsupported format: ${file.type}. Supported formats: JPEG, PNG, WebP`;
        return;
      }

      isUploading = true;

      // Read file as ArrayBuffer
      const arrayBuffer = await file.arrayBuffer();
      const fileData = Array.from(new Uint8Array(arrayBuffer));

      // Call upload command with bytes
      const result = await commands.uploadModelImageBytes({
        modelId,
        fileName: file.name,
        fileData
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
      console.error('Drop upload failed:', e);
      error = e instanceof Error ? e.message : upload_error_unknown();
    } finally {
      isUploading = false;
    }
  }
</script>

<div class="space-y-4">
  <!-- Drop Zone -->
  <div
    role="button"
    tabindex="0"
    class="relative flex min-h-[200px] flex-col items-center justify-center rounded-lg border-2 border-dashed p-6 text-center transition-colors {isDragging
      ? 'border-primary bg-primary/5'
      : 'border-muted-foreground/25 hover:border-muted-foreground/50'} {isUploading
      ? 'cursor-not-allowed opacity-50'
      : 'cursor-pointer'}"
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
  >
    {#if isUploading}
      <Loader2 class="mb-4 h-12 w-12 animate-spin text-primary" />
      <p class="text-sm font-medium">{uploading()}</p>
    {:else if isDragging}
      <Upload class="mb-4 h-12 w-12 text-primary" />
      <p class="text-lg font-semibold text-primary">{drop_image_here()}</p>
    {:else}
      <Upload class="mb-4 h-12 w-12 text-muted-foreground" />
      <p class="mb-2 text-sm font-semibold">{drag_and_drop_hint()}</p>
      <p class="text-xs text-muted-foreground">JPEG, PNG, or WebP (max 50MB)</p>
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

  <!-- Error Message -->
  {#if error}
    <Alert variant="destructive">
      <AlertDescription>
        {error}
      </AlertDescription>
    </Alert>
  {/if}
</div>
