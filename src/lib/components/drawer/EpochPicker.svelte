<script lang="ts">
  const BASE_EPOCHS = ['I', 'II', 'III', 'IV', 'V', 'Vm', 'VI'] as const;

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
  <span class="text-xs text-zinc-400">{label}{required ? ' *' : ''}</span>
  <div class="flex flex-wrap gap-1.5 pt-0.5">
    {#each BASE_EPOCHS as epoch (epoch)}
      <button
        type="button"
        {disabled}
        class="rounded-full border px-3 py-1 text-xs font-semibold transition-colors
          {selected.includes(epoch)
          ? 'border-amber-500 bg-amber-500/15 text-amber-400'
          : 'border-zinc-700 bg-zinc-800/60 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}
          {disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}"
        onclick={() => toggle(epoch)}
      >
        {epoch}
      </button>
    {/each}
  </div>
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
