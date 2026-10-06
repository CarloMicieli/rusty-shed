import { browser, expect } from '@wdio/globals';

describe('Rusty Shed desktop app', () => {
  it('opens the main window', async () => {
    await browser.waitUntil(
      async () => {
        const [isReady, loaderVisible] = await browser.execute(() => [
          document.readyState === 'complete',
          Boolean(document.getElementById('app-loading'))
        ]);

        return isReady && !loaderVisible;
      },
      {
        timeout: 120_000,
        interval: 1_000,
        timeoutMsg: 'Timed out waiting for the Tauri app shell to load'
      }
    );

    const url = await browser.getUrl();
    expect(url).toContain('tauri://localhost');
  });
});
