<script lang="ts">
  import { Wrench } from 'lucide-svelte';
  import type { MaintenanceCardEventView } from '$lib/bindings';

  let { events } = $props<{ events: MaintenanceCardEventView[] }>();

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date
      .toLocaleDateString(undefined, { year: 'numeric', month: '2-digit', day: '2-digit' })
      .replace(/\//g, '.');
  }
</script>

{#if events.length === 0}
  <div class="flex flex-col items-center justify-center gap-4 py-16 text-zinc-600">
    <Wrench size={48} strokeWidth={1} />
    <p class="text-sm font-medium">No events logged yet.</p>
  </div>
{:else}
  <div class="space-y-3">
    {#each events as event (event.id)}
      <div class="space-y-2 rounded-xl border border-[#1F1F1F] bg-[#0c0c0c] p-4">
        <div class="flex items-center justify-between">
          <span class="font-mono text-sm font-bold text-zinc-200"
            >{formatDate(event.datePerformed)}</span
          >
          {#if event.maintenanceType}
            <span
              class="rounded bg-zinc-800 px-2 py-0.5 text-[10px] font-bold tracking-widest text-zinc-500 uppercase"
            >
              {event.maintenanceType.replace(/_/g, ' ')}
            </span>
          {/if}
        </div>
        {#if event.notes}
          <p class="text-xs text-zinc-500">{event.notes}</p>
        {/if}
      </div>
    {/each}
  </div>
{/if}
