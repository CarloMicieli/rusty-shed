<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  const AccordionItem = Accordion.Item;
  import { goto } from '$app/navigation';
  import { resolveRoute } from '$app/paths';
  import { commands } from '$lib/bindings';
  import {
    createRailwayModelSchema,
    type CreateRailwayModelInput
  } from '$lib/schemas/railway-model';
  import type { ZodError } from 'zod';

  // Import JSON data
  import manufacturersData from '$lib/data/manufacturers.json';
  import railwayCompaniesData from '$lib/data/railway-companies.json';

  // Reactive form state
  let formData = $state<CreateRailwayModelInput>({
    manufacturer_id: '',
    product_code: '',
    description: '',
    details: null,
    power_method: 'DC',
    scale: 'H0',
    epoch: 'III',
    category: 'LOCOMOTIVES',
    delivery_date: null,
    availability_status: null,
    rolling_stocks: []
  });

  let errors = $state<Record<string, string>>({});
  let isSubmitting = $state(false);

  // Enum options
  const powerMethods = ['AC', 'DC', 'TRIX_EXPRESS'];
  const scales = ['H0', 'H0m', 'H0e', 'N', 'TT', 'Z', 'G', 'Scale1', 'Scale0', 'Scale00'];
  const categories = [
    'LOCOMOTIVES',
    'TRAIN_SETS',
    'STARTER_SETS',
    'FREIGHT_CARS',
    'PASSENGER_CARS',
    'ELECTRIC_MULTIPLE_UNITS',
    'RAILCARS'
  ];
  const availabilityStatuses = ['ANNOUNCED', 'AVAILABLE', 'CANCELLED', 'DISCONTINUED'];
  const controls = ['DCC_READY', 'DCC_FITTED', 'DCC_SOUND', 'NO_DCC'];
  const locomotiveTypes = ['STEAM_LOCOMOTIVE', 'DIESEL_LOCOMOTIVE', 'ELECTRIC_LOCOMOTIVE'];
  const passengerCarTypes = [
    'BAGGAGE_CAR',
    'BUFFET_CAR',
    'COMBINE_CAR',
    'COMPARTMENT_COACH',
    'DINING_CAR',
    'DOUBLE_DECKER',
    'DOME_CAR',
    'DRIVING_TRAILER',
    'LOUNGE',
    'OBSERVATION',
    'OPEN_COACH',
    'RAILWAY_POST_OFFICE',
    'SLEEPING_CAR',
    'SLEEPERETTE'
  ];
  const freightCarTypes = [
    'AUTO_TRANSPORT_CARS',
    'BRAKE_WAGON',
    'CONTAINER_CARS',
    'COVERED_FREIGHT_CARS',
    'DEEP_WELL_FLAT_CARS',
    'DUMP_CARS',
    'GONDOLA',
    'HEAVY_GOODS_WAGONS',
    'HINGED_COVER_WAGONS',
    'HOPPER_WAGON',
    'REFRIGERATOR_CARS',
    'SILO_CONTAINER_CARS',
    'SLIDE_TARPAULIN_WAGON',
    'SLIDING_WALL_BOXCARS',
    'SPECIAL_TRANSPORT',
    'STAKE_WAGONS',
    'SWING_ROOF_WAGON',
    'TANK_CARS',
    'TELESCOPE_HOOD_WAGONS'
  ];
  const serviceLevels = [
    'FIRST',
    'SECOND',
    'THIRD',
    'FIRST_SECOND',
    'SECOND_THIRD',
    'FIRST_SECOND_THIRD'
  ];
  const dccInterfaces = [
    'NEM_651',
    'NEM_652',
    'NEM_654',
    'PLUX_8',
    'PLUX_12',
    'PLUX_16',
    'PLUX_22',
    'NEXT_18',
    'NEXT_18_S',
    'MTC_21'
  ];
  const electricMultipleUnitTypes = [
    'DRIVING_CAR',
    'HIGH_SPEED_TRAIN',
    'MOTOR_CAR',
    'POWER_CAR',
    'TRAILER_CAR',
    'TRAIN_SET'
  ];

  function addRollingStock() {
    formData.rolling_stocks.push({
      category: 'Locomotive',
      railway_company_id: '',
      class_name: '',
      road_number: '',
      series: undefined,
      depot: undefined,
      livery: undefined,
      locomotive_type: 'DIESEL_LOCOMOTIVE',
      is_dummy: false,
      control: undefined,
      dcc_interface: undefined,
      length_over_buffers: undefined,
      technical_specifications: undefined
    });
  }

  function deleteRollingStock(index: number) {
    formData.rolling_stocks.splice(index, 1);
  }

  function duplicateRollingStock(index: number) {
    const copy = structuredClone(formData.rolling_stocks[index]);
    formData.rolling_stocks.push(copy);
  }

  async function handleSubmit() {
    isSubmitting = true;
    errors = {};

    try {
      // Validate with Zod
      const validated = createRailwayModelSchema.parse(formData);

      // Submit to backend
      const result = await commands.createRailwayModel(validated);

      if (result.status === 'ok') {
        // Navigate to model detail page

        await goto(`/models/${result.data}`);
      } else {
        errors.general = result.error;
      }
    } catch (err) {
      if ((err as ZodError).issues) {
        // Zod validation errors
        const zodErr = err as ZodError;
        zodErr.issues.forEach((issue) => {
          const path = issue.path.join('.');
          errors[path] = issue.message;
        });
      } else {
        errors.general = 'An unexpected error occurred';
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="container mx-auto p-8">
  <h1 class="mb-8 h2">Add New Railway Model</h1>

  {#if errors.general}
    <div class="variant-filled-error mb-4 card p-4">
      {errors.general}
    </div>
  {/if}

  <form
    onsubmit={(e) => {
      e.preventDefault();
      handleSubmit();
    }}
  >
    <Accordion>
      <!-- Section 1: Basic Information -->
      <AccordionItem open>
        <svelte:fragment slot="summary">
          <h3 class="h4">Basic Information</h3>
        </svelte:fragment>
        <svelte:fragment slot="content">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <!-- Manufacturer -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Manufacturer *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.manufacturer_id}
              >
                <option value="">-- Select Manufacturer --</option>
                {#each manufacturersData as m (m.id)}
                  <option value={m.id}>{m.name}</option>
                {/each}
              </select>
              {#if errors.manufacturer_id}
                <span class="text-sm text-error-500">{errors.manufacturer_id}</span>
              {/if}
            </label>

            <!-- Product Code -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Product Code *</span
              >
              <input
                class="input border-surface-600 bg-surface-800 font-mono"
                type="text"
                bind:value={formData.product_code}
                placeholder="e.g., 37858"
              />
              {#if errors.product_code}
                <span class="text-sm text-error-500">{errors.product_code}</span>
              {/if}
            </label>

            <!-- Description -->
            <label class="label lg:col-span-2">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Description *</span
              >
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.description}
                placeholder="e.g., Class 218 Diesel Locomotive"
              />
              {#if errors.description}
                <span class="text-sm text-error-500">{errors.description}</span>
              {/if}
            </label>

            <!-- Category -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Category *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.category}
              >
                {#each categories as cat (cat)}
                  <option value={cat}>{$_(`enums.category.${cat}`)}</option>
                {/each}
              </select>
            </label>

            <!-- Scale -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Scale *</span
              >
              <select class="select border-surface-600 bg-surface-800" bind:value={formData.scale}>
                {#each scales as s (s)}
                  <option value={s}>{$_(`enums.scale.${s}`)}</option>
                {/each}
              </select>
            </label>

            <!-- Power Method -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Power Method *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.power_method}
              >
                {#each powerMethods as pm (pm)}
                  <option value={pm}>{$_(`enums.power_method.${pm}`)}</option>
                {/each}
              </select>
            </label>

            <!-- Epoch -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Epoch *</span
              >
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.epoch}
                placeholder="e.g., III, IV, V"
              />
              {#if errors.epoch}
                <span class="text-sm text-error-500">{errors.epoch}</span>
              {/if}
            </label>
          </div>
        </svelte:fragment>
      </AccordionItem>

      <!-- Section 2: Delivery & Availability -->
      <AccordionItem>
        <svelte:fragment slot="summary">
          <h3 class="h4">Delivery & Availability</h3>
        </svelte:fragment>
        <svelte:fragment slot="content">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <!-- Delivery Date -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Delivery Date</span
              >
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.delivery_date}
                placeholder="2025, 2025/06, or 2025/Q2"
              />
              {#if errors.delivery_date}
                <span class="text-sm text-error-500">{errors.delivery_date}</span>
              {/if}
            </label>

            <!-- Availability Status -->
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Availability Status</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.availability_status}
              >
                <option value={null}>-- Select --</option>
                {#each availabilityStatuses as status (status)}
                  <option value={status}>{$_(`enums.availability_status.${status}`)}</option>
                {/each}
              </select>
            </label>

            <!-- Details -->
            <label class="label lg:col-span-2">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >Additional Details</span
              >
              <textarea
                class="textarea border-surface-600 bg-surface-800"
                rows="3"
                bind:value={formData.details}
                placeholder="Optional notes about this model..."
              ></textarea>
            </label>
          </div>
        </svelte:fragment>
      </AccordionItem>

      <!-- Section 3: Rolling Stock -->
      <AccordionItem>
        <svelte:fragment slot="summary">
          <h3 class="h4">
            Rolling Stock <span class="variant-soft-primary badge"
              >{formData.rolling_stocks.length}</span
            >
          </h3>
        </svelte:fragment>
        <svelte:fragment slot="content">
          <div class="space-y-4">
            {#each formData.rolling_stocks as rs, index (index)}
              <div class="variant-filled-surface card p-4">
                <div class="mb-4 flex items-center justify-between">
                  <h4 class="h5">Rolling Stock #{index + 1}</h4>
                  <div class="flex gap-2">
                    <button
                      type="button"
                      class="variant-ghost-surface btn-icon btn-sm"
                      onclick={() => duplicateRollingStock(index)}
                      title="Duplicate"
                    >
                      <span>📋</span>
                    </button>
                    <button
                      type="button"
                      class="variant-ghost-surface btn-icon btn-sm"
                      onclick={() => deleteRollingStock(index)}
                      title="Delete"
                    >
                      <span>🗑️</span>
                    </button>
                  </div>
                </div>

                <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                  <!-- Railway Company -->
                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >Railway Company *</span
                    >
                    <select
                      class="select border-surface-600 bg-surface-800"
                      bind:value={rs.railway_company_id}
                    >
                      <option value="">-- Select Railway Company --</option>
                      {#each railwayCompaniesData as r (r.id)}
                        <option value={r.id}>{r.name}</option>
                      {/each}
                    </select>
                  </label>

                  <!-- Category Selector -->
                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >Rolling Stock Category *</span
                    >
                    <select
                      class="select border-surface-600 bg-surface-800"
                      bind:value={rs.category}
                    >
                      <option value="Locomotive">Locomotive</option>
                      <option value="PassengerCar">Passenger Car</option>
                      <option value="FreightCar">Freight Car</option>
                      <option value="Railcar">Railcar</option>
                      <option value="ElectricMultipleUnit">Electric Multiple Unit</option>
                    </select>
                  </label>

                  <!-- Livery (common to all types) -->
                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >Livery</span
                    >
                    <input
                      class="input border-surface-600 bg-surface-800"
                      type="text"
                      bind:value={rs.livery}
                      placeholder="E.g., Deutsche Bahn AG"
                    />
                  </label>

                  <!-- Category-specific fields -->
                  {#if rs.category === 'Locomotive'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Class Name *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.class_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Road Number *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Series</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Depot</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Type *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.locomotive_type}
                      >
                        {#each locomotiveTypes as type (type)}
                          <option value={type}>{$_(`enums.locomotive_type.${type}`)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label flex items-center gap-2">
                      <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Is Dummy</span
                      >
                    </label>

                    <!-- Technical Details Accordion -->
                    <div class="lg:col-span-2">
                      <Accordion>
                        <AccordionItem>
                          <svelte:fragment slot="summary">
                            <span class="text-sm font-semibold">🔧 Technical Details</span>
                          </svelte:fragment>
                          <svelte:fragment slot="content">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >Control</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value={null}>Select control...</option>
                                  {#each controls as control (control)}
                                    <option value={control}>{$_(`enums.control.${control}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >DCC Interface</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value={null}>Select DCC interface...</option>
                                  {#each dccInterfaces as dccInterface (dccInterface)}
                                    <option value={dccInterface}
                                      >{$_(`enums.dcc_interface.${dccInterface}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </svelte:fragment>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'PassengerCar'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Type Name *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Passenger Car Type *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.passenger_car_type}
                      >
                        {#each passengerCarTypes as type (type)}
                          <option value={type}>{$_(`enums.passenger_car_type.${type}`)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Road Number</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Series</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Depot</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <!-- Technical Details Accordion -->
                    <div class="lg:col-span-2">
                      <Accordion>
                        <AccordionItem>
                          <svelte:fragment slot="summary">
                            <span class="text-sm font-semibold">🔧 Technical Details</span>
                          </svelte:fragment>
                          <svelte:fragment slot="content">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >Service Level</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.service_level}
                                >
                                  <option value={null}>Select service level...</option>
                                  {#each serviceLevels as level (level)}
                                    <option value={level}
                                      >{$_(`enums.service_level.${level}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </svelte:fragment>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'FreightCar'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Type Name *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Freight Car Type</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.freight_car_type}
                      >
                        <option value={null}>Select freight car type...</option>
                        {#each freightCarTypes as type (type)}
                          <option value={type}>{$_(`enums.freight_car_type.${type}`)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Road Number</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Series</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Depot</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>
                  {:else if rs.category === 'Railcar'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Type Name *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Road Number</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Series</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Depot</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <!-- Technical Details Accordion -->
                    <div class="lg:col-span-2">
                      <Accordion>
                        <AccordionItem>
                          <svelte:fragment slot="summary">
                            <span class="text-sm font-semibold">🔧 Technical Details</span>
                          </svelte:fragment>
                          <svelte:fragment slot="content">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >Control</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value={null}>Select control...</option>
                                  {#each controls as control (control)}
                                    <option value={control}>{$_(`enums.control.${control}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >DCC Interface</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value={null}>Select DCC interface...</option>
                                  {#each dccInterfaces as dccInterface (dccInterface)}
                                    <option value={dccInterface}
                                      >{$_(`enums.dcc_interface.${dccInterface}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </svelte:fragment>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'ElectricMultipleUnit'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Type Name *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >EMU Type *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.electric_multiple_unit_type}
                      >
                        {#each electricMultipleUnitTypes as type (type)}
                          <option value={type}
                            >{$_(`enums.electric_multiple_unit_type.${type}`)}</option
                          >
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Road Number</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Series</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Depot</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <label class="label flex items-center gap-2">
                      <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >Is Dummy</span
                      >
                    </label>

                    <!-- Technical Details Accordion -->
                    <div class="lg:col-span-2">
                      <Accordion>
                        <AccordionItem>
                          <svelte:fragment slot="summary">
                            <span class="text-sm font-semibold">🔧 Technical Details</span>
                          </svelte:fragment>
                          <svelte:fragment slot="content">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >Control</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value={null}>Select control...</option>
                                  {#each controls as control (control)}
                                    <option value={control}>{$_(`enums.control.${control}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >DCC Interface</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value={null}>Select DCC interface...</option>
                                  {#each dccInterfaces as dccInterface (dccInterface)}
                                    <option value={dccInterface}
                                      >{$_(`enums.dcc_interface.${dccInterface}`)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </svelte:fragment>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {/if}
                </div>
              </div>
            {/each}

            <button
              type="button"
              class="variant-filled-primary btn w-full"
              onclick={addRollingStock}
            >
              + Add Rolling Stock
            </button>
          </div>
        </svelte:fragment>
      </AccordionItem>
    </Accordion>

    <!-- Form Actions -->
    <div class="mt-8 flex gap-4">
      <button type="submit" class="variant-filled-primary btn flex-1" disabled={isSubmitting}>
        {isSubmitting ? 'Creating...' : 'Create Railway Model'}
      </button>
      <button
        type="button"
        class="variant-ghost-surface btn"
        onclick={() => goto(resolveRoute('/'))}
      >
        Cancel
      </button>
    </div>
  </form>
</div>
