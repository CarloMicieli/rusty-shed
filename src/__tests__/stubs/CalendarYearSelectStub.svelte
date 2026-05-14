<script lang="ts">
  let { value }: { value?: number } = $props();
</script>

<div data-testid="calendar-year-select">year-select:{value ?? 'none'}</div>