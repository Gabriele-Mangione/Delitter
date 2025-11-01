export type Finding = {
    id: string;
    lat: number;
    lng: number;
    weight: number | null;
    category: string;
    material: string;
    file: number[];
    date: string;
    brand?: string | null;
};