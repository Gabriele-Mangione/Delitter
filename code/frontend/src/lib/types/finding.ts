export type Finding = {
    id: string;
    lat: number;
    lng: number;
    file: number[];
    date: string;
    entries: FindingEntry[]
};

export type FindingEntry = {
    category?: string;
    material?: string;
    weight?: number;
    brand?: string;
}