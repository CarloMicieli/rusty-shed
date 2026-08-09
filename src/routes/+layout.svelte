<script lang="ts">
  import { page } from '$app/state';
  import '../app.css';
  import './layout.css';
  import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
  import BottomNavigation from '$lib/components/BottomNavigation.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import SignalFailureView from '$lib/components/signal-failure/SignalFailureView.svelte';
  import GlobalDrawers from '$lib/components/GlobalDrawers.svelte';
  import { TrainFront } from 'lucide-svelte';
  import { fade } from 'svelte/transition';
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
  import { onMount } from 'svelte';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { financeState } from '$lib/state/finance.svelte';
  import WelcomeWizard from '$lib/features/onboarding/WelcomeWizard.svelte';
  import { completeOnboardingStatus } from '$lib/features/onboarding/onboarding-state.svelte';
  import { bootstrapApp, postOnboardingBoot } from '$lib/features/app/AppBootstrap.svelte';

  type LayoutViewState = 'loading' | 'needs-onboarding' | 'ready' | 'error';

  let viewState = $state<LayoutViewState>('loading');
  let needsOnboarding = $state(false);
  let error = $state<string | null>(null);
  let sidebarCollapsed = $state(false);
  let { children } = $props();

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

  // Subscribe to finance events; Tauri drawer events are handled inside GlobalDrawers.
  $effect(() => {
    let cancelled = false;

    void (async () => {
      if (!cancelled) {
        await financeState.startListening();
      }
    })();

    return () => {
      cancelled = true;
      financeState.stopListening();
    };
  });

  onMount(() => {
    void (async () => {
      const result = await bootstrapApp({
        onBridgeReady: () => {
          const loader = document.getElementById('app-loading');
          if (loader) loader.remove();
        },
        dashboardState,
        wishlistState,
        budgetState
      });

      if (result.status === 'needs-onboarding') {
        needsOnboarding = true;
        viewState = 'needs-onboarding';
      } else if (result.status === 'error') {
        error = result.message;
        viewState = 'error';
      } else {
        viewState = 'ready';
      }
    })();
  });
</script>

{#if viewState === 'error'}
  <SignalFailureView
    errorId={generateErrorId()}
    moduleLabel={m.module_label_signal_box()}
    onReset={() => {
      window.location.href = '/';
    }}
  />
{:else if viewState === 'loading'}
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
{:else if viewState === 'needs-onboarding'}
  <WelcomeWizard
    onComplete={async () => {
      needsOnboarding = false;
      viewState = 'loading';
      try {
        await completeOnboardingStatus(settingsState);
        await postOnboardingBoot({ dashboardState, wishlistState, budgetState });
        viewState = 'ready';
      } catch (err) {
        error = err instanceof Error ? err.message : String(err);
        viewState = 'error';
      }
    }}
  />
{:else if viewState === 'ready'}
  <Tooltip.Provider>
    <div
      class="flex h-screen w-full flex-col overflow-hidden font-sans text-foreground lg:flex-row"
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
              <span class="text-sm font-bold tracking-widest uppercase">{m.app_name()}</span>
            </div>

            <!-- Right Actions -->
            <div class="ml-auto flex items-center gap-4">
              <SearchBar />
            </div>
          </div>
        </header>

        <!-- Page Content -->
        <main class="relative flex-1 overflow-hidden">
          <div class="h-full w-full max-w-full overflow-y-auto p-4 pb-24 lg:p-8 lg:pb-8">
            {#key page.url.pathname}
              <div in:fade={{ duration: 150, delay: 1 }} class="space-y-8">
                <GlobalDrawers>
                  {@render children()}
                </GlobalDrawers>
              </div>
            {/key}
          </div>
        </main>

        <!-- Footer / Bottom Nav -->
        <BottomNavigation />
        <Toaster richColors position="top-right" />
      </div>
    </div>
  </Tooltip.Provider>
{/if}
