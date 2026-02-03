<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    status: 'in-service' | 'under-repair';
    class?: string;
  }

  const { status, class: className = '' }: Props = $props();

  const statusConfig = {
    'in-service': {
      label: m.depot_status_in_service,
      classes: 'bg-success-500/20 text-success-400 border-success-500/30'
    },
    'under-repair': {
      label: m.depot_status_under_repair,
      classes: 'bg-warning-500/20 text-warning-400 border-warning-500/30'
    }
  };

  const config = $derived(statusConfig[status]);
</script>

<span
  class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold {config.classes} {className}"
>
  {config.label()}
</span>
