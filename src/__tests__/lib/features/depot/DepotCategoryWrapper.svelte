<script lang="ts">
  // @ts-nocheck - test helper wrapper; AccordionRootProps children type mismatch is a known Svelte 5 + bits-ui quirk
  import * as Accordion from '$lib/components/ui/accordion';
  import DepotCategory from '$lib/features/depot/components/DepotCategory.svelte';

  type Props = {
    value: string;
    title: string;

    icon: any;

    items: any[];
    categoryId: string;
  };
  let { value, title, icon, items, categoryId }: Props = $props();
</script>

<Accordion.Root>
  <DepotCategory {value} {title} {icon} {items} {categoryId} />
</Accordion.Root>
