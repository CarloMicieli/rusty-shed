<script lang="ts">
	import './layout.css';
	import '$lib/i18n'; // Initialize i18n
	import { isLoading, _ } from 'svelte-i18n';
	import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
	import BottomNavigation from '$lib/components/BottomNavigation.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { Bell, TrainFront } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { setAppVersion } from '$lib/stores/app';

	let { children } = $props();

	onMount(async () => {
		// Prefer generated bindings if available (tauri-specta). Fallback to direct invoke.
		try {
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore: generated bindings may not exist during Vite-only dev
			const bindings = await import('$lib/bindings');
			if (bindings && bindings.commands && typeof bindings.commands.getAppVersion === 'function') {
				const v = await bindings.commands.getAppVersion();
				setAppVersion(v as string);
				return;
			}
		} catch (e) {
			// ignore and fallback
		}

		try {
			// Dynamically import the Tauri API in a way that avoids Vite's static
			// import analysis (which fails when running Vite-only dev). Using
			// `new Function` prevents Vite from seeing the string at build time.
			const tauriModule = await new Function("return import('@tauri-apps/api/tauri')")().catch(
				() => null
			);
			if (tauriModule && typeof tauriModule.invoke === 'function') {
				const v = await tauriModule.invoke<string>('get_app_version');
				setAppVersion(v);
			}
		} catch (e) {
			// Not running under Tauri or other error — leave default/fallback version
			// Optionally set a dev fallback like 'dev'
		}
	});
</script>

<!-- Immediate render to prevent blank screen -->
{#if $isLoading}
	<div
		class="bg-background flex h-screen w-full items-center justify-center text-surface-50"
		style="font-family: system-ui, -apple-system, sans-serif;"
	>
		<div class="flex flex-col items-center gap-4">
			<div class="h-8 w-8 animate-spin rounded-full border-4 border-primary-500 border-t-transparent"></div>
			<p class="text-sm opacity-70">Loading application...</p>
		</div>
	</div>
{:else}
	<div
		class="bg-background flex h-screen w-full flex-col overflow-hidden font-sans text-surface-50 selection:bg-primary-500/30 lg:flex-row"
	>
		<!-- Sidebar Left (Desktop) -->
		<div class="hidden h-full w-0 shrink-0 transition-all duration-300 lg:block lg:w-64">
			<SidebarNavigation />
		</div>

		<!-- Main Content Wrapper -->
		<div class="relative flex h-full min-w-0 flex-1 flex-col">
			<!-- Header -->
			<header
				class="bg-background/80 sticky top-0 z-40 border-b border-surface-700/50 backdrop-blur-md"
				data-tauri-drag-region
			>
				<div class="flex items-center justify-between p-4">
					<!-- Mobile: Logo / Brand (Visible only when sidebar is hidden) -->
					<div class="flex items-center gap-2 lg:hidden">
						<TrainFront class="text-accent-500" size={24} />
						<span class="text-sm font-bold tracking-widest uppercase">Rusty Shed</span>
					</div>

					<!-- Desktop: Spacer (Sidebar handles branding) -->
					<div class="hidden lg:block">
						<h2 class="h3 font-bold">Dashboard</h2>
					</div>

					<!-- Right Actions -->
					<div class="flex items-center gap-4">
						<SearchBar />

						<button class="variant-ghost-surface relative btn-icon btn-icon-sm">
							<Bell size={20} />
							<span class="absolute top-0 right-0 h-2 w-2 animate-pulse rounded-full bg-error-500"
							></span>
						</button>
					</div>
				</div>
			</header>

			<!-- Page Content -->
			<main
				class="mx-auto w-full max-w-[1600px] flex-1 space-y-8 overflow-y-auto p-4 pb-24 lg:p-8 lg:pb-8"
			>
				{@render children()}
			</main>

			<!-- Footer / Bottom Nav -->
			<BottomNavigation />
		</div>
	</div>
{/if}
