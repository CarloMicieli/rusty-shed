import { writable } from 'svelte/store';
import { RecordCounts, ValidationError, ImportWarning } from '$lib/bindings';
import type { ImportProgress } from './types';

export class ImportController {
  private sessionId = writable<string | null>(null);
  private recordCounts = writable<RecordCounts | null>(null);
  private errors = writable<ValidationError[]>([]);
  private warnings = writable<ImportWarning[]>([]);
  private progress = writable<ImportProgress | null>(null);
  private canImport = writable(false);
  private isLoading = writable(false);

  // Expose stores as readonly
  readonly sessionId$ = { subscribe: this.sessionId.subscribe };
  readonly recordCounts$ = { subscribe: this.recordCounts.subscribe };
  readonly errors$ = { subscribe: this.errors.subscribe };
  readonly warnings$ = { subscribe: this.warnings.subscribe };
  readonly progress$ = { subscribe: this.progress.subscribe };
  readonly canImport$ = { subscribe: this.canImport.subscribe };
  readonly isLoading$ = { subscribe: this.isLoading.subscribe };

  async analyzePackage(filePath: string) {
    this.isLoading.set(true);
    try {
      const { analyzeImportPackage } = await import('$lib/bindings');
      const result = await analyzeImportPackage({ filePath });
      
      this.sessionId.set(result.sessionId);
      this.recordCounts.set(result.recordCounts);
      this.canImport.set(result.validationStatus === 'Valid');
      
      if (result.validationStatus !== 'Valid') {
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
      const { getImportPreview } = await import('$lib/bindings');
      const result = await getImportPreview({ sessionId });
      
      this.errors.set(result.errors);
      this.warnings.set(result.warnings);
      this.canImport.set(result.canImport);
    } catch (error) {
      console.error('Failed to get preview:', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  async executeImport() {
    const sessionId = this.getSessionId();
    if (!sessionId) return;
    
    this.isLoading.set(true);
    try {
      const { executeImport } = await import('$lib/bindings');
      const result = await executeImport({ sessionId });
      
      this.recordCounts.set({
        manufacturers: result.added.manufacturers,
        railwayCompanies: result.added.railwayCompanies,
        railwayModels: result.added.railwayModels,
        collectionItems: result.added.collectionItems,
        sellers: result.added.sellers,
        maintenanceCards: result.added.maintenanceCards
      });
      this.warnings.set(result.warnings);
      this.canImport.set(false);
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
      const { cancelImportSession } = await import('$lib/bindings');
      await cancelImportSession({ sessionId });
    } catch (error) {
      console.error('Failed to cancel session:', error);
    } finally {
      this.reset();
    }
  }

  private getSessionId(): string | null {
    let currentSessionId: string | null = null;
    this.sessionId$.subscribe(id => { currentSessionId = id; });
    return currentSessionId;
  }

  reset() {
    this.sessionId.set(null);
    this.recordCounts.set(null);
    this.errors.set([]);
    this.warnings.set([]);
    this.progress.set(null);
    this.canImport.set(false);
    this.isLoading.set(false);
  }
}

export function createImportController() {
  return new ImportController();
}
