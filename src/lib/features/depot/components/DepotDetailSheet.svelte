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

<Sheet {open} {onOpenChange} class="border-white/10 bg-[#0c0c0c]/90 backdrop-blur-xl">
  <div class="p-6 text-white">
    <div class="mb-6">
      <h3 class="font-mono text-sm tracking-widest text-[#f59e0b] uppercase">
        System Operations
      </h3>
      <p class="text-xs text-zinc-500">
        {roadNumber} ({manufacturer} {productCode})
      </p>
    </div>
    <div class="space-y-8 py-10">
      <div class="rounded-lg border border-white/5 bg-white/5 p-4">
        <h4 class="mb-4 text-[10px] font-bold tracking-widest text-zinc-500 uppercase">
          DCC Controller
        </h4>
        <div class="flex items-center justify-between">
          <div class="flex flex-col gap-1">
            <span class="text-xs text-zinc-400">{m.model_rolling_stock_digital_address()}</span>
            <span class="font-mono text-2xl font-bold text-[#f59e0b]">{dccAddress ?? '—'}</span>
          </div>
          <Button class="bg-[#f59e0b] text-black">Update ADDR</Button>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Button variant="outline" class="border-white/10 hover:bg-white/5">
          <Cpu size={14} class="mr-2" />
          Diagnostics
        </Button>
        <Button variant="outline" class="border-white/10 hover:bg-white/5">
          <Info size={14} class="mr-2" />
          Model Logs
        </Button>
      </div>

      <div class="pt-10">
        <Button
          variant="outline"
          class="w-full border-red-500/20 text-red-500 hover:bg-red-500/10"
        >
          <Trash2 size={14} class="mr-2" />
          Decommission from Depot
        </Button>
      </div>
    </div>
    <div class="mt-8 flex justify-end gap-3 border-t border-white/5 pt-8">
      <Button variant="ghost" onclick={() => onOpenChange(false)}>{m.common_cancel()}</Button>
    </div>
  </div>
</Sheet>
