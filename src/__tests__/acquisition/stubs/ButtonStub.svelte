<script lang="ts">
  interface Props {
    type?: 'button' | 'submit' | 'reset';
    onclick?: () => void;
    disabled?: boolean;
    children?: import('svelte').Snippet;
  }

  let { type = 'button', onclick, disabled = false, children }: Props = $props();
</script>

<button {type} {disabled} {onclick}>
  {@render children?.()}
</button>
