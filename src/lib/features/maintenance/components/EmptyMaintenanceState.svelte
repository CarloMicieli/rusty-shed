<script lang="ts">
  import { Wrench } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';

  let { onCreate } = $props<{ onCreate: () => void }>();
</script>

<div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
  <div
    class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 px-4 py-24 text-center"
  >
    <!-- Thematic Iconography -->
    <div class="relative">
      <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
      <div
        class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
      >
        <Wrench size={56} class="text-zinc-600 opacity-50" />
      </div>
    </div>

    <!-- Branded Call-to-Action -->
    <div class="flex max-w-sm flex-col items-center gap-3 text-center">
      <h3 class="text-2xl font-bold text-zinc-200">
        {m.maintenance_empty_title()}
      </h3>
      <p class="text-sm leading-relaxed text-zinc-500">
        {m.maintenance_empty_message()}
      </p>
    </div>

    <!-- Central Action Button -->
    <button
      type="button"
      class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
      onclick={onCreate}
    >
      <div
        class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
      ></div>
      <Wrench class="h-5 w-5" />
      <span>{m.maintenance_empty_action()}</span>
    </button>
  </div>
</div>
