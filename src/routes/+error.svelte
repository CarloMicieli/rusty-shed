<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { page } from '$app/stores';
  import SignalFailureView from '$lib/components/signal-failure/SignalFailureView.svelte';
  import { generateErrorId } from '$lib/services/error-id';
  import { getModuleLabel } from '$lib/services/module-label';

  const errorId = $state(generateErrorId());
  const moduleLabel = $derived(getModuleLabel($page.url.pathname));
</script>

<svelte:head>
  <title>Signal Failure | {m.app_name()}</title>
</svelte:head>

<SignalFailureView {errorId} {moduleLabel} />
