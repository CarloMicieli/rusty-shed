<script lang="ts">
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  const AccordionItem = Accordion.Item;
  const AccordionItemTrigger = Accordion.ItemTrigger;
  const AccordionItemContent = Accordion.ItemContent;
  const AccordionItemIndicator = Accordion.ItemIndicator;
  import { commands } from '$lib/bindings';
  import {
    createRailwayModelSchema,
    type CreateRailwayModelInput,
    type RollingStockInput
  } from '$lib/schemas/railway-model';
  import type { ZodError } from 'zod';

  import manufacturersData from '$lib/data/manufacturers.json';
  import railwayCompaniesData from '$lib/data/railway-companies.json';
  import availabilityStatusesData from '$lib/data/constants/availabilityStatuses.json';
  import categoriesData from '$lib/data/constants/categories.json';
  import controlsData from '$lib/data/constants/controls.json';
  import dccInterfacesData from '$lib/data/constants/dccInterfaces.json';
  import electricMultipleUnitTypesData from '$lib/data/constants/electricMultipleUnitTypes.json';
  import epochsData from '$lib/data/constants/epochs.json';
  import freightCarTypesData from '$lib/data/constants/freightCarTypes.json';
  import locomotiveTypesData from '$lib/data/constants/locomotiveTypes.json';
  import passengerCarTypesData from '$lib/data/constants/passengerCarTypes.json';
  import powerMethodsData from '$lib/data/constants/powerMethods.json';
  import rollingStockCategoriesData from '$lib/data/constants/rollingStockCategories.json';
  import scalesData from '$lib/data/constants/scales.json';
  import serviceLevelsData from '$lib/data/constants/serviceLevels.json';
  import { resolveLabel } from '../../utils/resolveLabel';
  import type { ConstantItem } from '$lib/types/constant_item';

  const formLabels: Record<string, ConstantItem> = {
    title: { id: 'add-new-railway-model', labelKey: 'form_new_model_title' },
    basicInfo: { id: 'basic-information', labelKey: 'form_new_model_basic_info' },
    manufacturer: { id: 'manufacturer', labelKey: 'form_new_model_manufacturer' },
    productCode: { id: 'product-code', labelKey: 'form_new_model_product_code' },
    productCodePlaceholder: {
      id: 'product-code-placeholder',
      labelKey: 'form_new_model_product_code_placeholder'
    },
    description: { id: 'description', labelKey: 'form_new_model_description' },
    descriptionPlaceholder: {
      id: 'description-placeholder',
      labelKey: 'form_new_model_description_placeholder'
    },
    category: { id: 'category', labelKey: 'form_new_model_category' },
    scale: { id: 'scale', labelKey: 'form_new_model_scale' },
    powerMethod: { id: 'power-method', labelKey: 'form_new_model_power_method' },
    epoch: { id: 'epoch', labelKey: 'form_new_model_epoch' },
    selectPlaceholder: { id: 'select-placeholder', labelKey: 'form_new_model_select_placeholder' },
    deliveryAvailability: {
      id: 'delivery-availability',
      labelKey: 'form_new_model_delivery_availability'
    },
    deliveryDate: { id: 'delivery-date', labelKey: 'form_new_model_delivery_date' },
    deliveryDatePlaceholder: {
      id: 'delivery-date-placeholder',
      labelKey: 'form_new_model_delivery_date_placeholder'
    },
    availabilityStatus: {
      id: 'availability-status',
      labelKey: 'form_new_model_availability_status'
    },
    additionalDetails: { id: 'additional-details', labelKey: 'form_new_model_additional_details' },
    detailsPlaceholder: {
      id: 'details-placeholder',
      labelKey: 'form_new_model_details_placeholder'
    },
    rollingStock: { id: 'rolling-stock', labelKey: 'form_new_model_rolling_stock' },
    railwayCompany: { id: 'railway-company', labelKey: 'form_new_model_railway_company' },
    rollingStockCategory: {
      id: 'rolling-stock-category',
      labelKey: 'form_new_model_rolling_stock_category'
    },
    livery: { id: 'livery', labelKey: 'form_new_model_livery' },
    liveryPlaceholder: { id: 'livery-placeholder', labelKey: 'form_new_model_livery_placeholder' },
    className: { id: 'class-name', labelKey: 'form_new_model_class_name' },
    roadNumber: { id: 'road-number', labelKey: 'form_new_model_road_number' },
    series: { id: 'series', labelKey: 'form_new_model_series' },
    depot: { id: 'depot', labelKey: 'form_new_model_depot' },
    type: { id: 'type', labelKey: 'form_new_model_type' },
    typeName: { id: 'type-name', labelKey: 'form_new_model_type_name' },
    passengerCarType: { id: 'passenger-car-type', labelKey: 'form_new_model_passenger_car_type' },
    freightCarType: { id: 'freight-car-type', labelKey: 'form_new_model_freight_car_type' },
    emuType: { id: 'emu-type', labelKey: 'form_new_model_emu_type' },
    isDummy: { id: 'is-dummy', labelKey: 'form_new_model_is_dummy' },
    technicalDetails: { id: 'technical-details', labelKey: 'form_new_model_technical_details' },
    control: { id: 'control', labelKey: 'form_new_model_control' },
    dccInterface: { id: 'dcc-interface', labelKey: 'form_new_model_dcc_interface' },
    serviceLevel: { id: 'service-level', labelKey: 'form_new_model_service_level' },
    duplicate: { id: 'duplicate', labelKey: 'form_new_model_duplicate' },
    delete: { id: 'delete', labelKey: 'form_new_model_delete' },
    addRollingStock: { id: 'add-rolling-stock', labelKey: 'form_new_model_add_rolling_stock' },
    create: { id: 'create-railway-model', labelKey: 'form_new_model_create' },
    cancel: { id: 'cancel', labelKey: 'form_new_model_cancel' }
  };

  type NullableEnum<T extends string = string> = T | '';

  type RollingStockForm = {
    category: '' | (typeof rollingStockCategoriesData)[number]['id'];
    railway_company_id: string;
    class_name?: string;
    road_number?: string;
    series: string | null;
    depot: string | null;
    livery: string | null;
    locomotive_type?: NullableEnum<(typeof locomotiveTypesData)[number]['id']>;
    passenger_car_type?: NullableEnum<(typeof passengerCarTypesData)[number]['id']>;
    freight_car_type?: NullableEnum<(typeof freightCarTypesData)[number]['id']>;
    electric_multiple_unit_type?: NullableEnum<
      (typeof electricMultipleUnitTypesData)[number]['id']
    >;
    type_name?: string;
    service_level?: NullableEnum<(typeof serviceLevelsData)[number]['id']>;
    is_dummy?: boolean;
    control: NullableEnum<(typeof controlsData)[number]['id']> | null;
    dcc_interface: NullableEnum<(typeof dccInterfacesData)[number]['id']> | null;
    length_over_buffers: CreateRailwayModelInput['rolling_stocks'][number]['length_over_buffers'];
    technical_specifications: CreateRailwayModelInput['rolling_stocks'][number]['technical_specifications'];
  };

  type FormState = {
    manufacturer_id: string;
    product_code: string;
    description: string;
    details: string | null;
    power_method: NullableEnum<(typeof powerMethodsData)[number]['id']>;
    scale: NullableEnum<(typeof scalesData)[number]['id']>;
    epoch: NullableEnum<(typeof epochsData)[number]['id']>;
    category: NullableEnum<(typeof categoriesData)[number]['id']>;
    delivery_date: string | null;
    availability_status: NullableEnum<(typeof availabilityStatusesData)[number]['id']> | null;
    rolling_stocks: RollingStockForm[];
  };

  let accordionValues = $state<string[]>(['basic-info', 'delivery-availability', 'rolling-stock']);

  let formData = $state<FormState>({
    manufacturer_id: '',
    product_code: '',
    description: '',
    details: null,
    power_method: '',
    scale: '',
    epoch: '',
    category: '',
    delivery_date: null,
    availability_status: null,
    rolling_stocks: []
  });

  let errors = $state<Record<string, string>>({});
  let isSubmitting = $state(false);

  function addRollingStock() {
    formData.rolling_stocks.push({
      category: '',
      railway_company_id: '',
      class_name: '',
      road_number: '',
      series: null,
      depot: null,
      livery: null,
      locomotive_type: '',
      passenger_car_type: '',
      freight_car_type: '',
      electric_multiple_unit_type: '',
      type_name: '',
      service_level: '',
      is_dummy: false,
      control: '',
      dcc_interface: '',
      length_over_buffers: null,
      technical_specifications: null
    });
  }

  function deleteRollingStock(index: number) {
    formData.rolling_stocks.splice(index, 1);
  }

  function duplicateRollingStock(index: number) {
    const copy = structuredClone(formData.rolling_stocks[index]);
    formData.rolling_stocks.push(copy);
  }

  function navigate(path: string) {
    window.location.assign(path);
  }

  function normalizeRollingStock(rs: RollingStockForm): RollingStockInput {
    return {
      ...rs,
      control: rs.control || null,
      dcc_interface: rs.dcc_interface || null,
      livery: rs.livery || null,
      series: rs.series || null,
      depot: rs.depot || null,
      service_level: rs.service_level || null,
      freight_car_type: rs.freight_car_type || null,
      electric_multiple_unit_type: rs.electric_multiple_unit_type || null,
      length_over_buffers: rs.length_over_buffers ?? null,
      technical_specifications: rs.technical_specifications ?? null
    } as RollingStockInput;
  }

  async function handleSubmit() {
    isSubmitting = true;
    errors = {};

    try {
      const payload: CreateRailwayModelInput = {
        manufacturer_id: formData.manufacturer_id,
        product_code: formData.product_code,
        description: formData.description,
        details: formData.details,
        power_method: formData.power_method as CreateRailwayModelInput['power_method'],
        scale: formData.scale as CreateRailwayModelInput['scale'],
        epoch: formData.epoch,
        category: formData.category as CreateRailwayModelInput['category'],
        delivery_date: formData.delivery_date,
        availability_status:
          formData.availability_status === ''
            ? null
            : (formData.availability_status as CreateRailwayModelInput['availability_status']),
        rolling_stocks: formData.rolling_stocks.map(normalizeRollingStock)
      };

      const validated = createRailwayModelSchema.parse(payload);
      const result = await commands.createRailwayModel(validated);

      if (result.status === 'ok') {
        navigate(`/models/${result.data}`);
      } else {
        errors.general =
          typeof result.error === 'string' ? result.error : JSON.stringify(result.error);
      }
    } catch (err) {
      if ((err as ZodError).issues) {
        const zodErr = err as ZodError;
        zodErr.issues.forEach((issue) => {
          const path = issue.path.join('.');
          errors[path] = issue.message;
        });
      } else {
        errors.general = typeof err === 'string' ? err : 'An unexpected error occurred';
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="container mx-auto p-8">
  <h1 class="mb-8 h2">{resolveLabel(formLabels.title)}</h1>
  {#if errors.general}
    <div class="variant-filled-error mb-4 card p-4">{errors.general}</div>
  {/if}

  <form
    onsubmit={(e) => {
      e.preventDefault();
      handleSubmit();
    }}
  >
    <Accordion
      value={accordionValues}
      onValueChange={(details) => (accordionValues = details.value)}
      multiple
      collapsible
      class="space-y-3"
    >
      <AccordionItem value="basic-info" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">{resolveLabel(formLabels.basicInfo)}</h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.manufacturer)} *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.manufacturer_id}
              >
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each manufacturersData as m (m.id)}
                  <option value={m.id}>{m.name}</option>
                {/each}
              </select>
              {#if errors.manufacturer_id}
                <span class="text-sm text-error-500">{errors.manufacturer_id}</span>
              {/if}
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.productCode)} *</span
              >
              <input
                class="input border-surface-600 bg-surface-800 font-mono"
                type="text"
                bind:value={formData.product_code}
                placeholder={resolveLabel(formLabels.productCodePlaceholder)}
              />
              {#if errors.product_code}
                <span class="text-sm text-error-500">{errors.product_code}</span>
              {/if}
            </label>

            <label class="label lg:col-span-2">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.description)} *</span
              >
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.description}
                placeholder={resolveLabel(formLabels.descriptionPlaceholder)}
              />
              {#if errors.description}
                <span class="text-sm text-error-500">{errors.description}</span>
              {/if}
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.category)} *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.category}
              >
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each categoriesData as cat (cat.id)}
                  <option value={cat.id}>{resolveLabel(cat as ConstantItem)}</option>
                {/each}
              </select>
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.scale)} *</span
              >
              <select class="select border-surface-600 bg-surface-800" bind:value={formData.scale}>
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each scalesData as s (s.id)}
                  <option value={s.id}>{resolveLabel(s as ConstantItem)}</option>
                {/each}
              </select>
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.powerMethod)} *</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.power_method}
              >
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each powerMethodsData as pm (pm.id)}
                  <option value={pm.id}>{resolveLabel(pm as ConstantItem)}</option>
                {/each}
              </select>
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.epoch)} *</span
              >
              <select class="select border-surface-600 bg-surface-800" bind:value={formData.epoch}>
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each epochsData as ep (ep.id)}
                  <option value={ep.id}>{resolveLabel(ep as ConstantItem)}</option>
                {/each}
              </select>
              {#if errors.epoch}
                <span class="text-sm text-error-500">{errors.epoch}</span>
              {/if}
            </label>
          </div>
        </AccordionItemContent>
      </AccordionItem>

      <AccordionItem value="delivery-availability" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">{resolveLabel(formLabels.deliveryAvailability)}</h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.deliveryDate)}</span
              >
              <input
                class="input border-surface-600 bg-surface-800"
                type="text"
                bind:value={formData.delivery_date}
                placeholder={resolveLabel(formLabels.deliveryDatePlaceholder)}
              />
              {#if errors.delivery_date}
                <span class="text-sm text-error-500">{errors.delivery_date}</span>
              {/if}
            </label>

            <label class="label">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.availabilityStatus)}</span
              >
              <select
                class="select border-surface-600 bg-surface-800"
                bind:value={formData.availability_status}
              >
                <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                {#each availabilityStatusesData as status (status.id)}
                  <option value={status.id}>{resolveLabel(status as ConstantItem)}</option>
                {/each}
              </select>
            </label>

            <label class="label lg:col-span-2">
              <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                >{resolveLabel(formLabels.additionalDetails)}</span
              >
              <textarea
                class="textarea border-surface-600 bg-surface-800"
                rows="3"
                bind:value={formData.details}
                placeholder={resolveLabel(formLabels.detailsPlaceholder)}
              ></textarea>
            </label>
          </div>
        </AccordionItemContent>
      </AccordionItem>

      <AccordionItem value="rolling-stock" class="rounded-lg border border-surface-600">
        <AccordionItemTrigger class="flex w-full items-center justify-between px-3 py-2 text-left">
          <h3 class="mb-0 h4">
            {resolveLabel(formLabels.rollingStock)}
            <span class="variant-soft-primary badge">{formData.rolling_stocks.length}</span>
          </h3>
          <AccordionItemIndicator class="text-muted text-sm" />
        </AccordionItemTrigger>

        <AccordionItemContent class="px-3 pt-1 pb-4">
          <div class="space-y-4">
            {#each formData.rolling_stocks as rs, index (index)}
              <div class="variant-filled-surface card p-4">
                <div class="mb-4 flex items-center justify-between">
                  <h4 class="h5">{resolveLabel(formLabels.rollingStock)} #{index + 1}</h4>
                  <div class="flex gap-2">
                    <button
                      type="button"
                      class="variant-ghost-surface btn-icon btn-sm"
                      onclick={() => duplicateRollingStock(index)}
                      title={resolveLabel(formLabels.duplicate)}
                    >
                      <span>📋</span>
                    </button>
                    <button
                      type="button"
                      class="variant-ghost-surface btn-icon btn-sm"
                      onclick={() => deleteRollingStock(index)}
                      title={resolveLabel(formLabels.delete)}
                    >
                      <span>🗑️</span>
                    </button>
                  </div>
                </div>

                <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >{resolveLabel(formLabels.railwayCompany)} *</span
                    >
                    <select
                      class="select border-surface-600 bg-surface-800"
                      bind:value={rs.railway_company_id}
                    >
                      <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                      {#each railwayCompaniesData as r (r.id)}
                        <option value={r.id}>{r.name}</option>
                      {/each}
                    </select>
                  </label>

                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >{resolveLabel(formLabels.rollingStockCategory)} *</span
                    >
                    <select
                      class="select border-surface-600 bg-surface-800"
                      bind:value={rs.category}
                    >
                      <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                      {#each rollingStockCategoriesData as option (option.id)}
                        <option value={option.id}>{resolveLabel(option as ConstantItem)}</option>
                      {/each}
                    </select>
                  </label>

                  <label class="label lg:col-span-2">
                    <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                      >{resolveLabel(formLabels.livery)}</span
                    >
                    <input
                      class="input border-surface-600 bg-surface-800"
                      type="text"
                      bind:value={rs.livery}
                      placeholder={resolveLabel(formLabels.liveryPlaceholder)}
                    />
                  </label>

                  {#if rs.category === 'Locomotive'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.className)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.class_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.roadNumber)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.series)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.depot)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.type)} *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.locomotive_type}
                      >
                        <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                        {#each locomotiveTypesData as type (type.id)}
                          <option value={type.id}>{resolveLabel(type as ConstantItem)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label flex items-center gap-2">
                      <input class="checkbox" type="checkbox" bind:checked={rs.is_dummy} />
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.isDummy)}</span
                      >
                    </label>

                    <div class="lg:col-span-2">
                      <Accordion collapsible>
                        <AccordionItem value={`technical-${index}-locomotive`}>
                          <AccordionItemTrigger
                            class="flex w-full items-center justify-between px-2 py-1 text-left"
                          >
                            <span class="text-sm font-semibold"
                              >{resolveLabel(formLabels.technicalDetails)}</span
                            >
                            <AccordionItemIndicator class="text-muted text-xs" />
                          </AccordionItemTrigger>
                          <AccordionItemContent class="px-2 pt-1 pb-2">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.control)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each controlsData as control (control.id)}
                                    <option value={control.id}
                                      >{resolveLabel(control as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.dccInterface)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each dccInterfacesData as dccInterface (dccInterface.id)}
                                    <option value={dccInterface.id}
                                      >{resolveLabel(dccInterface as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </AccordionItemContent>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'PassengerCar'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.typeName)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.passengerCarType)} *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.passenger_car_type}
                      >
                        <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                        {#each passengerCarTypesData as type (type.id)}
                          <option value={type.id}>{resolveLabel(type as ConstantItem)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.roadNumber)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.series)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.depot)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <div class="lg:col-span-2">
                      <Accordion collapsible>
                        <AccordionItem value={`technical-${index}-passenger`}>
                          <AccordionItemTrigger
                            class="flex w-full items-center justify-between px-2 py-1 text-left"
                          >
                            <span class="text-sm font-semibold"
                              >{resolveLabel(formLabels.technicalDetails)}</span
                            >
                            <AccordionItemIndicator class="text-muted text-xs" />
                          </AccordionItemTrigger>
                          <AccordionItemContent class="px-2 pt-1 pb-2">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.serviceLevel)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.service_level}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each serviceLevelsData as level (level.id)}
                                    <option value={level.id}
                                      >{resolveLabel(level as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </AccordionItemContent>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'FreightCar'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.typeName)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.freightCarType)}</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.freight_car_type}
                      >
                        <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                        {#each freightCarTypesData as type (type.id)}
                          <option value={type.id}>{resolveLabel(type as ConstantItem)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.roadNumber)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.series)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.depot)}</span
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
                        >{resolveLabel(formLabels.typeName)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.roadNumber)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.series)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.depot)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.depot}
                      />
                    </label>

                    <div class="lg:col-span-2">
                      <Accordion collapsible>
                        <AccordionItem value={`technical-${index}-railcar`}>
                          <AccordionItemTrigger
                            class="flex w-full items-center justify-between px-2 py-1 text-left"
                          >
                            <span class="text-sm font-semibold"
                              >{resolveLabel(formLabels.technicalDetails)}</span
                            >
                            <AccordionItemIndicator class="text-muted text-xs" />
                          </AccordionItemTrigger>
                          <AccordionItemContent class="px-2 pt-1 pb-2">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.control)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each controlsData as control (control.id)}
                                    <option value={control.id}
                                      >{resolveLabel(control as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.dccInterface)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each dccInterfacesData as dccInterface (dccInterface.id)}
                                    <option value={dccInterface.id}
                                      >{resolveLabel(dccInterface as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </AccordionItemContent>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {:else if rs.category === 'ElectricMultipleUnit'}
                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.typeName)} *</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.type_name}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.emuType)} *</span
                      >
                      <select
                        class="select border-surface-600 bg-surface-800"
                        bind:value={rs.electric_multiple_unit_type}
                      >
                        <option value="">{resolveLabel(formLabels.selectPlaceholder)}</option>
                        {#each electricMultipleUnitTypesData as type (type.id)}
                          <option value={type.id}>{resolveLabel(type as ConstantItem)}</option>
                        {/each}
                      </select>
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.roadNumber)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.road_number}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.series)}</span
                      >
                      <input
                        class="input border-surface-600 bg-surface-800"
                        type="text"
                        bind:value={rs.series}
                      />
                    </label>

                    <label class="label">
                      <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                        >{resolveLabel(formLabels.depot)}</span
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
                        >{resolveLabel(formLabels.isDummy)}</span
                      >
                    </label>

                    <div class="lg:col-span-2">
                      <Accordion collapsible>
                        <AccordionItem value={`technical-${index}-emu`}>
                          <AccordionItemTrigger
                            class="flex w-full items-center justify-between px-2 py-1 text-left"
                          >
                            <span class="text-sm font-semibold"
                              >{resolveLabel(formLabels.technicalDetails)}</span
                            >
                            <AccordionItemIndicator class="text-muted text-xs" />
                          </AccordionItemTrigger>
                          <AccordionItemContent class="px-2 pt-1 pb-2">
                            <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.control)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.control}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each controlsData as control (control.id)}
                                    <option value={control.id}
                                      >{resolveLabel(control as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>

                              <label class="label">
                                <span
                                  class="text-sm font-bold tracking-wider text-surface-300 uppercase"
                                  >{resolveLabel(formLabels.dccInterface)}</span
                                >
                                <select
                                  class="select border-surface-600 bg-surface-800"
                                  bind:value={rs.dcc_interface}
                                >
                                  <option value=""
                                    >{resolveLabel(formLabels.selectPlaceholder)}</option
                                  >
                                  {#each dccInterfacesData as dccInterface (dccInterface.id)}
                                    <option value={dccInterface.id}
                                      >{resolveLabel(dccInterface as ConstantItem)}</option
                                    >
                                  {/each}
                                </select>
                              </label>
                            </div>
                          </AccordionItemContent>
                        </AccordionItem>
                      </Accordion>
                    </div>
                  {/if}
                </div>
              </div>
            {/each}

            <button type="button" class="cta-btn cta-primary btn" onclick={addRollingStock}>
              + {resolveLabel(formLabels.addRollingStock)}
            </button>
          </div>
        </AccordionItemContent>
      </AccordionItem>
    </Accordion>

    <div class="mt-8 flex gap-4">
      <button type="submit" class="cta-btn cta-primary btn" disabled={isSubmitting}>
        {isSubmitting ? `${resolveLabel(formLabels.create)}...` : resolveLabel(formLabels.create)}
      </button>
      <button type="button" class="cta-btn cta-secondary btn" onclick={() => navigate('/')}>
        {resolveLabel(formLabels.cancel)}
      </button>
    </div>
  </form>
</div>

<style>
  .cta-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-weight: 700;
    text-decoration: none;
    border: 1px solid transparent;
    transition:
      background-color 150ms ease,
      border-color 150ms ease,
      color 150ms ease;
  }

  .cta-primary {
    background-color: var(--primary-500, #2563eb);
    color: var(--on-primary, #0b1224);
    border-color: var(--primary-600, #1d4ed8);
  }

  .cta-primary:hover {
    background-color: var(--primary-600, #1d4ed8);
  }

  .cta-secondary {
    background-color: var(--surface-700, #2d2f36);
    color: var(--on-surface, #f8fafc);
    border-color: var(--surface-500, #3f4148);
  }

  .cta-secondary:hover {
    background-color: var(--surface-600, #32343b);
  }
</style>
