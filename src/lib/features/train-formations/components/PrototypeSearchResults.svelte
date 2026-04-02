<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Skeleton } from '$lib/components/ui/skeleton';
  import { Plus } from 'lucide-svelte';
  import PrototypeIcon from './icons/PrototypeIcon.svelte';
  import type { PrototypeGroupView } from '$lib/bindings.js';
  import { prototypeIconType } from '../domain/prototype.js';

  let {
    groups,
    isLoading,
    onSelect
  }: {
    groups: PrototypeGroupView[];
    isLoading: boolean;
    onSelect: (prototypeId: string) => void;
  } = $props();
</script>

{#if isLoading}
  <div class="space-y-2">
    {#each Array.from({ length: 4 }) as _, index (index)}
      <Skeleton class="h-10 w-full" />
    {/each}
  </div>
{:else if groups.length === 0}
  <p class="py-6 text-center text-sm text-muted-foreground">No prototypes found</p>
{:else}
  <div class="overflow-y-auto">
    {#each groups as group (group.railway_company_id)}
      <p
        class="sticky top-0 z-10 border-b border-border bg-background/90 px-2 py-1 text-[10px] font-semibold tracking-widest text-muted-foreground uppercase backdrop-blur-sm"
      >
        {group.company_name}
      </p>

      {#each group.prototypes as proto (proto.id)}
        <div
          class="group flex cursor-pointer items-center justify-between border-b border-border px-2 py-2 transition-colors hover:bg-primary/5"
          role="button"
          tabindex="0"
          onclick={() => onSelect(proto.id)}
          onkeydown={(e) => e.key === 'Enter' && onSelect(proto.id)}
        >
          <div class="flex items-center gap-3">
            <PrototypeIcon type={prototypeIconType(proto)} isOwned={false} class="size-6 shrink-0" />
            <div class="flex flex-col">
              <span class="text-sm leading-tight font-bold">{proto.series_code}</span>
              {#if proto.service_level}
                <span class="font-mono text-[10px] text-muted-foreground uppercase">
                  {proto.service_level}
                </span>
              {/if}
              {#if proto.is_custom}
                <span class="font-mono text-[10px] text-muted-foreground uppercase">custom</span>
              {/if}
            </div>
          </div>

          <Button
            variant="ghost"
            size="icon"
            class="size-7 shrink-0 opacity-0 transition-all group-hover:border-primary/40 group-hover:bg-primary/15 group-hover:text-primary group-hover:opacity-100"
            onclick={(e) => {
              e.stopPropagation();
              onSelect(proto.id);
            }}
          >
            <Plus class="size-3.5" />
          </Button>
        </div>
      {/each}
    {/each}
  </div>
{/if}
