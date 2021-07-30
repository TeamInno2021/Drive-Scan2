export interface File {
    path: string;
    size: number;
    directory: boolean;
}

export interface Directory {
    path: string;
    size: number;
    files: File[];
}

export function init(): void;
export function scan(dir: string): void;
export function query(dir: string): Directory | undefined;
