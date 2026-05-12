<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';

  const BASE_EPOCHS = ['I', 'II', 'III', 'IV', 'V', 'Vm', 'VI'] as const;
  const HALF_EPOCHS = ['IIa', 'IIb', 'IIIa', 'IIIb', 'IVa', 'IVb'] as const;

  const EPOCH_RANGES = {
    I: { start: 1835, end: 1920, context: 'State Railways' },
    II: { start: 1920, end: 1945, context: 'Large Network Era' },
    III: { start: 1945, end: 1970, context: 'Post-War/Economic Miracle' },
    IV: { start: 1970, end: 1990, context: 'International Standardization' },
    V: { start: 1990, end: 2006, context: 'Privatization & High Speed' },
    Vm: { start: 2000, end: 2006, context: 'Modern Transition' },
    VI: { start: 2007, end: new Date().getFullYear(), context: 'Current Era' }
  } as const;

  interface Props {
    label: string;
    value?: string | null;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    onchange?: (v: string | null) => void;
  }

  let {
    label,
    value = $bindable<string | null>(null),
    disabled = false,
    error,
    required = false,
    onchange
  }: Props = $props();

  const selected = $derived(value ? value.split('/').filter(Boolean) : []);

  const tooltipId = (epoch: string) => `epoch-tooltip-${epoch.toLowerCase()}`;

  function toggle(epoch: string) {
    const next = selected.includes(epoch)
      ? selected.filter((e) => e !== epoch)
      : [...selected, epoch];
    value = next.length > 0 ? next.join('/') : null;
    onchange?.(value);
  }

  function selectHalf(epoch: string) {
    value = value === epoch ? null : epoch;
    onchange?.(value);
  }
</script>

<div class="space-y-1">
  <span class="text-[10px] font-bold text-muted-foreground uppercase"
    >{label}{required ? ' *' : ''}</span
  >

  {#snippet epochTooltip(epoch: (typeof BASE_EPOCHS)[number])}
    <div class="space-y-1.5">
      <div class="text-sm font-bold text-orange-500">Epoch {epoch}</div>
      <div class="font-mono text-xs text-zinc-300">
        {EPOCH_RANGES[epoch].start}–{EPOCH_RANGES[epoch].end}
      </div>
      <p class="text-xs leading-snug text-zinc-400">
        {EPOCH_RANGES[epoch].context}
      </p>
    </div>
  {/snippet}

  <div class="flex flex-wrap gap-1.5 pt-0.5">
    {#each BASE_EPOCHS as epoch (epoch)}
      <Tooltip.Root>
        <Tooltip.Trigger>
          <button
            type="button"
            {disabled}
            aria-describedby={tooltipId(epoch)}
            class="rounded-sm border px-3 py-1 text-xs font-semibold transition-colors
              {selected.includes(epoch)
              ? 'border-primary bg-primary text-primary-foreground'
              : 'border-border bg-muted text-muted-foreground hover:bg-primary/10 hover:text-foreground'}
              {disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}"
            onclick={() => toggle(epoch)}
          >
            {epoch}
          </button>
        </Tooltip.Trigger>
        <Tooltip.Portal to="body">
          <Tooltip.Content
            id={tooltipId(epoch)}
            role="tooltip"
            side="top"
            align="center"
            sideOffset={8}
            class="z-[1200] max-w-xs rounded-md border border-zinc-800 bg-zinc-900 p-3 shadow-xl"
          >
            {@render epochTooltip(epoch)}
            <Tooltip.Arrow class="fill-zinc-900 stroke-zinc-800" />
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    {/each}
  </div>

  <div class="flex flex-wrap gap-1.5 pt-1">
    {#each HALF_EPOCHS as epoch (epoch)}
      <button
        type="button"
        {disabled}
        class="rounded-sm border px-2.5 py-0.5 text-[11px] font-medium transition-colors
          {value === epoch
          ? 'border-primary bg-primary text-primary-foreground'
          : 'border-border bg-muted/50 text-muted-foreground hover:bg-primary/10 hover:text-foreground'}
          {disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}"
        onclick={() => selectHalf(epoch)}
      >
        {epoch}
      </button>
    {/each}
  </div>
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
