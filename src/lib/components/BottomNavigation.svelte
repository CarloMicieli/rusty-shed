<script lang="ts">
	import { LayoutDashboard, Library, Heart, Box, Settings } from 'lucide-svelte';
	import { page } from '$app/stores';
	import { _ } from 'svelte-i18n';
</script>

<div
	class="pb-safe-area fixed right-0 bottom-0 left-0 z-50 border-t border-surface-700/50 bg-surface-900 lg:hidden"
>
	<div class="flex h-16 items-center justify-around">
		<a
			href="/"
			class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
			class:text-accent-500={($page.url.pathname as string) === '/'}
			class:text-surface-400={($page.url.pathname as string) !== '/'}
		>
			<LayoutDashboard size={20} />
			<span class="text-[10px] font-bold tracking-wider uppercase">{$_('app.dashboard')}</span>
		</a>
		<a
			href="/collection"
			class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
			class:text-accent-500={($page.url.pathname as string) === '/collection'}
			class:text-surface-400={($page.url.pathname as string) !== '/collection'}
		>
			<Library size={20} />
			<span class="text-[10px] font-bold tracking-wider uppercase">{$_('app.collection')}</span>
		</a>
		<a
			href="/wishlists"
			class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
			class:text-accent-500={($page.url.pathname as string) === '/wishlists'}
			class:text-surface-400={($page.url.pathname as string) !== '/wishlists'}
		>
			<Heart size={20} />
			<span class="text-[10px] font-bold tracking-wider uppercase">{$_('app.wishlists')}</span>
		</a>
		<a
			href="/depot"
			class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
			class:text-accent-500={($page.url.pathname as string) === '/depot'}
			class:text-surface-400={($page.url.pathname as string) !== '/depot'}
		>
			<Box size={20} />
			<span class="text-[10px] font-bold tracking-wider uppercase">{$_('app.depot')}</span>
		</a>
		<a
			href="/settings"
			class="flex h-full w-full flex-col items-center justify-center gap-1 transition-transform active:scale-95"
			class:text-accent-500={($page.url.pathname as string) === '/settings'}
			class:text-surface-400={($page.url.pathname as string) !== '/settings'}
		>
			<Settings size={20} />
			<span class="text-[10px] font-bold tracking-wider uppercase">{$_('app.settings')}</span>
		</a>
	</div>
</div>
