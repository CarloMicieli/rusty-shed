<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { CalendarDate } from '@internationalized/date';
  import { DatePickerField } from '$lib/components';
  import type { MaintenanceType } from '$lib/bindings';

  interface Card {
    id: string;
    ownedRollingStockId: string;
    nextMaintenanceDate: string | null;
  }

  interface Props {
    cards: Card[];
    selectedCardId: string | null;
    datePerformed: string | null;
    maintenanceType: string | null;
    notes: string;
    error: string | null;
    maintenanceTypes: Array<{ value: MaintenanceType; label: string }>;
    onCardChange: (cardId: string | null) => void;
    onDateChange: (date: string | null) => void;
    onTypeChange: (type: string | null) => void;
    onNotesChange: (notes: string) => void;
  }

  let {
    cards,
    selectedCardId,
    datePerformed,
    maintenanceType,
    notes,
    error,
    maintenanceTypes,
    onCardChange,
    onDateChange,
    onTypeChange,
    onNotesChange
  }: Props = $props();

  function formatCard(card: Card): string {
    const idParts = card.ownedRollingStockId.split(':');
    const shortId = idParts[idParts.length - 1]?.slice(0, 8) ?? card.ownedRollingStockId;
    const due = card.nextMaintenanceDate ? `Due ${card.nextMaintenanceDate}` : 'No due date';
    return `${shortId}… — ${due}`;
  }

  const todayCalendar = $derived.by(() => {
    const n = new Date();
    return new CalendarDate(n.getFullYear(), n.getMonth() + 1, n.getDate());
  });

  const selectInputClass =
    'flex h-10 w-full rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none';
  const selectInputClassWithDisabled = `${selectInputClass} disabled:cursor-not-allowed disabled:opacity-50`;
  const textareaClass =
    'scrollbar-thin scrollbar-track-zinc-900 scrollbar-thumb-zinc-700 w-full resize-none rounded-md border border-white/10 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 ring-offset-black transition-all placeholder:text-zinc-600 focus:border-amber-500/50 focus:ring-2 focus:ring-amber-500 focus:outline-none';
</script>

<!-- Maintenance Card selector -->
<div class="space-y-2">
  <label
    for="event-card-select"
    class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.maintenance_add_event_select_card()}
  </label>
  <select
    id="event-card-select"
    class={selectInputClassWithDisabled}
    value={selectedCardId ?? ''}
    onchange={(e) => onCardChange((e.target as HTMLSelectElement).value || null)}
  >
    <option value="" disabled>{m.maintenance_add_event_card_placeholder()}</option>
    {#each cards as card (card.id)}
      <option value={card.id} class="bg-zinc-950 text-zinc-100">
        {formatCard(card)}
      </option>
    {/each}
  </select>
  {#if cards.length === 0}
    <p class="text-[11px] text-amber-500/70">
      No maintenance cards available. Create a card first.
    </p>
  {/if}
</div>

<!-- Date Performed -->
<div class="space-y-2">
  <label
    for="event-date-performed"
    class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.maintenance_add_event_date_label()}
  </label>
  <DatePickerField
    id="event-date-performed"
    value={datePerformed}
    onSelect={onDateChange}
    maxValue={todayCalendar}
  />
</div>

<!-- Maintenance Type -->
<div class="space-y-2">
  <label
    for="event-maintenance-type"
    class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase"
  >
    {m.maintenance_add_event_type_label()}
  </label>
  <select
    id="event-maintenance-type"
    class={selectInputClass}
    value={maintenanceType ?? ''}
    onchange={(e) => onTypeChange((e.target as HTMLSelectElement).value || null)}
  >
    <option value="">{m.maintenance_add_event_type_placeholder()}</option>
    {#each maintenanceTypes as type (type.value)}
      <option value={type.value} class="bg-zinc-950 text-zinc-100">{type.label}</option>
    {/each}
  </select>
</div>

<!-- Notes -->
<div class="space-y-2">
  <label for="event-notes" class="text-[10px] font-bold tracking-[0.2em] text-zinc-500 uppercase">
    {m.maintenance_add_event_notes_label()}
  </label>
  <textarea
    id="event-notes"
    rows={3}
    placeholder={m.maintenance_add_event_notes_placeholder()}
    value={notes}
    onchange={(e) => onNotesChange((e.target as HTMLTextAreaElement).value)}
    class={textareaClass}
  ></textarea>
</div>

<!-- Error -->
{#if error}
  <div class="rounded-lg border border-red-500/20 bg-red-500/10 p-3 text-xs text-red-400">
    {error}
  </div>
{/if}

<!-- Validation hints -->
{#if !selectedCardId && cards.length > 0}
  <p class="text-xs font-medium text-amber-500/70">
    {m.maintenance_add_event_validation_card()}
  </p>
{/if}
