<script lang="ts">
  import { onMount } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import type { CommandError } from '$lib/bindings';
  import { getBuyers, getManufacturers, getSellers, type LibraryEntityRow } from '$lib/services/entityLibrary';
  import EntitySearch from './EntitySearch.svelte';
  import EntityTabs from './EntityTabs.svelte';

  let manufacturers = $state<LibraryEntityRow[]>([]);
  let sellers = $state<LibraryEntityRow[]>([]);
  let buyers = $state<LibraryEntityRow[]>([]);

  const query = $derived(settingsState.librarySearchQuery.trim().toLowerCase());

  const filteredManufacturers = $derived.by(() =>
    manufacturers.filter((row) =>
      `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query)
    )
  );

  const filteredSellers = $derived.by(() =>
    sellers.filter((row) => `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query))
  );

  const filteredBuyers = $derived.by(() =>
    buyers.filter((row) => `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query))
  );

  function getCommandErrorMessage(error: CommandError): string {
    if ('DatabaseError' in error) return error.DatabaseError ?? m.settings_load_failed();
    if ('NotFound' in error) return error.NotFound ?? m.settings_load_failed();
    if ('PermissionDenied' in error) return error.PermissionDenied ?? m.settings_load_failed();
    if ('BusinessRule' in error) return error.BusinessRule ?? m.settings_load_failed();
    if ('Conflict' in error) return error.Conflict ?? m.settings_load_failed();
    if ('Unknown' in error) return error.Unknown?.message ?? m.settings_load_failed();
    return m.settings_load_failed();
  }

  async function loadLibraryRows(): Promise<void> {
    settingsState.libraryLoading = true;
    settingsState.libraryError = null;

    const [manufacturerRes, sellerRes, buyerRes] = await Promise.all([
      getManufacturers(),
      getSellers(),
      getBuyers()
    ]);

    if (manufacturerRes.status === 'ok') manufacturers = manufacturerRes.data;
    if (sellerRes.status === 'ok') sellers = sellerRes.data;
    if (buyerRes.status === 'ok') buyers = buyerRes.data;

    const firstError = [manufacturerRes, sellerRes, buyerRes].find((result) => result.status === 'error');
    if (firstError && firstError.status === 'error') {
      settingsState.libraryError = getCommandErrorMessage(firstError.error);
    }

    settingsState.libraryLoading = false;
  }

  onMount(() => {
    void loadLibraryRows();
  });
</script>

<section class="rounded-lg border border-border bg-card p-6" aria-label={m.settings_library_title()}>
  <header class="mb-4">
    <h2 class="text-lg font-semibold">{m.settings_library_title()}</h2>
    <p class="text-sm text-muted-foreground">{m.settings_library_subtitle()}</p>
  </header>

  <EntitySearch
    value={settingsState.librarySearchQuery}
    onChange={(value) => settingsState.setLibrarySearchQuery(value)}
  />

  {#if settingsState.libraryError}
    <p class="mt-3 text-sm text-destructive">{settingsState.libraryError}</p>
  {/if}

  <div class="mt-4">
    <EntityTabs
      activeTab={settingsState.libraryActiveTab}
      onTabChange={(tab) => settingsState.setLibraryTab(tab)}
      manufacturers={filteredManufacturers}
      sellers={filteredSellers}
      buyers={filteredBuyers}
    />
  </div>
</section>
