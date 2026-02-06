import { render } from '@testing-library/svelte';
import { describe, expect, it } from 'vitest';
import ConnectivityIndicator from '$lib/features/cloud-backup/components/ConnectivityIndicator.svelte';
import { connectivityStore } from '$lib/features/cloud-backup/stores/connectivity';
import * as m from '$lib/paraglide/messages.js';

describe('ConnectivityIndicator', () => {
  it('renders checking state by default', () => {
    connectivityStore.set(null);

    const { getByText } = render(ConnectivityIndicator);

    expect(getByText(m.cloud_backup_connectivity_checking())).toBeTruthy();
  });

  it('renders online status', () => {
    connectivityStore.set({ isOnline: true, checkedAt: new Date().toISOString() });

    const { container, getByText } = render(ConnectivityIndicator);

    expect(getByText(m.cloud_backup_connectivity_online())).toBeTruthy();

    const lastCheckedPrefix = m.cloud_backup_connectivity_last_checked({ timestamp: '' }).trim();
    expect(container.textContent).toContain(lastCheckedPrefix);
  });

  it('renders offline status', () => {
    connectivityStore.set({ isOnline: false, checkedAt: new Date().toISOString() });

    const { container, getByText } = render(ConnectivityIndicator);

    expect(getByText(m.cloud_backup_connectivity_offline())).toBeTruthy();

    const lastCheckedPrefix = m.cloud_backup_connectivity_last_checked({ timestamp: '' }).trim();
    expect(container.textContent).toContain(lastCheckedPrefix);
  });
});
