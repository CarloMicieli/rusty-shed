<script lang="ts">
  import { ClipboardList } from 'lucide-svelte';
  import { getMaintenanceState } from '../MaintenanceState.svelte';
  import type { MaintenanceType } from '$lib/bindings';
  import { toaster } from '$lib/toaster';
  import * as m from '$lib/paraglide/messages.js';
  import MaintenanceEventHeader from './MaintenanceEventHeader.svelte';
  import MaintenanceEventFormFields from './MaintenanceEventFormFields.svelte';
  import MaintenanceEventActions from './MaintenanceEventActions.svelte';

  let { open, onClose } = $props<{ open: boolean; onClose: () => void }>();

  const maintenanceState = getMaintenanceState();
  const cards = $derived(maintenanceState.cards);

  let selectedCardId = $state<string | null>(null);
  let datePerformed = $state<string>(new Date().toISOString().split('T')[0]);
  let maintenanceType = $state<string | null>(null);
  let notes = $state<string>('');
  let isSubmitting = $state(false);
  let error = $state<string | null>(null);

  const isFormValid = $derived(selectedCardId !== null && datePerformed !== '');

  // Maintenance type options
  const maintenanceTypes: Array<{ value: MaintenanceType; label: string }> = [
    { value: 'WHEEL_CLEANING', label: m.maintenance_type_wheel_cleaning() },
    { value: 'TRACK_CLEANING', label: m.maintenance_type_track_cleaning() },
    { value: 'CONTACT_CLEANING', label: m.maintenance_type_contact_cleaning() },
    { value: 'LUBRICATION', label: m.maintenance_type_lubrication() },
    { value: 'GENERAL_INSPECTION', label: m.maintenance_type_inspection() },
    { value: 'DETAIL_REPAIR', label: m.maintenance_type_repair() }
  ];

  // Prevent background scrolling when modal is open
  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });

  async function handleSubmit() {
    if (!isFormValid || !selectedCardId || !datePerformed) return;

    isSubmitting = true;
    error = null;

    try {
      await maintenanceState.addMaintenanceEvent({
        id: crypto.randomUUID(),
        maintenanceCardId: selectedCardId,
        datePerformed,
        maintenanceType,
        notes: notes.trim() || null
      });

      toaster.success({
        id: crypto.randomUUID(),
        title: m.maintenance_add_event_success(),
        duration: 3000
      });

      resetForm();
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : m.maintenance_add_event_error();
      console.error('[AddMaintenanceEventModal] Submit error:', err);
    } finally {
      isSubmitting = false;
    }
  }

  function resetForm() {
    selectedCardId = null;
    datePerformed = new Date().toISOString().split('T')[0];
    maintenanceType = null;
    notes = '';
    error = null;
  }

  function handleClose() {
    resetForm();
    onClose();
  }
</script>

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 p-4 backdrop-blur-md"
    aria-modal="true"
    role="dialog"
  >
    <!-- Modal container -->
    <div
      class="relative w-full max-w-lg overflow-hidden rounded-2xl border border-amber-500/20 bg-[#0c0c0c] shadow-2xl"
    >
      <!-- Background context icon -->
      <div class="pointer-events-none absolute -top-6 -right-6 text-white/5">
        <ClipboardList size={120} strokeWidth={1} />
      </div>

      <!-- Header -->
      <MaintenanceEventHeader onClose={handleClose} />

      <!-- Form -->
      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="space-y-5 p-6"
      >
        <MaintenanceEventFormFields
          {cards}
          {selectedCardId}
          {datePerformed}
          {maintenanceType}
          {notes}
          {error}
          {isFormValid}
          {maintenanceTypes}
          onCardChange={(cardId) => (selectedCardId = cardId)}
          onDateChange={(date) => (datePerformed = date)}
          onTypeChange={(type) => (maintenanceType = type)}
          onNotesChange={(newNotes) => (notes = newNotes)}
        />

        <MaintenanceEventActions
          onCancel={handleClose}
          onSubmit={handleSubmit}
          {isFormValid}
          {isSubmitting}
        />
      </form>
    </div>
  </div>
{/if}
