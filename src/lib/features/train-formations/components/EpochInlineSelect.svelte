<script lang="ts">
  import * as Popover from '$lib/components/ui/popover';
  import epochs from '$lib/data/constants/epochs.json';

  interface Props {
    value: string | null;
    placeholder?: string;
    onSave: (value: string) => Promise<void>;
  }

  let { value, placeholder = '—', onSave }: Props = $props();

  let open = $state(false);
  let isSaving = $state(false);

  async function select(epochId: string) {
    if (isSaving) return;
    isSaving = true;
    open = false;
    try {
      await onSave(epochId);
    } finally {
      isSaving = false;
    }
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <button
        type="button"
        {...props}
        disabled={isSaving}
        class="group relative -mx-1 cursor-pointer rounded border-0 bg-transparent p-1 text-left transition-colors duration-150 hover:border hover:border-dashed hover:border-primary/40 hover:bg-primary/15 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
      >
        {#if value}
          <span class="font-mono text-sm text-foreground">{value}</span>
        {:else}
          <span class="font-mono text-sm text-muted-foreground italic">{placeholder}</span>
        {/if}
      </button>
    {/snippet}
  </Popover.Trigger>

  <Popover.Content class="w-40 rounded-sm p-2" align="start" sideOffset={4}>
    <div class="grid grid-cols-3 gap-1">
      {#each epochs as epoch (epoch.id)}
        <button
          type="button"
          class="rounded-sm px-1.5 py-1.5 font-mono text-xs transition-colors
            {value === epoch.id
            ? 'bg-primary/20 text-primary ring-1 ring-primary/40 ring-inset'
            : 'text-foreground hover:bg-primary/10 hover:text-primary'}"
          onclick={() => select(epoch.id)}
        >
          {epoch.display}
        </button>
      {/each}
    </div>

    {#if value}
      <!-- Clear selection -->
      <div class="mt-1.5 border-t border-border/40 pt-1.5">
        <button
          type="button"
          class="w-full rounded-sm px-1.5 py-1 font-mono text-[10px] tracking-wider text-muted-foreground uppercase transition-colors hover:bg-destructive/10 hover:text-destructive"
          onclick={() => select('')}
        >
          clear
        </button>
      </div>
    {/if}
  </Popover.Content>
</Popover.Root>
