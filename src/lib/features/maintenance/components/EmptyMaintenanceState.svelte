<script lang="ts">
  import { Wrench } from '@lucide/svelte';
  import * as m from '$lib/paraglide/messages.js';
  import EmptyState from '$lib/components/EmptyState.svelte';

  type Props = { onCreate: () => void };
  let { onCreate }: Props = $props();
</script>

<EmptyState
  icon={Wrench}
  title={m.maintenance_empty_title()}
  description={m.maintenance_empty_message()}
  ctaLabel={m.maintenance_empty_action()}
  onCta={onCreate}
/>
