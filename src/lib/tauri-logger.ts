// Lightweight Tauri-aware logger wrapper
// Exports: debug, info, warn, error, trace (and `log` object)
// Behavior:
// - Tries to detect running inside Tauri and lazily import @tauri-apps/api/log
// - If tauri logging is available, forwards to it; otherwise falls back to window.console
// - Safe-serializes objects to avoid circular errors

type LogLevel = 'debug' | 'info' | 'warn' | 'error' | 'trace'

const isBrowser = typeof window !== 'undefined'
const tauriDetected = isBrowser && (!!(window as any).__TAURI__ || !!(window as any).__TAURI_IPC__)

let tauriLogModule: any = null
let loader: Promise<void> | null = null

function safeSerialize(arg: unknown): unknown {
	if (arg === null || arg === undefined) return arg
	const t = typeof arg
	if (t === 'string' || t === 'number' || t === 'boolean') return arg
	// Try structured cloning via JSON; fall back to string
	try {
		return JSON.parse(JSON.stringify(arg))
	} catch {
		try {
			return String(arg)
		} catch {
			return '[unserializable]'
		}
	}
}

function ensureTauriLog() {
	if (!isBrowser || !tauriDetected) return
	if (tauriLogModule || loader) return
	// Lazy import the tauri log module; don't await — we fall back to console until loaded
	loader = import('@tauri-apps/api')
		.then((m: any) => {
			// Prefer the named `log` export if present
			uriLogModuleSetter(m?.log ?? m)
		})
		.catch(() => {
			// ignore — will fall back to console
			tauriLogModule = null
		})
}

function uriLogModuleSetter(m: any) {
	// Support both named exports and a default export container
	if (!m) return
	// if the module exports functions directly, use it; otherwise, check default
	tauriLogModule = m
}

function callTauriOrConsole(level: LogLevel, args: unknown[]) {
	if (tauriLogModule && typeof tauriLogModule[level] === 'function') {
		try {
			(tauriLogModule[level] as Function)(...args.map(safeSerialize))
			return
		} catch {
			// fall through to console
		}
	}
	const cons = (console as any)[level] ?? console.log
	try {
		cons.apply(console, args)
	} catch {
		// last resort: stringify
		cons(String(args))
	}
}

export function debug(...args: unknown[]) {
	ensureTauriLog()
	callTauriOrConsole('debug', args)
}

export function info(...args: unknown[]) {
	ensureTauriLog()
	callTauriOrConsole('info', args)
}

export function warn(...args: unknown[]) {
	ensureTauriLog()
	callTauriOrConsole('warn', args)
}

export function error(...args: unknown[]) {
	ensureTauriLog()
	callTauriOrConsole('error', args)
}

export function trace(...args: unknown[]) {
	ensureTauriLog()
	callTauriOrConsole('trace', args)
}

export const log = { debug, info, warn, error, trace }
