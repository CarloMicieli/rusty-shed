<script lang="ts">
  import { PageHeader } from '$lib/components';
  import ImportDropZone from '$lib/features/import/components/ImportDropZone.svelte';
  import ImportPreview from '$lib/features/import/components/ImportPreview.svelte';
  import ImportReport from '$lib/features/import/components/ImportReport.svelte';
  import { createImportController } from '$lib/features/import/import.controller.svelte';
  import * as m from '$lib/paraglide/messages.js';

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
  }

  function handleCloseReport() {
    controller.reset();
  }
</script>

<PageHeader
  title={m['import.title']()}
  subtitle={m.app_settings()}
  description={m.import_page_description()}
/>

<div class="mt-8 space-y-6">
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
