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
      // Placeholder - will be implemented in US1
      console.log('Analyzing package:', filePath);
    } finally {
      this.isLoading.set(false);
    }
  }

  async getPreview() {
    this.isLoading.set(true);
    try {
      // Placeholder - will be implemented in US2
      console.log('Getting preview');
    } finally {
      this.isLoading.set(false);
    }
  }

  async executeImport() {
    this.isLoading.set(true);
    try {
      // Placeholder - will be implemented in US1
      console.log('Executing import');
    } finally {
      this.isLoading.set(false);
    }
  }

  async cancelSession() {
    try {
      // Placeholder - will be implemented in polish phase
      console.log('Canceling session');
    } finally {
      this.sessionId.set(null);
      this.recordCounts.set(null);
      this.errors.set([]);
      this.warnings.set([]);
    }
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
