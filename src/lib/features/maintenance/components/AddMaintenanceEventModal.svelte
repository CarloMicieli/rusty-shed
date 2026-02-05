<script lang="ts">
  import { X } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { Button, Input, Textarea } from '$lib/components';
  import { getMaintenanceState } from '../MaintenanceState.svelte';
  import MaintenanceCardSelector from './MaintenanceCardSelector.svelte';
  import type { MaintenanceType } from '$lib/bindings';
  import { toaster } from '$lib/toaster';

  let { open, onClose } = $props<{ open: boolean; onClose: () => void }>();

  const maintenanceState = getMaintenanceState();

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

      // Show success toast
      toaster.success({
        id: crypto.randomUUID(),
        title: m.maintenance_add_event_success(),
        duration: 3000
      });

      // Reset and close
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
  <div class="modal-backdrop bg-surface-900/80 fixed inset-0 z-50 flex items-center justify-center">
    <div class="card m-4 w-full max-w-md space-y-4 p-6">
      <!-- Header -->
      <div class="flex items-center justify-between">
        <h3 class="h3">{m.maintenance_add_event_title()}</h3>
        <Button type="button" variant="ghost" size="icon" onclick={handleClose}>
          <X size={20} />
        </Button>
      </div>

      <!-- Form -->
      <form
        onsubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
        class="space-y-4"
      >
        <MaintenanceCardSelector bind:selectedId={selectedCardId} />

        <div class="space-y-2">
          <label for="date-performed" class="label">
            {m.maintenance_add_event_date_label()}
          </label>
          <Input
            id="date-performed"
            type="date"
            bind:value={datePerformed}
            max={new Date().toISOString().split('T')[0]}
          />
        </div>

        <div class="space-y-2">
          <label for="maintenance-type" class="label">
            {m.maintenance_add_event_type_label()}
          </label>
          <select id="maintenance-type" class="select" bind:value={maintenanceType}>
            <option value={null}>{m.maintenance_add_event_type_placeholder()}</option>
            {#each maintenanceTypes as type (type.value)}
              <option value={type.value}>{type.label}</option>
            {/each}
          </select>
        </div>

        <div class="space-y-2">
          <label for="notes" class="label">
            {m.maintenance_add_event_notes_label()}
          </label>
          <Textarea
            id="notes"
            rows={3}
            placeholder={m.maintenance_add_event_notes_placeholder()}
            bind:value={notes}
          />
        </div>

        {#if error}
          <div class="alert variant-filled-error">
            <p class="text-sm">{error}</p>
          </div>
        {/if}

        {#if !selectedCardId}
          <p class="text-surface-500 text-sm">{m.maintenance_add_event_validation_card()}</p>
        {/if}

        {#if !datePerformed}
          <p class="text-surface-500 text-sm">{m.maintenance_add_event_validation_date()}</p>
        {/if}

        <!-- Actions -->
        <div class="flex justify-end gap-3">
          <Button type="button" variant="ghost" onclick={handleClose}>
            {m.maintenance_add_event_cancel()}
          </Button>
          <Button type="submit" variant="default" disabled={!isFormValid || isSubmitting}>
            {isSubmitting ? m.app_loading() : m.maintenance_add_event_submit()}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
