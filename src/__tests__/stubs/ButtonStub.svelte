<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    type?: 'button' | 'submit' | 'reset';
    disabled?: boolean;
    class?: string;
    onclick?: () => void;
    children?: Snippet;
  }

  let {
    type = 'button',
    disabled = false,
    class: className = '',
    onclick,
    children
  }: Props = $props();
</script>

<button {type} {disabled} class={className} {onclick}>
  {@render children?.()}
</button>
