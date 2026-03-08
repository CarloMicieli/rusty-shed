<script lang="ts">
  import * as m from '$lib/paraglide/messages';

  interface Props {
    seriesCode: string;
    roadNumber: string;
    livery: string;
    depot: string;
  }

  let { seriesCode = $bindable(), roadNumber = $bindable(), livery = $bindable(), depot = $bindable() }: Props = $props();
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
      <input
        id="drawer-series-code"
        type="text"
        bind:value={seriesCode}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      />
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-road-number">
        {m.rolling_stock_field_road_number()}
      </label>
      <input
        id="drawer-road-number"
        type="text"
        bind:value={roadNumber}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      />
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-livery">
        {m.rolling_stock_field_livery()}
      </label>
      <input
        id="drawer-livery"
        type="text"
        bind:value={livery}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      />
    </div>
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="drawer-depot">
        {m.rolling_stock_field_depot()}
      </label>
      <input
        id="drawer-depot"
        type="text"
        bind:value={depot}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      />
    </div>
  </div>
</section>
