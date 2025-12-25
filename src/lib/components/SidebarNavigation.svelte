<script lang="ts">
	import { LayoutDashboard, Library, Heart, Box, Settings, TrainFront } from 'lucide-svelte';
	import { page } from '$app/stores';
	import { _ } from 'svelte-i18n';
</script>

<nav class="hidden h-full w-64 flex-col border-r border-surface-700/50 bg-surface-900 p-4 lg:flex">
	<div class="mb-8 flex items-center gap-3 px-4">
		<TrainFront class="text-accent-500" size={32} />
		<h2 class="h3 font-bold tracking-tight uppercase">{$_('app.name')}</h2>
	</div>

	<ul class="space-y-2">
		<li>
			<a
				href="/"
				class="hover:variant-soft-primary btn w-full justify-start gap-3"
				class:variant-filled-primary={($page.url.pathname as string) === '/'}
				class:variant-ghost-surface={($page.url.pathname as string) !== '/'}
			>
				<LayoutDashboard size={20} />
				<span class="font-medium tracking-wide">{$_('app.dashboard')}</span>
			</a>
		</li>
		<li>
			<a
				href="/collection"
				class="hover:variant-soft-primary btn w-full justify-start gap-3"
				class:variant-filled-primary={($page.url.pathname as string) === '/collection'}
				class:variant-ghost-surface={($page.url.pathname as string) !== '/collection'}
			>
				<Library size={20} />
				<span class="font-medium tracking-wide">{$_('app.collection')}</span>
			</a>
		</li>
		<li>
			<a
				href="/wishlists"
				class="hover:variant-soft-primary btn w-full justify-start gap-3"
				class:variant-filled-primary={($page.url.pathname as string) === '/wishlists'}
				class:variant-ghost-surface={($page.url.pathname as string) !== '/wishlists'}
			>
				<Heart size={20} />
				<span class="font-medium tracking-wide">{$_('app.wishlists')}</span>
			</a>
		</li>
		<li>
			<a
				href="/depot"
				class="hover:variant-soft-primary btn w-full justify-start gap-3"
				class:variant-filled-primary={($page.url.pathname as string) === '/depot'}
				class:variant-ghost-surface={($page.url.pathname as string) !== '/depot'}
			>
				<Box size={20} />
				<span class="font-medium tracking-wide">{$_('app.depot')}</span>
			</a>
		</li>
	</ul>

	<div class="mt-auto space-y-2 border-t border-surface-700/50 pt-4">
		<a
			href="/settings"
			class="hover:variant-soft-primary variant-ghost-surface btn w-full justify-start gap-3"
			class:variant-filled-primary={($page.url.pathname as string) === '/settings'}
		>
			<Settings size={20} />
			<span class="font-medium tracking-wide">{$_('app.settings')}</span>
		</a>
		<div class="px-4 py-2 text-center text-xs tracking-widest text-surface-400 uppercase">
			v1.0.0
		</div>
	</div>
</nav>
