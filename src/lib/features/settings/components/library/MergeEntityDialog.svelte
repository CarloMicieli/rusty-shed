<script lang="ts">
  import * as AlertDialog from '$lib/components/ui/alert-dialog';
  import * as m from '$lib/paraglide/messages.js';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';

  interface Props {
    open: boolean;
    sourceName: string;
    sourceId: string;
    options: LibraryEntityRow[];
    targetId: string | null;
    merging?: boolean;
    onTargetChange: (targetId: string) => void;
    onOpenChange: (next: boolean) => void;
    onConfirm: () => void;
  }

  let {
    open,
    sourceName,
    sourceId,
    options,
    targetId,
    merging = false,
    onTargetChange,
    onOpenChange,
    onConfirm
  }: Props = $props();

  const canConfirm = $derived(targetId !== null && targetId.length > 0);
</script>

<AlertDialog.Root {open} {onOpenChange}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{m.settings_library_merge_title()}</AlertDialog.Title>
      <AlertDialog.Description>
        {m.settings_library_merge_source_message({ name: sourceName })}
      </AlertDialog.Description>
    </AlertDialog.Header>

    <div class="space-y-2">
      <label class="text-xs text-muted-foreground" for="merge-target-select">
        {m.settings_library_merge_target_label()}
      </label>
      <select
        id="merge-target-select"
        class="w-full rounded-sm border border-border bg-background px-2 py-1 text-sm"
        value={targetId ?? ''}
        onchange={(event) => {
          const value = (event.currentTarget as HTMLSelectElement).value;
          if (value) {
            onTargetChange(value);
          }
        }}
      >
        <option value="">{m.settings_library_merge_target_placeholder()}</option>
        {#each options as option (option.id)}
          {#if option.id !== sourceId}
            <option value={option.id}>{option.name}</option>
          {/if}
        {/each}
      </select>
      {#if !canConfirm}
        <p class="text-xs text-muted-foreground">{m.settings_library_merge_target_required()}</p>
      {/if}
    </div>

    <AlertDialog.Footer>
      <AlertDialog.Cancel disabled={merging}>{m.common_cancel()}</AlertDialog.Cancel>
      <AlertDialog.Action onclick={onConfirm} disabled={!canConfirm || merging}>
        {m.settings_library_merge_action()}
      </AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
