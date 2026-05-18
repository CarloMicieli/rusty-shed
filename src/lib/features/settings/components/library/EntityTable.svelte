<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { deriveEntityStatusBadge } from '$lib/features/settings/types';
  import type { LibraryEntityRow } from '$lib/services/entityLibrary';
  import EntityRowActions from './EntityRowActions.svelte';

  interface Props {
    rows: LibraryEntityRow[];
    onEdit: (row: LibraryEntityRow) => void;
    onDelete: (row: LibraryEntityRow) => void;
    onMerge: (row: LibraryEntityRow) => void;
  }

  let { rows, onEdit, onDelete, onMerge }: Props = $props();
</script>

<ul class="space-y-2">
  {#each rows as row (row.id)}
    {@const badge = deriveEntityStatusBadge(row.isSystemSeeded, row.usageCount)}
    <li class="rounded-sm border border-border px-3 py-2 text-sm">
      <div class="flex items-center justify-between gap-3">
        <div>
          <span class="font-semibold">{row.name}</span>
          {#if row.countryCode}
            <span class="ml-2 text-muted-foreground">{row.countryCode}</span>
          {/if}
        </div>

        <EntityRowActions
          row={row}
          onEdit={onEdit}
          onDelete={onDelete}
          onMerge={onMerge}
          editDisabled={badge.kind === 'protected'}
          deleteDisabled={badge.kind === 'protected' || badge.kind === 'in-use'}
          mergeDisabled={badge.kind === 'protected'}
        />
      </div>

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
    </li>
  {/each}
</ul>
