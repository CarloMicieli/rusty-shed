<script lang="ts">
  type RecentItem = {
    id: string;
    title: string;
    subtitle?: string | null;
    imageUrl?: string | null;
  };

  let { item } = $props<{ item: RecentItem }>();
</script>

<div
  class="group overflow-hidden card transition-all duration-200 hover:ring-1 hover:ring-primary-500"
>
  <div class="relative aspect-video overflow-hidden">
    {#if item.imageUrl}
      <img
        src={item.imageUrl}
        alt={item.title}
        class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
      />
      <div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div>
    {:else}
      <div
        class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-surface-800 to-surface-700 text-2xl font-semibold text-surface-200"
      >
        {item.title.slice(0, 2).toUpperCase()}
      </div>
    {/if}
    <div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div>
    <div class="absolute bottom-0 left-0 p-4">
      <h4 class="h4 font-bold text-surface-50">{item.title}</h4>
      {#if item.subtitle}
        <p class="text-xs tracking-wider text-surface-300 uppercase">{item.subtitle}</p>
      {/if}
    </div>
  </div>
</div>
