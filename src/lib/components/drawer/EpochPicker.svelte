<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';

  const BASE_EPOCHS = ['I', 'II', 'III', 'IV', 'V', 'Vm', 'VI'] as const;

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

  function toggle(epoch: string) {
    const next = selected.includes(epoch)
      ? selected.filter((e) => e !== epoch)
      : [...selected, epoch];
    value = next.length > 0 ? next.join('/') : null;
    onchange?.(value);
  }
</script>

<div class="space-y-1">
  <span class="text-[10px] font-bold text-muted-foreground uppercase"
    >{label}{required ? ' *' : ''}</span
  >
  <div class="flex flex-wrap gap-1.5 pt-0.5">
    {#each BASE_EPOCHS as epoch (epoch)}
      <Tooltip.Root>
        <Tooltip.Trigger>
          <button
            type="button"
            {disabled}
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
        <Tooltip.Content class="max-w-xs">
          <div class="space-y-1">
            <div class="text-sm font-semibold">Epoch {epoch}</div>
            <div class="font-mono text-xs">
              {EPOCH_RANGES[epoch as keyof typeof EPOCH_RANGES].start}–{EPOCH_RANGES[
                epoch as keyof typeof EPOCH_RANGES
              ].end}
            </div>
            <div class="text-xs text-muted-foreground">
              {EPOCH_RANGES[epoch as keyof typeof EPOCH_RANGES].context}
            </div>
          </div>
        </Tooltip.Content>
      </Tooltip.Root>
    {/each}
  </div>
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
