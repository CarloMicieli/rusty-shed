<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';

  interface Props {
    row: LibraryEntityRow;
    onEdit: (row: LibraryEntityRow) => void;
    onDelete: (row: LibraryEntityRow) => void;
    editDisabled?: boolean;
    deleteDisabled?: boolean;
  }

  let { row, onEdit, onDelete, editDisabled = false, deleteDisabled = false }: Props = $props();
</script>

<div class="flex items-center gap-2">
  <button
    type="button"
    class="rounded-sm border border-border bg-background px-2 py-1 text-xs"
    aria-label={m.settings_library_edit_row({ name: row.name })}
    disabled={editDisabled}
    onclick={() => onEdit(row)}
  >
    {m.settings_library_edit()}
  </button>
  <button
    type="button"
    class="rounded-sm border border-border bg-background px-2 py-1 text-xs"
    aria-label={m.settings_library_delete_row({ name: row.name })}
    disabled={deleteDisabled}
    onclick={() => onDelete(row)}
  >
    {m.common_delete()}
  </button>
</div>
