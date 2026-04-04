<script lang="ts">
  import { type FeatureFlag } from '$lib/bindings';

  interface Props {
    id?: string;
    label: string;
    value: FeatureFlag | null | undefined;
    disabled?: boolean;
    compact?: boolean;
    onUpdate?: (value: FeatureFlag) => void;
  }

  let {
    id,
    label,
    value = $bindable<'YES' | 'NO' | 'NOT_APPLICABLE'>('NOT_APPLICABLE'),
    disabled = false,
    compact = false,
    onUpdate
  }: Props = $props();

  // Handle Legacy null/undefined values by mapping them to NOT_APPLICABLE
  if (value === null || value === undefined) {
    value = 'NOT_APPLICABLE';
  }

  const cycleState = () => {
    if (disabled) return;

    const nextState: FeatureFlag =
      value === 'YES' ? 'NO' : value === 'NO' ? 'NOT_APPLICABLE' : 'YES';

    value = nextState;
    onUpdate?.(nextState);
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === ' ' || e.key === 'Enter') {
      e.preventDefault();
      cycleState();
    }
  };

  const getAriaLabel = () => {
    switch (value) {
      case 'YES':
        return `${label}: Yes`;
      case 'NO':
        return `${label}: No`;
      case 'NOT_APPLICABLE':
        return `${label}: Not Applicable`;
      default:
        return label;
    }
  };
</script>

{#snippet statusLamp(state: FeatureFlag | null | undefined)}
  <div
    class="relative h-4 w-4 rounded-full border border-zinc-700/50 transition-all duration-300 ease-out"
    class:bg-primary={state === 'YES'}
    class:shadow-[0_0_12px_theme(colors.primary.DEFAULT)]={state === 'YES'}
    class:bg-zinc-900={state === 'NO'}
    class:bg-zinc-800={state === 'NOT_APPLICABLE'}
    class:opacity-30={state === 'NOT_APPLICABLE'}
  >
    {#if state === 'NOT_APPLICABLE'}
      <div class="absolute inset-0 flex items-center justify-center overflow-hidden rounded-full">
        <div class="h-[1px] w-full rotate-45 bg-muted-foreground/50"></div>
        <div class="absolute h-[1px] w-full -rotate-45 bg-muted-foreground/50"></div>
      </div>
    {/if}
    <!-- Inner filament effect -->
    {#if state === 'YES'}
      <div class="absolute inset-[3px] rounded-full bg-white/20 blur-[1px]"></div>
    {/if}
  </div>
{/snippet}

<div class={compact ? '' : 'flex flex-col gap-1'}>
  {#if !compact}
    <span class="text-[10px] tracking-tighter whitespace-nowrap text-muted-foreground uppercase">
      {label}
    </span>
  {/if}

  <button
    {id}
    type="button"
    class="group flex items-center gap-3 rounded-sm border border-border bg-card px-3 transition-colors hover:bg-white/5 focus-visible:ring-1 focus-visible:ring-primary focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 {compact
      ? 'h-7 border-none bg-transparent p-0'
      : 'h-9 w-full'}"
    onclick={cycleState}
    onkeydown={handleKeyDown}
    aria-label={getAriaLabel()}
    {disabled}
  >
    <div class="flex-shrink-0">
      {@render statusLamp(value)}
    </div>

    <span
      class="font-mono text-[11px] tracking-wider uppercase transition-colors duration-200 {value ===
      'YES'
        ? 'text-primary'
        : value === 'NO'
          ? 'text-muted-foreground'
          : 'text-muted-foreground/30'}"
    >
      {#if value === 'YES'}
        Yes
      {:else if value === 'NO'}
        No
      {:else}
        N/A
      {/if}
    </span>
  </button>
</div>
