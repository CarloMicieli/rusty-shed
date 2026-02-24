<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import type { Source } from '$lib/bindings';
  import * as m from '$lib/paraglide/messages.js';

  type RecentItem = {
    id: string;
    title: string;
    subtitle?: string | null;
    imageUrl?: string | null;
    source?: Source;
  };

  let { item } = $props<{ item: RecentItem }>();

  function handleClick() {
    if (!item.source) return;

    if (item.source === 'Collection') {
      goto(resolve('/collection'));
    } else {
      goto(resolve('/wishlists'));
    }
  }
</script>

<button
  onclick={handleClick}
  class="group card hover:ring-primary-500 w-full overflow-hidden transition-all duration-200 hover:scale-[1.02] hover:ring-2 active:scale-[0.98]"
  disabled={!item.source}
>
  <div class="relative aspect-video overflow-hidden">
    {#if item.imageUrl}
      <img
        src={item.imageUrl}
        alt={item.title}
        class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-110"
      />
      <div class="absolute inset-0 bg-gradient-to-t from-background/90 to-transparent"></div>
    {:else}
      <div class="absolute inset-0 p-4">
        <div
          class="blueprint-panel text-surface-200 flex h-full w-full flex-col items-center justify-center gap-2 rounded-lg text-center"
        >
          <span class="text-xs font-semibold tracking-[0.3em] uppercase">
            {m.dashboard_blueprint_label()}
          </span>
          <span class="text-lg font-bold">{item.title}</span>
        </div>
      </div>
    {/if}
    <div class="absolute inset-0 bg-gradient-to-t from-background/95 to-transparent"></div>
    <div class="absolute bottom-0 left-0 p-5">
      <h4 class="h3 text-surface-50 font-bold drop-shadow-lg">{item.title}</h4>
      {#if item.subtitle}
        <p class="text-surface-200 text-sm tracking-wider uppercase drop-shadow">{item.subtitle}</p>
      {/if}
    </div>
  </div>
</button>
