<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { DatePickerField } from '$lib/components';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';
  import { CalendarDate } from '@internationalized/date';
  import type { SellerView } from '$lib/bindings';
  import type { BatchDefaults } from '../types.js';
  import {
    scaleOptions,
    powerMethodOptions,
    powerMethodLabel,
    SCALE_DISPLAY_MAP
  } from '$lib/utils/enum-options';

  interface Props {
    sellerId: string | null;
    onSellerChange: (id: string | null) => void;
    purchaseDate: string;
    onDateChange: (date: string) => void;
    batchDefaults: BatchDefaults;
    onBatchDefaultChange: (field: 'scale' | 'powerMethod', value: string | null) => void;
    sellers: SellerView[];
  }

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
        class="h-10 w-full border-border bg-background text-foreground focus-visible:border-primary/60 focus-visible:ring-1 focus-visible:ring-primary data-[placeholder]:text-muted-foreground"
      >
        {batchDefaults.scale
          ? (SCALE_DISPLAY_MAP[batchDefaults.scale] ?? batchDefaults.scale)
          : '—'}
      </Select.Trigger>
      <Select.Content class="border-border bg-background text-foreground">
        {#each scaleOptions() as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
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
        class="h-10 w-full border-border bg-background text-foreground focus-visible:border-primary/60 focus-visible:ring-1 focus-visible:ring-primary data-[placeholder]:text-muted-foreground"
      >
        {batchDefaults.powerMethod ? powerMethodLabel(batchDefaults.powerMethod) : '—'}
      </Select.Trigger>
      <Select.Content class="border-border bg-background text-foreground">
        {#each powerMethodOptions() as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
</div>
