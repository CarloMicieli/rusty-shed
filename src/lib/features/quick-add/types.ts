import type { Manufacturer, Seller } from '$lib/bindings';

export type QuickAddTarget = 'manufacturer' | 'seller' | 'buyer';

export type QuickAddEntity = Manufacturer | Seller;
