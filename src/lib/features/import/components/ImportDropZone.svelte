<script lang="ts">
	let draggingOver = $state(false);
	import { translate } = $lib.i18n;

	interface Props {
		onFilesSelected?: (files: FileList) => Promise<void>;
		accepting?: string;
		disabled?: boolean;
	}

	const {
		onFilesSelected = async () => {},
		accept = '.zip,.tar.gz,.tgz',
		disabled = false
	}: Props = $props();

	let fileInput: HTMLInputElement | undefined = $state();

	const handleDragOver = (e: DragEvent) => {
		if (disabled) return;
		e.preventDefault();
		e.stopPropagation();
		draggingOver = true;
	};

	const handleDragLeave = (e: DragEvent) => {
		e.preventDefault();
		e.stopPropagation();
		draggingOver = false;
	};

	const handleDrop = async (e: DragEvent) => {
		if (disabled) return;
		e.preventDefault();
		e.stopPropagation();
		draggingOver = false;

		const files = e.dataTransfer?.files;
		if (files && files.length > 0) {
			await onFilesSelected(files);
		}
	};

	const handleFileSelect = async (e: Event) => {
		const input = e.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			await onFilesSelected(input.files);
		}
	};

	const handleClick = () => {
		if (!disabled && fileInput) {
			fileInput.click();
		}
	};
</script>

<div
	class="import-drop-zone"
	class:dragging={draggingOver}
	class:disabled={disabled}
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
	onclick={handleClick}
	role="button"
	tabindex={disabled ? -1 : 0}
>
	<input
		bind:this={fileInput}
		type="file"
		{accept}
		style="display: none"
		onchange={handleFileSelect}
		disabled={disabled}
	/>

	<div class="content">
		<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
			<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
			<polyline points="17 8 12 3 7 8" />
			<line x1="12" y1="3" x2="12" y2="15" />
		</svg>
		<h3>{translate('import.dropzone.title')}</h3>
		<p>{translate('import.dropzone.subtitle')}</p>
		<button class="select-btn" onclick={handleClick} type="button" disabled={disabled}>
			{translate('import.dropzone.select')}
		</button>
	</div>
</div>

<style>
	.import-drop-zone {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		border: 2px dashed hsl(var(--border));
		border-radius: var(--radius-lg);
		background-color: hsl(var(--muted));
		cursor: pointer;
		transition: all 200ms ease-in-out;
		position: relative;
	}

	.import-drop-zone:hover:not(.disabled) {
		border-color: hsl(var(--primary));
		background-color: hsl(var(--primary) / 0.05);
	}

	.import-drop-zone.dragging {
		border-color: hsl(var(--primary));
		background-color: hsl(var(--primary) / 0.1);
	}

	.import-drop-zone.disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}

	.content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		text-align: center;
		pointer-events: none;
	}

	.icon {
		width: 3rem;
		height: 3rem;
		color: hsl(var(--muted-foreground));
		flex-shrink: 0;
	}

	h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	p {
		margin: 0;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
		max-width: 300px;
	}

	.select-btn {
		padding: 0.5rem 1.5rem;
		border-radius: var(--radius-md);
		background-color: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
		border: none;
		cursor: pointer;
		font-weight: 500;
		transition: background-color 200ms ease-in-out;
		pointer-events: auto;
	}

	.select-btn:hover:not(:disabled) {
		background-color: hsl(var(--primary) / 0.9);
	}

	.select-btn:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}
</style>
