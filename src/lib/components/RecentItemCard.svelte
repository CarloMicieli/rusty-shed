<script lang="ts">
	import type { RecentItem } from '$lib/data/mock';

	let { item } = $props<{ item: RecentItem }>();
</script>

<div
	class="group overflow-hidden card transition-all duration-200 hover:ring-1 hover:ring-primary-500"
>
	<div class="relative aspect-video overflow-hidden">
		<img
			src={item.imageUrl}
			alt={item.title}
			class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
		/>
		<div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div>
		<div class="absolute bottom-0 left-0 p-4">
			<h4 class="h4 font-bold text-surface-50">{item.title}</h4>
			<p class="text-xs tracking-wider text-surface-300 uppercase">{item.subtitle}</p>
		</div>
	</div>
</div>
