<script lang="ts">
  import { Ellipsis } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';

  interface Props {
    row: LibraryEntityRow;
    onEdit: (row: LibraryEntityRow) => void;
    onDelete: (row: LibraryEntityRow) => void;
    onMerge: (row: LibraryEntityRow) => void;
    editDisabled?: boolean;
    deleteDisabled?: boolean;
    mergeDisabled?: boolean;
  }

  let {
    row,
    onEdit,
    onDelete,
    onMerge,
    editDisabled = false,
    deleteDisabled = false,
    mergeDisabled = false
  }: Props = $props();
</script>

<details class="group relative inline-block text-left">
  <summary
    class="inline-flex h-8 w-8 list-none items-center justify-center rounded-sm border border-border bg-background/50 text-muted-foreground transition-colors hover:bg-background hover:text-foreground"
    aria-label={m.settings_library_row_actions({ name: row.name })}
  >
    <Ellipsis class="h-4 w-4" />
  </summary>

  <div
    class="absolute top-9 right-0 z-20 w-40 rounded-sm border border-border bg-card p-1 shadow-lg"
    role="menu"
    aria-label={m.settings_library_row_actions({ name: row.name })}
  >
    <button
      type="button"
      class="w-full rounded-sm px-2 py-1.5 text-left text-xs transition-colors hover:bg-background/50 disabled:cursor-not-allowed disabled:opacity-50"
      aria-label={m.settings_library_edit_row({ name: row.name })}
      disabled={editDisabled}
      onclick={() => onEdit(row)}
    >
      {m.settings_library_edit()}
    </button>
    <button
      type="button"
      class="w-full rounded-sm px-2 py-1.5 text-left text-xs transition-colors hover:bg-background/50 disabled:cursor-not-allowed disabled:opacity-50"
      aria-label={m.settings_library_merge_row({ name: row.name })}
      disabled={mergeDisabled}
      onclick={() => onMerge(row)}
    >
      {m.settings_library_merge_action()}
    </button>
    <button
      type="button"
      class="w-full rounded-sm px-2 py-1.5 text-left text-xs text-destructive transition-colors hover:bg-background/50 disabled:cursor-not-allowed disabled:opacity-50"
      aria-label={m.settings_library_delete_row({ name: row.name })}
      disabled={deleteDisabled}
      onclick={() => onDelete(row)}
    >
      {m.common_delete()}
    </button>
  </div>
</details>
