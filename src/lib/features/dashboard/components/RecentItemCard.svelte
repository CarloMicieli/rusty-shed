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
  class="group card hover:ring-primary-500 overflow-hidden transition-all duration-200 hover:ring-1"
>
  <div class="relative aspect-video overflow-hidden">
    {#if item.imageUrl}
      <img
        src={item.imageUrl}
        alt={item.title}
        class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
      />
      <div class="absolute inset-0 bg-gradient-to-t from-background/90 to-transparent"></div>
    {:else}
      <div
        class="from-surface-800 to-surface-700 text-surface-200 absolute inset-0 flex items-center justify-center bg-gradient-to-br text-2xl font-semibold"
      >
        {item.title.slice(0, 2).toUpperCase()}
      </div>
    {/if}
    <div class="absolute inset-0 bg-gradient-to-t from-background/90 to-transparent"></div>
    <div class="absolute bottom-0 left-0 p-4">
      <h4 class="h4 text-surface-50 font-bold">{item.title}</h4>
      {#if item.subtitle}
        <p class="text-surface-300 text-xs tracking-wider uppercase">{item.subtitle}</p>
      {/if}
    </div>
  </div>
</div>
