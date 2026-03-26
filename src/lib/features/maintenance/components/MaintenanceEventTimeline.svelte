<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Wrench, Trash2 } from 'lucide-svelte';
  import type { MaintenanceCardEventView } from '$lib/bindings';
  import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

  let { events, onDeleteEvent } = $props<{
    events: MaintenanceCardEventView[];
    onDeleteEvent?: (id: string) => void;
  }>();

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date
      .toLocaleDateString(regionalManager.locale, {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit'
      })
      .replace(/\//g, '.');
  }
</script>

{#if events.length === 0}
  <div class="flex flex-col items-center justify-center gap-4 py-16 text-zinc-600">
    <Wrench size={48} strokeWidth={1} />
    <p class="text-sm font-medium">{m.maintenance_no_events()}</p>
  </div>
{:else}
  <div class="space-y-3">
    {#each events as event (event.id)}
      <div class="space-y-2 rounded-xl border border-layout-border bg-layout-surface p-4">
        <div class="flex items-center justify-between">
          <span class="font-mono text-sm font-bold text-zinc-200"
            >{formatDate(event.datePerformed)}</span
          >
          <div class="flex items-center gap-2">
            {#if event.maintenanceType}
              <span
                class="rounded bg-zinc-800 px-2 py-0.5 text-[10px] font-bold tracking-widest text-zinc-500 uppercase"
              >
                {event.maintenanceType.replace(/_/g, ' ')}
              </span>
            {/if}
            {#if onDeleteEvent}
              <button
                type="button"
                onclick={() => onDeleteEvent(event.id)}
                class="rounded p-1 text-zinc-600 transition-colors hover:bg-red-950/40 hover:text-red-500"
                aria-label="Delete event"
              >
                <Trash2 size={14} />
              </button>
            {/if}
          </div>
        </div>
        {#if event.notes}
          <p class="text-xs text-zinc-500">{event.notes}</p>
        {/if}
      </div>
    {/each}
  </div>
{/if}
