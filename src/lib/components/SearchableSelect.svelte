<script lang="ts">
  import { Search, ChevronDown, Check } from 'lucide-svelte';
  import type { Component, Snippet } from 'svelte';
  import * as Popover from '$lib/components/ui/popover';

  export interface SelectOption {
    value: string;
    label: string;
    /** The legally registered company name (optional). */
    registeredCompanyName?: string | null;
    /** The ISO 3166-1 alpha-2 country code (optional). */
    countryCode?: string | null;
  }

  interface Props {
    /** All selectable options */
    options?: SelectOption[];
    /** Currently selected value — two-way bindable */
    value?: string;
    /** Placeholder shown when no value is selected */
    placeholder?: string;
    /** Optional item inserted at the top representing "no selection" */
    emptyOption?: SelectOption;
    /** Disable the trigger and prevent interaction */
    disabled?: boolean;
    /** HTML id applied to the trigger button */
    id?: string;
    /** Optional lucide-svelte icon rendered left of the selected label */
    icon?: Component<{ size?: number | undefined; class?: string | undefined }>;
    /** Optional callback fired when a selection is made */
    onSelect?: (value: string) => void;
    /** Optional custom rendering for each list item */
    item?: Snippet<[SelectOption]>;
    /** Optional custom rendering for the trigger content */
    trigger?: Snippet<[SelectOption | undefined]>;
  }

  let {
    options = [],
    value = $bindable(''),
    placeholder = 'Select…',
    emptyOption,
    disabled = false,
    id,
    icon: LeadingIcon,
    onSelect,
    item,
    trigger
  }: Props = $props();

  let open = $state(false);
  let filter = $state('');
  let triggerRef = $state<HTMLButtonElement | null>(null);
  let searchInputEl = $state<HTMLInputElement | null>(null);
  let contentWidth = $state(240);

  // Measure trigger width once the element is available so content can match it
  $effect(() => {
    if (triggerRef) {
      contentWidth = triggerRef.offsetWidth;
    }
  });

  // Focus the search input as soon as the popover opens; clear on close
  $effect(() => {
    if (open) {
      // defer one tick to ensure the input is rendered
      setTimeout(() => searchInputEl?.focus(), 0);
    } else {
      filter = '';
    }
  });

  const allOptions: SelectOption[] = $derived(emptyOption ? [emptyOption, ...options] : options);

  const filtered: SelectOption[] = $derived(
    filter.trim() === ''
      ? allOptions
      : allOptions.filter((o) => o.label.toLowerCase().includes(filter.toLowerCase()))
  );

  const selectedOption = $derived(allOptions.find((o) => o.value === value));
  const selectedLabel = $derived(selectedOption?.label ?? '');

  function select(opt: SelectOption) {
    value = opt.value;
    onSelect?.(opt.value);
    open = false;
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger
    {id}
    {disabled}
    bind:ref={triggerRef}
    class="flex h-10 w-full items-center justify-between rounded-sm border border-border bg-background px-3 text-sm text-foreground transition-colors
      focus-visible:border-primary/60 focus-visible:ring-1 focus-visible:ring-primary focus-visible:outline-none
      disabled:cursor-not-allowed disabled:opacity-50
      {open ? 'border-primary/60' : ''}"
  >
    <span class="flex min-w-0 items-center gap-3">
      {#if LeadingIcon}
        <LeadingIcon size={16} class="shrink-0 text-muted-foreground" />
      {/if}
      <span class="truncate {value ? 'text-foreground' : 'text-muted-foreground'}">
        {#if trigger}
          {@render trigger(selectedOption)}
        {:else if selectedLabel}
          {selectedLabel}
        {:else}
          {placeholder}
        {/if}
      </span>
    </span>
    <ChevronDown
      size={16}
      class="ml-2 shrink-0 text-muted-foreground transition-transform duration-150 {open
        ? 'rotate-180'
        : ''}"
    />
  </Popover.Trigger>

  <Popover.Content
    align="start"
    sideOffset={4}
    style="width: {contentWidth}px;"
    class="rounded-sm border-border bg-card p-2 shadow-xl"
  >
    <!-- Filter input -->
    <div class="relative mb-2">
      <Search
        size={13}
        class="pointer-events-none absolute top-1/2 left-2.5 -translate-y-1/2 text-muted-foreground"
      />
      <input
        bind:this={searchInputEl}
        type="text"
        bind:value={filter}
        placeholder="Search…"
        class="h-8 w-full rounded-sm border border-border bg-background pr-3 pl-7 text-xs text-foreground placeholder:text-muted-foreground focus:border-primary/60 focus:ring-1 focus:ring-primary focus:outline-none"
        onkeydown={(e) => {
          if (e.key === 'Escape') {
            open = false;
          }
        }}
      />
    </div>

    <!-- Options list -->
    <div class="max-h-48 space-y-px overflow-y-auto">
      {#if filtered.length === 0}
        <p class="py-3 text-center text-xs text-zinc-600">No results</p>
      {:else}
        {#each filtered as opt (opt.value)}
          <button
            type="button"
            class="flex w-full items-center gap-2 rounded-sm px-3 py-2 text-left text-sm transition-colors hover:bg-primary/10 hover:text-primary {value ===
            opt.value
              ? 'border-primary bg-primary/10 text-primary'
              : 'text-foreground'}"
            onclick={() => select(opt)}
          >
            {#if value === opt.value}
              <Check size={13} class="shrink-0 text-primary" />
            {:else}
              <span class="w-[13px] shrink-0"></span>
            {/if}
            <span class="truncate">
              {#if item}
                {@render item(opt)}
              {:else}
                {opt.label}
              {/if}
            </span>
          </button>
        {/each}
      {/if}
    </div>
  </Popover.Content>
</Popover.Root>
