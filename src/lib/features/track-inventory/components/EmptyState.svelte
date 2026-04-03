<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { TrainTrack } from 'lucide-svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  interface Props {
    onCreateClick?: () => void;
  }

  const { onCreateClick }: Props = $props();
</script>

<EmptyState
  icon={TrainTrack}
  title={m.track_inventories_empty_title()}
  description={m.track_inventories_empty_message()}
  ctaLabel={onCreateClick ? m.track_inventories_create_button() : undefined}
  onCta={onCreateClick}
/>
