<script lang="ts">
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
    CardFooter
  } from '$lib/components/ui/card';

  let { includeFooter = true }: { includeFooter?: boolean } = $props();
</script>

<Card class="custom-card">
  <CardHeader class="custom-header">
    <CardTitle class="custom-title">Plan</CardTitle>
    <CardDescription class="custom-description">Current sprint</CardDescription>
  </CardHeader>
  <CardContent class="custom-content">
    <p>Body content</p>
  </CardContent>
  {#if includeFooter}
    <CardFooter class="custom-footer">
      <button type="button">Save</button>
    </CardFooter>
  {/if}
</Card>
