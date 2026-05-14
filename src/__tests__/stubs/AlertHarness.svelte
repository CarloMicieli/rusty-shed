<script lang="ts">
  import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
  import type { AlertVariant } from '$lib/components/ui/alert/types';

  let { variant = 'default' as AlertVariant }: { variant?: AlertVariant } = $props();
  let visible = $state(true);
</script>

<button type="button" onclick={() => (visible = !visible)} data-testid="toggle-alert">toggle</button
>

{#if visible}
  <Alert {variant} class="alert-shell">
    <AlertTitle class="alert-title">Heads up</AlertTitle>
    <AlertDescription class="alert-description">Reactive alert body</AlertDescription>
  </Alert>
{/if}
