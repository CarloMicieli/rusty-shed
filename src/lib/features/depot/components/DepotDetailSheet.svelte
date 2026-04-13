<script lang="ts">
  import { Cpu, Info, Trash2 } from 'lucide-svelte';
  import { Button } from '$lib/components';
  import { Sheet } from '$lib/components/ui/sheet';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    roadNumber: string;
    manufacturer: string;
    productCode: string;
    dccAddress: number | null;
  }

  let { open, onOpenChange, roadNumber, manufacturer, productCode, dccAddress }: Props = $props();
</script>

<Sheet {open} {onOpenChange} class="border-border bg-card/95 backdrop-blur-xl">
  <div class="p-6 text-foreground">
    <div class="mb-6">
      <h3 class="font-bebas text-lg tracking-widest text-primary uppercase">
        {m.depot_detail_system_ops()}
      </h3>
      <p class="text-xs text-muted-foreground">
        {roadNumber} ({manufacturer}
        {productCode})
      </p>
    </div>
    <div class="space-y-8 py-10">
      <div class="rounded-sm border border-border bg-background/50 p-4">
        <h4 class="mb-4 text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
          {m.depot_detail_dcc_controller()}
        </h4>
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-1">
            <span class="text-[10px] font-bold tracking-widest text-muted-foreground uppercase"
              >{m.model_rolling_stock_digital_address()}</span
            >
            <span class="font-mono text-2xl font-bold text-primary">{dccAddress ?? '—'}</span>
          </div>
          <Button class="rounded-sm bg-primary text-primary-foreground hover:bg-primary/90">
            {m.depot_detail_update_addr()}
          </Button>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Button variant="outline" class="rounded-sm border-border hover:bg-background/50">
          <Cpu size={14} class="mr-2" />
          {m.depot_detail_diagnostics()}
        </Button>
        <Button variant="outline" class="rounded-sm border-border hover:bg-background/50">
          <Info size={14} class="mr-2" />
          {m.depot_detail_model_logs()}
        </Button>
      </div>

      <div class="pt-10">
        <Button
          variant="outline"
          class="w-full rounded-sm border-destructive/30 text-destructive hover:bg-destructive/10"
        >
          <Trash2 size={14} class="mr-2" />
          {m.depot_detail_decommission()}
        </Button>
      </div>
    </div>
    <div class="mt-8 flex justify-end gap-3 border-t border-border pt-8">
      <Button variant="ghost" onclick={() => onOpenChange(false)}>{m.common_cancel()}</Button>
    </div>
  </div>
</Sheet>
