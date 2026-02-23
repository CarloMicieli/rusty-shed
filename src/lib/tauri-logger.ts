// Lightweight Tauri-aware logger wrapper
/* eslint-disable @typescript-eslint/no-explicit-any */
// Exports: debug, info, warn, error, trace (and `log` object)
// Behavior:
// - Tries to detect running inside Tauri and lazily import @tauri-apps/api/log
// - If tauri logging is available, forwards to it; otherwise falls back to window.console
// - Safe-serializes objects to avoid circular errors

type LogLevel = 'debug' | 'info' | 'warn' | 'error' | 'trace';

const isBrowser = typeof window !== 'undefined';
// More robust Tauri check
const isTauri = isBrowser && (window as any).__TAURI_INTERNALS__ !== undefined;

let tauriLog: any = null;
let isImporting = false;

function serialize(arg: unknown): any {
  const cleanArg = arg;

  // 1. Check if it's an Error first
  if (cleanArg instanceof Error) {
    return { name: cleanArg.name, message: cleanArg.message, stack: cleanArg.stack };
  }

  // 2. Handle Svelte 5 Proxies safely
  // We check for the internal Proxy marker that Svelte 5 uses
  // or use JSON.stringify which naturally unwraps Proxies.
  if (cleanArg !== null && typeof cleanArg === 'object') {
    try {
      // In Svelte 5, JSON.stringify on a $state proxy
      // automatically produces the raw underlying data.
      return JSON.parse(JSON.stringify(cleanArg));
    } catch {
      // Fallback for circular references or complex objects
      return String(cleanArg);
    }
  }

  return cleanArg;
}

async function initTauriLog() {
  if (!isTauri || tauriLog || isImporting) return;
  isImporting = true;
  try {
    // Import everything as a namespace
    const m = await import('@tauri-apps/plugin-log');

    // In Tauri v2, the functions (info, error, etc.) are top-level exports
    tauriLog = m;

    log.debug('[Logger] Tauri log plugin attached');
  } catch (e) {
    console.error('[Logger] Failed to load Tauri log plugin', e);
  } finally {
    isImporting = false;
  }
}

const createLogFn =
  (level: LogLevel) =>
  (...args: unknown[]) => {
    // Trigger async load, but don't await it to keep logging synchronous
    initTauriLog();

    const serialized = args.map(serialize);

    if (tauriLog && typeof tauriLog[level] === 'function') {
      tauriLog[level](...serialized);
    } else {
      // Fallback to browser console
      const method = level === 'trace' ? 'debug' : level;
      (console as any)[method]?.(...args);
    }
  };

export const log = {
  debug: createLogFn('debug'),
  info: createLogFn('info'),
  warn: createLogFn('warn'),
  error: createLogFn('error'),
  trace: createLogFn('trace')
};
