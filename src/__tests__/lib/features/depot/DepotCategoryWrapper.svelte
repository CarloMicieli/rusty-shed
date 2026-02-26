<script lang="ts">
  // @ts-nocheck - test helper wrapper; AccordionRootProps children type mismatch is a known Svelte 5 + bits-ui quirk
  import * as Accordion from '$lib/components/ui/accordion';
  import DepotCategory from '$lib/features/depot/components/DepotCategory.svelte';

  let { value, title, icon, items, categoryId } = $props<{
    value: string;
    title: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon: any;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    items: any[];
    categoryId: string;
  }>();
</script>

<Accordion.Root>
  <DepotCategory {value} {title} {icon} {items} {categoryId} />
</Accordion.Root>
