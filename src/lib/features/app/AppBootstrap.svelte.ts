/**
 * Application bootstrap utilities.
 *
 * Consolidates the startup sequence so `+layout.svelte` can call a single
 * function instead of an inline 40-line `onMount` block, and so the
 * onboarding-completion path can reuse the same post-boot steps without
 * duplicating them.
 */

import { SvelteDate } from 'svelte/reactivity';
import { appState } from '$lib/stores/app.svelte';
import { themeState } from '$lib/stores/themeStore.svelte';
import { log } from '$lib/tauri-logger';
import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
import { settingsState } from '$lib/features/settings/SettingsState.svelte';
import { bootstrapNeedsOnboarding } from '$lib/features/onboarding/onboarding-state.svelte';
import { collectionStore } from '$lib/state/collection.svelte';
import { financeState } from '$lib/state/finance.svelte';
import { safeInvoke } from '$lib/services';

// ─────────────────────────────────────────────────────────────
// INTERNAL HELPERS
// ─────────────────────────────────────────────────────────────

type TauriAwareWindow = Window & {
  __TAURI_INTERNALS__?: unknown;
};

function formatCommandError(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === 'string') return error;
  if (typeof error === 'object' && error !== null && 'message' in error) {
    const message = (error as { message?: unknown }).message;
    if (typeof message === 'string') return message;
  }
  return JSON.stringify(error);
}

function hasTauriBridge(): boolean {
  return typeof window !== 'undefined' && (window as TauriAwareWindow).__TAURI_INTERNALS__ != null;
}

export function isTauriHosted(): boolean {
  return typeof window !== 'undefined' && window.location.host === 'tauri.localhost';
}

export async function waitForTauriBridge(timeoutMs = 4_000, pollMs = 50): Promise<boolean> {
  if (hasTauriBridge()) return true;

  const startedAt = Date.now();
  while (Date.now() - startedAt < timeoutMs) {
    await new Promise((resolve) => setTimeout(resolve, pollMs));
    if (hasTauriBridge()) {
      return true;
    }
  }

  return false;
}

const FINANCE_SELECTED_YEAR_STORAGE_KEY = 'finance:selected-year';

export function getInitialFinanceYear(): number {
  const currentYear = new SvelteDate().getFullYear();
  const validYears = Array.from({ length: 5 }, (_, i) => currentYear - i);

  try {
    const storedYear = window.localStorage.getItem(FINANCE_SELECTED_YEAR_STORAGE_KEY);
    const parsedYear = Number(storedYear);
    return validYears.includes(parsedYear) ? parsedYear : currentYear;
  } catch (error) {
    log.warn(`Failed to read Finance selected year: ${String(error)}`);
    return currentYear;
  }
}

// ─────────────────────────────────────────────────────────────
// BOOTSTRAP RESULT
// ─────────────────────────────────────────────────────────────

export type BootstrapResult =
  { status: 'normal' } | { status: 'needs-onboarding' } | { status: 'error'; message: string };

// ─────────────────────────────────────────────────────────────
// COLD-START BOOTSTRAP
// ─────────────────────────────────────────────────────────────

/**
 * Full cold-start bootstrap sequence.
 *
 * - Waits for the Tauri bridge to become ready (desktop only)
 * - Shows the main window
 * - Removes the HTML loading spinner
 * - Initialises settings and derives whether onboarding is needed
 * - Initialises the locale, theme, DB, and application state
 *
 * Returns a result that tells the layout whether to render the normal app,
 * the onboarding wizard, or an error view.
 */
export async function bootstrapApp(opts: {
  /** Callback invoked once the bridge is ready and before any data loading. */
  onBridgeReady: () => void;
  dashboardState: { load: () => Promise<void> };
  wishlistState: { fetchWishlists: () => Promise<void> };
  budgetState: {
    load: () => Promise<void>;
    hasConfig: boolean;
    loadMonthlyRecords: (year: number) => Promise<void>;
  };
}): Promise<BootstrapResult> {
  try {
    if (isTauriHosted()) {
      const bridgeReady = await waitForTauriBridge();
      if (!bridgeReady) {
        throw new Error('Tauri bridge did not become ready during startup');
      }
    }

    const showResult = await safeInvoke<null>('show_main_window');
    if (!showResult.ok) {
      log.warn(`Failed to show main window: ${formatCommandError(showResult.error)}`);
    }

    opts.onBridgeReady();

    // Initialise settings first — determines onboarding status.
    await settingsState.initialize();
    const needsOnboarding = bootstrapNeedsOnboarding(settingsState.settings);

    await regionalManager.init();
    await themeState.initializeFromSettings();

    if (needsOnboarding) {
      return { status: 'needs-onboarding' };
    }

    await postOnboardingBoot(opts);

    return { status: 'normal' };
  } catch (err) {
    log.error(`Startup failed: ${String(err)}`);
    const message = err instanceof Error ? err.message : String(err);
    return { status: 'error', message };
  }
}

// ─────────────────────────────────────────────────────────────
// POST-ONBOARDING BOOT
// ─────────────────────────────────────────────────────────────

/**
 * Data-loading sequence shared between cold-start (after onboarding check)
 * and the WelcomeWizard `onComplete` handler.
 *
 * Fetches the app version, initialises the database, and loads the initial
 * dashboard / collection / wishlist / finance / budget data.
 */
export async function postOnboardingBoot(opts: {
  dashboardState: { load: () => Promise<void> };
  wishlistState: { fetchWishlists: () => Promise<void> };
  budgetState: {
    load: () => Promise<void>;
    hasConfig: boolean;
    loadMonthlyRecords: (year: number) => Promise<void>;
  };
}): Promise<void> {
  const versionResult = await safeInvoke<string>('get_app_version');
  if (!versionResult.ok) {
    throw new Error(formatCommandError(versionResult.error) || 'Failed to read app version');
  }
  appState.setVersion(versionResult.data);

  const initResult = await safeInvoke<null>('init_database');
  if (!initResult.ok) {
    throw new Error(formatCommandError(initResult.error) || 'Database initialization failed');
  }

  const initialFinanceYear = getInitialFinanceYear();

  const dashboardLoad =
    typeof opts.dashboardState?.load === 'function'
      ? opts.dashboardState.load()
      : Promise.resolve();
  const wishlistLoad =
    typeof opts.wishlistState?.fetchWishlists === 'function'
      ? opts.wishlistState.fetchWishlists()
      : Promise.resolve();
  const financeLoad =
    typeof financeState.ensureLoaded === 'function'
      ? financeState.ensureLoaded()
      : Promise.resolve();

  await Promise.all([dashboardLoad, collectionStore.fetch(), wishlistLoad, financeLoad]);

  if (typeof opts.budgetState?.load === 'function') {
    await opts.budgetState.load();
  }
  if (opts.budgetState?.hasConfig && typeof opts.budgetState?.loadMonthlyRecords === 'function') {
    await opts.budgetState.loadMonthlyRecords(initialFinanceYear);
  }
}
