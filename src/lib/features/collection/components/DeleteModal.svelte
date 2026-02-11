<script lang="ts">
  import { Dialog as DialogPrimitive } from 'bits-ui';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { AlertTriangle } from 'lucide-svelte';

  const {
    open = false,
    title = 'Delete item',
    message = 'Are you sure?',
    onClose,
    onConfirm
  } = $props<{
    open?: boolean;
    title?: string;
    message?: string;
    onClose?: () => void;
    onConfirm?: () => void;
  }>();

  function handleOpenChange(newOpen: boolean) {
    // Only call onClose when the dialog is being closed
    if (!newOpen) {
      onClose?.();
    }
  }

  function handleClose() {
    onClose?.();
  }

  function handleConfirm() {
    onConfirm?.();
  }
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
  <Dialog.Portal>
    <!-- Custom overlay with backdrop blur -->
    <Dialog.Overlay class="bg-black/80 backdrop-blur-sm" />

    <!-- Dialog content -->
    <DialogPrimitive.Content
      class="fixed top-[50%] left-[50%] z-50 grid w-full max-w-md translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border border-border bg-card p-6 shadow-xl duration-200 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95"
    >
      <Dialog.Header class="gap-3">
        <div class="flex items-center gap-3">
          <div class="flex h-10 w-10 items-center justify-center rounded-full bg-destructive/20">
            <AlertTriangle class="size-5 text-destructive" />
          </div>
          <Dialog.Title class="text-lg font-bold text-white">
            {title}
          </Dialog.Title>
        </div>
      </Dialog.Header>

      <Dialog.Description class="text-sm text-muted-foreground">
        {message}
      </Dialog.Description>

      <Dialog.Footer class="mt-2 flex justify-end gap-3">
        <Button variant="ghost" onclick={handleClose}>Cancel</Button>
        <Button variant="destructive" onclick={handleConfirm}>Confirm</Button>
      </Dialog.Footer>
    </DialogPrimitive.Content>
  </Dialog.Portal>
</Dialog.Root>
