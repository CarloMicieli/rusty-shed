import { writable } from 'svelte/store';
import type {
  RecordCounts,
  ImportValidationError,
  ImportWarning,
  ImportPreviewResponse,
  ImportResultResponse
} from '$lib/bindings';
import type { ImportProgress } from './types';

export class ImportController {
  private sessionId = writable<string | null>(null);
  private recordCounts = writable<RecordCounts | null>(null);
  private errors = writable<ImportValidationError[]>([]);
  private warnings = writable<ImportWarning[]>([]);
  private progress = writable<ImportProgress | null>(null);
  private preview = writable<ImportPreviewResponse | null>(null);
  private result = writable<ImportResultResponse | null>(null);
  private canImport = writable(false);
  private isLoading = writable(false);

  // Expose stores as readonly
  readonly sessionId$ = { subscribe: this.sessionId.subscribe };
  readonly recordCounts$ = { subscribe: this.recordCounts.subscribe };
  readonly errors$ = { subscribe: this.errors.subscribe };
  readonly warnings$ = { subscribe: this.warnings.subscribe };
  readonly progress$ = { subscribe: this.progress.subscribe };
  readonly preview$ = { subscribe: this.preview.subscribe };
  readonly result$ = { subscribe: this.result.subscribe };
  readonly canImport$ = { subscribe: this.canImport.subscribe };
  readonly isLoading$ = { subscribe: this.isLoading.subscribe };

  async analyzePackage(filePath: string) {
    this.isLoading.set(true);
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.analyzeImportPackage({ filePath });

      if (result.status === 'ok') {
        this.sessionId.set(result.data.sessionId);
        this.recordCounts.set(result.data.recordCounts);
        this.canImport.set(result.data.validationStatus === 'valid');

        if (result.data.validationStatus !== 'valid') {
          this.errors.set([]);
        }
      } else {
        console.error('Failed to analyze package:', result.error);
        this.errors.set([]);
      }
    } catch (error) {
      console.error('Failed to analyze package:', error);
      this.errors.set([]);
    } finally {
      this.isLoading.set(false);
    }
  }

  async getPreview() {
    const sessionId = this.getSessionId();
    if (!sessionId) return;

    this.isLoading.set(true);
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.getImportPreview({ sessionId });

      if (result.status === 'ok') {
        this.preview.set(result.data);
        this.errors.set(result.data.errors);
        this.warnings.set(result.data.warnings);
        this.canImport.set(result.data.canImport);
      } else {
        console.error('Failed to get preview:', result.error);
        this.preview.set(null);
      }
    } catch (error) {
      console.error('Failed to get preview:', error);
      this.preview.set(null);
    } finally {
      this.isLoading.set(false);
    }
  }

  async executeImport() {
    const sessionId = this.getSessionId();
    if (!sessionId) return;

    this.isLoading.set(true);
    try {
      const { commands } = await import('$lib/bindings');
      const result = await commands.executeImport({ sessionId });

      if (result.status === 'ok') {
        this.result.set(result.data);
        this.warnings.set(result.data.warnings);
        this.canImport.set(false);
      } else {
        console.error('Failed to execute import:', result.error);
      }
    } catch (error) {
      console.error('Failed to execute import:', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  async cancelSession() {
    const sessionId = this.getSessionId();
    if (!sessionId) return;

    try {
      const { commands } = await import('$lib/bindings');
      await commands.cancelImportSession({ sessionId });
    } catch (error) {
      console.error('Failed to cancel session:', error);
    } finally {
      this.reset();
    }
  }

  private getSessionId(): string | null {
    let currentSessionId: string | null = null;
    this.sessionId$.subscribe((id) => {
      currentSessionId = id;
    });
    return currentSessionId;
  }

  reset() {
    this.sessionId.set(null);
    this.recordCounts.set(null);
    this.errors.set([]);
    this.warnings.set([]);
    this.progress.set(null);
    this.preview.set(null);
    this.result.set(null);
    this.canImport.set(false);
    this.isLoading.set(false);
  }
}

export function createImportController() {
  return new ImportController();
}
