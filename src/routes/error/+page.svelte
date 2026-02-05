<script lang="ts">
  import { resolve } from '$app/paths';
  import { Button } from '$lib/components';
</script>

<svelte:head>
  <title>Ops, something bad happened | Rusty Shed</title>
</svelte:head>

<section class="mx-auto flex max-w-2xl flex-col items-center gap-4 text-center">
  <h1 class="h2 text-error-400 font-bold">Ops, something bad happened</h1>
  <p class="text-surface-300">
    We'll add more guidance here soon. In the meantime, you can head back to the dashboard.
  </p>
  <Button variant="default" href={resolve('/')}>Go Home</Button>
</section>
