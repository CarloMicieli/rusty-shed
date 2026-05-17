<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components';
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
      <Button
        type="button"
        size="icon-sm"
        variant="outline"
        class="variant-steampunk-lever rounded-sm border-border bg-background text-muted-foreground hover:border-primary/50 hover:bg-primary/5 hover:text-primary"
        onclick={onQuickAdd}
        aria-label={m.quick_add_drawer_title_manufacturer()}
      >
        <Plus size={12} />
      </Button>
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
