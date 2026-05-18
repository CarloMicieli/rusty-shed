<script lang="ts">
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    open: boolean;
    entityName: string;
    linkedCount: number;
    deleting?: boolean;
    onOpenChange: (next: boolean) => void;
    onConfirm: () => void;
  }

  let {
    open,
    entityName,
    linkedCount,
    deleting = false,
    onOpenChange,
    onConfirm
  }: Props = $props();
</script>

<AlertDialog.Root {open} onOpenChange={onOpenChange}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{m.settings_library_delete_title()}</AlertDialog.Title>
      <AlertDialog.Description>
        {m.settings_library_delete_confirm_message({ name: entityName })}
      </AlertDialog.Description>
      <p class="text-xs text-muted-foreground" data-testid="linked-items-count">
        {m.settings_library_delete_linked_items({ count: linkedCount })}
      </p>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel disabled={deleting}>{m.common_cancel()}</AlertDialog.Cancel>
      <AlertDialog.Action onclick={onConfirm} disabled={deleting}>
        {m.common_delete()}
      </AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
