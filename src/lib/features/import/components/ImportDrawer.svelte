<script lang="ts">
  import { ArchiveRestore } from 'lucide-svelte';
  import { DrawerShell, DrawerHeader } from '$lib/components/drawer';
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
    if (controller.sessionId) {
      controller.cancelSession();
    }
    controller.reset();
    open = false;
    onClose?.();
  }
</script>

<DrawerShell {open} onClose={close} size="lg" labelledby="import-drawer-title">
  {#snippet header({ requestClose })}
    <DrawerHeader
      id="import-drawer-title"
      title={m['import.title']()}
      icon={ArchiveRestore}
      onClose={requestClose}
    />
  {/snippet}

  {#if controller.result}
    <ImportReport result={controller.result} onClose={handleCloseReport} />
  {:else if !controller.sessionId}
    <ImportDropZone onFileSelected={handleFileSelected} disabled={isProcessing} />
  {:else if controller.preview}
    <ImportPreview
      preview={controller.preview}
      onConfirm={handleConfirmImport}
      onCancel={handleCancelImport}
      loading={isProcessing}
    />
  {/if}

  {#snippet footer({ requestClose: _ })}
    <!-- no drawer-level footer actions; sub-components own their actions -->
  {/snippet}
</DrawerShell>
