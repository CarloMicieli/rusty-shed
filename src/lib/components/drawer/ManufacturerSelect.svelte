<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { Plus } from 'lucide-svelte';
  import type { Manufacturer } from '$lib/bindings';

  interface Props {
    manufacturerId: string | null;
    manufacturers: Manufacturer[];
    isLoading?: boolean;
    disabled?: boolean;
    error?: string;
    required?: boolean;
    id?: string;
    onQuickAdd?: () => void;
  }

  let {
    manufacturerId = $bindable<string | null>(),
    manufacturers,
    isLoading = false,
    disabled = false,
    error,
    required = false,
    id = 'manufacturer-select',
    onQuickAdd
  }: Props = $props();

  const selectedManufacturer = $derived(
    manufacturers.find((manufacturer) => manufacturer.id === manufacturerId)
  );
</script>

<div class="space-y-1">
  <div class="flex items-center justify-between gap-2">
    <span class="text-[10px] font-bold tracking-[0.2em] text-muted-foreground uppercase"
      >{m.wishlist_modal_manufacturer()}{required ? ' *' : ''}</span
    >
    {#if onQuickAdd}
      <button
        type="button"
        class="rounded-sm border border-border p-1 text-muted-foreground hover:text-foreground"
        onclick={onQuickAdd}
        aria-label={m.quick_add_drawer_title_manufacturer()}
      >
        <Plus size={12} />
      </button>
    {/if}
  </div>
  {#if isLoading}
    <p class="font-mono text-xs text-muted-foreground">{m.wishlist_modal_loading()}</p>
  {:else}
    <Select.Root
      type="single"
      value={manufacturerId || undefined}
      {disabled}
      onValueChange={(value) => {
        manufacturerId = value;
      }}
    >
      <Select.Trigger
        {id}
        class="w-full border-border bg-background text-foreground"
        aria-label={m.wishlist_modal_manufacturer()}
      >
        {#if selectedManufacturer}
          {selectedManufacturer.name}
        {:else}
          <span class="text-muted-foreground">{m.wishlist_modal_manufacturer_placeholder()}</span>
        {/if}
      </Select.Trigger>
      <Select.Content>
        {#each manufacturers as manufacturer (manufacturer.id)}
          <Select.Item value={manufacturer.id} label={manufacturer.name} />
        {/each}
      </Select.Content>
    </Select.Root>
  {/if}
  {#if error}
    <p class="text-xs text-destructive">{error}</p>
  {/if}
</div>
