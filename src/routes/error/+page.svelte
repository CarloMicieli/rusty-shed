<script lang="ts">
  import { page } from '$app/stores';
  import SignalFailureView from '$lib/components/signal-failure/SignalFailureView.svelte';
  import { generateErrorId } from '$lib/services/error-id';
  import { getModuleLabel } from '$lib/services/module-label';
  import * as m from '$lib/paraglide/messages.js';

  const errorId = $state(generateErrorId());
  const moduleLabel = $derived(getModuleLabel($page.url.pathname));
</script>

<svelte:head>
  <title>{m.error_page_head_title()}</title>
</svelte:head>

<SignalFailureView {errorId} {moduleLabel} />
