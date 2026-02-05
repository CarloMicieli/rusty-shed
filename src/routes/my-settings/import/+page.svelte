<script lang="ts">
	import { PageHeader } from '$lib/components';
	import ImportDropZone from '$lib/features/import/components/ImportDropZone.svelte';
	import { createImportController } from '$lib/features/import/import.controller.svelte';
	import * as m from '$lib/paraglide/messages.js';

	const controller = createImportController();

	let selectedFile: File | null = $state(null);
	let isProcessing = $state(false);

	async function handleFilesSelected(files: FileList) {
		if (files.length === 0) return;

		selectedFile = files[0];
		isProcessing = true;

		try {
			const filePath = (selectedFile as any).path || selectedFile.name;
			await controller.analyzePackage(filePath);
		} catch (error) {
			console.error('Error analyzing package:', error);
		} finally {
			isProcessing = false;
		}
	}

	async function handleExecuteImport() {
		isProcessing = true;
		try {
			await controller.executeImport();
		} catch (error) {
			console.error('Error executing import:', error);
		} finally {
			isProcessing = false;
		}
	}

	async function handleCancel() {
		await controller.cancelSession();
		selectedFile = null;
	}
</script>

<PageHeader title={m.import_title()} subtitle="Import your railway collection data from .zip or .tar.gz archives" />

<div class="import-container">
	{#if !$controller.sessionId$}
		<div class="dropzone-section">
			<ImportDropZone
				{isProcessing}
				onFilesSelected={handleFilesSelected}
				disabled={isProcessing}
			/>
		</div>
	{:else}
		<div class="preview-section">
			<div class="status-card">
				<h3>{m.import_preview_title()}</h3>
				
				{#if $controller.recordCounts$}
					<div class="record-counts">
						<div class="count-item">
							<span class="label">Manufacturers</span>
							<span class="value">{$controller.recordCounts$.manufacturers ?? 0}</span>
						</div>
						<div class="count-item">
							<span class="label">Railway Models</span>
							<span class="value">{$controller.recordCounts$.railwayModels ?? 0}</span>
						</div>
						<div class="count-item">
							<span class="label">Collection Items</span>
							<span class="value">{$controller.recordCounts$.collectionItems ?? 0}</span>
						</div>
					</div>
				{/if}

				{#if $controller.errors$ && $controller.errors$.length > 0}
					<div class="errors">
						<h4>Validation Errors</h4>
						<ul>
							{#each $controller.errors$ as error}
								<li>{error.message || 'Unknown error'}</li>
							{/each}
						</ul>
					</div>
				{/if}

				{#if $controller.warnings$ && $controller.warnings$.length > 0}
					<div class="warnings">
						<h4>Warnings</h4>
						<ul>
							{#each $controller.warnings$ as warning}
								<li>{warning.message || 'Unknown warning'}</li>
							{/each}
						</ul>
					</div>
				{/if}

				<div class="actions">
					<button
						class="btn btn-primary"
						disabled={!$controller.canImport$ || isProcessing}
						onclick={handleExecuteImport}
					>
						{#if isProcessing}
							{m.import_progress_importing()}
						{:else}
							{m.import_confirm_button()}
						{/if}
					</button>
					<button class="btn btn-secondary" onclick={handleCancel} disabled={isProcessing}>
						{m.import_cancel_button()}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.import-container {
		max-width: 1200px;
		margin: 2rem auto;
		padding: 1rem;
	}

	.dropzone-section {
		width: 100%;
	}

	.preview-section {
		width: 100%;
	}

	.status-card {
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: var(--radius-lg);
		padding: 2rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.status-card h3 {
		margin-top: 0;
		font-size: 1.5rem;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.record-counts {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin: 1.5rem 0;
	}

	.count-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1rem;
		background: hsl(var(--muted));
		border-radius: var(--radius-md);
		text-align: center;
	}

	.count-item .label {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
		margin-bottom: 0.5rem;
	}

	.count-item .value {
		font-size: 1.5rem;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.errors {
		margin: 1.5rem 0;
		padding: 1rem;
		background: hsl(var(--destructive) / 0.1);
		border-left: 4px solid hsl(var(--destructive));
		border-radius: var(--radius-md);
	}

	.errors h4 {
		margin: 0 0 0.5rem 0;
		color: hsl(var(--destructive));
		font-weight: 600;
	}

	.errors ul {
		margin: 0;
		padding-left: 1.5rem;
		color: hsl(var(--muted-foreground));
	}

	.errors li {
		margin-bottom: 0.25rem;
	}

	.warnings {
		margin: 1.5rem 0;
		padding: 1rem;
		background: hsl(var(--yellow) / 0.1);
		border-left: 4px solid hsl(var(--yellow));
		border-radius: var(--radius-md);
	}

	.warnings h4 {
		margin: 0 0 0.5rem 0;
		color: hsl(var(--yellow));
		font-weight: 600;
	}

	.warnings ul {
		margin: 0;
		padding-left: 1.5rem;
		color: hsl(var(--muted-foreground));
	}

	.warnings li {
		margin-bottom: 0.25rem;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 1.5rem;
		justify-content: flex-end;
	}

	.btn {
		padding: 0.5rem 1.5rem;
		border-radius: var(--radius-md);
		border: none;
		cursor: pointer;
		font-weight: 500;
		transition: all 200ms ease-in-out;
	}

	.btn-primary {
		background-color: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
	}

	.btn-primary:hover:not(:disabled) {
		background-color: hsl(var(--primary) / 0.9);
	}

	.btn-secondary {
		background-color: hsl(var(--secondary));
		color: hsl(var(--secondary-foreground));
	}

	.btn-secondary:hover:not(:disabled) {
		background-color: hsl(var(--secondary) / 0.9);
	}

	.btn:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}
</style>
