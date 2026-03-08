<script lang="ts">
  import * as m from '$lib/paraglide/messages';

  interface Props {
    railwayCompanyId: string;
    railwayCompanyName: string;
    category: string;
    companyOptions: { id: string; label: string }[];
    categoryOptions: { value: string; label: string }[];
    onCompanySelect?: (id: string, name: string) => void;
  }

  let {
    railwayCompanyId = $bindable(),
    railwayCompanyName = $bindable(),
    category = $bindable(),
    companyOptions,
    categoryOptions,
    onCompanySelect
  }: Props = $props();
</script>

<section>
  <h3 class="mb-3 text-[10px] font-semibold tracking-widest text-zinc-500 uppercase">
    {m.rolling_stock_create_drawer_title()}
  </h3>
  <div class="space-y-4">
    <!-- Railway Company -->
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-company">
        {m.model_rolling_stock_field_company()} <span class="text-red-400">*</span>
      </label>
      {#if companyOptions.length > 0}
        <select
          id="create-company"
          value={railwayCompanyId}
          onchange={(e) => {
            const id = (e.target as HTMLSelectElement).value;
            const name = companyOptions.find((c) => c.id === id)?.label ?? id;
            railwayCompanyId = id;
            railwayCompanyName = name;
            onCompanySelect?.(id, name);
          }}
          class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
        >
          <option value="">—</option>
          {#each companyOptions as opt (opt.id)}
            <option value={opt.id}>{opt.label}</option>
          {/each}
        </select>
      {:else}
        <p class="text-xs text-zinc-500">Loading…</p>
      {/if}
    </div>

    <!-- Category -->
    <div>
      <label class="mb-1 block text-xs font-medium text-zinc-400" for="create-category">
        {m.rolling_stock_field_category()} <span class="text-red-400">*</span>
      </label>
      <select
        id="create-category"
        bind:value={category}
        class="w-full rounded-md border border-zinc-700 bg-zinc-900 px-3 py-1.5 text-sm text-zinc-100 outline-none focus:border-[#E2994F] focus:ring-1 focus:ring-[#E2994F]/30"
      >
        {#each categoryOptions as opt (opt.value)}
          <option value={opt.value}>{opt.label}</option>
        {/each}
      </select>
    </div>
  </div>
</section>
