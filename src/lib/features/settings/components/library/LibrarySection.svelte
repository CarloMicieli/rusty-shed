<script lang="ts">
  import { onMount } from 'svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { QuickAddShell } from '$lib/components/drawer';
  import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';
  import type { QuickAddTarget } from '$lib/features/quick-add/types';
  import { settingsState } from '$lib/features/settings/SettingsState.svelte';
  import { commands, type CommandError, type Manufacturer, type Seller } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import {
    getBuyers,
    getManufacturers,
    getSellers,
    type LibraryEntityRow
  } from '$lib/services/entityLibrary';
  import DeleteEntityDialog from './DeleteEntityDialog.svelte';
  import MergeEntityDialog from './MergeEntityDialog.svelte';
  import EntitySearch from './EntitySearch.svelte';
  import EntityTabs from './EntityTabs.svelte';

  let quickFormIntent = $state<'create' | 'edit'>('create');
  let quickAddTarget = $state<QuickAddTarget | null>(null);
  let editingRow = $state<LibraryEntityRow | null>(null);
  let pendingDeleteRow = $state<LibraryEntityRow | null>(null);
  let deleting = $state(false);
  let pendingMergeSourceRow = $state<LibraryEntityRow | null>(null);
  let pendingMergeTargetId = $state<string | null>(null);
  let merging = $state(false);
  let currentPage = $state(1);
  let itemsPerPage = $state(10);

  const query = $derived(settingsState.librarySearchQuery.trim().toLowerCase());

  const filteredManufacturers = $derived.by(() =>
    settingsState.libraryManufacturers.filter((row) =>
      `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query)
    )
  );

  const filteredSellers = $derived.by(() =>
    settingsState.librarySellers.filter((row) =>
      `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query)
    )
  );

  const filteredBuyers = $derived.by(() =>
    settingsState.libraryBuyers.filter((row) =>
      `${row.name} ${row.countryCode ?? ''}`.toLowerCase().includes(query)
    )
  );

  const mergeCandidates = $derived.by(() => {
    if (settingsState.libraryActiveTab === 'manufacturers') {
      return settingsState.libraryManufacturers;
    }
    if (settingsState.libraryActiveTab === 'buyers') {
      return settingsState.libraryBuyers;
    }
    return settingsState.librarySellers;
  });

  const visibleRows = $derived.by(() => {
    if (settingsState.libraryActiveTab === 'manufacturers') {
      return filteredManufacturers;
    }

    if (settingsState.libraryActiveTab === 'buyers') {
      return filteredBuyers;
    }

    return filteredSellers;
  });

  const totalItems = $derived(visibleRows.length);
  const totalPages = $derived(Math.ceil(totalItems / itemsPerPage));
  const pageStart = $derived(totalItems === 0 ? 0 : (currentPage - 1) * itemsPerPage + 1);
  const pageEnd = $derived(totalItems === 0 ? 0 : Math.min(currentPage * itemsPerPage, totalItems));

  const paginatedRows = $derived.by(() =>
    visibleRows.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
  );

  const quickAddTitle = $derived.by(() => {
    if (quickFormIntent === 'edit') {
      if (quickAddTarget === 'manufacturer') return m.settings_library_edit_manufacturer_title();
      if (quickAddTarget === 'buyer') return m.settings_library_edit_buyer_title();
      return m.settings_library_edit_seller_title();
    }

    if (quickAddTarget === 'manufacturer') return m.settings_library_add_manufacturer_title();
    if (quickAddTarget === 'buyer') return m.settings_library_add_buyer_title();
    return m.settings_library_add_seller_title();
  });

  const quickSubmitLabel = $derived.by(() =>
    quickFormIntent === 'edit' ? m.settings_library_update_action() : m.quick_add_save()
  );

  const quickFormInitialValues = $derived.by(() => {
    if (!editingRow) {
      return {
        name: '',
        websiteUrl: '',
        countryCode: '',
        notes: ''
      };
    }

    return {
      name: editingRow.name,
      websiteUrl: '',
      countryCode: editingRow.countryCode ?? '',
      notes: ''
    };
  });

  const quickAddNames = $derived.by(() => {
    const editingId = editingRow?.id ?? null;

    if (quickAddTarget === 'manufacturer') {
      return settingsState.libraryManufacturers
        .filter((row) => row.id !== editingId)
        .map((row) => row.name);
    }

    const merged = [
      ...settingsState.librarySellers.filter((row) => row.id !== editingId).map((row) => row.name),
      ...settingsState.libraryBuyers.filter((row) => row.id !== editingId).map((row) => row.name)
    ];
    return [...new Set(merged)];
  });

  $effect(() => {
    settingsState.libraryActiveTab;
    settingsState.librarySearchQuery;
    currentPage = 1;
  });

  $effect(() => {
    if (totalPages > 0 && currentPage > totalPages) {
      currentPage = totalPages;
    }
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

    settingsState.setLibraryRows({
      manufacturers: manufacturerRes.status === 'ok' ? manufacturerRes.data : [],
      sellers: sellerRes.status === 'ok' ? sellerRes.data : [],
      buyers: buyerRes.status === 'ok' ? buyerRes.data : []
    });

    const firstError = [manufacturerRes, sellerRes, buyerRes].find(
      (result) => result.status === 'error'
    );
    if (firstError && firstError.status === 'error') {
      settingsState.libraryError = getCommandErrorMessage(firstError.error);
    }

    settingsState.libraryLoading = false;
  }

  function openQuickAdd(): void {
    quickFormIntent = 'create';
    editingRow = null;

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
    editingRow = null;
    quickFormIntent = 'create';
  }

  function openQuickEdit(row: LibraryEntityRow): void {
    quickFormIntent = 'edit';
    editingRow = row;

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
      settingsState.upsertLibraryManufacturer(row);
      toaster.success(
        quickFormIntent === 'edit'
          ? m.settings_library_update_success_manufacturer({ name: row.name })
          : m.settings_library_create_success_manufacturer({ name: row.name })
      );
      closeQuickAdd();
      return;
    }

    settingsState.upsertCanonicalParty(row);

    if (quickFormIntent === 'edit') {
      if (quickAddTarget === 'buyer') {
        toaster.success(m.settings_library_update_success_buyer({ name: row.name }));
      } else {
        toaster.success(m.settings_library_update_success_seller({ name: row.name }));
      }
    } else if (quickAddTarget === 'buyer') {
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

  function openDeleteDialog(row: LibraryEntityRow): void {
    pendingDeleteRow = row;
  }

  function closeDeleteDialog(): void {
    if (deleting) {
      return;
    }
    pendingDeleteRow = null;
  }

  function toDeleteErrorMessage(error: CommandError): string {
    const source = getCommandErrorMessage(error);
    const lower = source.toLowerCase();

    if (lower.includes('protected')) {
      return m.settings_library_delete_error_protected();
    }

    if (lower.includes('in use')) {
      return m.settings_library_delete_error_in_use();
    }

    return m.settings_library_delete_error_generic({ message: source });
  }

  function openMergeDialog(row: LibraryEntityRow): void {
    pendingMergeSourceRow = row;
    pendingMergeTargetId = null;
  }

  function closeMergeDialog(): void {
    if (merging) {
      return;
    }
    pendingMergeSourceRow = null;
    pendingMergeTargetId = null;
  }

  function toMergeErrorMessage(error: CommandError): string {
    const source = getCommandErrorMessage(error);
    const lower = source.toLowerCase();

    if (lower.includes('protected')) {
      return m.settings_library_merge_error_protected();
    }

    return m.settings_library_merge_error_generic({ message: source });
  }

  async function handleMergeConfirm(): Promise<void> {
    if (!pendingMergeSourceRow || !pendingMergeTargetId) {
      return;
    }

    merging = true;

    const source = pendingMergeSourceRow;
    const targetId = pendingMergeTargetId;
    const result =
      settingsState.libraryActiveTab === 'manufacturers'
        ? await commands.mergeManufacturers({ sourceId: source.id, targetId })
        : settingsState.libraryActiveTab === 'buyers'
          ? await commands.mergeBuyers({ sourceId: source.id, targetId })
          : await commands.mergeSellers({ sourceId: source.id, targetId });

    if (result.status === 'ok') {
      if (settingsState.libraryActiveTab === 'manufacturers') {
        settingsState.mergeLibraryManufacturer(source.id);
      } else {
        settingsState.mergeCanonicalParty(source.id);
      }

      toaster.success(
        m.settings_library_merge_success({
          source: source.name,
          count: result.data.relinkedCount
        })
      );

      closeMergeDialog();
      merging = false;
      await loadLibraryRows();
      return;
    }

    toaster.signal(m.signal_toast_title(), {
      description: toMergeErrorMessage(result.error)
    });
    merging = false;
  }

  async function handleDeleteConfirm(): Promise<void> {
    if (!pendingDeleteRow) {
      return;
    }

    deleting = true;

    const row = pendingDeleteRow;
    const result =
      settingsState.libraryActiveTab === 'manufacturers'
        ? await commands.deleteManufacturer(row.id)
        : settingsState.libraryActiveTab === 'buyers'
          ? await commands.deleteBuyer(row.id)
          : await commands.deleteSeller(row.id);

    if (result.status === 'ok') {
      if (settingsState.libraryActiveTab === 'manufacturers') {
        settingsState.removeLibraryManufacturer(row.id);
      } else {
        settingsState.removeCanonicalParty(row.id);
      }
      toaster.success(m.settings_library_delete_success({ name: row.name }));
      pendingDeleteRow = null;
      deleting = false;
      return;
    }

    toaster.signal(m.signal_toast_title(), {
      description: toDeleteErrorMessage(result.error)
    });
    deleting = false;
  }

  async function handleQuickSubmit(values: {
    name: string;
    websiteUrl: string;
    countryCode: string;
    notes: string;
  }): Promise<Manufacturer | Seller> {
    if (quickFormIntent !== 'edit' || !editingRow || !quickAddTarget) {
      throw new Error(m.settings_library_edit_error_missing_target());
    }

    if (quickAddTarget === 'manufacturer') {
      const result = await commands.updateManufacturer({
        id: editingRow.id,
        name: values.name.trim(),
        websiteUrl: values.websiteUrl.trim() || null,
        countryCode: values.countryCode.trim().toUpperCase() || null
      });

      if (result.status === 'ok') {
        return result.data;
      }

      throw new Error(getCommandErrorMessage(result.error));
    }

    const payload = {
      id: editingRow.id,
      name: values.name.trim(),
      sellerType: 'SHOP' as const,
      email: null,
      phone: null,
      websiteUrl: values.websiteUrl.trim() || null,
      streetAddress: null,
      extendedAddress: null,
      city: null,
      stateRegion: null,
      postalCode: null,
      countryCode: values.countryCode.trim().toUpperCase() || null,
      createdAt: null
    };

    const result =
      quickAddTarget === 'buyer'
        ? await commands.updateBuyer(payload)
        : await commands.updateSeller(payload);

    if (result.status === 'ok') {
      return result.data;
    }

    throw new Error(getCommandErrorMessage(result.error));
  }

  onMount(() => {
    void loadLibraryRows();
  });
</script>

<section
  class="rounded-sm border border-border bg-card p-6"
  aria-label={m.settings_library_title()}
>
  <header class="mb-4 flex items-start justify-between gap-3">
    <div>
      <h2 class="text-lg font-semibold">{m.settings_library_title()}</h2>
      <p class="text-sm text-muted-foreground">{m.settings_library_subtitle()}</p>
    </div>
    <button
      type="button"
      class="rounded-sm border border-border bg-background/50 px-3 py-1 text-sm"
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
      onEdit={openQuickEdit}
      onDelete={openDeleteDialog}
      onMerge={openMergeDialog}
      rows={paginatedRows}
      totalItems={totalItems}
      totalPages={totalPages}
      pageStart={pageStart}
      pageEnd={pageEnd}
      currentPage={currentPage}
      onPageChange={(page) => {
        currentPage = page;
      }}
    />
  </div>
</section>

<MergeEntityDialog
  open={pendingMergeSourceRow !== null}
  sourceName={pendingMergeSourceRow?.name ?? ''}
  sourceId={pendingMergeSourceRow?.id ?? ''}
  options={mergeCandidates}
  targetId={pendingMergeTargetId}
  {merging}
  onTargetChange={(value) => {
    pendingMergeTargetId = value;
  }}
  onOpenChange={(next) => {
    if (!next) {
      closeMergeDialog();
    }
  }}
  onConfirm={handleMergeConfirm}
/>

<DeleteEntityDialog
  open={pendingDeleteRow !== null}
  entityName={pendingDeleteRow?.name ?? ''}
  linkedCount={pendingDeleteRow?.usageCount ?? 0}
  {deleting}
  onOpenChange={(next) => {
    if (!next) {
      closeDeleteDialog();
    }
  }}
  onConfirm={handleDeleteConfirm}
/>

<QuickAddShell open={quickAddTarget !== null} title={quickAddTitle} onDismiss={closeQuickAdd}>
  {#if quickAddTarget}
    <QuickAddEntityForm
      target={quickAddTarget}
      mode="FULL"
      existingNames={quickAddNames}
      initialValues={quickFormInitialValues}
      submitLabel={quickSubmitLabel}
      onSubmit={quickFormIntent === 'edit' ? handleQuickSubmit : undefined}
      onSuccess={handleQuickAddSuccess}
      onCancel={closeQuickAdd}
      onError={handleQuickAddError}
    />
  {/if}

  {#snippet footer()}
    <div class="text-xs text-muted-foreground">{m.quick_add_footer_hint()}</div>
  {/snippet}
</QuickAddShell>
