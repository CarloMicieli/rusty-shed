<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Button } from '$lib/components';

  interface Props {
    isSubmitting: boolean;
    isLoadingData: boolean;
    onAddItem: () => void;
    onFinalize: () => void;
  }

  let { isSubmitting, isLoadingData, onAddItem, onFinalize }: Props = $props();
</script>

<div class="flex justify-between gap-2 border-t border-white/10 p-4">
  <Button
    type="button"
    variant="outline"
    onclick={onAddItem}
    disabled={isSubmitting || isLoadingData}
  >
    {m.acquisition_add_item_button()}
  </Button>

  <Button
    type="button"
    variant="default"
    onclick={onFinalize}
    disabled={isSubmitting || isLoadingData}
  >
    {isSubmitting ? m.acquisition_finalizing_button() : m.acquisition_finalize_button()}
  </Button>
</div>
