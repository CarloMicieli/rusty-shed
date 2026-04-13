<script lang="ts">
  import { Wallet } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    /** Callback when button is clicked */
    onclick: () => void;
    class?: string;
  }

  const { onclick, class: className = '' }: Props = $props();
</script>

<Button variant="rusty" {onclick} size="sm" class={className}>
  <Wallet size={18} />
  {m.dashboard_chart_budget_set_cta()}
</Button>
