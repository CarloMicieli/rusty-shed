<script lang="ts">
  import '../app.css';
  import './layout.css';
  import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
  import BottomNavigation from '$lib/components/BottomNavigation.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import SignalFailureView from '$lib/components/signal-failure/SignalFailureView.svelte';
  import { Bell, TrainFront } from 'lucide-svelte';
  import { fade } from 'svelte/transition';
  import { setAppVersion } from '$lib/stores/app';
  import { themeStore } from '$lib/stores/themeStore.svelte';
  import { log } from '$lib/tauri-logger';
  import * as m from '$lib/paraglide/messages.js';
  import { generateErrorId } from '$lib/services/error-id';
  import {
    createCollectionState,
    setCollectionContext
  } from '$lib/features/collection/CollectionState.svelte';
  import {
    createWishlistState,
    setWishlistContext
  } from '$lib/features/wishlists/WishlistState.svelte';
  import {
    createDashboardState,
    setDashboardContext
  } from '$lib/features/dashboard/DashboardState.svelte';
  import { createDepotState, setDepotContext } from '$lib/features/depot/DepotState.svelte';
  import { TrackInventoryService, setTrackInventoryContext } from '$lib/features/track-inventory';
  import { Toaster } from '$lib/components/ui/sonner';
  import { safeInvoke } from '$lib/services';
  import { onMount } from 'svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { collectionStore } from '$lib/state/collection.svelte';

  let loading = $state(true);
  let error = $state<string | null>(null);
  let { children } = $props();

  // Create and provide contexts
  const collectionState = createCollectionState();
  const wishlistState = createWishlistState();
  const dashboardState = createDashboardState();
  const depotState = createDepotState();
  const trackInventoryService = new TrackInventoryService();

  setCollectionContext(collectionState);
  setWishlistContext(wishlistState);
  setDashboardContext(dashboardState);
  setDepotContext(depotState);
  setTrackInventoryContext(trackInventoryService);

  onMount(async () => {
    // 0. Initialize settings on first run (detects OS language)
    try {
      await settingsState.initialize();
    } catch (err) {
      log.warn(`Failed to initialize settings, using defaults: ${String(err)}`);
    }

    // 1. Initialize theme from settings
    await themeStore.initializeFromSettings();

    // 2. Show main window immediately so the user sees *something* (loading state)
    // We don't block on this failing, but log it if it does.
    safeInvoke<void>('show_main_window').then((res) => {
      if (!res.ok) log.warn(`Failed to show main window: ${String(res.error)}`);
    });

    // Ensure the initial app-loading spinner from app.html is removed
    const loader = document.getElementById('app-loading');
    if (loader) {
      loader.remove();
    }

    try {
      // 3. Fetch app version (non-critical, but good to have early)
      const versionResult = await safeInvoke<string>('get_app_version');
      if (versionResult.ok) {
        setAppVersion(versionResult.data);
      }

      // 4. Initialize Database (Critical)
      const initResult = await safeInvoke<void>('init_database');
      if (!initResult.ok) {
        const message =
          initResult.error?.message ?? String(initResult.error ?? 'Database initialization failed');
        throw new Error(message);
      }

      // 5. Preload data (only if DB is ready)
      await Promise.all([collectionStore.fetch(), wishlistState.fetchWishlists()]);
    } catch (err) {
      log.error(`Startup failed: ${String(err)}`);
      // Capture the error to show in the UI
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
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
    in:fade
    out:fade
  >
    <div class="flex flex-col items-center gap-4">
      <div class="flex items-center gap-3">
        <div
          class="h-10 w-10 animate-spin rounded-full border-2 border-primary border-t-transparent"
        ></div>
        <div class="flex items-center gap-2">
          <TrainFront class="text-primary" size={28} />
          <span class="text-lg font-semibold tracking-wide">Rusty Shed</span>
        </div>
      </div>
      <p class="text-sm text-muted-foreground">Preparing your collection...</p>
    </div>
  </div>
{:else}
  <div
    class="flex h-screen w-full flex-col overflow-hidden bg-background font-sans text-foreground lg:flex-row"
    in:fade
  >
    <!-- Sidebar Left (Desktop) -->
    <div class="hidden h-full w-0 shrink-0 transition-all duration-300 lg:block lg:w-64">
      <SidebarNavigation />
    </div>

    <!-- Main Content Wrapper -->
    <div class="relative flex h-full min-w-0 flex-1 flex-col">
      <!-- Header -->
      <header
        class="sticky top-0 z-40 border-b border-border bg-background/80 backdrop-blur-md"
        data-tauri-drag-region
      >
        <div class="flex items-center justify-between p-4">
          <!-- Mobile: Logo / Brand (Visible only when sidebar is hidden) -->
          <div class="flex items-center gap-2 lg:hidden">
            <TrainFront class="text-primary" size={24} />
            <span class="text-sm font-bold tracking-widest uppercase">Rusty Shed</span>
          </div>

          <!-- Right Actions -->
          <div class="ml-auto flex items-center gap-4">
            <SearchBar />

            <button
              class="relative rounded-md p-2 hover:bg-accent hover:text-accent-foreground"
              aria-label="Notifications"
            >
              <Bell size={20} />
              <span class="absolute top-1 right-1 h-2 w-2 animate-pulse rounded-full bg-destructive"
              ></span>
            </button>
          </div>
        </div>
      </header>

      <!-- Page Content -->
      <main
        class="mx-auto w-full max-w-[1600px] flex-1 space-y-8 overflow-y-auto p-4 pb-24 lg:p-8 lg:pb-8"
      >
        {@render children()}
      </main>

      <!-- Footer / Bottom Nav -->
      <BottomNavigation />
      <Toaster richColors position="top-right" />
    </div>
  </div>
{/if}
