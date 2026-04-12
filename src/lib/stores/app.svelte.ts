/**
 * Application Store - Runes-based singleton
 * Manages global application state (version, etc.)
 */

class AppStateClass {
  version = $state<string>('');

  setVersion(v: string): void {
    this.version = v;
  }
}

// Singleton instance
export const appState = new AppStateClass();

/**
 * Backwards-compat: Legacy function for migration path
 */
export function setAppVersion(v: string): void {
  appState.setVersion(v);
}
