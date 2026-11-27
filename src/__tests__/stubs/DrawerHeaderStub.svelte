<script lang="ts">
  interface Props {
    title: string;
    subtitle?: string;
    onClose?: () => void;
  }

  let { title, subtitle = '', onClose }: Props = $props();
</script>

<div data-testid="drawer-header-stub">
  <h2>{title}</h2>
  <p>{subtitle}</p>
  <button type="button" data-testid="drawer-header-close" onclick={onClose}>close</button>
</div>
