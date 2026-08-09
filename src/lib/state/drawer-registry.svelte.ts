export type DrawerDismissMode = 'gesture' | 'button' | 'back';
export type DrawerLayerKind = 'parent' | 'child';

export interface DrawerLayer {
  id: string;
  kind: DrawerLayerKind;
  payload?: Record<string, unknown> | null;
}

export interface DrawerRegistry {
  readonly stack: DrawerLayer[];
  readonly depth: number;
  readonly activeLayerId: string | null;
  readonly sourceRoute: string;
  setSourceRoute: (route: string) => void;
  openParent: (id: string, payload?: Record<string, unknown> | null) => boolean;
  openChild: (
    id: string,
    options?: { payload?: Record<string, unknown> | null; replaceIfFull?: boolean }
  ) => boolean;
  closeTop: (mode: DrawerDismissMode) => DrawerLayer | null;
  closeById: (id: string, mode: DrawerDismissMode) => boolean;
  clear: (mode: DrawerDismissMode) => void;
}

const MAX_DRAWER_DEPTH = 2;

export function createDrawerRegistry(): DrawerRegistry {
  let stack = $state<DrawerLayer[]>([]);
  let sourceRoute = $state('');

  const activeLayerId = $derived.by(() => {
    const top = stack.at(-1);
    return top?.id ?? null;
  });

  const setSourceRoute = (route: string) => {
    sourceRoute = route;
  };

  const openParent = (id: string, payload: Record<string, unknown> | null = null): boolean => {
    stack = [{ id, kind: 'parent', payload }];
    return true;
  };

  const openChild = (
    id: string,
    options: { payload?: Record<string, unknown> | null; replaceIfFull?: boolean } = {}
  ): boolean => {
    if (stack.length === 0) {
      return false;
    }

    const child: DrawerLayer = {
      id,
      kind: 'child',
      payload: options.payload ?? null
    };

    if (stack.length < MAX_DRAWER_DEPTH) {
      stack = [...stack, child];
      return true;
    }

    if (options.replaceIfFull) {
      stack = [stack[0], child];
      return true;
    }

    return false;
  };

  const closeTop = (_mode: DrawerDismissMode): DrawerLayer | null => {
    if (stack.length === 0) {
      return null;
    }

    const removed = stack.at(-1) ?? null;
    stack = stack.slice(0, -1);
    return removed;
  };

  const closeById = (id: string, mode: DrawerDismissMode): boolean => {
    const top = stack.at(-1);
    if (!top || top.id !== id) {
      return false;
    }

    return closeTop(mode) != null;
  };

  const clear = (_mode: DrawerDismissMode) => {
    stack = [];
  };

  return {
    get stack() {
      return stack;
    },
    get depth() {
      return stack.length;
    },
    get activeLayerId() {
      return activeLayerId;
    },
    get sourceRoute() {
      return sourceRoute;
    },
    setSourceRoute,
    openParent,
    openChild,
    closeTop,
    closeById,
    clear
  };
}

export const drawerRegistry = createDrawerRegistry();
