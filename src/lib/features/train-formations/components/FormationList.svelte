<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';
  import { Plus, Combine } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import FormationCard from './FormationCard.svelte';
  import FormationForm from './FormationForm.svelte';
  import * as Dialog from '$lib/components/ui/dialog';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  let { state: ctx }: { state: TrainFormationState } = $props();

  let showCreateDialog = $state(false);
</script>

<div class="container mx-auto max-w-6xl space-y-6 p-6">
  <PageHeader
    title={m.formations_page_title()}
    subtitle="Trains"
    description=""
    class="variant-steampunk-riveted rounded-sm border border-border bg-card px-5 py-4 shadow-sm"
  >
    {#snippet actions()}
      <Button
        class="variant-steampunk-lever rounded-sm border-border bg-primary text-primary-foreground transition-all duration-150 ease-out hover:bg-primary/90"
        onclick={() => (showCreateDialog = true)}
      >
        <Plus class="mr-2 size-4" />
        {m.formations_new_formation()}
      </Button>
    {/snippet}
  </PageHeader>

  {#if ctx.isLoading}
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
      {#each Array.from({ length: 3 }) as _, index (index)}
        <div
          class="variant-steampunk-riveted h-40 animate-pulse rounded-sm border border-border bg-card/60"
        ></div>
      {/each}
    </div>
  {:else if ctx.summaries.length === 0}
    <div
      class="variant-steampunk-parchment flex flex-col items-center justify-center gap-5 rounded-sm px-6 py-20 text-center shadow-sm"
    >
      <div
        class="rounded-full border border-primary/30 bg-background/70 p-5 text-primary shadow-sm"
      >
        <Combine class="size-12 opacity-80" />
      </div>
      <p class="max-w-md text-sm leading-relaxed text-muted-foreground">
        {m.formations_empty_list()}
      </p>
      <Button
        class="variant-steampunk-lever rounded-sm border-border bg-primary text-primary-foreground transition-all duration-150 ease-out hover:bg-primary/90"
        onclick={() => (showCreateDialog = true)}
      >
        <Plus class="mr-2 size-4" />
        {m.formations_new_formation()}
      </Button>
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
      {#each ctx.summaries as summary (summary.id)}
        <FormationCard {summary} onclick={() => goto(resolve(`/train-formations/${summary.id}`))} />
      {/each}
    </div>
  {/if}
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
        const id = await ctx.create(args);
        showCreateDialog = false;
        if (id) goto(resolve(`/train-formations/${id}`));
      }}
      oncancel={() => (showCreateDialog = false)}
    />
  </Dialog.Content>
</Dialog.Root>
