<script lang="ts">
  import { Plus } from 'lucide-svelte';
  import { TableRow, TableCell, Badge, Button } from '$lib/components';
  import type { EnhancedMonthlyRecord } from '../BudgetState.svelte';

  let { record, isCurrent, monthName, onExtra } = $props<{
    record: EnhancedMonthlyRecord;
    isCurrent: boolean;
    monthName: string;
    onExtra: (year: number, month: number) => void;
  }>();

  // Purely visual logic stays in the component
  const remainingColor = $derived.by(() => {
    if (record.remainingPercentage >= 50) return 'text-emerald-500';
    if (record.remainingPercentage >= 20) return 'text-amber-500';
    return 'text-red-500';
  });
</script>

<TableRow class={isCurrent ? 'border-l-2 border-l-primary bg-primary/5' : ''}>
  <TableCell class="font-bold">
    {monthName}
    {#if isCurrent}<span class="ml-2 text-[10px] tracking-tighter text-primary uppercase"
        >[Active]</span
      >{/if}
  </TableCell>

  <TableCell class="text-right font-mono text-xs">{record.formattedBase}</TableCell>
  <TableCell class="text-right font-mono text-xs text-zinc-500">
    {record.extraBudget > 0 ? record.formattedExtra : '—'}
  </TableCell>

  <TableCell class="text-right font-mono text-xs font-bold">{record.formattedAvailable}</TableCell>
  <TableCell class="text-right font-mono text-xs">{record.formattedSpent}</TableCell>

  <TableCell class="text-right font-mono text-xs font-bold {remainingColor}">
    {record.formattedRemaining}
    <span class="ml-1 text-[9px] opacity-70">({record.remainingPercentage.toFixed(0)}%)</span>
  </TableCell>

  <TableCell>
    <Badge variant={isCurrent ? 'default' : 'outline'} class="text-[10px] uppercase">
      {record.statusLabel}
    </Badge>
  </TableCell>

  <TableCell class="text-right">
    <Button
      variant="ghost"
      size="icon"
      class="h-8 w-8"
      onclick={() => onExtra(record.year, record.month)}
    >
      <Plus size={14} />
    </Button>
  </TableCell>
</TableRow>
