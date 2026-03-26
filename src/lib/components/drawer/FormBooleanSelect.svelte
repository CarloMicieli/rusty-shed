<script lang="ts">
  import * as Select from '$lib/components/ui/select';
  import * as m from '$lib/paraglide/messages';

  interface Props {
    label: string;
    value?: boolean | null;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    id?: string;
  }

  let {
    label,
    value = $bindable<boolean | null>(null),
    disabled = false,
    error,
    required = false,
    id
  }: Props = $props();

  const strValue = $derived(value === true ? 'true' : value === false ? 'false' : undefined);
</script>

<div class="space-y-1">
  <span class="text-xs text-zinc-400">{label}{required ? ' *' : ''}</span>
  <Select.Root
    type="single"
    value={strValue}
    {disabled}
    onValueChange={(v) => {
      value = v === 'true' ? true : v === 'false' ? false : null;
    }}
  >
    <Select.Trigger {id} class="w-full border-layout-border bg-layout-surface text-foreground">
      {#if value === true}
        {m.boolean_yes()}
      {:else if value === false}
        {m.boolean_no()}
      {:else}
        <span class="text-zinc-500">—</span>
      {/if}
    </Select.Trigger>
    <Select.Content>
      <Select.Item value="true" label={m.boolean_yes()} />
      <Select.Item value="false" label={m.boolean_no()} />
    </Select.Content>
  </Select.Root>
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
