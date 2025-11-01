  export type Finding = {
    id: string;
    lat: number;
    lng: number;
    weight: number | null;
    category: string;
    material: string;
    brand?: string | null;
  };