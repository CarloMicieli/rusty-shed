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
      <div class="space-y-4 rounded-lg border border-white/10 bg-black/20 p-4">
        <div
          class="flex flex-col items-center justify-center gap-8 rounded-3xl border border-white/5 bg-layout-surface/50 px-4 py-24 text-center"
        >
          <div class="relative">
            <div class="absolute inset-0 rounded-full bg-zinc-500/10 blur-3xl"></div>
            <div
              class="relative flex h-32 w-32 items-center justify-center rounded-full border border-white/10 bg-zinc-900/50"
            >
              <Combine size={56} class="text-zinc-600 opacity-50" />
            </div>
          </div>
          <div class="flex max-w-sm flex-col items-center gap-3 text-center">
            <h3 class="text-2xl font-bold text-zinc-200">
              {m.formations_empty_heading()}
            </h3>
            <p class="text-sm leading-relaxed text-zinc-500">
              {m.formations_empty_sub()}
            </p>
          </div>
          <button
            type="button"
            class="group relative mt-2 inline-flex cursor-pointer items-center gap-3 overflow-hidden rounded-full bg-amber-500 px-8 py-4 font-bold tracking-wide text-black transition-all hover:scale-105 hover:bg-amber-400 hover:shadow-[0_0_20px_rgba(245,158,11,0.4)] active:scale-95"
            onclick={() => (showCreateDialog = true)}
          >
            <div
              class="absolute inset-0 translate-y-full bg-white/20 transition-transform duration-300 group-hover:translate-y-0"
            ></div>
            <Plus class="h-5 w-5" />
            <span>{m.formations_new_formation()}</span>
          </button>
        </div>
      </div>
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
