<script lang="ts">
  import { setContext } from 'svelte';
  import { beforeNavigate } from '$app/navigation';
  import { listen } from '@tauri-apps/api/event';
  import AcquisitionDrawer from '$lib/features/acquisition/AcquisitionDrawer.svelte';
  import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';
  import LogMaintenanceDrawer from '$lib/features/maintenance/components/LogMaintenanceDrawer.svelte';
  import { collectionState } from '$lib/features/collection/CollectionState.svelte';
  import { collectionStore } from '$lib/state/collection.svelte';
  import { getDashboardContext } from '$lib/features/dashboard/DashboardState.svelte';

  let { children }: { children: import('svelte').Snippet } = $props();

  let showAcquisitionDrawer = $state(false);
  let showWishlistDrawer = $state(false);
  let showLogMaintenanceDrawer = $state(false);

  const dashboardState = getDashboardContext();

  // Close all drawers when the user navigates to another page.
  if (typeof beforeNavigate === 'function') {
    beforeNavigate(() => {
      showAcquisitionDrawer = false;
      showWishlistDrawer = false;
      showLogMaintenanceDrawer = false;
    });
  }

  // Expose openers so any descendant can open a drawer via context.
  setContext('openAcquisitionDrawer', () => {
    showAcquisitionDrawer = true;
  });
  setContext('openWishlistDrawer', () => {
    showWishlistDrawer = true;
  });
  setContext('openLogMaintenanceDrawer', () => {
    showLogMaintenanceDrawer = true;
  });

  // Also listen for Tauri backend events that request a drawer to open.
  $effect(() => {
    let cancelled = false;
    let unlistenAcquisition: (() => void) | undefined;
    let unlistenMaintenance: (() => void) | undefined;

    void (async () => {
      try {
        const u1 = await listen('open-acquisition-drawer', () => {
          showAcquisitionDrawer = true;
        });
        const u2 = await listen('open-maintenance-drawer', () => {
          showLogMaintenanceDrawer = true;
        });

        if (cancelled) {
          u1();
          u2();
          return;
        }

        unlistenAcquisition = u1;
        unlistenMaintenance = u2;
      } catch {
        // Tauri not available (e.g. test environment) — skip listener setup.
      }
    })();

    return () => {
      cancelled = true;
      unlistenAcquisition?.();
      unlistenMaintenance?.();
    };
  });
</script>

{@render children()}

{#if showAcquisitionDrawer}
  <AcquisitionDrawer
    open={showAcquisitionDrawer}
    onClose={() => (showAcquisitionDrawer = false)}
    onSuccess={() => {
      showAcquisitionDrawer = false;
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
    onClose={() => (showWishlistDrawer = false)}
    onSaved={() => (showWishlistDrawer = false)}
  />
{/if}

{#if showLogMaintenanceDrawer}
  <LogMaintenanceDrawer
    open={showLogMaintenanceDrawer}
    onClose={() => (showLogMaintenanceDrawer = false)}
  />
{/if}
