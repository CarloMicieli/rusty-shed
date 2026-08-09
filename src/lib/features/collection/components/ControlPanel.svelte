<script lang="ts">
  import { X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import type { FilterState, StatusFilter } from '$lib/features/collection/CollectionState.svelte';
  import { categoryLabel as enumCategoryLabel } from '$lib/utils/enum-options';
  import type { Category } from '$lib/bindings';
  import { resolveTagMeta, tagIcon } from '$lib/config/tags';
  import { cn } from '$lib/utils';

  interface ScaleOption {
    id: string;
    display: string;
  }

  interface Props {
    filters: FilterState;
    availableScales: ScaleOption[];
    availableCompanies: string[];
    availableCategories: string[];
    availableEpochs: string[];
    availableTags: string[];
    hasActiveFilters: boolean;
    onToggleScale?: (scale: string) => void;
    onToggleCompany?: (company: string) => void;
    onToggleCategory?: (category: string) => void;
    onToggleEpoch?: (epoch: string) => void;
    onToggleTag?: (tag: string) => void;
    onSetStatus?: (status: StatusFilter) => void;
    onClear?: () => void;
    onToggleSidebar?: () => void;
    onCloseMobileSheet?: () => void;
  }

  const {
    filters,
    availableScales,
    availableCompanies,
    availableCategories,
    availableEpochs,
    availableTags,
    hasActiveFilters,
    onToggleScale,
    onToggleCompany,
    onToggleCategory,
    onToggleEpoch,
    onToggleTag,
    onSetStatus,
    onClear,
    onToggleSidebar,
    onCloseMobileSheet
  }: Props = $props();

  function categoryLabel(cat: string): string {
    return enumCategoryLabel(cat as Category);
  }

  const closeAction = $derived(onCloseMobileSheet ?? onToggleSidebar);

  function sectionLabelClasses() {
    return 'mb-2 text-xs font-medium tracking-wider text-muted-foreground uppercase';
  }

  function buttonBaseClasses() {
    return 'w-full min-h-11 cursor-pointer rounded-sm border border-border bg-card px-2.5 py-1.5 text-left text-xs transition-all active:scale-[0.98] active:bg-muted/50';
  }
</script>

<div class="flex h-full flex-col bg-layout-surface">
  <!-- Header -->
  <div
    class="flex flex-shrink-0 items-center justify-between border-b border-layout-border px-4 py-3"
  >
    <span class="font-bebas text-base tracking-widest whitespace-nowrap text-foreground uppercase">
      {m.collection_filters_title()}
    </span>
    <div class="flex items-center gap-2">
      {#if hasActiveFilters}
        <button
          type="button"
          class="min-h-11 rounded-sm border border-border bg-card px-3 text-xs font-medium tracking-wider text-muted-foreground uppercase transition-all hover:bg-muted/70 hover:text-foreground active:scale-[0.98] active:bg-muted"
          onclick={onClear}
        >
          {m.controlpanel_clear_all()}
        </button>
      {/if}
      <button
        type="button"
        class="min-h-11 min-w-11 rounded-sm border border-border bg-card p-2 text-muted-foreground transition-all hover:bg-muted/70 hover:text-foreground active:scale-[0.98] active:bg-muted"
        onclick={closeAction}
        title={m.collection_toggle_filters_title()}
        aria-label={m.collection_toggle_filters_title()}
      >
        <X size={14} />
      </button>
    </div>
  </div>

  <!-- Scrollable filter sections -->
  <div class="flex-1 overflow-y-auto">
    <!-- Status -->
    <section class="border-b border-layout-border px-4 py-3">
      <h4 class={sectionLabelClasses()}>
        {m.collection_filter_status()}
      </h4>
      <div class="flex flex-col gap-1">
        {#each [{ value: 'active' as StatusFilter, label: m.collection_filter_status_active() }, { value: 'preordered' as StatusFilter, label: m.collection_filter_status_preordered() }, { value: 'sold' as StatusFilter, label: m.collection_filter_status_sold() }, { value: 'all' as StatusFilter, label: m.collection_filter_status_all() }] as opt (opt.value)}
          <button
            type="button"
            class={cn(
              buttonBaseClasses(),
              filters.status === opt.value
                ? 'border-primary bg-primary/15 font-medium text-primary'
                : 'text-muted-foreground hover:bg-muted/70 hover:text-foreground'
            )}
            onclick={() => onSetStatus?.(opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </section>
    <!-- Scales -->
    {#if availableScales.length > 0}
      <section class="border-b border-layout-border px-4 py-3">
        <h4 class={sectionLabelClasses()}>
          {m.controlpanel_section_scales()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableScales as scale (scale.id)}
            <button
              type="button"
              class={cn(
                'min-h-11 cursor-pointer rounded-sm border px-3 py-1.5 font-mono text-xs transition-all active:scale-[0.98] active:bg-muted/50',
                filters.scales.has(scale.id)
                  ? 'border-primary bg-primary/15 text-primary'
                  : 'border-border bg-card text-muted-foreground hover:bg-muted/70 hover:text-foreground'
              )}
              onclick={() => onToggleScale?.(scale.id)}
            >
              {scale.display}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Companies -->
    {#if availableCompanies.length > 0}
      <section class="border-b border-layout-border px-4 py-3">
        <h4 class={sectionLabelClasses()}>
          {m.controlpanel_section_companies()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableCompanies as company (company)}
            <button
              type="button"
              class={cn(
                'min-h-11 cursor-pointer rounded-sm border px-3 py-1.5 text-xs transition-all active:scale-[0.98] active:bg-muted/50',
                filters.companies.has(company)
                  ? 'border-primary bg-primary/15 text-primary'
                  : 'border-border bg-card text-muted-foreground hover:bg-muted/70 hover:text-foreground'
              )}
              onclick={() => onToggleCompany?.(company)}
            >
              {company}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Category -->
    {#if availableCategories.length > 0}
      <section class="border-b border-layout-border px-4 py-3">
        <h4 class={sectionLabelClasses()}>
          {m.controlpanel_section_categories()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableCategories as category (category)}
            <button
              type="button"
              class={cn(
                'min-h-11 cursor-pointer rounded-sm border px-3 py-1.5 text-xs transition-all active:scale-[0.98] active:bg-muted/50',
                filters.categories.has(category)
                  ? 'border-primary bg-primary/15 text-primary'
                  : 'border-border bg-card text-muted-foreground hover:bg-muted/70 hover:text-foreground'
              )}
              onclick={() => onToggleCategory?.(category)}
            >
              {categoryLabel(category)}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Epochs -->
    {#if availableEpochs.length > 0}
      <section class="border-b border-layout-border px-4 py-3">
        <h4 class={sectionLabelClasses()}>
          {m.controlpanel_section_epochs()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableEpochs as epoch (epoch)}
            <button
              type="button"
              class={cn(
                'min-h-11 cursor-pointer rounded-sm border px-3 py-1.5 font-mono text-xs transition-all active:scale-[0.98] active:bg-muted/50',
                filters.epochs.has(epoch)
                  ? 'border-primary bg-primary/15 text-primary'
                  : 'border-border bg-card text-muted-foreground hover:bg-muted/70 hover:text-foreground'
              )}
              onclick={() => onToggleEpoch?.(epoch)}
            >
              {epoch}
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Tags -->
    {#if availableTags.length > 0}
      <section class="px-4 py-3">
        <h4 class={sectionLabelClasses()}>
          {m.collection_filter_tags()}
        </h4>
        <div class="flex flex-wrap gap-2">
          {#each availableTags as tag (tag)}
            {#if tag}
              {@const Icon = tagIcon(tag)}
              <button
                type="button"
                class={cn(
                  'inline-flex min-h-11 items-center gap-2 rounded-sm border px-3 py-1.5 text-xs transition-all active:scale-[0.98] active:bg-muted/50',
                  filters.tags.has(tag)
                    ? 'border-primary bg-primary/15 text-primary'
                    : 'border-border bg-card text-muted-foreground hover:bg-muted/70 hover:text-foreground'
                )}
                onclick={() => onToggleTag?.(tag)}
              >
                {#if Icon}
                  <Icon size={14} />
                {/if}
                <span>{resolveTagMeta(tag).label()}</span>
              </button>
            {/if}
          {/each}
        </div>
      </section>
    {/if}
  </div>
</div>
