<script lang="ts">
  import type { CollectionItemView, SellerView } from '$lib/bindings';
  import CollectionItemDetails from './CollectionItemDetails.svelte';
  import CollectionItemConditions from './CollectionItemConditions.svelte';

  interface Props {
    item: CollectionItemView;
    seller: SellerView | null;
    onItemUpdated?: () => Promise<void>;
  }

  let { item, seller, onItemUpdated }: Props = $props();
</script>

<aside class="w-full shrink-0 space-y-4 lg:w-80">
  <CollectionItemDetails {item} {seller} {onItemUpdated} />
  <CollectionItemConditions {item} {onItemUpdated} />
</aside>
