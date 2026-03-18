<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import DrawerInput from '$lib/components/drawer/DrawerInput.svelte';
  import FormSelect from '$lib/components/drawer/FormSelect.svelte';

  interface Props {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
    railwayCompanyId?: string | null;
    companyOptions?: { value: string; label: string }[];
  }

  let {
    seriesCode = $bindable(),
    roadNumber = $bindable(),
    livery = $bindable(),
    depot = $bindable(),
    railwayCompanyId = $bindable(null),
    companyOptions = []
  }: Props = $props();
</script>

<div class="overflow-hidden rounded-lg border border-[#1F1F1F] bg-[#0F0F0F] p-4">
  <section>
    <p class="mb-4 text-[10px] font-bold tracking-[0.2em] text-[#808080] uppercase">
      {m.specs_drawer_section_identification()}
    </p>
    <div class="grid grid-cols-2 gap-3">
      <div class="col-span-2">
        <FormSelect
          id="drawer-company"
          label={m.model_rolling_stock_field_company()}
          options={companyOptions}
          bind:value={railwayCompanyId}
          isSearchable={true}
          placeholder={m.rolling_stock_select_company()}
          disabled={companyOptions.length === 0}
          required
        />
      </div>
      <div class="col-span-2">
        <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-series-code">
          {m.rolling_stock_field_series_code()} <span class="text-red-400">*</span>
        </label>
        <DrawerInput
          id="drawer-series-code"
          bind:value={seriesCode}
          placeholder={m.rolling_stock_placeholder_series_code()}
        />
      </div>
      <div>
        <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-road-number">
          {m.rolling_stock_field_road_number()}
        </label>
        <DrawerInput
          id="drawer-road-number"
          bind:value={roadNumber}
          placeholder={m.rolling_stock_placeholder_road_number()}
        />
      </div>
      <div>
        <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-livery">
          {m.rolling_stock_field_livery()}
        </label>
        <DrawerInput
          id="drawer-livery"
          bind:value={livery}
          placeholder={m.rolling_stock_placeholder_livery()}
        />
      </div>
      <div>
        <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-depot">
          {m.rolling_stock_field_depot()}
        </label>
        <DrawerInput
          id="drawer-depot"
          bind:value={depot}
          placeholder={m.rolling_stock_placeholder_depot()}
        />
      </div>
    </div>
  </section>
</div>
