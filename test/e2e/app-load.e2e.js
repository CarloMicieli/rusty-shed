describe('Rusty Shed desktop app', () => {
  it('opens the main window', async () => {
    await browser.waitUntil(
      async () => {
        const title = await browser.getTitle();
        return title.includes('Rusty Shed');
      },
      {
        timeout: 120_000,
        interval: 1_000,
        timeoutMsg: 'Timed out waiting for the Tauri window to load'
      }
    );

    const title = await browser.getTitle();
    expect(title).toContain('Rusty Shed');
  });
});
