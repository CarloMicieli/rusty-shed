<script lang="ts">
  import { X, ArchiveRestore } from 'lucide-svelte';
  import { Sheet } from '$lib/components/ui/sheet';
  import { createImportController } from '$lib/features/import/import.controller.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import ImportDropZone from './ImportDropZone.svelte';
  import ImportPreview from './ImportPreview.svelte';
  import ImportReport from './ImportReport.svelte';

  interface Props {
    open?: boolean;
    onClose?: () => void;
  }

  let { open = $bindable(false), onClose }: Props = $props();

  const controller = createImportController();
  const sessionId = controller.sessionId$;
  const preview = controller.preview$;
  const result = controller.result$;

  let isProcessing = $state(false);

  async function handleFileSelected(filePath: string) {
    isProcessing = true;
    try {
      await controller.analyzePackage(filePath);
      await controller.getPreview();
    } catch (error) {
      console.error('Error analyzing package:', error);
    } finally {
      isProcessing = false;
    }
  }

  async function handleConfirmImport() {
    isProcessing = true;
    try {
      await controller.executeImport();
    } catch (error) {
      console.error('Error executing import:', error);
    } finally {
      isProcessing = false;
    }
  }

  async function handleCancelImport() {
    await controller.cancelSession();
    close();
  }

  function handleCloseReport() {
    controller.reset();
    close();
  }

  function close() {
    if ($sessionId) {
      controller.cancelSession();
    }
    controller.reset();
    open = false;
    onClose?.();
  }

  function handleOpenChange(newOpen: boolean) {
    if (!newOpen) close();
  }

  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });
</script>

<Sheet
  {open}
  side="right"
  onOpenChange={handleOpenChange}
  class="flex w-full flex-col border-zinc-800 bg-[#0c0c0c] sm:w-full sm:max-w-2xl"
>
  <!-- Header -->
  <div class="flex items-center justify-between border-b border-white/5 p-6">
    <div class="flex items-center gap-3">
      <div
        class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-500/10 text-amber-500"
      >
        <ArchiveRestore size={20} />
      </div>
      <h2 class="text-xl font-bold tracking-tight text-zinc-100 uppercase">
        {m['import.title']()}
      </h2>
    </div>
    <button
      onclick={close}
      class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
      disabled={isProcessing}
    >
      <X size={20} />
    </button>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto p-6">
    {#if $result}
      <ImportReport result={$result} onClose={handleCloseReport} />
    {:else if !$sessionId}
      <ImportDropZone onFileSelected={handleFileSelected} disabled={isProcessing} />
    {:else if $preview}
      <ImportPreview
        preview={$preview}
        onConfirm={handleConfirmImport}
        onCancel={handleCancelImport}
        loading={isProcessing}
      />
    {/if}
  </div>
</Sheet>
