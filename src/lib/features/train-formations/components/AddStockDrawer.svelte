<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { Input } from '$lib/components/ui/input';
  import { Button } from '$lib/components/ui/button';
  import { Sheet } from '$lib/components/ui/sheet';
  import { Plus } from 'lucide-svelte';
  import PrototypeSearchResults from './PrototypeSearchResults.svelte';
  import CreatePrototypeForm from './CreatePrototypeForm.svelte';
  import type { TrainFormationState } from '../TrainFormationState.svelte.js';

  let {
    state: ctx,
    formationId,
    open = $bindable(false)
  }: {
    state: TrainFormationState;
    formationId: string;
    open?: boolean;
  } = $props();

  let searchQuery = $state('');
  let showCreateForm = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  $effect(() => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      ctx.searchPrototypes(searchQuery);
    }, 200);
    return () => clearTimeout(debounceTimer);
  });

  async function handleAddPrototype(prototypeId: string) {
    await ctx.addElement(formationId, { prototype_id: prototypeId, owned_rolling_stock_id: null });
  }
</script>

<Sheet
  bind:open
  side="right"
  class="w-full overflow-y-auto border-l border-stone-800 bg-stone-950 shadow-2xl shadow-black/50 sm:max-w-md"
>
  <div class="bg-inherit p-6">
    <h2 class="mb-4 text-lg font-semibold">{m.formations_add_stock()}</h2>

    <div class="flex flex-col gap-4">
      <Input
        placeholder={m.formations_search_placeholder()}
        bind:value={searchQuery}
        autocomplete="off"
      />

      {#if showCreateForm}
        <CreatePrototypeForm
          state={ctx}
          onCreated={async (prototypeId) => {
            showCreateForm = false;
            await handleAddPrototype(prototypeId);
          }}
          onCancel={() => (showCreateForm = false)}
        />
      {:else}
        <PrototypeSearchResults
          groups={ctx.prototypeGroups}
          isLoading={ctx.isPrototypesLoading}
          onSelect={handleAddPrototype}
        />

        {#if !ctx.isPrototypesLoading}
          <Button variant="outline" class="w-full gap-2" onclick={() => (showCreateForm = true)}>
            <Plus class="size-4" />
            {m.formations_add_prototype_action()}
          </Button>
        {/if}
      {/if}
    </div>
  </div>
</Sheet>
