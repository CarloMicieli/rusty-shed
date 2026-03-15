<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { Button } from '$lib/components';
  import * as m from '$lib/paraglide/messages.js';

  let draggingOver = $state(false);

  interface Props {
    onFileSelected?: (path: string) => Promise<void>;
    disabled?: boolean;
  }

  const { onFileSelected = async () => {}, disabled = false }: Props = $props();

  // Use Tauri's native drag-drop event to get absolute file paths.
  // DOM DataTransfer File objects do not have reliable .path properties.
  $effect(() => {
    let cleanup = () => {};
    let active = true;

    getCurrentWebviewWindow()
      .onDragDropEvent((event) => {
        if (!active || disabled || event.payload.type !== 'drop') return;
        const paths = (event.payload as { type: 'drop'; paths: string[] }).paths;
        if (paths.length > 0) {
          onFileSelected(paths[0]);
        }
      })
      .then((unlisten) => {
        if (active) cleanup = unlisten;
        else unlisten();
      });

    return () => {
      active = false;
      cleanup();
    };
  });

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

  const handleDrop = (e: DragEvent) => {
    // Prevent browser navigation. Path extraction is handled by onDragDrop above.
    e.preventDefault();
    e.stopPropagation();
    draggingOver = false;
  };

  const handleClick = async () => {
    if (disabled) return;
    const path = await open({
      multiple: false,
      filters: [{ name: 'Archive', extensions: ['zip', 'tar.gz', 'tgz'] }]
    });
    if (path) {
      await onFileSelected(path as string);
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (disabled) return;
    // Trigger on Enter or Space key
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleClick();
    }
  };
</script>

<div
  class="import-drop-zone"
  class:dragging={draggingOver}
  class:disabled
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  onclick={handleClick}
  onkeydown={handleKeyDown}
  role="button"
  tabindex={disabled ? -1 : 0}
>
  <div class="content">
    <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
      <polyline points="17 8 12 3 7 8" />
      <line x1="12" y1="3" x2="12" y2="15" />
    </svg>
    <h3>{m['import.dropzone.title']()}</h3>
    <p>{m['import.dropzone.subtitle']()}</p>
    <Button onclick={handleClick} type="button" {disabled} class="pointer-events-auto">
      {m['import.dropzone.select']()}
    </Button>
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
</style>
