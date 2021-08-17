export interface File {
    path: string;
    size: number;
    children?: File[];
}

export function init(): void;
export function scan(dir: string): void;
export function query(dir: string): File | null;
