<script lang="ts">
  interface Props {
    id: string;
    title: string;
    onClose: () => void;
  }

  let { id, title, onClose }: Props = $props();
</script>

<div>
  <h2 {id}>{title}</h2>
  <button type="button" onclick={onClose}>header-close</button>
</div>
