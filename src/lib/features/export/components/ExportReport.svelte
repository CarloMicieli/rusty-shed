<script lang="ts">
  /**
   * ExportReport component
   * Displays export completion report with summary and warnings
   */
  import type { ExportResult } from '../types';

  export let result: ExportResult | null = null;
</script>

<div class="export-report">
  {#if result}
    <div class="report-content">
      <h3>Export Completed</h3>
      <div class="report-summary">
        <p>Archive: {result.archive_path}</p>
        <p>Size: {(result.file_size_bytes / 1024 / 1024).toFixed(2)} MB</p>
        <p>Records: {result.records_exported}</p>
      </div>
      {#if result.warnings.length > 0}
        <div class="warnings">
          <h4>Warnings:</h4>
          {#each result.warnings as warning (warning)}
            <p>{warning}</p>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
