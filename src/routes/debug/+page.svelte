<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Button,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow
  } from '$lib/components';
  import { fetchDbStats, fetchRecentLogs } from '$lib/services';
  import { getToastMessage } from '$lib/services/errors';
  import type { DatabaseTableStat } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  let dbStats = $state<DatabaseTableStat[]>([]);
  let logs = $state<string[]>([]);
  let dbLoading = $state(true);
  let logsLoading = $state(true);
  let dbError = $state<string | null>(null);
  let logsError = $state<string | null>(null);

  const latestLogCount = $derived(logs.length);

  onMount(async () => {
    await Promise.all([loadDbStats(), loadLogs()]);
  });

  async function loadDbStats() {
    dbLoading = true;
    const result = await fetchDbStats();
    if (result.ok) {
      dbStats = result.data;
      dbError = null;
    } else {
      dbError = getToastMessage(result.error);
    }
    dbLoading = false;
  }

  async function loadLogs() {
    logsLoading = true;
    const result = await fetchRecentLogs();
    if (result.ok) {
      logs = result.data;
      logsError = null;
    } else {
      logsError = getToastMessage(result.error);
    }
    logsLoading = false;
  }

  function formatBytes(bytes: number | null | undefined): string {
    if (bytes == null) {
      return m.debug_size_unavailable();
    }

    if (bytes < 1024) {
      return `${bytes} B`;
    }

    const kilobytes = bytes / 1024;
    if (kilobytes < 1024) {
      return `${kilobytes.toFixed(kilobytes >= 100 ? 0 : 1)} KB`;
    }

    const megabytes = kilobytes / 1024;
    if (megabytes < 1024) {
      return `${megabytes.toFixed(megabytes >= 100 ? 0 : 1)} MB`;
    }

    const gigabytes = megabytes / 1024;
    return `${gigabytes.toFixed(gigabytes >= 100 ? 0 : 1)} GB`;
  }

  function formatLineNumber(index: number): string {
    return String(index + 1).padStart(3, '0');
  }
</script>

<svelte:head>
  <title>{m.app_name()} | {m.app_debug()}</title>
</svelte:head>

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <div class="flex flex-col gap-1">
      <p class="text-sm font-semibold tracking-[0.2em] text-muted-foreground uppercase">
        {m.debug_subtitle()}
      </p>
      <h1 class="font-bebas text-4xl tracking-widest text-primary uppercase lg:text-5xl">
        {m.debug_heading()}
      </h1>
      <p class="max-w-3xl text-sm text-muted-foreground">{m.debug_description()}</p>
    </div>
  </div>

  <div class="grid gap-6 xl:grid-cols-[minmax(0,1.1fr)_minmax(0,0.9fr)]">
    <section class="rounded-sm border border-border bg-card shadow-xl">
      <header class="border-b border-border p-6">
        <p class="text-xs font-semibold tracking-widest text-muted-foreground uppercase">
          {m.debug_database_title()}
        </p>
        <h2 class="mt-2 font-bebas text-2xl tracking-widest text-foreground uppercase">
          {m.debug_database_title()}
        </h2>
        <p class="mt-2 max-w-2xl text-sm text-muted-foreground">
          {m.debug_database_description()}
        </p>
      </header>

      <div class="p-6 pt-0">
        {#if dbLoading}
          <div class="animate-pulse space-y-3 py-6">
            <div class="h-10 rounded-sm bg-background/70"></div>
            <div class="h-10 rounded-sm bg-background/70"></div>
            <div class="h-10 rounded-sm bg-background/70"></div>
          </div>
        {:else if dbError}
          <div class="mt-6 rounded-sm border border-destructive/30 bg-destructive/10 p-6">
            <p class="font-semibold text-destructive">{dbError}</p>
            <Button variant="default" class="mt-4" onclick={loadDbStats}>
              {m.errors_retry_page()}
            </Button>
          </div>
        {:else if dbStats.length === 0}
          <div class="mt-6 rounded-sm border border-border bg-background/50 p-6">
            <p class="text-sm text-muted-foreground">{m.debug_database_empty()}</p>
          </div>
        {:else}
          <div class="mt-6 overflow-hidden rounded-sm border border-border bg-card">
            <Table>
              <TableHeader>
                <TableRow class="bg-background/50 hover:bg-background/50">
                  <TableHead
                    class="h-10 px-3 text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
                  >
                    {m.debug_database_column_table()}
                  </TableHead>
                  <TableHead
                    class="h-10 px-3 text-right text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
                  >
                    {m.debug_database_column_rows()}
                  </TableHead>
                  <TableHead
                    class="h-10 px-3 text-right text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
                  >
                    {m.debug_database_column_size()}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each dbStats as stat (stat.tableName)}
                  <TableRow class="border-border bg-card hover:bg-background/50">
                    <TableCell class="px-3 py-3 font-semibold text-foreground">
                      {stat.tableName}
                    </TableCell>
                    <TableCell class="px-3 py-3 text-right font-mono text-sm text-foreground">
                      {stat.rowCount.toLocaleString()}
                    </TableCell>
                    <TableCell class="px-3 py-3 text-right font-mono text-sm text-muted-foreground">
                      {formatBytes(stat.estimatedBytes)}
                    </TableCell>
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      </div>
    </section>

    <section class="rounded-sm border border-border bg-card shadow-xl">
      <header class="flex items-start justify-between gap-4 border-b border-border p-6">
        <div>
          <p class="text-xs font-semibold tracking-widest text-muted-foreground uppercase">
            {m.debug_logs_title()}
          </p>
          <h2 class="mt-2 font-bebas text-2xl tracking-widest text-foreground uppercase">
            {m.debug_logs_title()}
          </h2>
          <p class="mt-2 text-sm text-muted-foreground">{m.debug_logs_description()}</p>
        </div>

        <div class="flex items-center gap-3">
          <div class="text-right">
            <p class="text-xs tracking-widest text-muted-foreground uppercase">
              {m.debug_logs_latest_label()}
            </p>
            <p class="font-mono text-sm text-foreground">{latestLogCount}</p>
          </div>
          <Button variant="outline" onclick={loadLogs} disabled={logsLoading}>
            {m.debug_logs_refresh()}
          </Button>
        </div>
      </header>

      <div class="p-6 pt-0">
        {#if logsLoading}
          <div class="mt-6 animate-pulse rounded-sm border border-border bg-background/80 p-4">
            <div class="space-y-2">
              {#each Array(8) as _item, index (index)}
                <div class="h-4 rounded-sm bg-card"></div>
              {/each}
            </div>
          </div>
        {:else if logsError}
          <div class="mt-6 rounded-sm border border-destructive/30 bg-destructive/10 p-6">
            <p class="font-semibold text-destructive">{logsError}</p>
            <Button variant="default" class="mt-4" onclick={loadLogs}>
              {m.errors_retry_page()}
            </Button>
          </div>
        {:else if logs.length === 0}
          <div class="mt-6 rounded-sm border border-border bg-background/80 p-6">
            <p class="font-mono text-sm text-muted-foreground">{m.debug_logs_empty()}</p>
          </div>
        {:else}
          <div
            class="mt-6 max-h-96 overflow-y-auto rounded-sm border border-border bg-background/80 p-2 font-mono"
          >
            {#each logs as line, index (`${index}-${line}`)}
              <div
                class="grid grid-cols-[auto_minmax(0,1fr)] gap-3 border-b border-border/60 px-3 py-2 last:border-b-0"
              >
                <span class="text-xs tracking-wider text-muted-foreground">
                  {formatLineNumber(index)}
                </span>
                <pre
                  class="overflow-x-auto text-sm break-words whitespace-pre-wrap text-foreground">{line}</pre>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </section>
  </div>
</div>
