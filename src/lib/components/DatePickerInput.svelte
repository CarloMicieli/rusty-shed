<script lang="ts">
  import * as Popover from '$lib/components/ui/popover';
  import { Calendar } from '$lib/components/ui/calendar';
  import { CalendarDate } from '@internationalized/date';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';
  import { Calendar as CalendarIcon } from 'lucide-svelte';

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
    class="flex h-9 w-full min-w-0 cursor-pointer items-center justify-between rounded-sm border border-border bg-background px-3 text-sm transition-all duration-150 ease-out outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {className}"
  >
    {#if value}
      <span class="min-w-0 truncate pr-2 text-left font-mono text-sm text-foreground">
        {formatDate(value)}
      </span>
    {:else}
      <span class="min-w-0 truncate pr-2 text-left text-sm text-muted-foreground italic">
        {placeholder}
      </span>
    {/if}
    <CalendarIcon class="size-4 shrink-0 text-muted-foreground" />
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
