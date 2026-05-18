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

<ul class="space-y-3" aria-label={m.settings_library_title()}>
  {#each rows as row (row.id)}
    {@const badge = deriveEntityStatusBadge(row.isSystemSeeded, row.usageCount)}
    <li class="rounded-sm border border-border bg-card p-4">
      <div class="flex items-start justify-between gap-2">
        <div class="min-w-0 flex-1">
          <p class="truncate font-semibold">{row.name}</p>
          {#if row.countryCode}
            <p class="mt-0.5 font-mono text-xs text-muted-foreground">{row.countryCode}</p>
          {/if}
          <div class="mt-2">
            {#if badge.kind === 'protected'}
              <span
                class="rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="protected"
              >
                {m.settings_library_status_protected()}
              </span>
            {:else if badge.kind === 'in-use'}
              <span
                class="rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="in-use"
              >
                {m.settings_library_status_in_use({ count: badge.usageCount })}
              </span>
            {:else}
              <span
                class="rounded-sm border border-border bg-background/50 px-2 py-0.5 font-mono text-[10px] tracking-wide uppercase"
                data-status-kind="unused"
              >
                {m.settings_library_status_unused()}
              </span>
            {/if}
          </div>
        </div>
        <EntityRowActions
          {row}
          {onEdit}
          {onDelete}
          {onMerge}
          editDisabled={badge.kind === 'protected'}
          deleteDisabled={badge.kind === 'protected' || badge.kind === 'in-use'}
          mergeDisabled={badge.kind === 'protected'}
        />
      </div>
    </li>
  {/each}
</ul>
