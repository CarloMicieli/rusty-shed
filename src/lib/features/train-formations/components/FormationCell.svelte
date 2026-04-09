<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components/ui/button';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import PrototypeIcon from './icons/PrototypeIcon.svelte';
  import OwnershipBadge from './OwnershipBadge.svelte';
  import type { FormationElementView } from '$lib/bindings.js';
  import { prototypeIconType } from '../domain/prototype.js';

  let {
    element,
    onRemove,
    onOpenPicker,
    onTractionToggle
  }: {
    element: FormationElementView;
    onRemove?: (id: string) => void;
    onOpenPicker?: (elementId: string) => void;
    onTractionToggle?: (elementId: string, override: number) => void;
  } = $props();

  const prototype = $derived(element.prototype);
  const ownedCount = $derived(Number(element.owned_count_for_prototype));

  const displayCode = $derived(element.snapshot_series_code ?? prototype.series_code);

  const displayCompany = $derived(element.snapshot_company_name ?? prototype.company_name);
</script>

<div
  class="variant-steampunk-riveted group relative flex min-w-[6.25rem] flex-col items-center gap-2 overflow-visible rounded-sm border p-3 shadow-sm transition-all duration-150 ease-out
    {element.owned_rolling_stock_id
    ? 'border-primary/45 bg-primary/8 text-foreground'
    : ownedCount > 0
      ? 'border-primary/30 bg-background/90 text-foreground'
      : 'border-dashed border-border bg-background/70 text-foreground'}"
>
  <!-- Stock-not-found tombstone indicator -->
  {#if element.stock_not_found}
    <Tooltip.Root>
      <Tooltip.Trigger class="absolute top-1 right-1 size-2 rounded-full bg-red-500"
      ></Tooltip.Trigger>
      <Tooltip.Content>{m.formations_stock_not_found()}</Tooltip.Content>
    </Tooltip.Root>
  {/if}

  <div
    class="absolute top-2 left-2 max-w-[4.5rem] truncate rounded-sm border border-border bg-background/85 px-1.5 py-0.5 font-mono text-[9px] tracking-[0.16em] text-muted-foreground uppercase"
  >
    {displayCompany}
  </div>

  <PrototypeIcon
    type={prototypeIconType(prototype)}
    isOwned={!!element.owned_rolling_stock_id || ownedCount > 0}
    class="mt-4 size-8"
  />

  <div class="flex flex-col items-center gap-0.5 text-center">
    <span class="max-w-[5.5rem] truncate font-mono text-[11px] leading-tight text-foreground">
      {displayCode}
    </span>
    {#if prototype.service_level}
      <span
        class="max-w-[5.5rem] truncate font-mono text-[10px] leading-tight text-muted-foreground uppercase"
      >
        {prototype.service_level}
      </span>
    {/if}
  </div>

  <OwnershipBadge {ownedCount} assignedId={element.owned_rolling_stock_id} />

  <div class="flex w-full flex-col gap-1 pt-0.5">
    {#if !element.owned_rolling_stock_id}
      {#if ownedCount >= 1}
        <Button
          variant="outline"
          size="sm"
          class="variant-steampunk-lever h-6 rounded-sm border-border bg-background/80 px-1.5 text-[9px] tracking-[0.14em] text-foreground uppercase"
          onclick={() => onOpenPicker?.(element.id)}
        >
          {ownedCount === 1 ? m.formations_quick_assign() : m.formations_assign_model()}
        </Button>
      {/if}
    {:else}
      <Button
        variant="outline"
        size="sm"
        class="variant-steampunk-lever h-6 rounded-sm border-border bg-background/80 px-1.5 text-[9px] tracking-[0.14em] text-muted-foreground uppercase"
        onclick={() => onOpenPicker?.(element.id)}
      >
        {m.formations_unassign_model()}
      </Button>
    {/if}
  </div>

  {#if prototype.is_motorized}
    <Tooltip.Root>
      <Tooltip.Trigger>
        <button
          type="button"
          class="variant-steampunk-lever absolute top-1.5 right-1.5 flex size-5 items-center justify-center rounded-sm border border-border bg-background/85 text-[8px] leading-none
            {element.traction_override === 1
            ? 'border-primary/50 bg-primary/15 text-primary'
            : element.traction_override === -1
              ? 'border-border bg-muted text-muted-foreground'
              : 'text-muted-foreground hover:bg-muted'}"
          onclick={() => {
            const next =
              element.traction_override === 0 ? 1 : element.traction_override === 1 ? -1 : 0;
            onTractionToggle?.(element.id, next);
          }}
          aria-label={m.formation_toggle_traction()}
        >
          <span aria-hidden="true">⚡</span>
        </button>
      </Tooltip.Trigger>
      <Tooltip.Content>
        {element.traction_override === 1
          ? m.formations_traction_override_disable()
          : m.formations_traction_override_enable()}
      </Tooltip.Content>
    </Tooltip.Root>
  {/if}

  {#if onRemove}
    <button
      class="text-destructive-foreground absolute -top-2 -right-2 hidden size-5 items-center justify-center rounded-full border border-destructive/30 bg-destructive text-[10px] leading-none shadow-sm group-focus-within:flex group-hover:flex"
      onclick={() => onRemove(element.id)}
      aria-label={m.formation_remove_element()}
    >
      ×
    </button>
  {/if}
</div>
