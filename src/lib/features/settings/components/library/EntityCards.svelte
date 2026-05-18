<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { deriveEntityStatusBadge } from '$lib/features/settings/types';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';

  interface Props {
    rows: LibraryEntityRow[];
    onEdit: (row: LibraryEntityRow) => void;
    onDelete: (row: LibraryEntityRow) => void;
    onMerge: (row: LibraryEntityRow) => void;
  }

  let { rows, onEdit, onDelete, onMerge }: Props = $props();
</script>

<ul class="space-y-3" aria-label={m.settings_library_title()}>
  {#each rows as row (row.id)}
    {@const badge = deriveEntityStatusBadge(row.isSystemSeeded, row.usageCount)}
    <li class="rounded-md border border-border bg-card p-4">
      <div class="flex items-start justify-between gap-2">
        <div class="min-w-0 flex-1">
          <p class="truncate font-semibold">{row.name}</p>
          {#if row.countryCode}
            <p class="mt-0.5 text-xs text-muted-foreground">{row.countryCode}</p>
          {/if}
          <div class="mt-2">
            {#if badge.kind === 'protected'}
              <span
                class="rounded-sm border border-border bg-muted px-2 py-0.5 text-xs"
                data-status-kind="protected"
              >
                {m.settings_library_status_protected()}
              </span>
            {:else if badge.kind === 'in-use'}
              <span
                class="rounded-sm border border-border bg-muted px-2 py-0.5 text-xs"
                data-status-kind="in-use"
              >
                {m.settings_library_status_in_use({ count: badge.usageCount })}
              </span>
            {:else}
              <span
                class="rounded-sm border border-border bg-muted px-2 py-0.5 text-xs"
                data-status-kind="unused"
              >
                {m.settings_library_status_unused()}
              </span>
            {/if}
          </div>
        </div>
      </div>

      <div class="mt-3 flex gap-2 border-t border-border pt-3">
        <button
          type="button"
          class="flex-1 rounded-sm border border-border bg-background py-2 text-xs"
          aria-label={m.settings_library_edit_row({ name: row.name })}
          disabled={badge.kind === 'protected'}
          onclick={() => onEdit(row)}
        >
          {m.settings_library_edit()}
        </button>
        <button
          type="button"
          class="flex-1 rounded-sm border border-border bg-background py-2 text-xs"
          aria-label={m.settings_library_delete_row({ name: row.name })}
          disabled={badge.kind === 'protected' || badge.kind === 'in-use'}
          onclick={() => onDelete(row)}
        >
          {m.common_delete()}
        </button>
        <button
          type="button"
          class="flex-1 rounded-sm border border-border bg-background py-2 text-xs"
          aria-label={m.settings_library_merge_row({ name: row.name })}
          disabled={badge.kind === 'protected'}
          onclick={() => onMerge(row)}
        >
          {m.settings_library_merge_action()}
        </button>
      </div>
    </li>
  {/each}
</ul>
