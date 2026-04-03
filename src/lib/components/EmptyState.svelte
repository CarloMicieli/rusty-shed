<script lang="ts">
  import type { IconComponent } from '$lib/config/icons';

  interface Props {
    icon: IconComponent;
    title: string;
    description: string;
    ctaLabel?: string;
    ctaIcon?: IconComponent;
    onCta?: () => void;
  }

  const { icon: Icon, ctaIcon, title, description, ctaLabel, onCta }: Props = $props();
  const CtaIcon = $derived(ctaIcon ?? Icon);
</script>

<div
  class="flex flex-col items-center justify-center gap-8 rounded-sm border border-dashed border-border bg-card/50 px-4 py-24 text-center"
>
  <div
    class="flex h-32 w-32 items-center justify-center rounded-full border-2 border-border bg-background"
  >
    <Icon size={56} strokeWidth={2} class="text-muted-foreground" />
  </div>

  <div class="flex max-w-sm flex-col items-center gap-3 text-center">
    <h3 class="font-bebas text-3xl tracking-widest text-foreground uppercase">{title}</h3>
    <p class="text-sm leading-relaxed text-muted-foreground">{description}</p>
  </div>

  {#if ctaLabel && onCta}
    <button
      type="button"
      class="mt-2 inline-flex cursor-pointer items-center gap-2 rounded-sm bg-primary px-8 py-3 font-bebas text-lg tracking-widest text-primary-foreground uppercase shadow-[0_3px_0_hsl(var(--primary)/0.5)] transition-all hover:brightness-110 active:translate-y-0.5 active:shadow-none"
      onclick={onCta}
    >
      <CtaIcon class="h-5 w-5" />
      <span>{ctaLabel}</span>
    </button>
  {/if}
</div>
