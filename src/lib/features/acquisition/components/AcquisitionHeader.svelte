<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { DatePickerField } from '$lib/components';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';
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
  const SCALE_DISPLAY_MAP: Record<string, string> = {
    H0: 'H0 (1:87)',
    H0m: 'H0m (1:87)',
    H0e: 'H0e (1:87)',
    N: 'N (1:160)',
    TT: 'TT (1:120)',
    Z: 'Z (1:220)',
    '0': '0 (1:43)',
    G: 'G (1:22.5)',
    S: 'S (1:64)',
    II: 'II (1:22.5)'
  };

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
    <SearchableSelect
      id="acq-seller"
      options={sellers.map((s) => ({ value: s.id, label: s.name }))}
      value={sellerId ?? ''}
      placeholder="—"
      onSelect={(v: string) => onSellerChange(v || null)}
    />
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
    <Select.Root
      type="single"
      value={batchDefaults.scale ?? undefined}
      onValueChange={(v) => onBatchDefaultChange('scale', v || null)}
    >
      <Select.Trigger
        id="acq-batch-scale"
        class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]"
      >
        {batchDefaults.scale
          ? (SCALE_DISPLAY_MAP[batchDefaults.scale] ?? batchDefaults.scale)
          : '—'}
      </Select.Trigger>
      <Select.Content>
        {#each SCALE_OPTIONS as scale (scale)}
          <Select.Item value={scale} label={SCALE_DISPLAY_MAP[scale] ?? scale} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>

  <!-- Default Power Method -->
  <div class="space-y-1">
    <label
      for="acq-batch-power"
      class="ml-1 text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
    >
      {m.acquisition_batch_power_label()}
    </label>
    <Select.Root
      type="single"
      value={batchDefaults.powerMethod ?? undefined}
      onValueChange={(v) => onBatchDefaultChange('powerMethod', v || null)}
    >
      <Select.Trigger
        id="acq-batch-power"
        class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]"
      >
        {batchDefaults.powerMethod ?? '—'}
      </Select.Trigger>
      <Select.Content>
        {#each POWER_METHOD_OPTIONS as pm (pm)}
          <Select.Item value={pm} label={pm} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
</div>
