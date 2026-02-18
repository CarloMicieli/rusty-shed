<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { TrainTrack } from 'lucide-svelte';
  import { Button } from '$lib/components';

  interface Props {
    onCreateClick?: () => void;
  }

  const { onCreateClick }: Props = $props();
</script>

<div
  class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-[#0c0c0c]/50 py-24"
>
  <div class="relative">
    <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
    <div
      class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
    >
      <TrainTrack size={56} class="text-zinc-600 opacity-50" />
    </div>
  </div>

  <div class="flex max-w-sm flex-col items-center gap-3 text-center">
    <h3 class="text-2xl font-bold text-zinc-200">{m.track_inventories_empty_title()}</h3>
    <p class="text-sm leading-relaxed text-zinc-500">{m.track_inventories_empty_message()}</p>
  </div>

  {#if onCreateClick}
    <Button
      variant="secondary"
      onclick={onCreateClick}
      class="mt-2 rounded-xl border border-white/10 px-8 py-6 font-bold transition-all hover:bg-zinc-800"
    >
      {m.track_inventories_create_button()}
    </Button>
  {/if}
</div>
