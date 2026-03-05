<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import LocalizedFieldInput from './LocalizedFieldInput.svelte';

  interface Props {
    enDescription: string | null;
    enDetails: string | null;
    itDescription: string | null;
    itDetails: string | null;
  }

  let {
    enDescription = $bindable(null),
    enDetails = $bindable(null),
    itDescription = $bindable(null),
    itDetails = $bindable(null)
  }: Props = $props();

  let activeTab = $state<'en' | 'it'>('en');
</script>

<div class="flex flex-col gap-3">
  <!-- Tab bar -->
  <div class="flex gap-1 rounded-lg border border-zinc-800 bg-zinc-900 p-1">
    <button
      type="button"
      class="flex-1 rounded-md px-3 py-1.5 text-xs font-medium transition-colors {activeTab === 'en'
        ? 'bg-amber-500/10 text-amber-400'
        : 'text-zinc-400 hover:text-zinc-200'}"
      onclick={() => (activeTab = 'en')}
    >
      {m.translation_section_english()}
    </button>
    <button
      type="button"
      class="flex-1 rounded-md px-3 py-1.5 text-xs font-medium transition-colors {activeTab === 'it'
        ? 'bg-amber-500/10 text-amber-400'
        : 'text-zinc-400 hover:text-zinc-200'}"
      onclick={() => (activeTab = 'it')}
    >
      {m.translation_section_italian()}
    </button>
  </div>

  <!-- English tab -->
  {#if activeTab === 'en'}
    <div class="flex flex-col gap-3">
      <LocalizedFieldInput
        lang="en"
        label={m.translation_section_description()}
        bind:value={enDescription}
        required={true}
        singleLine={true}
      />
      <div
        class="rounded-md border border-zinc-800 bg-[#0F0F0F] p-3 transition-colors focus-within:border-amber-500/50"
      >
        <LocalizedFieldInput
          lang="en"
          label={m.translation_section_details()}
          bind:value={enDetails}
          required={false}
          rows={5}
        />
      </div>
    </div>
  {/if}

  <!-- Italian tab -->
  {#if activeTab === 'it'}
    <div class="flex flex-col gap-3">
      <LocalizedFieldInput
        lang="it"
        label={m.translation_section_description()}
        bind:value={itDescription}
        required={false}
        singleLine={true}
      />
      <div
        class="rounded-md border border-zinc-800 bg-[#0F0F0F] p-3 transition-colors focus-within:border-amber-500/50"
      >
        <LocalizedFieldInput
          lang="it"
          label={m.translation_section_details()}
          bind:value={itDetails}
          required={false}
          rows={5}
        />
      </div>
    </div>
  {/if}
</div>
