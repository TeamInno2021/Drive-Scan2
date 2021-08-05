export interface File {
    path: string;
    size: number;
    children: File[] | undefined;
}

export function init(): void;
export function scan(dir: string): void;
export function query(dir: string): File | undefined;
