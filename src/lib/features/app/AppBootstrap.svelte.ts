/**
 * Application bootstrap utilities.
 *
 * Consolidates the startup sequence so `+layout.svelte` can call a single
 * function instead of an inline 40-line `onMount` block, and so the
 * onboarding-completion path can reuse the same post-boot steps without
 * duplicating them.
 */

import { commands } from '$lib/bindings';
import { appState } from '$lib/stores/app.svelte';
import { themeState } from '$lib/stores/themeStore.svelte';
import { log } from '$lib/tauri-logger';
import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
import { settingsState } from '$lib/features/settings/SettingsState.svelte';
import { bootstrapNeedsOnboarding } from '$lib/features/onboarding/onboarding-state.svelte';
import { collectionStore } from '$lib/state/collection.svelte';
import { financeState } from '$lib/state/finance.svelte';

// ─────────────────────────────────────────────────────────────
// INTERNAL HELPERS
// ─────────────────────────────────────────────────────────────

type TauriAwareWindow = Window & {
  __TAURI_INTERNALS__?: unknown;
};

function hasTauriBridge(): boolean {
  return (
    typeof window !== 'undefined' && (window as TauriAwareWindow).__TAURI_INTERNALS__ != null
  );
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
  const currentYear = new Date().getFullYear();
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
  | { status: 'normal' }
  | { status: 'needs-onboarding' }
  | { status: 'error'; message: string };

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
  budgetState: { load: () => Promise<void>; hasConfig: boolean; loadMonthlyRecords: (year: number) => Promise<void> };
}): Promise<BootstrapResult> {
  try {
    if (isTauriHosted()) {
      const bridgeReady = await waitForTauriBridge();
      if (!bridgeReady) {
        throw new Error('Tauri bridge did not become ready during startup');
      }
    }

    const showResult = await commands.showMainWindow();
    if (showResult.status === 'error') {
      log.warn(`Failed to show main window: ${JSON.stringify(showResult.error)}`);
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
  budgetState: { load: () => Promise<void>; hasConfig: boolean; loadMonthlyRecords: (year: number) => Promise<void> };
}): Promise<void> {
  const versionResult = await commands.getAppVersion();
  appState.setVersion(versionResult);

  const initResult = await commands.initDatabase();
  if (initResult.status === 'error') {
    throw new Error(
      typeof initResult.error === 'string'
        ? initResult.error
        : JSON.stringify(initResult.error) || 'Database initialization failed'
    );
  }

  const initialFinanceYear = getInitialFinanceYear();

  await Promise.all([
    opts.dashboardState.load(),
    collectionStore.fetch(),
    opts.wishlistState.fetchWishlists(),
    financeState.ensureLoaded()
  ]);

  await opts.budgetState.load();
  if (opts.budgetState.hasConfig) {
    await opts.budgetState.loadMonthlyRecords(initialFinanceYear);
  }
}
