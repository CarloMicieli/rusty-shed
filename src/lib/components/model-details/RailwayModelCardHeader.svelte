<script lang="ts">
  import { tick } from 'svelte';
  import type { RailwayModel } from '$lib/types/railway-model';
  import { Badge } from '$lib/components/ui/badge';
  import { getLocale } from '$lib/paraglide/runtime.js';
  import type { Language } from '$lib/bindings';
  import LanguageFallbackBadge from '$lib/components/LanguageFallbackBadge.svelte';
  import * as m from '$lib/paraglide/messages';

  interface _Props {
    model: RailwayModel;
    editable?: boolean;
    powerMethodLabel?: string;
    onDescriptionSave?: (value: string) => Promise<void>;
  }

  const { model, editable = false, powerMethodLabel, onDescriptionSave }: _Props = $props();

  const currentLocale = getLocale() as Language;

  let isEditingDescription = $state(false);
  let descriptionDraft = $state('');
  let isSavingDescription = $state(false);
  let descriptionInput = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (!isEditingDescription) {
      descriptionDraft = model.description ?? '';
    }
  });

  $effect(() => {
    if (isEditingDescription) {
      void tick().then(() => descriptionInput?.focus());
    }
  });

  function startDescriptionEditing() {
    if (!editable) return;
    isEditingDescription = true;
  }

  function cancelDescriptionEditing() {
    isEditingDescription = false;
    descriptionDraft = model.description ?? '';
  }

  async function saveDescription() {
    if (!onDescriptionSave || descriptionDraft === model.description) {
      isEditingDescription = false;
      return;
    }

    try {
      isSavingDescription = true;
      await onDescriptionSave(descriptionDraft);
    } finally {
      isSavingDescription = false;
      isEditingDescription = false;
    }
  }
</script>

<div class="flex items-start justify-between">
  <div class="min-w-0 flex-1">
    <div class="flex flex-wrap items-center gap-1.5">
      <span class="text-xs font-semibold text-foreground">{model.manufacturer}</span>
      <span class="text-muted-foreground/40" aria-hidden="true">·</span>
      <span class="font-mono text-xs text-muted-foreground">{model.product_code}</span>
    </div>
    <div class="mt-0.5">
      {#if editable}
        {#if isEditingDescription}
          <input
            bind:this={descriptionInput}
            class="w-full rounded-sm border border-border bg-card/70 px-1.5 py-0.5 text-sm text-foreground outline-none focus:border-primary"
            bind:value={descriptionDraft}
            onblur={saveDescription}
            onkeydown={(e) => {
              if (e.key === 'Enter') {
                e.preventDefault();
                void saveDescription();
              }
              if (e.key === 'Escape') {
                e.preventDefault();
                cancelDescriptionEditing();
              }
            }}
            disabled={isSavingDescription}
          />
        {:else}
          <button
            type="button"
            class="line-clamp-1 cursor-text text-left text-sm text-muted-foreground transition-colors hover:text-foreground"
            onclick={startDescriptionEditing}
          >
            {model.description || m.details_placeholder()}
          </button>
        {/if}
      {:else if model.description}
        <p class="line-clamp-1 text-sm text-muted-foreground">{model.description}</p>
      {/if}

      {#if model.description && model.descriptionLang !== currentLocale}
        <div class="mt-1">
          <LanguageFallbackBadge lang={model.descriptionLang} />
        </div>
      {/if}
    </div>
  </div>

  {#if model.power_method}
    <Badge
      class="ml-2 shrink-0 border-transparent bg-[#E2994F] px-1.5 py-0.5 text-[10px] font-bold text-black"
    >
      {powerMethodLabel || ''}
    </Badge>
  {/if}
</div>
