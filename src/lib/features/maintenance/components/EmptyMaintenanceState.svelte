<script lang="ts">
  import { Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let { onCreate } = $props<{ onCreate: () => void }>();
</script>

<div class="flex flex-col items-center justify-center px-4 py-24 text-center">
  <!-- Thematic Iconography -->
  <div class="relative mb-8">
    <div class="absolute inset-0 rounded-full bg-amber-500/20 blur-3xl"></div>
    <div
      class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-white/5 shadow-2xl backdrop-blur-sm"
    >
      <Wrench class="h-16 w-16 stroke-1 text-zinc-600/80" />
    </div>
  </div>

  <!-- Branded Call-to-Action -->
  <div class="mb-10 max-w-md space-y-3">
    <h3 class="mb-2 text-2xl font-bold tracking-tight text-white">
      {m.maintenance_empty_title()}
    </h3>
    <p class="leading-relaxed text-zinc-400">
      {m.maintenance_empty_message()}
    </p>
  </div>

  <!-- Central Action Button -->
  <button
    type="button"
    class="group relative inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
    onclick={onCreate}
  >
    <div
      class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
    ></div>
    <Wrench class="h-5 w-5" />
    <span>{m.maintenance_empty_action()}</span>
  </button>
</div>
