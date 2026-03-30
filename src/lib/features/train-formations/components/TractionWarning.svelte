<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import * as Tooltip from '$lib/components/ui/tooltip';
  import { Zap } from 'lucide-svelte';

  let { hasTraction }: { hasTraction: boolean } = $props();
</script>

{#if !hasTraction}
  <Tooltip.Root>
    <Tooltip.Trigger>
      <div
        class="variant-steampunk-riveted flex items-center gap-2 rounded-sm border border-primary/35 bg-primary/10 px-3 py-2 text-xs font-semibold tracking-[0.18em] text-primary uppercase shadow-sm"
        role="alert"
      >
        <div class="rounded-sm border border-primary/25 bg-background/65 p-1">
          <Zap class="size-3.5 shrink-0" />
        </div>
        {m.formations_no_traction_warning()}
      </div>
    </Tooltip.Trigger>
    <Tooltip.Content>
      {m.formations_traction_warning_tooltip()}
    </Tooltip.Content>
  </Tooltip.Root>
{/if}
