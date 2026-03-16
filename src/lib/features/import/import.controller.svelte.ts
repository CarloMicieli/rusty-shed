import { setContext, getContext } from 'svelte';
import type {
  RecordCounts,
  ImportValidationError,
  ImportWarning,
  ImportPreviewResponse,
  ImportResultResponse
} from '$lib/bindings';
import type { ImportProgress } from './types';

const IMPORT_CONTEXT_KEY = Symbol('import-controller');

export class ImportController {
  sessionId = $state<string | null>(null);
  recordCounts = $state<RecordCounts | null>(null);
  errors = $state<ImportValidationError[]>([]);
  warnings = $state<ImportWarning[]>([]);
  progress = $state<ImportProgress | null>(null);
  preview = $state<ImportPreviewResponse | null>(null);
  result = $state<ImportResultResponse | null>(null);
  canImport = $state(false);
  isLoading = $state(false);

  async analyzePackage(filePath: string) {
    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.analyzeImportPackage({ filePath });

      if (result.status === 'ok') {
        this.sessionId = result.data.sessionId;
        this.recordCounts = result.data.recordCounts;
        this.canImport = result.data.validationStatus === 'valid';

        if (result.data.validationStatus !== 'valid') {
          this.errors = [];
        }
      } else {
        console.error('Failed to analyze package:', result.error);
        this.errors = [];
      }
    } catch (error) {
      console.error('Failed to analyze package:', error);
      this.errors = [];
    } finally {
      this.isLoading = false;
    }
  }

  async getPreview() {
    if (!this.sessionId) return;

    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.getImportPreview({ sessionId: this.sessionId });

      if (result.status === 'ok') {
        this.preview = result.data;
        this.errors = result.data.errors;
        this.warnings = result.data.warnings;
        this.canImport = result.data.canImport;
      } else {
        console.error('Failed to get preview:', result.error);
        this.preview = null;
      }
    } catch (error) {
      console.error('Failed to get preview:', error);
      this.preview = null;
    } finally {
      this.isLoading = false;
    }
  }

  async executeImport() {
    if (!this.sessionId) return;

    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.executeImport({ sessionId: this.sessionId });

      if (result.status === 'ok') {
        this.result = result.data;
        this.warnings = result.data.warnings;
        this.canImport = false;
      } else {
        console.error('Failed to execute import:', result.error);
      }
    } catch (error) {
      console.error('Failed to execute import:', error);
    } finally {
      this.isLoading = false;
    }
  }

  async cancelSession() {
    if (!this.sessionId) return;

    try {
      const { commands } = await import('$lib/bindings');
      await commands.cancelImportSession({ sessionId: this.sessionId });
    } catch (error) {
      console.error('Failed to cancel session:', error);
    } finally {
      this.reset();
    }
  }

  reset() {
    this.sessionId = null;
    this.recordCounts = null;
    this.errors = [];
    this.warnings = [];
    this.progress = null;
    this.preview = null;
    this.result = null;
    this.canImport = false;
    this.isLoading = false;
  }
}

export function createImportController() {
  return new ImportController();
}

export function setImportContext(controller: ImportController) {
  setContext(IMPORT_CONTEXT_KEY, controller);
}

export function getImportContext(): ImportController {
  return getContext(IMPORT_CONTEXT_KEY);
}
