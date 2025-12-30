<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { Heart, Plus, RefreshCw, House } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  // Components
  import StatsCard from '$lib/components/StatsCard.svelte';
  import QuickActionButtons, { type QuickAction } from '$lib/components/QuickActionButtons.svelte';
  import RecentItemCard from '$lib/components/RecentItemCard.svelte';
  import DepotView from '$lib/components/DepotView.svelte';
  import AddWishlistItemModal from '$lib/components/AddWishlistItemModal.svelte';

  // Stores
  import { dashboardStore } from '$lib/stores/dashboardStore.svelte';
  import { wishlistService } from '$lib/stores/WishlistService.svelte';

  const dashboard = dashboardStore;

  // Data derived from store
  const totals = $derived(dashboard.data?.totals ?? null);
  const stats = $derived(byStats(totals));
  const recent = $derived(dashboard.data?.recent_items ?? []);
  const depot = $derived(dashboard.data?.depot_items ?? []);

  let showWishlistModal = $state(false);

  onMount(() => {
    void dashboard.load();
  });

  // Actionable logic
  function handleRetry() {
    void dashboard.retry();
  }

  // ESCAPE HATCH: Redirects to root with error_reset flag to prevent infinite loops
  function handleReturn() {
    goto(resolve('/'));
  }

  function formatMoney(amount?: { amount: number; currency: string } | null) {
    if (!amount) return '—';
    const major = amount.amount / 100;
    return `${amount.currency} ${major.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
  }

  function byStats(data: typeof totals) {
    const collectionValue = formatMoney(data?.total_value ?? null);
    const rollingStocks = data?.collection_items ?? 0;
    const maintenance = data?.maintenance_due ?? 0;

    return [
      {
        label: m.stats_total_collection_value(),
        value: collectionValue,
        trend: 'neutral' as const,
        trendValue: ''
      },
      {
        label: m.stats_rolling_stocks(),
        value: `${rollingStocks}`,
        trend: 'neutral' as const,
        trendValue: ''
      },
      {
        label: m.stats_maintenance_alerts(),
        value: `${maintenance}`,
        trend: maintenance > 0 ? ('down' as const) : ('neutral' as const),
        trendValue: maintenance > 0 ? `${maintenance} ${m.dashboard_due()}` : '—'
      }
    ];
  }

  const actions = $derived<QuickAction[]>([
    {
      id: 'add-railway-model',
      label: m.actions_add_railway_model(),
      icon: Plus,
      onClick: () => goto(resolve('/catalogue/new-model'))
    },
    {
      id: 'add-wishlist-item',
      label: m.actions_add_wishlist_item(),
      icon: Heart,
      onClick: () => {
        if (!wishlistService.wishlists.length) void wishlistService.fetchWishlists();
        showWishlistModal = true;
      }
    }
  ]);
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_dashboard()}</title>
</svelte:head>

{#if dashboard.error}
  <div
    class="variant-soft-error flex flex-col items-center justify-center rounded-container border border-error-500/30 p-12 text-center"
  >
    <div class="variant-filled-error mb-4 badge-icon h-12 w-12"><RefreshCw /></div>
    <h2 class="h2 font-bold">{m.errors_dashboard_title()}</h2>
    <p class="mt-2 text-surface-200">{m.errors_dashboard_message()}</p>
    <div class="mt-6 flex gap-4">
      <button class="variant-filled-primary btn btn-lg" onclick={handleRetry}>
        {m.errors_retry_page()}
      </button>
      <button class="variant-ghost-surface btn btn-lg" onclick={handleReturn}>
        <House class="mr-2 h-4 w-4" />
        {m.errors_return_dashboard()}
      </button>
    </div>
  </div>
{:else}
  <div class="space-y-8">
    <section>
      <div class="mb-4 flex items-center justify-between">
        <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
          {m.dashboard_yard_statistics()}
        </h3>
        {#if totals?.maintenance_due}
          <span class="variant-soft-error badge animate-pulse font-semibold"
            >{totals.maintenance_due} {m.dashboard_due_soon()}</span
          >
        {/if}
      </div>

      <div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
        {#if dashboard.isLoading}
          {#each Array(4) as _item, index (index)}<div
              class="skeleton h-28 rounded-container"
            ></div>{/each}
        {:else}
          {#each stats as stat (stat.label)}
            <StatsCard {stat} />
          {/each}
        {/if}
      </div>
    </section>

    <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
      <div class="space-y-8 lg:col-span-2">
        <section>
          <div class="mb-4 flex items-center justify-between">
            <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
              {m.dashboard_recently_added()}
            </h3>
            <a href={resolve('/my-collection')} class="text-accent-500 text-sm font-bold hover:underline"
              >{m.dashboard_view_all()}</a
            >
          </div>

          {#if dashboard.isLoading}
            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
              {#each Array(2) as _item, index (index)}<div
                  class="skeleton aspect-video w-full rounded-container"
                ></div>{/each}
            </div>
          {:else if !recent.length}
            <div
              class="variant-soft-surface rounded-container border border-dashed border-surface-700/60 p-10 text-center text-surface-300"
            >
              <p>{m.dashboard_empty_recent()}</p>
            </div>
          {:else}
            <div
              class="hide-scrollbar flex snap-x snap-mandatory gap-4 overflow-x-auto pb-4 lg:grid lg:grid-cols-2"
            >
              {#each recent as item (item.id)}
                <div class="min-w-[80%] snap-center lg:min-w-0"><RecentItemCard {item} /></div>
              {/each}
            </div>
          {/if}
        </section>

        <section>
          <div class="mb-4 flex items-center justify-between">
            <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
              {m.dashboard_the_depot()}
            </h3>
          </div>

          {#if !dashboard.isLoading && depot.length === 0}
            <div class="card border-2 border-dashed border-surface-500/20 p-8 text-center">
              <p class="mb-4 text-surface-300">{m.dashboard_empty_depot()}</p>
              <button
                class="variant-filled-secondary btn"
                onclick={() => goto(resolve('/catalogue/new-model'))}
              >
                <Plus class="mr-2" />
                {m.actions_add_railway_model()}
              </button>
            </div>
          {:else}
            <DepotView data={depot} isLoading={dashboard.isLoading} />
          {/if}
        </section>
      </div>

      <aside>
        <div class="sticky top-24 space-y-4">
          <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
            {m.dashboard_quick_actions()}
          </h3>
          <QuickActionButtons {actions} />
        </div>
      </aside>
    </div>
  </div>
{/if}

{#if showWishlistModal}
  <AddWishlistItemModal on:close={() => (showWishlistModal = false)} />
{/if}
