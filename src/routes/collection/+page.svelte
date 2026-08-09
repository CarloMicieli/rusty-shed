<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { getPageTitleContext } from '$lib/state/page-title.svelte';
  import CollectionDashboard from '$lib/features/collection/CollectionDashboard.svelte';

  const pageTitle = getPageTitleContext();

  $effect(() => {
    pageTitle.setTitle(m.app_collection());

    return () => {
      pageTitle.clearTitle();
    };
  });
</script>

<div class="safe-area-pad-bottom-nav">
  <CollectionDashboard />
</div>
