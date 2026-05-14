<script lang="ts">
  let { value }: { value?: number } = $props();
</script>

<div data-testid="calendar-month-select">month-select:{value ?? 'none'}</div>