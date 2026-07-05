import type { ArchiveFormat } from '$lib/bindings';

export type ImportSessionState =
  'pending' | 'analyzed' | 'validated' | 'previewed' | 'importing' | 'completed' | 'failed';

export interface ImportSession {
  id: string;
  sourcePath: string;
  format: ArchiveFormat;
  state: ImportSessionState;
  createdAt: string;
  completedAt?: string;
}

export interface ImportProgress {
  sessionId: string;
  phase: string;
  current: number;
  total: number;
  percentage: number;
}
