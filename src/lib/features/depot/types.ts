export type Locomotive = {
  id: string;
  group: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  dccAddress?: number | null;
};

export type TrainSet = {
  id: string;
  group: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  dccAddress?: number | null;
};

export type CarCategory = 'passenger' | 'freight';

export type Car = {
  id: string;
  type: string;
  roadNumber: string | null;
  railwayCompany: string | null;
  livery: string | null;
  category: CarCategory;
  serviceLevel?: string | null;
  dccAddress?: number | null;
};
