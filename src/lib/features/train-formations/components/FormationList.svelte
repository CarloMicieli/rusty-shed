<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { Plus, Combine } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import FormationCard from './FormationCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import FormationForm from './FormationForm.svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  let { state: ctx }: { state: TrainFormationState } = $props();

  let showCreateDialog = $state(false);
</script>

<div class="flex flex-col">
  <div
    class="-mx-4 -mt-4 mb-6 border-b border-border bg-card/50 px-6 py-4 lg:-mx-8 lg:-mt-8 lg:mb-8"
  >
    <PageHeader
      title={m.formations_page_title()}
      subtitle="Trains"
      description={m.formations_description()}
    >
      {#snippet actions()}
        {#if ctx.summaries.length > 0}
          <Button variant="rusty" size="sm" onclick={() => (showCreateDialog = true)}>
            <Plus class="h-4 w-4" />
            {m.formations_new_formation()}
          </Button>
        {/if}
      {/snippet}
    </PageHeader>
  </div>

  <div class="space-y-6">
    {#if ctx.isLoading}
      <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
        {#each Array.from({ length: 3 }) as _, index (index)}
          <div
            class="variant-steampunk-riveted h-40 animate-pulse rounded-sm border border-border bg-card/60"
          ></div>
        {/each}
      </div>
    {:else if ctx.summaries.length === 0}
      <EmptyState
        icon={Combine}
        ctaIcon={Plus}
        title={m.formations_empty_heading()}
        description={m.formations_empty_sub()}
        ctaLabel={m.formations_new_formation()}
        onCta={() => (showCreateDialog = true)}
      />
    {:else}
      <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
        {#each ctx.summaries as summary (summary.id)}
          <FormationCard
            {summary}
            onclick={() => goto(resolve(`/train-formations/${summary.id}`))}
            ondelete={() => ctx.delete(summary.id)}
          />
        {/each}
      </div>
    {/if}
  </div>
</div>

<Dialog.Root bind:open={showCreateDialog}>
  <Dialog.Content
    class="variant-steampunk-riveted rounded-sm border border-border bg-card sm:max-w-lg"
  >
    <Dialog.Header>
      <Dialog.Title>{m.formations_new_formation()}</Dialog.Title>
    </Dialog.Header>
    <FormationForm
      categories={ctx.categories}
      onsubmit={async (args) => {
        await ctx.create(args);
        showCreateDialog = false;
      }}
      oncancel={() => (showCreateDialog = false)}
    />
  </Dialog.Content>
</Dialog.Root>
