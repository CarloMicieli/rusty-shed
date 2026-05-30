<script module lang="ts">
  let dashboardInitInFlight: Promise<void> | null = null;
</script>

<script lang="ts">
  import { getContext, onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { Heart, ShoppingBag, RefreshCw, House, Wrench, Wallet, Train } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Card, CardContent, Skeleton } from '$lib/components';

  // Components
  import PageHeader from '$lib/components/PageHeader.svelte';
  import StatsCard from '$lib/components/StatsCard.svelte';
  import QuickActionButtons, { type QuickAction } from '$lib/components/QuickActionButtons.svelte';
  import { DashboardCharts, PurchaseGroupCard } from '$lib/features/dashboard';

  // Refactored Components
  import DashboardAction from '$lib/features/dashboard/components/DashboardAction.svelte';
  import DashboardSectionHeader from '$lib/features/dashboard/components/DashboardSectionHeader.svelte';

  // Contexts
  import { getDashboardContext } from '$lib/features/dashboard/DashboardState.svelte';
  import { getWishlistContext } from '$lib/features/wishlists/WishlistState.svelte';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  const dashboard = getDashboardContext();
  const wishlistService = getWishlistContext();
  const openAcquisitionDrawer = getContext<() => void>('openAcquisitionDrawer');
  const openWishlistDrawer = getContext<() => void>('openWishlistDrawer');
  const openLogMaintenanceDrawer = getContext<() => void>('openLogMaintenanceDrawer');
  const totals = $derived(dashboard.data?.totals ?? null);
  const purchaseGroups = $derived(dashboard.data?.purchaseGroups ?? []);
  const currencyCode = $derived(dashboard.budgetData?.currency ?? 'EUR');

  const stats = $derived(byStats(totals));
  const statIcons = [Wallet, Train, Wrench] as const;

  // Check if budget is configured by checking if remainingPercentage is not null
  const hasBudget = $derived(
    dashboard.budgetData?.remainingPercentage !== null &&
      dashboard.budgetData?.remainingPercentage !== undefined
  );

  const budgetChartData = $derived.by(() => {
    const budgetData = dashboard.budgetData;
    if (!budgetData) return undefined;

    // Always include monthly spending data, but only include budget percentage if configured
    return {
      budget:
        budgetData.remainingPercentage !== null ? budgetData.remainingPercentage / 100 : undefined,
      monthlySpending: budgetData.monthlySpending.map((p) => ({
        month: p.month - 1,
        amount: Number(p.amount) / 100
      }))
    };
  });

  const actions = $derived<QuickAction[]>([
    {
      id: 'add-railway-model',
      label: m.dashboard_action_new_acquisition(),
      icon: ShoppingBag,
      onClick: () => openAcquisitionDrawer()
    },
    {
      id: 'add-wishlist-item',
      label: m.actions_add_wishlist_item(),
      icon: Heart,
      onClick: () => {
        if (!wishlistService.wishlists.length) void wishlistService.fetchWishlists();
        openWishlistDrawer();
      }
    },
    {
      id: 'log-maintenance',
      label: m.actions_log_maintenance(),
      icon: Wrench,
      onClick: () => openLogMaintenanceDrawer()
    }
  ]);

  const statSkeletonKeys = ['stat-total', 'stat-rolling-stock', 'stat-maintenance'] as const;
  const acquisitionSkeletonKeys = ['acq-1', 'acq-2'] as const;

  onMount(() => {
    if (dashboardInitInFlight) return;
  });

  // Helpers
  function formatMoney(amount?: { amount: number; currency: string } | null) {
    if (!amount) return '—';
    return regionalManager.formatCurrencyWith(amount.amount, amount.currency);
  }

  function byStats(data: typeof totals) {
    return [
      {
        label: m.stats_total_collection_value(),
        value: formatMoney(data?.totalValue ?? null),
        trend: 'neutral' as const,
        trendValue: ''
      },
      {
        label: m.stats_rolling_stocks(),
        value: `${data?.collectionItems ?? 0}`,
        trend: 'neutral' as const,
        trendValue: ''
      },
      {
        label: m.stats_maintenance_alerts(),
        value: `${data?.maintenanceDue ?? 0}`,
        trend: (data?.maintenanceDue ?? 0) > 0 ? ('down' as const) : ('neutral' as const),
        trendValue:
          (data?.maintenanceDue ?? 0) > 0 ? `${data?.maintenanceDue} ${m.dashboard_due()}` : '—'
      }
    ];
  }

  const handleModelClick = (collectionItemId: string) =>
    goto(resolve(`/collection/${collectionItemId.split(':').pop()}`));
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_dashboard()}</title>
</svelte:head>

{#if dashboard.error}
  <Card class="border-destructive/40 bg-destructive/5">
    <CardContent class="flex flex-col items-center justify-center p-12 text-center">
      <div class="mb-4 rounded-full bg-destructive/15 p-3 text-destructive"><RefreshCw /></div>
      <h2 class="h2 font-bold">{m.errors_dashboard_title()}</h2>
      <p class="mt-2 text-muted-foreground">{m.errors_dashboard_message()}</p>
      <div class="mt-6 flex gap-4">
        <Button variant="default" size="lg" onclick={() => dashboard.retry()}
          >{m.errors_retry_page()}</Button
        >
        <Button variant="ghost" size="lg" onclick={() => goto(resolve('/'))}
          ><House class="mr-2 h-4 w-4" />{m.errors_return_dashboard()}</Button
        >
      </div>
    </CardContent>
  </Card>
{:else}
  <div class="flex w-full max-w-none flex-col">
    <div
      class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
    >
      <PageHeader
        title={m.dashboard_title()}
        subtitle={m.dashboard_subtitle()}
        description={m.dashboard_description()}
      />
    </div>

    <div class="w-full max-w-none space-y-8">
      <section>
        <DashboardSectionHeader
          title={m.dashboard_yard_statistics()}
          badgeValue={totals?.maintenanceDue}
        />

        <div class="grid w-full grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {#if dashboard.isLoading}
            {#each statSkeletonKeys as key (key)}
              <Skeleton class="h-28 w-full" />
            {/each}
          {:else}
            {#each stats as stat, i (stat.label)}
              <StatsCard {stat} icon={statIcons[i]} />
            {/each}
          {/if}
        </div>

        <div class="my-6 border-t border-border/50"></div>

        <div
          class="mt-6 grid grid-cols-1 gap-4 lg:grid-cols-[minmax(0,3fr)_minmax(0,1fr)] xl:grid-cols-[minmax(0,4fr)_minmax(0,1fr)]"
        >
          <div class="space-y-6">
            <div class="lg:hidden">
              <div class="gauge-frame space-y-3 p-4">
                <p
                  class="text-[0.65rem] font-semibold tracking-[0.35em] text-muted-foreground uppercase"
                >
                  {m.dashboard_command_center()}
                </p>
                <QuickActionButtons {actions} class="gap-2" />
              </div>
            </div>
            <DashboardCharts compact={false} data={budgetChartData} {currencyCode} {hasBudget} />
          </div>

          <aside class="hidden lg:block">
            <div class="gauge-frame h-full space-y-3 p-4">
              <p
                class="text-[0.65rem] font-semibold tracking-[0.35em] text-muted-foreground uppercase"
              >
                {m.dashboard_command_center()}
              </p>
              <div class="flex flex-col gap-3">
                {#each actions as action (action.id)}
                  <DashboardAction {action} isPrimary={action.id === 'add-railway-model'} />
                {/each}
              </div>
            </div>
          </aside>
        </div>
      </section>

      <section>
        <DashboardSectionHeader
          title={m.dashboard_recent_acquisitions()}
          link={{ href: '/collection', label: m.dashboard_view_all() }}
        />

        {#if dashboard.isLoading}
          <div class="space-y-4">
            {#each acquisitionSkeletonKeys as key (key)}
              <Skeleton class="h-48 w-full" />
            {/each}
          </div>
        {:else if !purchaseGroups.length}
          <Card class="blueprint-panel border-border/60">
            <CardContent class="p-10 text-center">
              <p class="text-base font-semibold">{m.dashboard_empty_acquisitions()}</p>
              <p class="mt-2 mb-5 text-sm text-muted-foreground">
                {m.dashboard_empty_acquisitions_message()}
              </p>
              <button
                type="button"
                aria-label={m.dashboard_action_new_acquisition()}
                class="inline-flex cursor-pointer items-center gap-2 rounded-sm bg-primary px-8 py-3 font-bebas text-lg tracking-widest text-primary-foreground uppercase shadow-[0_3px_0_hsl(var(--primary)/0.5)] transition-all hover:brightness-110 active:translate-y-0.5 active:shadow-none"
                onclick={() => openAcquisitionDrawer()}
              >
                <ShoppingBag class="h-5 w-5" />
                <span>{m.dashboard_action_new_acquisition()}</span>
              </button>
            </CardContent>
          </Card>
        {:else}
          <div class="space-y-4">
            {#each purchaseGroups as group (group.id)}
              <PurchaseGroupCard {group} onModelClick={handleModelClick} />
            {/each}
          </div>
        {/if}
      </section>
    </div>
  </div>
{/if}
