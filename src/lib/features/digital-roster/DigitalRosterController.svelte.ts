import { commands } from '$lib/bindings';
import { DigitalRosterState } from './DigitalRosterState.svelte';
import { toaster } from '$lib/toaster';

/**
 * Controller for the Digital Roster feature
 * Handles all business logic and command invocations
 */
export class DigitalRosterController {
  constructor(public state: DigitalRosterState) {}

  private commands = commands;

  /**
   * Load the digital summary statistics
   */
  async loadSummary(): Promise<void> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const result = await commands.getDigitalSummary();
      this.state.setSummary(result);
    } catch (error) {
      console.error('Error loading summary:', error);
      toaster.error({ title: 'An error occurred while loading the summary', duration: 5000 });
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
      this.state.setRollingStocks(result);
    } catch (error) {
      console.error('Error loading rolling stocks:', error);
      toaster.error({ title: 'An error occurred while loading rolling stocks', duration: 5000 });
    } finally {
      this.state.setLoading(false);
    }
  }

  /**
   * Load both summary and rolling stocks
   */
  async loadAll(): Promise<void> {
    try {
      this.state.setLoading(true);
      this.state.clearError();

      const [summaryResult, rollingStocksResult] = await Promise.all([
        this.commands.getDigitalSummary(),
        this.commands.getDigitalRollingStocks()
      ]);
      this.state.setSummary(summaryResult);
      this.state.setRollingStocks(rollingStocksResult);
    } catch (error) {
      console.error('Error loading data:', error);
      toaster.error({
        title: 'An error occurred while loading data',
        description: error instanceof Error ? error.message : String(error),
        duration: 5000
      });
    } finally {
      this.state.setLoading(false);
    }
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

      await commands.changeDccAddress({ id, newDccAddress: newAddress });
      await this.loadRollingStocks();
      toaster.success({ title: 'DCC address updated successfully', duration: 2000 });
      return true;
    } catch (error) {
      console.error('Error changing DCC address:', error);
      toaster.error({ title: 'An error occurred while changing the DCC address', duration: 5000 });
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
      return {
        isDuplicate: result.is_duplicate,
        existingId: result.existing_rolling_stock_id ?? undefined
      };
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

      await commands.newDigitalRollingStock({
        ownedRollingStockId,
        decoderId,
        dccAddress
      });
      await this.loadAll();
      toaster.success({ title: 'Decoder installed successfully', duration: 2000 });
      return true;
    } catch (error) {
      console.error('Error installing decoder:', error);
      toaster.error({ title: 'An error occurred while installing the decoder', duration: 5000 });
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

      await commands.changeDecoder({
        id: digitalRollingStockId,
        decoderId: newDecoderId
      });
      await this.loadAll();
      toaster.success({ title: 'Decoder replaced successfully', duration: 2000 });
      return true;
    } catch (error) {
      console.error('Error replacing decoder:', error);
      toaster.error({ title: 'An error occurred while replacing the decoder', duration: 5000 });
      return false;
    } finally {
      this.state.setLoading(false);
    }
  }
}
