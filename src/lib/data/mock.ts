export interface Stat {
    label: string;
    value: string;
    trend: 'up' | 'down' | 'neutral';
    trendValue: string;
}

export interface QuickAction {
    id: string;
    label: string;
    icon: string; // lucide icon name
}

export interface RecentItem {
    id: string;
    title: string;
    subtitle: string;
    imageUrl: string;
}

export interface DepotEntry {
    id: string;
    manufacturer: string;
    productCode: string;
    category: string;
    scale: string;
    railwayCompany: string;
    description: string;
}

export const stats: Stat[] = [
    { label: 'total_collection_value', value: '€ 124,500', trend: 'up', trendValue: '+12%' },
    { label: 'rolling_stocks', value: '42', trend: 'up', trendValue: '+3' },
    { label: 'maintenance_alerts', value: '3', trend: 'down', trendValue: '-1' }
];

export const quickActions: QuickAction[] = [
    { id: '1', label: 'add_railway_model', icon: 'Plus' },
    { id: '2', label: 'schedule_maintenance', icon: 'Wrench' },
    { id: '3', label: 'generate_report', icon: 'FileText' }
];

export const recentItems: RecentItem[] = [
    {
        id: '1',
        title: 'Sector 7 Update',
        subtitle: 'Infrastructure upgraded',
        imageUrl: 'https://images.unsplash.com/photo-1565008447742-97f6f38c985c?auto=format&fit=crop&w=400&q=80'
    },
    {
        id: '2',
        title: 'Locomotive 204',
        subtitle: 'Maintenance completed',
        imageUrl: 'https://images.unsplash.com/photo-1474487548417-781cb71495f3?auto=format&fit=crop&w=400&q=80'
    },
    {
        id: '3',
        title: 'Cargo Bay 3',
        subtitle: 'Inspection pending',
        imageUrl: 'https://images.unsplash.com/photo-1586528116311-ad8dd3c8310d?auto=format&fit=crop&w=400&q=80'
    }
];

export const depotData: DepotEntry[] = [
    {
        id: '1',
        manufacturer: 'Märklin',
        productCode: '37123',
        category: 'Locomotive',
        scale: 'H0',
        railwayCompany: 'DB',
        description: 'Class BR 103 Electric Locomotive'
    },
    {
        id: '2',
        manufacturer: 'Roco',
        productCode: '73928',
        category: 'Locomotive',
        scale: 'H0',
        railwayCompany: 'ÖBB',
        description: 'Vectron Class 1293 Electric Locomotive with Sound'
    },
    {
        id: '3',
        manufacturer: 'Fleischmann',
        productCode: '731195',
        category: 'Locomotive',
        scale: 'N',
        railwayCompany: 'SBB',
        description: 'Re 460 "Zugerland" Electric Locomotive'
    },
    {
        id: '4',
        manufacturer: 'PIKO',
        productCode: '58530',
        category: 'Freight Car',
        scale: 'H0',
        railwayCompany: 'DR',
        description: 'Eurofima Express Train Passenger Car 2nd Class'
    },
    {
        id: '5',
        manufacturer: 'Trix',
        productCode: '22055',
        category: 'Steam Loco',
        scale: 'H0',
        railwayCompany: 'K.Bay.Sts.B',
        description: 'Class Gt 2x4/4 Mallet Steam Locomotive'
    }
];
