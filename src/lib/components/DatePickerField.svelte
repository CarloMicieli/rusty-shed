<script lang="ts">
  import * as Popover from '$lib/components/ui/popover';
  import { Calendar } from '$lib/components/ui/calendar';
  import { CalendarDate } from '@internationalized/date';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  interface Props {
    value?: string | null;
    onSelect?: (iso: string | null) => void;
    maxValue?: CalendarDate;
    minValue?: CalendarDate;
    placeholder?: string;
    disabled?: boolean;
    align?: 'start' | 'center' | 'end';
    id?: string;
    class?: string;
  }

  let {
    value = $bindable(null),
    onSelect,
    maxValue,
    minValue,
    placeholder = 'Select date',
    disabled = false,
    align = 'end',
    id,
    class: className = ''
  }: Props = $props();

  function isoToCalendarDate(iso: string): CalendarDate {
    const [y, mo, d] = iso.split('-').map(Number);
    return new CalendarDate(y, mo, d);
  }

  function calendarDateToIso(date: CalendarDate): string {
    return `${date.year}-${String(date.month).padStart(2, '0')}-${String(date.day).padStart(2, '0')}`;
  }

  function formatDate(iso: string): string {
    return regionalManager.formatDate(iso);
  }

  const calendarValue = $derived(value ? isoToCalendarDate(value) : undefined);

  let open = $state(false);

  function handleSelect(date: CalendarDate | undefined) {
    if (!date) return;
    const iso = calendarDateToIso(date);
    value = iso;
    onSelect?.(iso);
    open = false;
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger
    {disabled}
    {id}
    class="-mx-1 flex cursor-pointer items-center justify-start rounded p-1 text-left transition-colors duration-150 outline-none hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 disabled:opacity-50 {className}"
  >
    {#if value}
      <span class="text-xs font-semibold text-foreground">
        {formatDate(value)}
      </span>
    {:else}
      <span class="text-xs font-semibold text-muted-foreground italic">{placeholder}</span>
    {/if}
  </Popover.Trigger>
  <Popover.Content class="w-auto border-border bg-card p-0" {align}>
    <Calendar
      type="single"
      value={calendarValue}
      {maxValue}
      {minValue}
      onValueChange={(v) => handleSelect(v as CalendarDate | undefined)}
    />
  </Popover.Content>
</Popover.Root>
