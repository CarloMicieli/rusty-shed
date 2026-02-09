// Export feature controller
// Manages the frontend state for the export workflow

import type { ExportPreview, ExportEntitySelection, ExportResult } from './types';

export class ExportController {
  // UI state
  isOpen: boolean = $state(false);
  isLoading: boolean = $state(false);
  error: string | null = $state(null);
  progress: number = $state(0);
  currentPhase: 'collecting' | 'compressing' | 'finalizing' | null = $state(null);

  // Export data
  entitySelection: ExportEntitySelection = $state({
    include_railway_models: true,
    include_collection_items: true,
    include_sellers: true,
    include_maintenance_logs: true,
    include_dcc_roster: true,
    include_orphaned_images: false
  });

  preview: ExportPreview | null = $state(null);
  selectedPath: string | null = $state(null);
  result: ExportResult | null = $state(null);

  // Methods
  openDialog() {
    this.isOpen = true;
    this.error = null;
    this.preview = null;
    this.selectedPath = null;
    this.result = null;
  }

  closeDialog() {
    this.isOpen = false;
  }

  resetState() {
    this.isLoading = false;
    this.error = null;
    this.progress = 0;
    this.currentPhase = null;
  }

  setError(message: string) {
    this.error = message;
  }

  updateProgress(percentage: number, phase: 'collecting' | 'compressing' | 'finalizing') {
    this.progress = percentage;
    this.currentPhase = phase;
  }
}

export function createExportController() {
  return new ExportController();
}
