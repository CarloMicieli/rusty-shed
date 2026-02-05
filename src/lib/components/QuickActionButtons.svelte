<script lang="ts">
  import type { ComponentType } from 'svelte';

  export type QuickAction = {
    id: string;
    label: string;
    icon: ComponentType;
    onClick?: () => void;
  };

  let { actions = [], class: className } = $props<{ actions: QuickAction[]; class?: string }>();
</script>

<div class={['grid grid-cols-1 gap-3', className]}>
  {#each actions as action (action.id)}
    {@const Icon = action.icon}
    <button
      type="button"
      aria-label={action.label}
      class="group flex w-full items-center justify-start gap-3 rounded-lg border-2 border-border/70 bg-card/80 px-4 py-3 text-foreground shadow-[inset_0_1px_0_rgba(255,255,255,0.06),inset_0_-2px_0_rgba(0,0,0,0.4),0_8px_16px_rgba(0,0,0,0.3)] transition-all duration-200 hover:-translate-y-0.5 hover:border-primary/70 hover:bg-primary/15 active:translate-y-0"
      onclick={() => action.onClick?.()}
    >
      <Icon
        class="text-primary transition-transform group-hover:scale-110 group-hover:text-primary-foreground"
        size={20}
      />
      <span
        class="text-xs font-semibold tracking-[0.2em] uppercase group-hover:text-primary-foreground"
      >
        {action.label}
      </span>
    </button>
  {/each}
</div>
