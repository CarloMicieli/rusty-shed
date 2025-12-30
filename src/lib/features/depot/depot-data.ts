export interface Locomotive {
  id: string;
  group: string;
  roadNumber: string;
  railwayCompany: string;
  livery: string;
  dccAddress: string;
}

export interface TrainSet {
  id: string;
  group: string;
  roadNumber: string;
  railwayCompany: string;
  livery: string;
  dccAddress: string;
}

export type CarCategory = 'passenger' | 'freight';

export interface Car {
  id: string;
  type: string;
  roadNumber: string;
  railwayCompany: string;
  livery: string;
  category: CarCategory;
  serviceLevel?: string;
  dccAddress?: string;
}

export const locomotives: Locomotive[] = [
  {
    id: 'loco-103',
    group: 'DB BR 103',
    roadNumber: '103 184-8',
    railwayCompany: 'DB AG',
    livery: 'Orientrot/Beige',
    dccAddress: '103'
  },
  {
    id: 'loco-444',
    group: 'FS E.444R',
    roadNumber: 'E.444 098',
    railwayCompany: 'Trenitalia',
    livery: 'XMPR',
    dccAddress: '444'
  },
  {
    id: 'loco-26000',
    group: 'SNCF BB 26000',
    roadNumber: '26021',
    railwayCompany: 'SNCF',
    livery: 'En Voyage',
    dccAddress: '26'
  },
  {
    id: 'loco-1216',
    group: 'OeBB 1216',
    roadNumber: '1216 025',
    railwayCompany: 'OeBB',
    livery: 'Nightjet',
    dccAddress: '1216'
  }
];

export const trains: TrainSet[] = [
  {
    id: 'emu-ice3',
    group: 'DB ICE 3',
    roadNumber: '403 554-7',
    railwayCompany: 'DB Fernverkehr',
    livery: 'ICE White/Red',
    dccAddress: '403'
  },
  {
    id: 'emu-etr500',
    group: 'FS ETR 500',
    roadNumber: 'ETR 500-32',
    railwayCompany: 'Trenitalia',
    livery: 'Frecciarossa',
    dccAddress: '500'
  },
  {
    id: 'emu-rabe503',
    group: 'SBB RABe 503',
    roadNumber: '503 018-3',
    railwayCompany: 'SBB',
    livery: 'Giruno',
    dccAddress: '503'
  },
  {
    id: 'railcar-bm73',
    group: 'NSB BM73',
    roadNumber: '73-41',
    railwayCompany: 'Vy',
    livery: 'Burgundy/Silver',
    dccAddress: '73'
  }
];

export const cars: Car[] = [
  {
    id: 'car-uic-1st',
    type: 'UIC-Z 1st Class',
    roadNumber: '61 80 19-90 123-4',
    railwayCompany: 'DB AG',
    livery: 'ICE Red/White',
    category: 'passenger',
    serviceLevel: '1st'
  },
  {
    id: 'car-uic-2nd',
    type: 'UIC-Z 2nd Class',
    roadNumber: '61 80 21-90 456-1',
    railwayCompany: 'DB AG',
    livery: 'ICE Red/White',
    category: 'passenger',
    serviceLevel: '2nd'
  },
  {
    id: 'car-uic-sleeper',
    type: 'UIC-X Sleeper',
    roadNumber: '61 83 71-70 012-3',
    railwayCompany: 'Trenitalia',
    livery: 'Notte',
    category: 'passenger',
    serviceLevel: 'Sleeper'
  },
  {
    id: 'car-sgns-container',
    type: 'Sgns Container Wagon',
    roadNumber: '31 80 4551 123-4',
    railwayCompany: 'DB Cargo',
    livery: 'Red/Gray',
    category: 'freight'
  },
  {
    id: 'car-zacns',
    type: 'Zacns Tank Car',
    roadNumber: '33 80 7920 012-7',
    railwayCompany: 'VTG',
    livery: 'Blue/Gray',
    category: 'freight'
  },
  {
    id: 'car-metropolitan-restaurant',
    type: 'Metropolitan Restaurant',
    roadNumber: '61 80 88-94 001-2',
    railwayCompany: 'DB AG',
    livery: 'Metropolitan Gray',
    category: 'passenger',
    serviceLevel: 'Restaurant'
  }
];

export const depotData = {
  locomotives,
  trains,
  cars
};
