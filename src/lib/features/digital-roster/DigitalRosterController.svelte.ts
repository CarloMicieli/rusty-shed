import { commands } from '$lib/bindings';
import { DigitalRosterState } from './DigitalRosterState.svelte';

/**
 * Controller for the Digital Roster feature
 * Handles all business logic and command invocations
 */
export class DigitalRosterController {
  constructor(public state: DigitalRosterState) {}

  /**
   * Load the digital summary statistics
   */
  async loadSummary(): Promise<void> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.getDigitalSummary();

      if (result.status === 'ok') {
        this.state.setSummary(result.data);
      } else {
        this.state.setError('Failed to load digital summary');
      }
    } catch (error) {
      console.error('Error loading summary:', error);
      this.state.setError('An error occurred while loading the summary');
    } finally {
      this.state.setLoading(false);
    }
  }

  /**
   * Load all digital rolling stocks
   */
  async loadRollingStocks(): Promise<void> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.getDigitalRollingStocks();

      if (result.status === 'ok') {
        this.state.setRollingStocks(result.data);
      } else {
        this.state.setError('Failed to load digital rolling stocks');
      }
    } catch (error) {
      console.error('Error loading rolling stocks:', error);
      this.state.setError('An error occurred while loading rolling stocks');
    } finally {
      this.state.setLoading(false);
    }
  }

  /**
   * Load both summary and rolling stocks
   */
  async loadAll(): Promise<void> {
    await Promise.all([this.loadSummary(), this.loadRollingStocks()]);
  }

  /**
   * Update the filter text
   */
  updateFilter(text: string): void {
    this.state.setFilterText(text);
  }

  /**
   * Clear the current filter
   */
  clearFilter(): void {
    this.state.setFilterText('');
  }

  /**
   * Change the DCC address of a rolling stock
   */
  async changeDccAddress(id: string, newAddress: number): Promise<boolean> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.changeDccAddress({ id, newDccAddress: newAddress });

      if (result.status === 'ok') {
        // Reload the list to reflect changes
        await this.loadRollingStocks();
        return true;
      } else {
        this.state.setError('Failed to change DCC address');
        return false;
      }
    } catch (error) {
      console.error('Error changing DCC address:', error);
      this.state.setError('An error occurred while changing the DCC address');
      return false;
    } finally {
      this.state.setLoading(false);
    }
  }

  /**
   * Check if a DCC address is duplicate
   */
  async checkDuplicateAddress(
    address: number,
    excludeId?: string | null
  ): Promise<{ isDuplicate: boolean; existingId?: string }> {
    try {
      const result = await commands.checkDccAddressDuplicate({
        dccAddress: address,
        excludeId: excludeId ?? null
      });

      if (result.status === 'ok') {
        return {
          isDuplicate: result.data.is_duplicate,
          existingId: result.data.existing_rolling_stock_id ?? undefined
        };
      }
      return { isDuplicate: false };
    } catch (error) {
      console.error('Error checking duplicate address:', error);
      return { isDuplicate: false };
    }
  }

  /**
   * Install a new decoder on a rolling stock
   */
  async installDecoder(
    ownedRollingStockId: string,
    decoderId: string,
    dccAddress: number
  ): Promise<boolean> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.newDigitalRollingStock({
        ownedRollingStockId,
        decoderId,
        dccAddress
      });

      if (result.status === 'ok') {
        // Reload both summary and rolling stocks
        await this.loadAll();
        return true;
      } else {
        this.state.setError('Failed to install decoder');
        return false;
      }
    } catch (error) {
      console.error('Error installing decoder:', error);
      this.state.setError('An error occurred while installing the decoder');
      return false;
    } finally {
      this.state.setLoading(false);
    }
  }

  /**
   * Replace an existing decoder on a rolling stock
   */
  async replaceDecoder(digitalRollingStockId: string, newDecoderId: string): Promise<boolean> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.changeDecoder({
        id: digitalRollingStockId,
        decoderId: newDecoderId
      });

      if (result.status === 'ok') {
        // Reload both summary and rolling stocks
        await this.loadAll();
        return true;
      } else {
        this.state.setError('Failed to replace decoder');
        return false;
      }
    } catch (error) {
      console.error('Error replacing decoder:', error);
      this.state.setError('An error occurred while replacing the decoder');
      return false;
    } finally {
      this.state.setLoading(false);
    }
  }
}
