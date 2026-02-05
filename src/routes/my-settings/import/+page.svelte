<script lang="ts">
  import { PageHeader } from '$lib/components';
  import ImportDropZone from '$lib/features/import/components/ImportDropZone.svelte';
  import ImportPreview from '$lib/features/import/components/ImportPreview.svelte';
  import ImportReport from '$lib/features/import/components/ImportReport.svelte';
  import { createImportController } from '$lib/features/import/import.controller.svelte';

  const controller = createImportController();
  const sessionId = controller.sessionId$;
  const preview = controller.preview$;
  const result = controller.result$;

  let selectedFile: File | null = $state(null);
  let isProcessing = $state(false);

  async function handleFilesSelected(files: FileList) {
    if (files.length === 0) return;

    selectedFile = files[0];
    isProcessing = true;

    try {
      // In Tauri, we can access the file path from the File object
      const filePath = (selectedFile as { path?: string }).path || selectedFile.name;
      await controller.analyzePackage(filePath);
      // After analysis, fetch the preview
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
    selectedFile = null;
  }

  function handleCloseReport() {
    controller.reset();
    selectedFile = null;
  }
</script>

<PageHeader
  title="Import Data"
  subtitle="Settings"
  description="Import your railway collection data from .zip or .tar.gz archives"
/>

<div class="import-container">
  {#if $result}
    <ImportReport result={$result} onClose={handleCloseReport} />
  {:else if !$sessionId}
    <div class="dropzone-section">
      <ImportDropZone onFilesSelected={handleFilesSelected} disabled={isProcessing} />
    </div>
  {:else if $preview}
    <ImportPreview
      preview={$preview}
      onConfirm={handleConfirmImport}
      onCancel={handleCancelImport}
      loading={isProcessing}
    />
  {/if}
</div>

<style>
  .import-container {
    max-width: 1200px;
    margin: 2rem auto;
    padding: 1rem;
  }

  .dropzone-section {
    width: 100%;
  }
</style>
