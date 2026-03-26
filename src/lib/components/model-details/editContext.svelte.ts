import { setContext, getContext } from 'svelte';

const KEY = Symbol('rolling-stock-edit');

class RollingStockEditContext {
  activeEditId = $state<string | null>(null);

  setActive(id: string) {
    this.activeEditId = id;
  }

  clearActive() {
    this.activeEditId = null;
  }
}

export function setEditContext(): RollingStockEditContext {
  const ctx = new RollingStockEditContext();
  setContext(KEY, ctx);
  return ctx;
}

/** Returns the nearest edit context, or an isolated fallback when used without a provider. */
export function getEditContext(): RollingStockEditContext {
  return getContext<RollingStockEditContext>(KEY) ?? new RollingStockEditContext();
}
