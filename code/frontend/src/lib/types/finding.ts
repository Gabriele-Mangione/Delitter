export type Finding = {
    id: string;
    lat: number;
    lng: number;
    file: number[];
    date: string;
    entries: FindingEntry[];
    // Fields for flattened view (used in map/insights)
    weight?: number | null;
    category?: string;
    material?: string;
    brand?: string | null;
};

export type FindingEntry = {
    category: string;
    material: string;
    weight_g_estimate?: number;
    brand?: string;
    confidence: number;
}