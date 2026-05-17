import type { Manufacturer, Seller } from '$lib/bindings';

export type QuickAddTarget = 'manufacturer' | 'seller' | 'buyer';
export type QuickAddMode = 'QUICK' | 'FULL';

export type QuickAddEntity = Manufacturer | Seller;
