<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { X, Settings2 } from 'lucide-svelte';

  interface Props {
    onClose: () => void;
    disabled?: boolean;
  }

  let { onClose, disabled = false }: Props = $props();
</script>

<div class="flex items-center justify-between border-b border-white/5 bg-zinc-950/30 p-6">
  <div class="flex items-center gap-3">
    <div
      class="flex h-10 w-10 items-center justify-center rounded-xl border border-white/5 bg-zinc-900 text-zinc-400"
    >
      <Settings2 size={20} />
    </div>
    <h2
      id="create-product-title"
      class="text-xl font-bold tracking-tight text-zinc-100 uppercase"
    >
      {m.track_product_dialog_title()}
    </h2>
  </div>
  <button
    onclick={onClose}
    class="rounded-lg p-2 text-zinc-500 transition-colors hover:bg-white/5 hover:text-white"
    disabled={disabled}
  >
    <X size={20} />
  </button>
</div>
