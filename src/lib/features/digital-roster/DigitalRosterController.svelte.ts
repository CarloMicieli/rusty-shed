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

      if (result.status === 'ok') {
        this.state.setSummary(result.data);
      } else {
        toaster.error({ title: 'Failed to load digital summary', duration: 5000 });
      }
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

      if (result.status === 'ok') {
        this.state.setRollingStocks(result.data);
      } else {
        toaster.error({ title: 'Failed to load digital rolling stocks', duration: 5000 });
      }
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

      let hasError = false;

      if (summaryResult.status === 'ok') {
        this.state.setSummary(summaryResult.data);
      } else {
        console.error('Failed to load digital summary:', JSON.stringify(summaryResult, null, 2));
        hasError = true;
      }

      if (rollingStocksResult.status === 'ok') {
        this.state.setRollingStocks(rollingStocksResult.data);
      } else {
        console.error(
          'Failed to load digital rolling stocks:',
          JSON.stringify(rollingStocksResult, null, 2)
        );
        hasError = true;
      }

      // Only show error toast if we actually had a failure
      if (hasError) {
        toaster.error({
          title: 'Failed to load digital roster data',
          description: 'Please check the console for more details',
          duration: 5000
        });
      }
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

      const result = await commands.changeDccAddress({ id, newDccAddress: newAddress });

      if (result.status === 'ok') {
        // Reload the list to reflect changes
        await this.loadRollingStocks();
        toaster.success({ title: 'DCC address updated successfully', duration: 2000 });
        return true;
      } else {
        toaster.error({ title: 'Failed to change DCC address', duration: 5000 });
        return false;
      }
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
        toaster.success({ title: 'Decoder installed successfully', duration: 2000 });
        return true;
      } else {
        toaster.error({ title: 'Failed to install decoder', duration: 5000 });
        return false;
      }
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

      const result = await commands.changeDecoder({
        id: digitalRollingStockId,
        decoderId: newDecoderId
      });

      if (result.status === 'ok') {
        // Reload both summary and rolling stocks
        await this.loadAll();
        toaster.success({ title: 'Decoder replaced successfully', duration: 2000 });
        return true;
      } else {
        toaster.error({ title: 'Failed to replace decoder', duration: 5000 });
        return false;
      }
    } catch (error) {
      console.error('Error replacing decoder:', error);
      toaster.error({ title: 'An error occurred while replacing the decoder', duration: 5000 });
      return false;
    } finally {
      this.state.setLoading(false);
    }
  }
}
