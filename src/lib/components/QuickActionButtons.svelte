<script lang="ts">
  import { Plus, Wrench, FileText } from 'lucide-svelte';
  import { _ } from 'svelte-i18n';
  import { goto } from '$app/navigation';

  // Define the actions directly within the component
  const actions = [
    { id: '1', label: 'add_railway_model', icon: Plus, url: '/catalogue/new-model' },
    { id: '2', label: 'schedule_maintenance', icon: Wrench, url: null },
    { id: '3', label: 'generate_report', icon: FileText, url: null }
  ];

  function handleClick(url: string | null) {
    if (url) {
      // resolveRoute is removed; SvelteKit handles paths directly
      goto(url);
    }
  }
</script>

<div class="grid grid-cols-1 gap-2 sm:grid-cols-3">
  {#each actions as action (action.id)}
    {@const Icon = action.icon}
    <button
      type="button"
      class="variant-ghost-surface hover:variant-filled-primary group btn justify-start border border-surface-700/50 p-4 transition-all duration-200"
      onclick={() => handleClick(action.url)}
    >
      <Icon
        class="text-accent-500 group-hover:text-on-primary mr-3 transition-transform group-hover:scale-110"
        size={20}
      />
      <span class="group-hover:text-on-primary font-semibold tracking-wide uppercase">
        {$_(`actions.${action.label}`)}
      </span>
    </button>
  {/each}
</div>
