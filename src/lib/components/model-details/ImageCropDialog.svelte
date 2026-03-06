<script lang="ts">
  import Cropper from 'cropperjs';
  // NO CSS import — v2 uses Shadow DOM styles built-in
  import { commands } from '$lib/bindings';
  import { Button } from '$lib/components/ui/button';
  import { Loader2 } from 'lucide-svelte';
  import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle
  } from '$lib/components/ui/dialog';
  import {
    crop_dialog_title,
    crop_confirm,
    crop_cancel,
    uploading
  } from '$lib/paraglide/messages.js';
  import type { RailwayModelId } from '$lib/bindings';

  interface Props {
    open: boolean;
    imageSrc: string;
    fileName: string;
    modelId: RailwayModelId;
    onSaveSuccess?: () => void;
    onCancel?: () => void;
  }

  let {
    open = $bindable(),
    imageSrc,
    fileName,
    modelId,
    onSaveSuccess,
    onCancel
  }: Props = $props();

  let imageEl = $state<HTMLImageElement | null>(null);
  let cropperInstance: Cropper | null = null;
  let isReady = $state(false);
  let isSaving = $state(false);
  let saveError = $state<string | null>(null);

  // Use $effect instead of onMount: bits-ui Dialog renders content lazily,
  // so imageEl is null until the dialog actually opens and the <img> is mounted.
  // $effect re-runs whenever imageEl changes from null → HTMLImageElement.
  $effect(() => {
    if (!imageEl) return;

    isReady = false;
    saveError = null;

    const instance = new Cropper(imageEl);
    cropperInstance = instance;

    const sel = instance.getCropperSelection()!;
    sel.aspectRatio = NaN;
    sel.initialCoverage = 0.8;
    sel.movable = true;
    sel.resizable = true;

    instance
      .getCropperImage()!
      .$ready()
      .then(() => {
        isReady = true;
      });

    return () => {
      instance.destroy();
      cropperInstance = null;
      isReady = false;
    };
  });

  function handleClose() {
    if (imageSrc.startsWith('blob:')) {
      URL.revokeObjectURL(imageSrc);
    }
    open = false;
  }

  function handleCancel() {
    onCancel?.();
    handleClose();
  }

  async function handleConfirm() {
    if (!cropperInstance || !isReady) return;

    isSaving = true;
    saveError = null;

    try {
      const sel = cropperInstance.getCropperSelection()!;
      const canvas = await sel.$toCanvas({ width: 2048, height: 2048 });

      const fileData = await new Promise<number[]>((resolve, reject) => {
        canvas.toBlob(
          async (blob) => {
            if (!blob) return reject(new Error('toBlob returned null'));
            resolve(Array.from(new Uint8Array(await blob.arrayBuffer())));
          },
          'image/jpeg',
          0.9
        );
      });

      const normalizedFileName = fileName.replace(/\.[^.]+$/, '.jpg');
      const result = await commands.uploadModelImageBytes({
        modelId,
        fileName: normalizedFileName,
        fileData
      });

      if (result.status === 'error') {
        const err = result.error;
        if (typeof err === 'object') {
          if ('BusinessRule' in err) {
            saveError = err.BusinessRule;
          } else if ('Unknown' in err) {
            saveError = err.Unknown;
          } else {
            saveError = 'Upload failed';
          }
        } else {
          saveError = 'Upload failed';
        }
        return;
      }

      onSaveSuccess?.();
      handleClose();
    } catch (e) {
      saveError = e instanceof Error ? e.message : 'Upload failed';
    } finally {
      isSaving = false;
    }
  }
</script>

<Dialog
  bind:open
  onOpenChange={(o) => {
    if (!o) handleCancel();
  }}
>
  <DialogContent class="max-w-2xl">
    <DialogHeader>
      <DialogTitle>{crop_dialog_title()}</DialogTitle>
    </DialogHeader>

    <div class="relative w-full overflow-hidden rounded-md bg-black" style="height: 400px;">
      <!-- svelte-ignore a11y_missing_attribute -->
      <img bind:this={imageEl} src={imageSrc} class="max-h-full max-w-full" />
    </div>

    {#if saveError}
      <p class="text-sm text-destructive">{saveError}</p>
    {/if}

    <DialogFooter>
      <Button variant="outline" onclick={handleCancel} disabled={isSaving}>
        {crop_cancel()}
      </Button>
      <Button onclick={handleConfirm} disabled={!isReady || isSaving}>
        {#if isSaving}
          <Loader2 class="mr-2 h-4 w-4 animate-spin" />
          {uploading()}
        {:else}
          {crop_confirm()}
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
