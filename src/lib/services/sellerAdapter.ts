import type {
  CreateSellerPayload,
  UpdateSellerPayload,
  Seller,
  Address,
  SellerType
} from '$lib/bindings';

export type FormSeller = {
  id?: string;
  name: string;
  sellerType: SellerType;
  email?: string | null;
  phone?: string | null;
  websiteUrl?: string | null;
  streetAddress?: string | null;
  extendedAddress?: string | null;
  city?: string | null;
  stateRegion?: string | null;
  postalCode?: string | null;
  countryCode?: string | null;
  createdAt?: string | null;
  updatedAt?: string | null;
};

export function mapFormToCreatePayload(f: FormSeller): CreateSellerPayload {
  return {
    name: f.name,
    sellerType: f.sellerType,
    email: f.email ?? null,
    phone: f.phone ?? null,
    websiteUrl: f.websiteUrl ?? null,
    streetAddress: f.streetAddress ?? null,
    extendedAddress: f.extendedAddress ?? null,
    city: f.city ?? null,
    stateRegion: f.stateRegion ?? null,
    postalCode: f.postalCode ?? null,
    countryCode: f.countryCode ?? null
  };
}

export function mapFormToUpdatePayload(f: FormSeller): UpdateSellerPayload {
  return {
    id: f.id ?? '',
    name: f.name,
    sellerType: f.sellerType,
    email: f.email ?? null,
    phone: f.phone ?? null,
    websiteUrl: f.websiteUrl ?? null,
    streetAddress: f.streetAddress ?? null,
    extendedAddress: f.extendedAddress ?? null,
    city: f.city ?? null,
    stateRegion: f.stateRegion ?? null,
    postalCode: f.postalCode ?? null,
    countryCode: f.countryCode ?? null,
    createdAt: f.createdAt ?? null
  };
}

export function mapSellerToForm(s: Seller): FormSeller {
  const addr: Address | null = s.address ?? null;
  return {
    id: s.id,
    name: s.name,
    sellerType: s.sellerType,
    email: s.email ?? null,
    phone: s.phone ?? null,
    websiteUrl: s.websiteUrl ?? null,
    streetAddress: addr?.street_address ?? null,
    extendedAddress: addr?.extended_address ?? null,
    city: addr?.city ?? null,
    stateRegion: addr?.region ?? null,
    postalCode: addr?.postal_code ?? null,
    countryCode: addr?.country ?? null,
    createdAt: s.createdAt ?? null,
    updatedAt: s.updatedAt ?? null
  };
}

export default {
  mapFormToCreatePayload,
  mapFormToUpdatePayload,
  mapSellerToForm
};
