<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import DrawerInput from '$lib/components/drawer/DrawerInput.svelte';

  interface Props {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
  }

  let {
    seriesCode = $bindable(),
    roadNumber = $bindable(),
    livery = $bindable(),
    depot = $bindable()
  }: Props = $props();
</script>

<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.specs_drawer_section_identification()}
  </h3>
  <div class="grid grid-cols-2 gap-3">
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
