<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    details?: string | null;
  }

  let { details }: Props = $props();
</script>

<div class="prose max-w-none dark:prose-invert">
  {#if details}
    <p>{details}</p>
  {:else}
    <p class="text-muted-foreground">{m.model_no_details()}</p>
  {/if}
</div>
