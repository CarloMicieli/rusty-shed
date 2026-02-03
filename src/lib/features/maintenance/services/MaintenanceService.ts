/**
 * Maintenance Service - Wrapper for Tauri maintenance commands.
 *
 * This service provides:
 * - Dashboard data fetching (top 10 due maintenance cards)
 * - Maintenance card creation
 * - Maintenance event logging
 */

import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import type {
  MaintenanceCardView,
  MaintenanceCardId,
  OwnedRollingStockId,
  AddMaintenanceArgs
} from '$lib/bindings';

/**
 * MaintenanceService encapsulates Tauri command invocations for maintenance operations.
 */
export class MaintenanceService {
  /**
   * Fetch the maintenance dashboard (due and overdue cards).
   *
   * @returns Promise resolving to an array of MaintenanceCardView
   */
  async getDashboard(): Promise<MaintenanceCardView[]> {
    const result = await safeInvoke<MaintenanceCardView[]>('get_maintenance_dashboard');
    if (!result.ok) throw new Error(result.error.message);
    return result.data;
  }

  /**
   * Create a new maintenance card for owned rolling stock.
   *
   * @param ownedRollingStockId - The ID of the owned rolling stock
   * @returns Promise resolving to the new MaintenanceCardId
   */
  async createCard(ownedRollingStockId: OwnedRollingStockId): Promise<MaintenanceCardId> {
    const result = await safeInvoke<MaintenanceCardId>('add_maintenance_card', {
      ownedRollingStockId
    });
    if (!result.ok) throw new Error(result.error.message);
    return result.data;
  }

  /**
   * Add a maintenance event to a card and update the next due date.
   *
   * @param args - The maintenance event arguments
   * @returns Promise resolving when the event is logged
   */
  async addEvent(args: AddMaintenanceArgs): Promise<void> {
    const result = await safeInvoke<null>('add_maintenance_event', { input: args });
    if (!result.ok) throw new Error(result.error.message);
  }
}
