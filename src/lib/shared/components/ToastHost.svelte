<script lang="ts">
  import { Toast } from '@skeletonlabs/skeleton-svelte';
  import { toaster } from '$lib/toaster';
</script>

<Toast.Group {toaster}>
  {#snippet children(toast)}
    <Toast {toast}>
      <Toast.Message>
        {#if toast.title}
          <Toast.Title>{toast.title}</Toast.Title>
        {/if}
        {#if toast.description}
          <Toast.Description>{toast.description}</Toast.Description>
        {/if}
      </Toast.Message>
      {#if toast.action}
        <Toast.ActionTrigger />
      {/if}
      <Toast.CloseTrigger />
    </Toast>
  {/snippet}
</Toast.Group>
