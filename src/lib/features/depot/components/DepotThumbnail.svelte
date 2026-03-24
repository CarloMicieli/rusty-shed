<script lang="ts">
  import { TrainFront } from 'lucide-svelte';
  import { readFile, BaseDirectory } from '@tauri-apps/plugin-fs';

  let { railwayModelId, productCode }: { railwayModelId: string; productCode: string } = $props();

  let src = $state<string | null>(null);

  const EXTENSIONS = ['jpg', 'png', 'jpeg'] as const;

  $effect(() => {
    const baseFilename = railwayModelId.replaceAll(':', '_');
    let stale = false;

    async function tryLoad() {
      const mimes: Record<string, string> = {
        jpg: 'image/jpeg',
        jpeg: 'image/jpeg',
        png: 'image/png'
      };

      for (const ext of EXTENSIONS) {
        if (stale) return;
        try {
          const bytes = await readFile(`models/${baseFilename}.${ext}`, {
            baseDir: BaseDirectory.AppLocalData
          });
          if (stale) return;
          const prev = src;
          src = URL.createObjectURL(new Blob([bytes], { type: mimes[ext] }));
          if (prev) URL.revokeObjectURL(prev);
          return;
        } catch {
          // Try next extension
        }
      }

      if (!stale) src = null;
    }

    void tryLoad();

    return () => {
      stale = true;
    };
  });
</script>

<div
  class="flex h-10 w-16 items-center justify-center overflow-hidden rounded border border-white/10 bg-black"
>
  {#if src}
    <img
      {src}
      alt={productCode}
      class="h-full w-full object-cover contrast-125 grayscale transition-all group-hover:contrast-100 group-hover:grayscale-0"
    />
  {:else}
    <TrainFront size={16} class="text-zinc-800" />
  {/if}
</div>
