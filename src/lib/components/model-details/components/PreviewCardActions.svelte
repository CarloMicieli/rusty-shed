<script lang="ts">
  /**
   * PreviewCardActions — delete trigger button + confirmation dialog for RailwayModelPreviewCard.
   *
   * Rendered only when the parent has an `onDelete` handler. Contains:
   *   - A ghost icon button (revealed on card hover via Tailwind `group-hover:` utilities applied
   *     by the parent's `group` class on the Card root).
   *   - An AlertDialog asking the user to confirm before the deletion is committed.
   */

  interface Props {
    /** Unique identifier of the model being deleted. */
    modelId: string;
    /** Human-readable series name shown in the confirmation dialog body. */
    modelSeries: string | null;
    /** Invoked with `modelId` after the user confirms deletion. */
    onDelete: (modelId: string) => void;
  }

  let { modelId, modelSeries, onDelete }: Props = $props();

  import { Button } from '$lib/components/ui/button';
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import { Trash2 } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let showDeleteDialog = $state(false);

  function handleDeleteConfirm() {
    onDelete(modelId);
    showDeleteDialog = false;
  }
</script>

<!-- Delete trigger: hidden by default, revealed on card hover via parent's `group` class -->
<Button
  variant="ghost"
  size="icon"
  class="-mt-1 -mr-1 h-7 w-7 shrink-0 opacity-0 transition-opacity duration-150 group-hover:opacity-100"
  aria-label={m.components_deleteButton()}
  onclick={(e) => {
    e.stopPropagation();
    showDeleteDialog = true;
  }}
>
  <Trash2 class="h-3.5 w-3.5 text-zinc-500 hover:text-red-400" />
</Button>

<!-- Delete confirmation dialog -->
<AlertDialog.Root bind:open={showDeleteDialog}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{m.components_deleteConfirmTitle()}</AlertDialog.Title>
      <AlertDialog.Description>
        {m.components_deleteConfirmMessage({ model: modelSeries ?? 'this model' })}
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>{m.common_cancel()}</AlertDialog.Cancel>
      <AlertDialog.Action onclick={handleDeleteConfirm}>
        {m.common_delete()}
      </AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
