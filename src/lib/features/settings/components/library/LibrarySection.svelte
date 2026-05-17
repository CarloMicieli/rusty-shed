<script lang="ts">
  import { onMount } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { QuickAddShell } from '$lib/components/drawer';
  import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';
  import type { QuickAddTarget } from '$lib/features/quick-add/types';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import type { CommandError, Manufacturer, Seller } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import {
    getBuyers,
    getManufacturers,
    getSellers,
    type LibraryEntityRow
  } from '$lib/services/entityLibrary';
  import EntitySearch from './EntitySearch.svelte';
  import EntityTabs from './EntityTabs.svelte';

  let manufacturers = $state<LibraryEntityRow[]>([]);
  let sellers = $state<LibraryEntityRow[]>([]);
  let buyers = $state<LibraryEntityRow[]>([]);
  let quickAddTarget = $state<QuickAddTarget | null>(null);

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

  const quickAddTitle = $derived.by(() => {
    if (quickAddTarget === 'manufacturer') return m.settings_library_add_manufacturer_title();
    if (quickAddTarget === 'buyer') return m.settings_library_add_buyer_title();
    return m.settings_library_add_seller_title();
  });

  const quickAddNames = $derived.by(() => {
    if (quickAddTarget === 'manufacturer') {
      return manufacturers.map((row) => row.name);
    }

    const merged = [...sellers.map((row) => row.name), ...buyers.map((row) => row.name)];
    return [...new Set(merged)];
  });

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

  function openQuickAdd(): void {
    if (settingsState.libraryActiveTab === 'manufacturers') {
      quickAddTarget = 'manufacturer';
      return;
    }

    if (settingsState.libraryActiveTab === 'buyers') {
      quickAddTarget = 'buyer';
      return;
    }

    quickAddTarget = 'seller';
  }

  function closeQuickAdd(): void {
    quickAddTarget = null;
  }

  function prependUnique(rows: LibraryEntityRow[], row: LibraryEntityRow): LibraryEntityRow[] {
    const remaining = rows.filter((entry) => entry.id !== row.id);
    return [row, ...remaining];
  }

  function toLibraryRow(entity: Manufacturer | Seller): LibraryEntityRow {
    if ('sellerType' in entity) {
      return {
        id: entity.id,
        name: entity.name,
        countryCode: entity.address?.country ?? null,
        usageCount: 0,
        isSystemSeeded: false
      };
    }

    return {
      id: entity.id,
      name: entity.name,
      countryCode: entity.countryCode,
      usageCount: 0,
      isSystemSeeded: false
    };
  }

  function handleQuickAddSuccess(entity: Manufacturer | Seller): void {
    const row = toLibraryRow(entity);

    if (quickAddTarget === 'manufacturer') {
      manufacturers = prependUnique(manufacturers, row);
      toaster.success(m.settings_library_create_success_manufacturer({ name: row.name }));
      closeQuickAdd();
      return;
    }

    sellers = prependUnique(sellers, row);
    buyers = prependUnique(buyers, row);

    if (quickAddTarget === 'buyer') {
      toaster.success(m.settings_library_create_success_buyer({ name: row.name }));
    } else {
      toaster.success(m.settings_library_create_success_seller({ name: row.name }));
    }

    closeQuickAdd();
  }

  function handleQuickAddError(message: string): void {
    toaster.signal(m.signal_toast_title(), {
      description: m.settings_library_create_error({ message })
    });
  }

  onMount(() => {
    void loadLibraryRows();
  });
</script>

<section class="rounded-lg border border-border bg-card p-6" aria-label={m.settings_library_title()}>
  <header class="mb-4 flex items-start justify-between gap-3">
    <div>
      <h2 class="text-lg font-semibold">{m.settings_library_title()}</h2>
      <p class="text-sm text-muted-foreground">{m.settings_library_subtitle()}</p>
    </div>
    <button
      type="button"
      class="rounded-sm border border-border bg-background px-3 py-1 text-sm"
      onclick={openQuickAdd}
    >
      {m.settings_library_add_new()}
    </button>
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

<QuickAddShell open={quickAddTarget !== null} title={quickAddTitle} onDismiss={closeQuickAdd}>
  {#if quickAddTarget}
    <QuickAddEntityForm
      target={quickAddTarget}
      mode="FULL"
      existingNames={quickAddNames}
      onSuccess={handleQuickAddSuccess}
      onCancel={closeQuickAdd}
      onError={handleQuickAddError}
    />
  {/if}

  {#snippet footer()}
    <div class="text-xs text-muted-foreground">{m.quick_add_footer_hint()}</div>
  {/snippet}
</QuickAddShell>
