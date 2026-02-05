<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { Heart, Plus, RefreshCw, House, Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { toaster } from '$lib/toaster';
  import { Button } from '$lib/components';

  // Components
  import PageHeader from '$lib/components/PageHeader.svelte';
  import StatsCard from '$lib/components/StatsCard.svelte';
  import QuickActionButtons, { type QuickAction } from '$lib/components/QuickActionButtons.svelte';
  import RecentItemCard from '$lib/components/RecentItemCard.svelte';
  import DepotView from '$lib/components/DepotView.svelte';
  import AddWishlistItemModal from '$lib/components/AddWishlistItemModal.svelte';
  import { DashboardCharts } from '$lib/features/dashboard';

  // Stores
  import { getDashboardContext } from '$lib/features/dashboard/DashboardState.svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';

  const dashboard = getDashboardContext();
  const wishlistService = getWishlistContext();

  // Data derived from store
  const totals = $derived(dashboard.data?.totals ?? null);
  const stats = $derived(byStats(totals));
  const recent = $derived(dashboard.data?.recentItems ?? []);
  const depot = $derived(dashboard.data?.depotItems ?? []);

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

  function formatMoney(amount?: { amount: bigint; currency: string } | null) {
    if (!amount) return '—';
    const major = Number(amount.amount) / 100;
    return `${amount.currency} ${major.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
  }

  function byStats(data: typeof totals) {
    const collectionValue = formatMoney(data?.totalValue ?? null);
    const rollingStocks = data?.collectionItems ?? 0;
    const maintenance = data?.maintenanceDue ?? 0;

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
    },
    {
      id: 'log-maintenance',
      label: m.actions_log_maintenance(),
      icon: Wrench,
      onClick: () => {
        toaster.info({
          title: m.actions_maintenance_coming_soon(),
          duration: 3000
        });
      }
    }
  ]);
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_dashboard()}</title>
</svelte:head>

{#if dashboard.error}
  <div
    class="variant-soft-error rounded-container border-error-500/30 flex flex-col items-center justify-center border p-12 text-center"
  >
    <div class="variant-filled-error badge-icon mb-4 h-12 w-12"><RefreshCw /></div>
    <h2 class="h2 font-bold">{m.errors_dashboard_title()}</h2>
    <p class="mt-2 text-surface-200">{m.errors_dashboard_message()}</p>
    <div class="mt-6 flex gap-4">
      <Button variant="default" size="lg" onclick={handleRetry}>
        {m.errors_retry_page()}
      </Button>
      <Button variant="ghost" size="lg" onclick={handleReturn}>
        <House class="mr-2 h-4 w-4" />
        {m.errors_return_dashboard()}
      </Button>
    </div>
  </div>
{:else}
  <div class="space-y-8">
    <!-- Page Header with Title and Description -->
    <PageHeader
      title={m.dashboard_title()}
      subtitle={m.dashboard_subtitle()}
      description={m.dashboard_description()}
    />

    <section>
      <div class="mb-4 flex items-center justify-between">
        <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
          {m.dashboard_yard_statistics()}
        </h3>
        {#if totals?.maintenanceDue}
          <span class="variant-soft-error badge animate-pulse font-semibold"
            >{totals.maintenanceDue} {m.dashboard_due_soon()}</span
          >
        {/if}
      </div>

      <div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
        {#if dashboard.isLoading}
          {#each Array(4) as _item, index (index)}<div
              class="skeleton rounded-container h-28"
            ></div>{/each}
        {:else}
          {#each stats as stat (stat.label)}
            <StatsCard {stat} />
          {/each}
        {/if}
      </div>

      <!-- Visual Separator between Stats and Charts -->
      <div class="border-surface-700/50 my-6 border-t"></div>

      <div class="mt-6">
        <h3 class="h3 mb-4 text-sm font-bold tracking-wider text-surface-300 uppercase">
          Charts & Analytics
        </h3>
        <DashboardCharts />
      </div>
    </section>

    <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
      <div class="space-y-8 lg:col-span-2">
        <section>
          <div class="mb-4 flex items-center justify-between">
            <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
              {m.dashboard_recently_added()}
            </h3>
            <a
              href={resolve('/my-collection')}
              class="text-accent-500 text-sm font-bold hover:underline">{m.dashboard_view_all()}</a
            >
          </div>

          {#if dashboard.isLoading}
            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
              {#each Array(2) as _item, index (index)}<div
                  class="skeleton rounded-container aspect-video w-full"
                ></div>{/each}
            </div>
          {:else if !recent.length}
            <div
              class="variant-soft-surface rounded-container border-surface-700/60 border border-dashed p-10 text-center text-surface-300"
            >
              <p class="mb-2">{m.dashboard_empty_recent()}</p>
              <p class="text-surface-400 text-sm">{m.dashboard_empty_recent_prompt()}</p>
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
            <div class="card border-surface-500/20 border-2 border-dashed p-8 text-center">
              <p class="mb-4 text-surface-300">{m.dashboard_empty_depot()}</p>
              <Button
                variant="secondary"
                onclick={() => goto(resolve('/catalogue/new-model'))}
              >
                <Plus class="mr-2" />
                {m.actions_add_railway_model()}
              </Button>
            </div>
          {:else}
            <DepotView data={depot} isLoading={dashboard.isLoading} />
          {/if}
        </section>
      </div>

      <aside>
        <div class="sticky top-24 space-y-4">
          <h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase">
            {m.dashboard_command_center()}
          </h3>
          <QuickActionButtons {actions} />
        </div>
      </aside>
    </div>
  </div>
{/if}

{#if showWishlistModal}
  <AddWishlistItemModal
    onClose={() => (showWishlistModal = false)}
    onSaved={() => (showWishlistModal = false)}
  />
{/if}
