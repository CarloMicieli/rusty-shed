<script lang="ts">
  import { Badge } from '$lib/components/ui/badge';

  let {
    ownedCount,
    assignedId
  }: {
    ownedCount: number;
    assignedId: string | null;
  } = $props();

  const label = $derived(
    assignedId ? 'Assigned' : ownedCount > 0 ? `${ownedCount} owned` : 'Planned'
  );

  const variant = $derived<'default' | 'secondary' | 'outline'>(
    assignedId ? 'default' : ownedCount > 0 ? 'secondary' : 'outline'
  );
</script>

<Badge {variant} class="text-xs">{label}</Badge>
