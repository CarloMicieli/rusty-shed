<script lang="ts">
  import { onMount } from 'svelte';
  import { Box as BoxIcon, Search, TrainFront, TramFront, X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import DepotSection from '$lib/features/depot/components/DepotSection.svelte';
  import LocomotiveCard from '$lib/features/depot/components/LocomotiveCard.svelte';
  import TrainCard from '$lib/features/depot/components/TrainCard.svelte';
  import CarCard from '$lib/features/depot/components/CarCard.svelte';
  import type { Car, Locomotive, TrainSet } from '$lib/features/depot/types';
  import { safeInvoke, getErrorMessage } from '$lib/services';
  import type { Collection, RailwayModel, RollingStock } from '$lib/bindings';
  import { debounce } from '$lib/utils/debounce';

  let loading = $state(true);
  let error = $state<string | null>(null);

  let depotLocomotives = $state<Locomotive[]>([]);
  let depotTrains = $state<TrainSet[]>([]);
  let depotCars = $state<Car[]>([]);

  let searchInput = $state('');
  let query = $state('');
  const stickyOffset = $state('var(--header-offset, 4rem)');

  const debouncedSearch = debounce((value: string) => {
    query = value.trim().toLowerCase();
  }, 150);

  function handleInput(value: string) {
    searchInput = value;
    debouncedSearch(value);
  }

  function clearSearch() {
    searchInput = '';
    query = '';
  }

  const normalizedQuery = $derived(query);

  const filterMatch = (value?: string | number | null) => {
    if (!normalizedQuery) return true;
    const text = value === null || value === undefined ? '' : String(value);
    return text.toLowerCase().includes(normalizedQuery);
  };

  const filteredLocomotives = $derived(
    normalizedQuery
      ? depotLocomotives.filter(
          (item) =>
            filterMatch(item.roadNumber) ||
            filterMatch(item.railwayCompany) ||
            filterMatch(item.group) ||
            filterMatch(item.livery) ||
            filterMatch(item.dccAddress)
        )
      : depotLocomotives
  );

  const filteredTrains = $derived(
    normalizedQuery
      ? depotTrains.filter(
          (item) =>
            filterMatch(item.roadNumber) ||
            filterMatch(item.railwayCompany) ||
            filterMatch(item.group) ||
            filterMatch(item.livery) ||
            filterMatch(item.dccAddress)
        )
      : depotTrains
  );

  const filteredCars = $derived(
    normalizedQuery
      ? depotCars.filter(
          (item) =>
            filterMatch(item.roadNumber) ||
            filterMatch(item.railwayCompany) ||
            filterMatch(item.type) ||
            filterMatch(item.livery) ||
            filterMatch(item.serviceLevel)
        )
      : depotCars
  );

  const totalFiltered = $derived(
    filteredLocomotives.length + filteredTrains.length + filteredCars.length
  );

  function pushRollingStock(
    model: RailwayModel,
    rolling: RollingStock,
    dccAddress: number | null,
    collections: {
      locomotives: Locomotive[];
      trains: TrainSet[];
      cars: Car[];
    }
  ) {
    const baseGroup = model.description || model.product_code;
    const railway = rolling.data.railway?.display ?? null;
    const livery = rolling.data.livery ?? null;

    if (rolling.category === 'Locomotive') {
      const data = rolling.data;
      collections.locomotives.push({
        id: data.id,
        group: baseGroup,
        roadNumber: data.road_number ?? null,
        railwayCompany: railway,
        livery,
        dccAddress
      });
      return;
    }

    if (rolling.category === 'ElectricMultipleUnit' || rolling.category === 'Railcar') {
      const data = rolling.data;
      collections.trains.push({
        id: data.id,
        group: baseGroup,
        roadNumber: data.road_number ?? null,
        railwayCompany: railway,
        livery,
        dccAddress
      });
      return;
    }

    if (rolling.category === 'PassengerCar' || rolling.category === 'FreightCar') {
      const data = rolling.data;

      // Build a safe, typed view of the data without using `any` so we can inspect fields
      const d = data as Record<string, unknown> & {
        id: string;
        friendly_name?: string;
        freight_car_type?: string | null;
        passenger_car_type?: string | null;
        road_number?: string | null;
        service_level?: string | null;
      };

      // compute a string label for the car type: prefer the typed enum field, fallback to friendly_name
      let typeLabel: string;
      if (d.freight_car_type && typeof d.freight_car_type === 'string') {
        typeLabel = d.freight_car_type;
      } else if (d.passenger_car_type && typeof d.passenger_car_type === 'string') {
        typeLabel = d.passenger_car_type;
      } else {
        typeLabel = d.friendly_name ?? '';
      }

      collections.cars.push({
        id: data.id,
        type: typeLabel,
        roadNumber: data.road_number ?? null,
        railwayCompany: railway,
        livery,
        category: rolling.category === 'PassengerCar' ? 'passenger' : 'freight',
        serviceLevel: 'service_level' in data ? (data.service_level ?? null) : null,
        dccAddress
      });
    }
  }

  function buildDepotView(collection: Collection, models: RailwayModel[]) {
    const modelMap = new Map(models.map((model) => [model.id, model]));

    const buckets = {
      locomotives: [] as Locomotive[],
      trains: [] as TrainSet[],
      cars: [] as Car[]
    };

    for (const item of collection.items) {
      const model = modelMap.get(item.railway_model_id);
      if (!model) continue;

      for (const owned of item.rolling_stocks) {
        const rolling = model.rolling_stocks.find(
          (rs) => rs.data.id === owned.rolling_stock_id || rs.data.id === owned.id
        );

        if (!rolling) continue;

        const dccAddress = owned.digital?.dcc_address ?? null;
        pushRollingStock(model, rolling, dccAddress, buckets);
      }
    }

    depotLocomotives = buckets.locomotives;
    depotTrains = buckets.trains;
    depotCars = buckets.cars;
  }

  async function loadDepot() {
    loading = true;
    error = null;
    depotLocomotives = [];
    depotTrains = [];
    depotCars = [];

    try {
      const collectionResult = await safeInvoke<Collection>('get_depot');
      if (!collectionResult.ok) {
        throw new Error(getErrorMessage(collectionResult.error));
      }

      const collection = collectionResult.data;
      const modelIds = Array.from(new Set(collection.items.map((item) => item.railway_model_id)));

      if (modelIds.length === 0) {
        return;
      }

      const modelsResult = await safeInvoke<RailwayModel[]>('get_railway_models_by_ids', {
        ids: modelIds
      });
      if (!modelsResult.ok) {
        throw new Error(getErrorMessage(modelsResult.error));
      }

      buildDepotView(collection, modelsResult.data);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unknown error loading depot';
    } finally {
      loading = false;
    }
  }

  onMount(loadDepot);
</script>

<svelte:head>
  <title>{m.depot_title()}</title>
</svelte:head>

<div class="mx-auto max-w-4xl space-y-6 p-4 pt-4" style="--header-offset: 4rem;">
  <div class="space-y-1">
    <p class="text-sm tracking-[0.2em] text-surface-400 uppercase">{m.app_depot()}</p>
    <h1 class="h2 font-bold">{m.depot_title()}</h1>
    <p class="text-sm text-surface-400">{m.depot_subtitle()}</p>
  </div>

  <div class="rounded-xl border border-surface-700/60 bg-surface-900 p-3">
    <div class="input-group items-center gap-2">
      <Search size={18} class="text-surface-500" />
      <input
        class="w-full bg-transparent text-sm outline-none placeholder:text-surface-500"
        placeholder={m.depot_search_placeholder()}
        value={searchInput}
        oninput={(event) => handleInput(event.currentTarget.value)}
      />
      {#if searchInput}
        <button
          class="variant-ghost-surface btn btn-sm px-2"
          aria-label={m.depot_clear_search()}
          onclick={clearSearch}
        >
          <X size={16} />
        </button>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="flex items-center gap-3 rounded-xl border border-surface-700/60 bg-surface-900 p-4">
      <div
        class="border-accent-400 h-4 w-4 animate-spin rounded-full border-2 border-t-transparent"
        aria-hidden="true"
      ></div>
      <p class="text-sm text-surface-300">Loading depot…</p>
    </div>
  {:else if error}
    <div
      class="flex flex-col gap-3 rounded-xl border border-amber-500/50 bg-amber-950/50 p-4 text-amber-100"
    >
      <p class="text-sm font-semibold">{error}</p>
      <div class="flex gap-2">
        <button class="variant-filled-primary btn btn-sm" onclick={loadDepot}>Retry</button>
        <button class="variant-ghost-surface btn btn-sm" onclick={clearSearch}
          >{m.depot_clear_search()}</button
        >
      </div>
    </div>
  {:else if totalFiltered === 0}
    <div
      class="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-surface-700/50 bg-surface-900 p-8 text-center"
    >
      <p class="text-lg font-semibold">{m.depot_no_results()}</p>
      <button class="variant-soft-primary btn" onclick={clearSearch}>
        {m.depot_clear_search()}
      </button>
    </div>
  {:else}
    <div class="space-y-8">
      <DepotSection
        title={m.depot_locomotives_title()}
        items={filteredLocomotives}
        icon={TrainFront}
        card={LocomotiveCard}
        toneClass="variant-filled-primary"
        {stickyOffset}
        emptyMessage={m.depot_empty_locomotives()}
      />

      <DepotSection
        title={m.depot_trains_title()}
        items={filteredTrains}
        icon={TramFront}
        card={TrainCard}
        toneClass="variant-filled-secondary"
        {stickyOffset}
        emptyMessage={m.depot_empty_trains()}
      />

      <DepotSection
        title={m.depot_cars_title()}
        items={filteredCars}
        icon={BoxIcon}
        card={CarCard}
        toneClass="variant-filled-surface"
        {stickyOffset}
        emptyMessage={m.depot_empty_cars()}
      />
    </div>
  {/if}
</div>
