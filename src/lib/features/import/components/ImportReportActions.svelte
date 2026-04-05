<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    onClose?: () => void;
  }

  const { onClose = () => {} }: Props = $props();
</script>

<div class="report-actions">
  <Button
    class="variant-steampunk-lever font-bebas font-bold tracking-widest"
    onclick={onClose}
    type="button"
  >
    {m.common_close()}
  </Button>
</div>

<style>
  .report-actions {
    display: flex;
    justify-content: center;
    padding-top: 1rem;
    border-top: 1px solid hsl(var(--border));
  }
</style>
