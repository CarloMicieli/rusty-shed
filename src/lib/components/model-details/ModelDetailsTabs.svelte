<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';

  interface Props {
    activeTab: 'details' | 'rolling-stock';
    onTabChange: (tab: 'details' | 'rolling-stock') => void;
  }

  let { activeTab, onTabChange }: Props = $props();
</script>

<div class="border-b border-border">
  <nav class="-mb-px flex space-x-8" aria-label="Tabs">
    <button
      type="button"
      class="whitespace-nowrap border-b-2 px-1 py-4 text-sm font-medium transition-colors {activeTab ===
      'details'
        ? 'border-primary text-primary'
        : 'border-transparent text-muted-foreground hover:border-border hover:text-foreground'}"
      onclick={() => onTabChange('details')}
      aria-current={activeTab === 'details' ? 'page' : undefined}
    >
      {m.model_details_tab()}
    </button>
    <button
      type="button"
      class="whitespace-nowrap border-b-2 px-1 py-4 text-sm font-medium transition-colors {activeTab ===
      'rolling-stock'
        ? 'border-primary text-primary'
        : 'border-transparent text-muted-foreground hover:border-border hover:text-foreground'}"
      onclick={() => onTabChange('rolling-stock')}
      aria-current={activeTab === 'rolling-stock' ? 'page' : undefined}
    >
      {m.model_rolling_stock_tab()}
    </button>
  </nav>
</div>
