<script lang="ts">
  import { X, ClipboardList } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();
</script>

<!-- Header -->
<div class="flex items-center justify-between border-b border-white/5 p-6">
  <div class="flex items-center gap-3">
    <div class="rounded-lg bg-amber-500/10 p-2 text-amber-500">
      <ClipboardList size={20} />
    </div>
    <h3 class="text-lg font-bold text-zinc-100">{m.maintenance_add_event_title()}</h3>
  </div>
  <button
    type="button"
    class="text-zinc-500 transition-colors hover:text-white"
    onclick={onClose}
    aria-label={m.dialog_close_button()}
  >
    <X size={20} />
  </button>
</div>
