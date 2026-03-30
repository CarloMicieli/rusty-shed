<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import TractionWarning from '$lib/features/train-formations/components/TractionWarning.svelte';

  let { hasTraction }: { hasTraction: boolean } = $props();
</script>

<Tooltip.Provider>
  <TractionWarning {hasTraction} />
</Tooltip.Provider>
