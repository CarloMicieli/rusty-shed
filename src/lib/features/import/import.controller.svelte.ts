import { setContext, getContext } from 'svelte';
import type {
  AnalyzeImportPackageResponse,
  RecordCounts,
  ImportValidationError,
  ImportWarning,
  ImportPreviewResponse,
  ImportResultResponse
} from '$lib/bindings';
import type { ImportProgress } from './types';
import * as m from '$lib/paraglide/messages.js';

const IMPORT_CONTEXT_KEY = Symbol('import-controller');

function extractErrorMessage(error: unknown): string {
  if (error && typeof error === 'object') {
    if ('NotFound' in error) return m['import.error.manifestMissing']();
    if ('DatabaseError' in error) return m['import.error.importFailed']();
  }
  return m['import.error.analyzeFailed']();
}

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
  fatalError = $state<string | null>(null);

  async analyzePackage(filePath: string) {
    this.fatalError = null;
    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const commandResult: unknown = await commands.analyzeImportPackage({ filePath });
      if (
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'error' &&
        'error' in commandResult
      ) {
        throw commandResult.error;
      }
      const result =
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'ok' &&
        'data' in commandResult
          ? (commandResult.data as AnalyzeImportPackageResponse)
          : (commandResult as AnalyzeImportPackageResponse);
      this.sessionId = result.sessionId;
      this.recordCounts = result.recordCounts;
      this.canImport = result.validationStatus === 'valid';
    } catch (error) {
      console.error('Failed to analyze package:', error);
      this.fatalError = extractErrorMessage(error);
    } finally {
      this.isLoading = false;
    }
  }

  async getPreview() {
    if (!this.sessionId) return;

    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const commandResult: unknown = await commands.getImportPreview({ sessionId: this.sessionId });
      if (
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'error' &&
        'error' in commandResult
      ) {
        throw commandResult.error;
      }
      const result =
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'ok' &&
        'data' in commandResult
          ? (commandResult.data as ImportPreviewResponse)
          : (commandResult as ImportPreviewResponse);
      this.preview = result;
      this.errors = result.errors;
      this.warnings = result.warnings;
      this.canImport = result.canImport;
    } catch (error) {
      console.error('Failed to get preview:', error);
      this.fatalError = extractErrorMessage(error);
      this.sessionId = null;
    } finally {
      this.isLoading = false;
    }
  }

  async executeImport() {
    if (!this.sessionId) return;

    this.isLoading = true;
    try {
      const { commands } = await import('$lib/bindings');
      const commandResult: unknown = await commands.executeImport({ sessionId: this.sessionId });
      if (
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'error' &&
        'error' in commandResult
      ) {
        throw commandResult.error;
      }
      const result =
        commandResult !== null &&
        typeof commandResult === 'object' &&
        'status' in commandResult &&
        commandResult.status === 'ok' &&
        'data' in commandResult
          ? (commandResult.data as ImportResultResponse)
          : (commandResult as ImportResultResponse);
      this.result = result;
      this.warnings = result.warnings;
      this.canImport = false;
    } catch (error) {
      console.error('Failed to execute import:', error);
      this.fatalError = extractErrorMessage(error);
      this.sessionId = null;
      this.preview = null;
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
    this.fatalError = null;
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
