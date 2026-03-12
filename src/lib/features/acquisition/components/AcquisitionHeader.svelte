<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { DatePickerField } from '$lib/components';
  import { CalendarDate } from '@internationalized/date';
  import type { SellerView } from '$lib/bindings';
  import type { BatchDefaults } from '../types.js';

  interface Props {
    sellerId: string | null;
    onSellerChange: (id: string | null) => void;
    purchaseDate: string;
    onDateChange: (date: string) => void;
    batchDefaults: BatchDefaults;
    onBatchDefaultChange: (field: 'scale' | 'powerMethod', value: string | null) => void;
    sellers: SellerView[];
  }

  const SCALE_OPTIONS = ['H0', 'H0m', 'H0e', 'N', 'TT', 'Z', '0', 'G', 'S', 'II'] as const;
  const POWER_METHOD_OPTIONS = ['AC', 'DC', 'TRIX_EXPRESS'] as const;

  let {
    sellerId,
    onSellerChange,
    purchaseDate,
    onDateChange,
    batchDefaults,
    onBatchDefaultChange,
    sellers
  }: Props = $props();

  const today = $derived.by(() => {
    const [y, mo, d] = new Date().toISOString().split('T')[0].split('-').map(Number);
    return new CalendarDate(y, mo, d);
  });
</script>

<div class="grid grid-cols-2 gap-4">
  <!-- Seller -->
  <div class="space-y-1">
    <label
      for="acq-seller"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.acquisition_seller_label()}
    </label>
    <select
      id="acq-seller"
      value={sellerId ?? ''}
      onchange={(e) => onSellerChange((e.currentTarget as HTMLSelectElement).value || null)}
      class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
    >
      <option value="">—</option>
      {#each sellers as seller (seller.id)}
        <option value={seller.id}>{seller.name}</option>
      {/each}
    </select>
  </div>

  <!-- Purchase Date -->
  <div class="space-y-1">
    <label
      for="acq-date"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.acquisition_date_label()}
    </label>
    <DatePickerField
      id="acq-date"
      value={purchaseDate}
      onSelect={(date) => onDateChange(date ?? new Date().toISOString().split('T')[0])}
      maxValue={today}
    />
  </div>

  <!-- Default Scale -->
  <div class="space-y-1">
    <label
      for="acq-batch-scale"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.acquisition_batch_scale_label()}
    </label>
    <select
      id="acq-batch-scale"
      value={batchDefaults.scale ?? ''}
      onchange={(e) =>
        onBatchDefaultChange('scale', (e.currentTarget as HTMLSelectElement).value || null)}
      class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
    >
      <option value="">—</option>
      {#each SCALE_OPTIONS as scale (scale)}
        <option value={scale}>{scale}</option>
      {/each}
    </select>
  </div>

  <!-- Default Power Method -->
  <div class="space-y-1">
    <label
      for="acq-batch-power"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.acquisition_batch_power_label()}
    </label>
    <select
      id="acq-batch-power"
      value={batchDefaults.powerMethod ?? ''}
      onchange={(e) =>
        onBatchDefaultChange('powerMethod', (e.currentTarget as HTMLSelectElement).value || null)}
      class="h-12 w-full appearance-none rounded-xl border border-white/10 bg-zinc-950 px-4 text-sm text-zinc-100 focus:border-white/20 focus:outline-none"
    >
      <option value="">—</option>
      {#each POWER_METHOD_OPTIONS as pm (pm)}
        <option value={pm}>{pm}</option>
      {/each}
    </select>
  </div>
</div>
