<script lang="ts">
  import Cropper from 'cropperjs';
  // NO CSS import — v2 uses Shadow DOM styles built-in
  import { commands } from '$lib/bindings';
  import { Button } from '$lib/components/ui/button';
  import { FlipHorizontal2, Loader2, RotateCcw, RotateCw } from 'lucide-svelte';
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
    crop_aspect_free,
    crop_aspect_wide,
    crop_flip_horizontal,
    crop_rotate_left,
    crop_rotate_right,
    crop_reset,
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
  let activeRatio = $state<'free' | '16:9' | '3:1'>('free');
  let resetKey = $state(0);

  // Use $effect instead of onMount: bits-ui Dialog renders content lazily,
  // so imageEl is null until the dialog actually opens and the <img> is mounted.
  // $effect re-runs whenever imageEl or resetKey changes.
  $effect(() => {
    void resetKey; // read to register dependency — incrementing triggers re-init
    if (!imageEl) return;

    isReady = false;
    saveError = null;
    activeRatio = 'free';

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
      })
      .catch((err: unknown) => {
        saveError = 'Failed to load image for cropping. Check the file format.';
        console.error('[ImageCropDialog] CropperJS $ready() failed:', err);
      });

    return () => {
      instance.destroy();
      cropperInstance = null;
      isReady = false;
    };
  });

  // Sync aspect ratio to cropper whenever activeRatio changes
  $effect(() => {
    if (!cropperInstance || !isReady) return;
    const sel = cropperInstance.getCropperSelection()!;
    if (activeRatio === 'free') sel.aspectRatio = NaN;
    else if (activeRatio === '16:9') sel.aspectRatio = 16 / 9;
    else if (activeRatio === '3:1') sel.aspectRatio = 3 / 1;
  });

  function handleRotate(degrees: number) {
    cropperInstance?.getCropperImage()?.$rotate(degrees);
  }

  function handleFlip() {
    cropperInstance?.getCropperImage()?.$scale(-1, 1);
  }

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
      const canvas = await sel.$toCanvas({ width: 2560 });

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
        console.error('[ImageCropDialog] upload error:', err);
        if (typeof err === 'object') {
          if ('BusinessRule' in err) {
            saveError = err.BusinessRule as string;
          } else if ('NotFound' in err) {
            saveError = err.NotFound as string;
          } else if ('Unknown' in err) {
            saveError = (err.Unknown as { message: string }).message;
          } else if ('DatabaseError' in err) {
            saveError = err.DatabaseError as string;
          } else if ('ValidationError' in err) {
            const allErrors = Object.values(
              err.ValidationError as Record<string, Array<{ message?: string }>>
            ).flat();
            saveError = allErrors[0]?.message ?? 'Validation error';
          } else if ('Conflict' in err) {
            saveError = err.Conflict as string;
          } else if ('PermissionDenied' in err) {
            saveError = err.PermissionDenied as string;
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
  <DialogContent class="max-w-5xl gap-3">
    <DialogHeader>
      <DialogTitle>{crop_dialog_title()}</DialogTitle>
    </DialogHeader>

    <!-- Cropper canvas — aspect-[16/7] fills the panoramic shape of a locomotive -->
    <div
      class="relative aspect-[16/7] w-full overflow-hidden rounded-md border border-white/10"
      style="background-color: #27272a; background-image: linear-gradient(45deg, #3f3f46 25%, transparent 25%), linear-gradient(-45deg, #3f3f46 25%, transparent 25%), linear-gradient(45deg, transparent 75%, #3f3f46 75%), linear-gradient(-45deg, transparent 75%, #3f3f46 75%); background-size: 16px 16px; background-position: 0 0, 0 8px, 8px -8px, -8px 0px;"
    >
      <!-- svelte-ignore a11y_missing_attribute -->
      <img
        bind:this={imageEl}
        src={imageSrc}
        crossorigin="anonymous"
        class="max-h-full max-w-full"
      />
    </div>

    <!-- Toolbar -->
    <div class="flex flex-wrap items-center gap-2">
      <!-- Aspect ratio segmented control -->
      <div class="flex items-center gap-0.5 rounded-lg bg-zinc-800/60 p-0.5 ring-1 ring-white/10">
        {#each [['free', 'free'] as const, ['16:9', '16:9'] as const, ['3:1', '3:1'] as const] as [ratio] (ratio)}
          <Button
            size="sm"
            class={activeRatio === ratio
              ? 'bg-zinc-700 text-white hover:bg-zinc-600'
              : 'bg-transparent text-zinc-400 shadow-none hover:bg-transparent hover:text-zinc-200'}
            onclick={() => (activeRatio = ratio)}
            disabled={!isReady}
          >
            {ratio === 'free' ? crop_aspect_free() : ratio === '3:1' ? crop_aspect_wide() : '16:9'}
          </Button>
        {/each}
      </div>

      <!-- Utility icon group: rotate + flip -->
      <div class="flex items-center gap-1 rounded-md border border-white/10 p-1">
        <Button
          size="icon"
          variant="ghost"
          onclick={() => handleRotate(-90)}
          disabled={!isReady}
          aria-label={crop_rotate_left()}
        >
          <RotateCcw class="size-4" />
        </Button>
        <Button
          size="icon"
          variant="ghost"
          onclick={() => handleRotate(90)}
          disabled={!isReady}
          aria-label={crop_rotate_right()}
        >
          <RotateCw class="size-4" />
        </Button>
        <Button
          size="icon"
          variant="ghost"
          onclick={handleFlip}
          disabled={!isReady}
          aria-label={crop_flip_horizontal()}
        >
          <FlipHorizontal2 class="size-4" />
        </Button>
      </div>

      <!-- Reset — visually separated via ml-auto -->
      <Button
        size="sm"
        variant="ghost"
        class="ml-auto"
        onclick={() => resetKey++}
        disabled={!isReady}
      >
        {crop_reset()}
      </Button>
    </div>

    <!-- Model ID metadata -->
    <p class="font-mono text-xs text-zinc-600">{modelId}</p>

    {#if saveError}
      <p class="text-sm text-destructive">{saveError}</p>
    {/if}

    <DialogFooter>
      <Button variant="outline" onclick={handleCancel} disabled={isSaving}>
        {crop_cancel()}
      </Button>
      <Button
        class="bg-amber-600 text-black hover:bg-amber-500 focus-visible:ring-amber-500"
        onclick={handleConfirm}
        disabled={!isReady || isSaving}
      >
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
