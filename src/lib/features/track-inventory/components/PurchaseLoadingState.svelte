<script lang="ts">
  import { Loader2 } from 'lucide-svelte';
</script>

<div class="flex flex-col items-center justify-center gap-4 py-20">
  <Loader2 size={32} class="animate-spin text-amber-500" />
  <p class="text-sm font-bold tracking-widest text-zinc-500 uppercase">Indexing Ledger...</p>
</div>
