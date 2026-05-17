<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';

  interface Props {
    row: LibraryEntityRow;
    onEdit: (row: LibraryEntityRow) => void;
    editDisabled?: boolean;
  }

  let { row, onEdit, editDisabled = false }: Props = $props();
</script>

<button
  type="button"
  class="rounded-sm border border-border bg-background px-2 py-1 text-xs"
  aria-label={m.settings_library_edit_row({ name: row.name })}
  disabled={editDisabled}
  onclick={() => onEdit(row)}
>
  {m.settings_library_edit()}
</button>
