export type Locomotive = {
  id: string;
  railwayModelId: string;
  group: string;
  manufacturer: string;
  seriesCode: string;
  productCode: string;
  categoryLabel?: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  control: string | null;
  dccAddress: number | null;
};

export type TrainSet = {
  id: string;
  railwayModelId: string;
  group: string;
  manufacturer: string;
  seriesCode: string;
  productCode: string;
  categoryLabel?: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  control: string | null;
  dccAddress: number | null;
};

export type CarCategory = 'passenger' | 'freight';

export type Car = {
  id: string;
  railwayModelId: string;
  type: string;
  manufacturer: string;
  seriesCode: string;
  productCode: string;
  categoryLabel?: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  category: CarCategory;
  serviceLevel?: string | null;
  control: string | null;
  dccAddress: number | null;
};
