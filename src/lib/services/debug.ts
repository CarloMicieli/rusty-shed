import type { DatabaseTableStat } from '$lib/bindings';
import type { SafeResult } from './errors';
import { safeInvoke } from './tauri';

const DEFAULT_LOG_LIMIT = 100;

/** Load SQLite table statistics for the Debug page. */
export async function fetchDbStats(): Promise<SafeResult<DatabaseTableStat[]>> {
  return safeInvoke<DatabaseTableStat[]>('get_db_stats');
}

/** Load recent persisted application log lines for the Debug page. */
export async function fetchRecentLogs(limit = DEFAULT_LOG_LIMIT): Promise<SafeResult<string[]>> {
  return safeInvoke<string[]>('get_recent_logs', { limit });
}

export type { DatabaseTableStat };
