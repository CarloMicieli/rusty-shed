<script lang="ts">
  import { goto } from '$app/navigation';
  import type { Source } from '$lib/bindings';

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
      goto(`/my-collection/${item.id}`);
    } else {
      goto(`/my-wishlists/${item.id}`);
    }
  }
</script>

<button
  onclick={handleClick}
  class="group w-full overflow-hidden card transition-all duration-200 hover:scale-[1.02] hover:ring-2 hover:ring-primary-500 active:scale-[0.98]"
  disabled={!item.source}
>
  <div class="relative aspect-video overflow-hidden">
    {#if item.imageUrl}
      <img
        src={item.imageUrl}
        alt={item.title}
        class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-110"
      />
      <div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div>
    {:else}
      <div
        class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-surface-800 to-surface-700 text-3xl font-bold text-surface-200"
      >
        {item.title.slice(0, 2).toUpperCase()}
      </div>
    {/if}
    <div class="from-background/95 absolute inset-0 bg-gradient-to-t to-transparent"></div>
    <div class="absolute bottom-0 left-0 p-5">
      <h4 class="h3 font-bold text-surface-50 drop-shadow-lg">{item.title}</h4>
      {#if item.subtitle}
        <p class="text-sm tracking-wider text-surface-200 uppercase drop-shadow">{item.subtitle}</p>
      {/if}
    </div>
  </div>
</button>
