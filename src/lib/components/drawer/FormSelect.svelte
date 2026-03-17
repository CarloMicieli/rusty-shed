<script lang="ts">
  import * as Select from '$lib/components/ui/select';
  import SearchableSelect from '$lib/components/SearchableSelect.svelte';
  import type { Component } from 'svelte';

  interface SelectOption {
    value: string;
    label: string;
  }

  interface Props {
    label: string;
    options?: SelectOption[];
    value?: string | null;
    isSearchable?: boolean;
    placeholder?: string;
    emptyOption?: SelectOption;
    icon?: Component<{ size?: number; class?: string }>;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    id?: string;
  }

  let {
    label,
    options = [],
    value = $bindable<string | null>(''),
    isSearchable = false,
    placeholder = '—',
    emptyOption,
    icon,
    disabled = false,
    error,
    required,
    id
  }: Props = $props();

  const selectedLabel = $derived(options.find((o) => o.value === value)?.label ?? '');
</script>

<div class="space-y-1">
  <span class="text-xs text-zinc-400">{label}{required ? ' *' : ''}</span>
  {#if isSearchable}
    <SearchableSelect
      {id}
      {options}
      bind:value={value as string}
      {placeholder}
      {emptyOption}
      {disabled}
      {icon}
    />
  {:else}
    <Select.Root
      type="single"
      value={value || undefined}
      {disabled}
      onValueChange={(v) => (value = v)}
    >
      <Select.Trigger {id} class="w-full border-[#1F1F1F] bg-[#0F0F0F] text-[#E0E0E0]">
        {#if value}
          {selectedLabel}
        {:else}
          <span class="text-zinc-500">{placeholder}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each options as opt (opt.value)}
          <Select.Item value={opt.value} label={opt.label} />
        {/each}
      </Select.Content>
    </Select.Root>
  {/if}
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
