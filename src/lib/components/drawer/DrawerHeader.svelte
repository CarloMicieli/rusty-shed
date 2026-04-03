<script lang="ts">
  import { X } from 'lucide-svelte';
  import type { IconComponent } from '$lib/config/icons';

  interface Props {
    id: string;
    title: string;
    subtitle?: string;
    icon: IconComponent;
    onClose: () => void;
    disabled?: boolean;
  }

  let { id, title, subtitle, icon: Icon, onClose, disabled = false }: Props = $props();
</script>

<div class="flex items-center justify-between p-4">
  <div class="flex items-center gap-3">
    <div class="flex h-10 w-10 items-center justify-center rounded-sm bg-primary/10">
      <Icon size={20} class="text-primary" />
    </div>
    <div>
      <h2 {id} class="font-bebas text-2xl tracking-widest text-foreground">
        {title}
      </h2>
      {#if subtitle}
        <p class="text-[10px] font-bold text-muted-foreground uppercase">{subtitle}</p>
      {/if}
    </div>
  </div>
  <button
    type="button"
    onclick={onClose}
    {disabled}
    aria-label="close"
    class="flex h-8 w-8 items-center justify-center rounded-sm text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
  >
    <X size={16} />
  </button>
</div>
