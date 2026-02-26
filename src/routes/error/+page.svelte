<script lang="ts">
  import { resolve } from '$app/paths';
  import { Button } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';
</script>

<svelte:head>
  <title>{m.error_page_title()} | {m.app_name()}</title>
</svelte:head>

<section class="mx-auto flex max-w-2xl flex-col items-center gap-4 text-center">
  <h1 class="text-2xl font-bold text-destructive">{m.error_page_title()}</h1>
  <p class="text-muted-foreground">{m.error_page_body()}</p>
  <Button variant="default" href={resolve('/')}>{m.error_go_home()}</Button>
</section>
