export type Locomotive = {
  id: string;
  group: string;
  manufacturer: string;
  seriesCode: string;
  productCode: string;
  categoryLabel?: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  control: string | null;
};

export type TrainSet = {
  id: string;
  group: string;
  manufacturer: string;
  seriesCode: string;
  productCode: string;
  categoryLabel?: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  control: string | null;
};

export type CarCategory = 'passenger' | 'freight';

export type Car = {
  id: string;
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
};
