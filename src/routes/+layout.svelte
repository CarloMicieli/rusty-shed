<script lang="ts">
  import './layout.css';
  import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
  import BottomNavigation from '$lib/components/BottomNavigation.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';
  import { Bell, TrainFront } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { setAppVersion } from '$lib/stores/app';
  import { collectionStore } from '$lib/stores/collectionStore.svelte';
  import { wishlistService } from '$lib/stores/WishlistService.svelte';
  import ToastHost from '$lib/components/ToastHost.svelte';
  import { safeInvoke } from '$lib/services';

  let loading = $state(true);
  let { children } = $props();

  onMount(async () => {
    // Run native DB init before showing the main window
    try {
      const initResult = await safeInvoke<void>('init_database');
      if (!initResult.ok) {
        const message =
          initResult.error?.message ?? String(initResult.error ?? 'Database initialization failed');
        throw new Error(message);
      }

      const showResult = await safeInvoke<void>('show_main_window');
      if (!showResult.ok) {
        const message =
          showResult.error?.message ?? String(showResult.error ?? 'Failed to show main window');
        throw new Error(message);
      }
    } catch (err) {
      console.error('Startup failed', err);
    } finally {
      loading = false;
    }

    // Preload collection for nav badges
    void collectionStore.fetchCollection();
    void wishlistService.fetchWishlists();

    // Fetch app version using service layer
    try {
      const result = await safeInvoke<string>('get_app_version');
      if (result.ok) {
        setAppVersion(result.data);
      }
    } catch {
      // Ignore version fetch errors silently
    }
  });
</script>

{#if loading}
  <div
    class="bg-background flex h-screen w-full flex-col items-center justify-center overflow-hidden font-sans text-surface-50 selection:bg-primary-500/30"
    in:fade
    out:fade
  >
    <div class="flex flex-col items-center gap-4">
      <div class="flex items-center gap-3">
        <div
          class="border-accent-500 h-10 w-10 animate-spin rounded-full border-2 border-t-transparent"
        ></div>
        <div class="flex items-center gap-2">
          <TrainFront class="text-accent-500" size={28} />
          <span class="text-lg font-semibold tracking-wide">Rusty Shed</span>
        </div>
      </div>
      <p class="text-sm text-surface-300">Preparing your collection...</p>
    </div>
  </div>
{:else}
  <div
    class="bg-background flex h-screen w-full flex-col overflow-hidden font-sans text-surface-50 selection:bg-primary-500/30 lg:flex-row"
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
        class="bg-background/80 sticky top-0 z-40 border-b border-surface-700/50 backdrop-blur-md"
        data-tauri-drag-region
      >
        <div class="flex items-center justify-between p-4">
          <!-- Mobile: Logo / Brand (Visible only when sidebar is hidden) -->
          <div class="flex items-center gap-2 lg:hidden">
            <TrainFront class="text-accent-500" size={24} />
            <span class="text-sm font-bold tracking-widest uppercase">Rusty Shed</span>
          </div>

          <!-- Desktop: Spacer (Sidebar handles branding) -->
          <div class="hidden lg:block">
            <h2 class="h3 font-bold">Dashboard</h2>
          </div>

          <!-- Right Actions -->
          <div class="flex items-center gap-4">
            <SearchBar />

            <button class="variant-ghost-surface relative btn-icon btn-icon-sm">
              <Bell size={20} />
              <span class="absolute top-0 right-0 h-2 w-2 animate-pulse rounded-full bg-error-500"
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
      <ToastHost />
    </div>
  </div>
{/if}
