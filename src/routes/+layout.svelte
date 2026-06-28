<script lang="ts">
  import { page } from '$app/stores';
  import { beforeNavigate } from '$app/navigation';
  import '../app.css';
  import './layout.css';
  import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
  import BottomNavigation from '$lib/components/BottomNavigation.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import SignalFailureView from '$lib/components/signal-failure/SignalFailureView.svelte';
  import { TrainFront } from 'lucide-svelte';
  import { fade } from 'svelte/transition';
  import { appState } from '$lib/stores/app.svelte';
  import { themeState } from '$lib/stores/themeStore.svelte';
  import { log } from '$lib/tauri-logger';
  import * as m from '$lib/paraglide/messages.js';
  import { generateErrorId } from '$lib/services/error-id';
  import {
    createWishlistState,
    setWishlistContext
  } from '$lib/features/wishlists/WishlistState.svelte';
  import {
    createDashboardState,
    setDashboardContext
  } from '$lib/features/dashboard/DashboardState.svelte';
  import { createBudgetState } from '$lib/features/budget/BudgetState.svelte';
  import { createBudgetService } from '$lib/features/budget/services/BudgetService.svelte';
  import { createDepotState, setDepotContext } from '$lib/features/depot/DepotState.svelte';
  import { TrackInventoryService, setTrackInventoryContext } from '$lib/features/track-inventory';
  import { Toaster } from '$lib/components/ui/sonner';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import { safeInvoke } from '$lib/services';
  import { onMount, setContext } from 'svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { collectionState } from '$lib/features/collection/CollectionState.svelte';
  import { collectionStore } from '$lib/state/collection.svelte';
  import { listen } from '@tauri-apps/api/event';
  import AcquisitionDrawer from '$lib/features/acquisition/AcquisitionDrawer.svelte';
  import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';
  import LogMaintenanceDrawer from '$lib/features/maintenance/components/LogMaintenanceDrawer.svelte';
  import { financeState } from '$lib/state/finance.svelte';
  import { drawerRegistry } from '$lib/state/drawer-registry.svelte';
  import { createMobileMatchMediaState } from '$lib/state/match-media.svelte';
  import { createPageTitleState, setPageTitleContext } from '$lib/state/page-title.svelte';
  import WelcomeWizard from '$lib/features/onboarding/WelcomeWizard.svelte';
  import {
    bootstrapNeedsOnboarding,
    completeOnboardingStatus
  } from '$lib/features/onboarding/onboarding-state.svelte';

  let loading = $state(true);
  let needsOnboarding = $state(false);
  let error = $state<string | null>(null);
  let showAcquisitionDrawer = $state(false);
  let showWishlistDrawer = $state(false);
  let showLogMaintenanceDrawer = $state(false);
  let sidebarCollapsed = $state(false);
  let isMobileViewport = $state(false);
  let { children } = $props();

  const LAYOUT_DRAWER_IDS = {
    acquisition: 'layout:acquisition',
    wishlist: 'layout:wishlist',
    maintenance: 'layout:maintenance'
  } as const;

  const mobileMedia = createMobileMatchMediaState();
  const pageTitleState = createPageTitleState();
  const mobileHeaderTitle = $derived(pageTitleState.title ?? m.app_name());

  setPageTitleContext(pageTitleState);

  function syncLayoutDrawerFlags(): void {
    const activeIds = new Set(drawerRegistry.stack.map((layer) => layer.id));
    showAcquisitionDrawer = activeIds.has(LAYOUT_DRAWER_IDS.acquisition);
    showWishlistDrawer = activeIds.has(LAYOUT_DRAWER_IDS.wishlist);
    showLogMaintenanceDrawer = activeIds.has(LAYOUT_DRAWER_IDS.maintenance);
  }

  function openLayoutDrawer(id: string): void {
    drawerRegistry.openParent(id);
    syncLayoutDrawerFlags();
  }

  function closeLayoutDrawer(id: string): void {
    drawerRegistry.closeById(id, 'button');
    syncLayoutDrawerFlags();
  }

  const constrainedPagePrefixes: string[] = [];

  const useConstrainedPageContent = $derived(
    constrainedPagePrefixes.some(
      (prefix) => $page.url.pathname === prefix || $page.url.pathname.startsWith(`${prefix}/`)
    )
  );

  // Close all layout-level drawers when the user navigates to another page
  beforeNavigate((navigation) => {
    if (navigation.type === 'popstate' && drawerRegistry.depth > 0) {
      drawerRegistry.closeTop('back');
      syncLayoutDrawerFlags();
      navigation.cancel();
      return;
    }

    drawerRegistry.clear('button');
    syncLayoutDrawerFlags();
  });

  // Expose open function so any child route can open the acquisition drawer
  setContext('openAcquisitionDrawer', () => {
    openLayoutDrawer(LAYOUT_DRAWER_IDS.acquisition);
  });

  // Expose open function so any child route can open the wishlist item drawer
  setContext('openWishlistDrawer', () => {
    openLayoutDrawer(LAYOUT_DRAWER_IDS.wishlist);
  });

  // Expose open function so any child route can open the log maintenance drawer
  setContext('openLogMaintenanceDrawer', () => {
    openLayoutDrawer(LAYOUT_DRAWER_IDS.maintenance);
  });

  // Create and provide contexts
  const wishlistState = createWishlistState();
  const dashboardState = createDashboardState();
  const budgetService = createBudgetService();
  const budgetState = createBudgetState(budgetService);
  const depotState = createDepotState();
  const trackInventoryService = new TrackInventoryService();

  setWishlistContext(wishlistState);
  setDashboardContext(dashboardState);
  setDepotContext(depotState);
  setTrackInventoryContext(trackInventoryService);

  const FINANCE_SELECTED_YEAR_STORAGE_KEY = 'finance:selected-year';

  type TauriAwareWindow = Window & {
    __TAURI_INTERNALS__?: unknown;
  };

  function hasTauriBridge(): boolean {
    return (
      typeof window !== 'undefined' && (window as TauriAwareWindow).__TAURI_INTERNALS__ != null
    );
  }

  function isTauriHosted(): boolean {
    return typeof window !== 'undefined' && window.location.host === 'tauri.localhost';
  }

  async function waitForTauriBridge(timeoutMs = 4_000, pollMs = 50): Promise<boolean> {
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

  function getInitialFinanceYear(): number {
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

  // Manage Tauri event listeners and finance state subscription using $effect
  // so setup and cleanup are co-located.
  $effect(() => {
    drawerRegistry.setSourceRoute($page.url.pathname);
  });

  $effect(() => {
    const unsubscribe = mobileMedia.subscribe((matches) => {
      isMobileViewport = matches;
    });

    return () => {
      unsubscribe();
    };
  });

  $effect(() => {
    let cancelled = false;
    let unlistenAcquisition: (() => void) | undefined;
    let unlistenMaintenance: (() => void) | undefined;

    const setupListeners = async () => {
      try {
        const u1 = await listen('open-acquisition-drawer', () => {
          openLayoutDrawer(LAYOUT_DRAWER_IDS.acquisition);
        });
        const u2 = await listen('open-maintenance-drawer', () => {
          openLayoutDrawer(LAYOUT_DRAWER_IDS.maintenance);
        });

        // If the effect was torn down while we were awaiting, clean up immediately.
        if (cancelled) {
          u1();
          u2();
          return;
        }

        unlistenAcquisition = u1;
        unlistenMaintenance = u2;
      } catch {
        // Tauri not available (e.g. test environment) — skip listener
      }

      if (!cancelled) {
        await financeState.startListening();
      }
    };

    setupListeners();

    return () => {
      cancelled = true;
      unlistenAcquisition?.();
      unlistenMaintenance?.();
      financeState.stopListening();
    };
  });

  onMount(() => {
    const onPopState = () => {
      if (drawerRegistry.depth === 0) {
        return;
      }

      drawerRegistry.closeTop('back');
      syncLayoutDrawerFlags();
    };

    window.addEventListener('popstate', onPopState);

    void (async () => {
      try {
        if (isTauriHosted()) {
          const bridgeReady = await waitForTauriBridge();
          if (!bridgeReady) {
            throw new Error('Tauri bridge did not become ready during startup');
          }
        }

        // Show the window before any backend work so Android never stays on a blank surface
        // if settings initialization or IPC setup is slow.
        const showWindowResult = await safeInvoke<void>('show_main_window');
        if (!showWindowResult.ok) {
          log.warn(`Failed to show main window: ${String(showWindowResult.error)}`);
        }

        // Ensure the initial app-loading spinner from app.html is removed promptly.
        const loader = document.getElementById('app-loading');
        if (loader) {
          loader.remove();
        }

        loading = false;

        // 0. Initialize settings and derive onboarding status.
        await settingsState.initialize();
        needsOnboarding = bootstrapNeedsOnboarding(settingsState.settings);

        // 0a. Detect OS locale for regional formatting.
        await regionalManager.init();

        // 1. Initialize theme from settings.
        await themeState.initializeFromSettings();

        if (needsOnboarding) {
          return;
        }

        // 3. Fetch app version (non-critical, but good to have early).
        const versionResult = await safeInvoke<string>('get_app_version');
        if (versionResult.ok) {
          appState.setVersion(versionResult.data);
        }

        // 4. Initialize Database (Critical).
        const initResult = await safeInvoke<void>('init_database');
        if (!initResult.ok) {
          const message =
            initResult.error?.message ??
            String(initResult.error ?? 'Database initialization failed');
          throw new Error(message);
        }

        const initialFinanceYear = getInitialFinanceYear();
        await Promise.all([
          dashboardState.load(),
          collectionStore.fetch(),
          wishlistState.fetchWishlists(),
          financeState.ensureLoaded()
        ]);

        await budgetState.load();
        if (budgetState.hasConfig) {
          await budgetState.loadMonthlyRecords(initialFinanceYear);
        }
      } catch (err) {
        log.error(`Startup failed: ${String(err)}`);
        // Capture the error to show in the UI.
        error = err instanceof Error ? err.message : String(err);
        loading = false;
      }
    })();

    return () => {
      window.removeEventListener('popstate', onPopState);
      mobileMedia.destroy();
    };
  });
</script>

{#if error}
  <SignalFailureView
    errorId={generateErrorId()}
    moduleLabel={m.module_label_signal_box()}
    onReset={() => {
      window.location.href = '/';
    }}
  />
{:else if loading}
  <div
    class="flex h-screen w-full flex-col items-center justify-center overflow-hidden bg-background font-sans text-foreground"
    in:fade={{ delay: 1 }}
    out:fade
  >
    <div class="flex flex-col items-center gap-4">
      <div class="flex items-center gap-3">
        <div
          class="h-10 w-10 animate-spin rounded-full border-2 border-primary border-t-transparent"
        ></div>
        <div class="flex items-center gap-2">
          <TrainFront class="text-primary" size={28} />
          <span class="text-lg font-semibold tracking-wide">{m.app_name()}</span>
        </div>
      </div>
      <p class="text-sm text-muted-foreground">{m.app_loading_message()}</p>
    </div>
  </div>
{:else if needsOnboarding}
  <WelcomeWizard
    onComplete={async () => {
      needsOnboarding = false;
      loading = true;
      try {
        await completeOnboardingStatus(settingsState);

        const versionResult = await safeInvoke<string>('get_app_version');
        if (versionResult.ok) {
          appState.setVersion(versionResult.data);
        }

        const initResult = await safeInvoke<void>('init_database');
        if (!initResult.ok) {
          const message =
            initResult.error?.message ??
            String(initResult.error ?? 'Database initialization failed');
          throw new Error(message);
        }

        const initialFinanceYear = getInitialFinanceYear();
        await Promise.all([
          collectionStore.fetch(),
          wishlistState.fetchWishlists(),
          financeState.ensureLoaded()
        ]);

        await budgetState.load();
        if (budgetState.hasConfig) {
          await budgetState.loadMonthlyRecords(initialFinanceYear);
        }
      } catch (err) {
        error = err instanceof Error ? err.message : String(err);
      } finally {
        loading = false;
      }
    }}
  />
{:else}
  <Tooltip.Provider>
    <div
      class="safe-area-pad flex h-screen w-full flex-col overflow-hidden font-sans text-foreground lg:flex-row"
      style:background-color="var(--sidebar)"
      in:fade={{ delay: 1 }}
    >
      <!-- Sidebar Left (Desktop) -->
      <div
        class="relative z-50 hidden h-full shrink-0 overflow-hidden lg:block"
        style:width={sidebarCollapsed ? '64px' : '256px'}
        style="transition: width 0.35s cubic-bezier(0.34, 1.56, 0.64, 1);"
      >
        <SidebarNavigation
          collapsed={sidebarCollapsed}
          onToggle={() => (sidebarCollapsed = !sidebarCollapsed)}
        />
      </div>

      <!-- Main Content Wrapper ("sheet" that sits on the sidebar shelf) -->
      <div
        class="relative flex h-full min-w-0 flex-1 flex-col overflow-hidden"
        style:border-top-left-radius="var(--layout-header-radius)"
        style:background-color="var(--background)"
      >
        <!-- Header -->
        <header
          class="sticky top-0 z-50 border-b"
          style:background-color="var(--background)"
          style:border-color="var(--layout-border)"
          data-tauri-drag-region
        >
          <div class="flex items-center justify-between p-4" style="pointer-events: auto;">
            <!-- Mobile: Logo / Brand (Visible only when sidebar is hidden) -->
            <div class="flex items-center gap-2 lg:hidden">
              <TrainFront class="text-primary" size={24} />
              <span class="max-w-[70vw] truncate text-sm font-bold tracking-widest uppercase">
                {mobileHeaderTitle}
              </span>
            </div>

            <!-- Right Actions -->
            <div class="ml-auto flex items-center gap-4">
              <SearchBar />
            </div>
          </div>
        </header>

        <!-- Page Content -->
        <main class="relative flex-1 overflow-hidden">
          <div
            class="h-full w-full max-w-full overflow-y-auto p-4 pb-24 lg:p-8 lg:pb-8"
            class:safe-area-pad-bottom-nav={isMobileViewport}
          >
            {#key $page.url.pathname}
              <div
                in:fade={{ duration: 150, delay: 1 }}
                class={['space-y-8', useConstrainedPageContent && 'page-content-constrained']}
              >
                {@render children()}
              </div>
            {/key}
          </div>
        </main>

        <!-- Footer / Bottom Nav -->
        <BottomNavigation />
        <Toaster richColors position="top-right" />
      </div>
    </div>

    {#if showAcquisitionDrawer}
      <AcquisitionDrawer
        open={showAcquisitionDrawer}
        onClose={() => closeLayoutDrawer(LAYOUT_DRAWER_IDS.acquisition)}
        onSuccess={() => {
          closeLayoutDrawer(LAYOUT_DRAWER_IDS.acquisition);
          void collectionStore.refresh();
          void collectionState.forceRefresh();
          void dashboardState.load();
          void dashboardState.loadBudget();
        }}
      />
    {/if}

    {#if showWishlistDrawer}
      <AddWishlistItemDrawer
        open={showWishlistDrawer}
        onClose={() => closeLayoutDrawer(LAYOUT_DRAWER_IDS.wishlist)}
        onSaved={() => closeLayoutDrawer(LAYOUT_DRAWER_IDS.wishlist)}
      />
    {/if}

    {#if showLogMaintenanceDrawer}
      <LogMaintenanceDrawer
        open={showLogMaintenanceDrawer}
        onClose={() => closeLayoutDrawer(LAYOUT_DRAWER_IDS.maintenance)}
      />
    {/if}
  </Tooltip.Provider>
{/if}
