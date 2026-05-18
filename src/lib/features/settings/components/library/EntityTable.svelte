<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
  } from '$lib/components/ui/table';
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

<div class="overflow-hidden rounded-sm border border-border bg-card">
  <Table>
    <TableHeader>
      <TableRow class="bg-background/50 hover:bg-background/50">
        <TableHead
          class="h-10 px-3 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
        >
          {m.settings_library_column_name()}
        </TableHead>
        <TableHead
          class="h-10 px-3 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
        >
          {m.settings_library_column_country()}
        </TableHead>
        <TableHead
          class="h-10 px-3 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
        >
          {m.settings_library_column_status()}
        </TableHead>
        <TableHead
          class="h-10 w-[80px] px-3 text-right text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
        >
          {m.settings_library_column_actions()}
        </TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      {#each rows as row (row.id)}
        {@const badge = deriveEntityStatusBadge(row.isSystemSeeded, row.usageCount)}
        <TableRow class="border-border bg-card hover:bg-background/50">
          <TableCell class="px-3 py-2.5">
            <p class="truncate font-semibold text-foreground">{row.name}</p>
          </TableCell>
          <TableCell class="px-3 py-2.5">
            <span class="font-mono text-xs text-muted-foreground">{row.countryCode ?? '-'}</span>
          </TableCell>
          <TableCell class="px-3 py-2.5">
            {#if badge.kind === 'protected'}
              <span
                class="inline-flex rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="protected"
              >
                {m.settings_library_status_protected()}
              </span>
            {:else if badge.kind === 'in-use'}
              <span
                class="inline-flex rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="in-use"
              >
                {m.settings_library_status_in_use({ count: badge.usageCount })}
              </span>
            {:else}
              <span
                class="inline-flex rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="unused"
              >
                {m.settings_library_status_unused()}
              </span>
            {/if}
          </TableCell>
          <TableCell class="px-3 py-2.5 text-right">
            <EntityRowActions
              {row}
              {onEdit}
              {onDelete}
              {onMerge}
              editDisabled={badge.kind === 'protected'}
              deleteDisabled={badge.kind === 'protected' || badge.kind === 'in-use'}
              mergeDisabled={badge.kind === 'protected'}
            />
          </TableCell>
        </TableRow>
      {/each}
    </TableBody>
  </Table>
</div>
