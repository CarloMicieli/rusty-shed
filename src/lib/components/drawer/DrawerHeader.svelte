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
    <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-[#1F1F1F]">
      <Icon size={20} class="text-amber-500" />
    </div>
    <div>
      <h2 {id} class="text-sm font-bold tracking-wider text-amber-500 uppercase">
        {title}
      </h2>
      {#if subtitle}
        <p class="text-sm text-muted-foreground">{subtitle}</p>
      {/if}
    </div>
  </div>
  <button
    type="button"
    onclick={onClose}
    {disabled}
    aria-label="close"
    class="flex h-8 w-8 items-center justify-center rounded-full text-[#808080] transition-colors hover:bg-white/5 hover:text-white"
  >
    <X size={16} />
  </button>
</div>
