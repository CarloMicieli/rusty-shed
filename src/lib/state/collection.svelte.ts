import { commands } from '$lib/bindings';
import type { CollectionView, CollectionItemView } from '$lib/bindings';

class CollectionStore {
  items = $state<CollectionItemView[]>([]);
  collection = $state<CollectionView | null>(null);
  loading = $state(false);

  getItemById(id: string): CollectionItemView | undefined {
    return this.items.find((item) => item.id === id);
  }

  async fetch(): Promise<void> {
    if (this.items.length > 0) return; // cache hit
    await this.refresh();
  }

  async refresh(): Promise<void> {
    if (this.loading) return;
    this.loading = true;
    try {
      const collection = await commands.getCollection();
      this.collection = collection;
      this.items = collection.items;
    } finally {
      this.loading = false;
    }
  }
}

export const collectionStore = new CollectionStore();
