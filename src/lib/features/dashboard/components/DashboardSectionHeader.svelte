<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Badge } from '$lib/components';
  import { resolve } from '$app/paths';

  let {
    title,
    badgeValue = 0,
    link
  } = $props<{
    title: string;
    badgeValue?: number;
    link?: { href: string; label: string };
  }>();
</script>

<div class="mb-4 flex items-center justify-between">
  <h3 class="text-surface-300 text-sm font-bold tracking-wider uppercase">
    {title}
  </h3>

  {#if badgeValue > 0}
    <Badge variant="destructive" class="animate-pulse font-semibold">
      {badgeValue}
      {m.dashboard_due_soon()}
    </Badge>
  {:else if link}
    <a href={resolve(link.href)} class="text-accent-500 text-sm font-bold hover:underline">
      {link.label}
    </a>
  {/if}
</div>
