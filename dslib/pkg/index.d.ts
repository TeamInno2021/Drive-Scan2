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
export function scan(dir: string): Promise<void>;
export function query(dir: string): Directory;
