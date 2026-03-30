<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Zap, BoxesIcon } from 'lucide-svelte';
  import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
  import { Badge } from '$lib/components/ui/badge';
  import type { TrainFormationSummary } from '$lib/bindings.js';

  let {
    summary,
    onclick
  }: {
    summary: TrainFormationSummary;
    onclick?: () => void;
  } = $props();
</script>

<button
  type="button"
  class="block w-full rounded-lg text-left disabled:pointer-events-none"
  {onclick}
  disabled={!onclick}
>
  <Card class="transition-colors hover:border-primary/50 {onclick ? 'cursor-pointer' : ''}">
    <CardHeader class="pb-2">
      <div class="flex items-start justify-between gap-2">
        <CardTitle class="text-base leading-tight">{summary.name}</CardTitle>
        {#if summary.has_traction}
          <Zap class="mt-0.5 size-4 shrink-0 text-yellow-500" aria-label="Has traction" />
        {/if}
      </div>
      <div class="flex flex-wrap gap-1 pt-1">
        {#if summary.category}
          <Badge variant="secondary" class="text-xs">{summary.category.name}</Badge>
        {/if}
        {#if summary.epoch}
          <Badge variant="outline" class="text-xs">Ep. {summary.epoch}</Badge>
        {/if}
      </div>
    </CardHeader>
    <CardContent class="pt-0">
      <div class="flex items-center gap-4 text-sm text-muted-foreground">
        <span class="flex items-center gap-1">
          <BoxesIcon class="size-3.5" />
          {m.formations_element_count({ n: Number(summary.element_count) })}
        </span>
        {#if Number(summary.owned_count) > 0}
          <span class="text-green-600 dark:text-green-400">
            {m.formations_owned_count({ n: Number(summary.owned_count) })}
          </span>
        {/if}
      </div>
    </CardContent>
  </Card>
</button>
