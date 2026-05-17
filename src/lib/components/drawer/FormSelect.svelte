<script lang="ts">
  import * as Select from '$lib/components/ui/select';
  import SearchableSelect, { type SelectOption } from '$lib/components/SearchableSelect.svelte';
  import { Button } from '$lib/components';
  import { Plus } from 'lucide-svelte';
  import type { Component, Snippet } from 'svelte';

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
    /** Optional custom rendering for each list item */
    item?: Snippet<[SelectOption]>;
    /** Optional custom rendering for the trigger content */
    trigger?: Snippet<[SelectOption | undefined]>;
    /** Optional quick-add action displayed to the right of the select */
    onQuickAdd?: () => void;
    /** Accessible label for the quick-add action */
    quickAddLabel?: string;
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
    id,
    item,
    trigger,
    onQuickAdd,
    quickAddLabel
  }: Props = $props();

  const selectedOption = $derived(options.find((o) => o.value === value));
  const selectedLabel = $derived(selectedOption?.label ?? '');
</script>

<div class="space-y-1">
  <span class="text-[10px] font-bold text-muted-foreground uppercase"
    >{label}{required ? ' *' : ''}</span
  >
  <div class="flex items-center gap-2">
    <div class="min-w-0 flex-1">
      {#if isSearchable}
        <SearchableSelect
          {id}
          {options}
          bind:value={value as string}
          {placeholder}
          {emptyOption}
          {disabled}
          {icon}
          {item}
          {trigger}
        />
      {:else}
        <Select.Root
          type="single"
          value={value || undefined}
          {disabled}
          onValueChange={(v) => (value = v)}
        >
          <Select.Trigger
            {id}
            class="h-10 w-full rounded-sm border border-border bg-background px-3 text-foreground"
          >
            {#if trigger}
              {@render trigger(selectedOption)}
            {:else if value}
              {selectedLabel}
            {:else}
              <span class="text-muted-foreground">{placeholder}</span>
            {/if}
          </Select.Trigger>
          <Select.Content>
            {#each options as opt (opt.value)}
              <Select.Item value={opt.value} label={opt.label}>
                {#if item}
                  {@render item(opt)}
                {:else}
                  {opt.label}
                {/if}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      {/if}
    </div>

    {#if onQuickAdd}
      <Button
        type="button"
        size="icon-lg"
        variant="outline"
        class="variant-steampunk-lever h-10 w-10 rounded-sm border border-border bg-card p-0 text-muted-foreground shadow-none transition-all duration-150 ease-out hover:border-primary hover:bg-primary/10 hover:text-primary"
        aria-label={quickAddLabel ?? label}
        onclick={onQuickAdd}
      >
        <Plus size={16} />
      </Button>
    {/if}
  </div>
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
