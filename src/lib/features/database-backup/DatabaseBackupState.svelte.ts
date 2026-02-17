export type DatabaseBackupState = {
  isExporting: boolean;
  isImporting: boolean;
  isOperationInProgress: boolean;
  error: string | null;
};
