<script lang="ts">
  import type { ComponentType } from 'svelte';

  export type QuickAction = {
    id: string;
    label: string;
    icon: ComponentType;
    onClick?: () => void;
  };

  let { actions = [] } = $props<{ actions: QuickAction[] }>();
</script>

<div class="grid grid-cols-1 gap-3">
  {#each actions as action (action.id)}
    {@const Icon = action.icon}
    <button
      type="button"
      class="variant-ghost-surface hover:variant-filled-primary group btn w-full justify-start border border-surface-700/50 p-4 transition-all duration-200"
      onclick={() => action.onClick?.()}
    >
      <Icon
        class="text-accent-500 group-hover:text-on-primary mr-3 transition-transform group-hover:scale-110"
        size={20}
      />
      <span class="group-hover:text-on-primary font-semibold tracking-wide uppercase">
        {action.label}
      </span>
    </button>
  {/each}
</div>
