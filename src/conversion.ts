export function strConvert(size:number) {
    if (size < 1024) {
        size = Math.ceil(size);
        return `${size}B`;
    }

    else if (size < 1024**2) {
        size = size/1024;
        size = Math.ceil(size);
        return `${size}KiB`;
    }

    else if (size < 1024**3) {
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size}MiB`;
    }

    else if (size < 1024**4) {
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size}GiB`;
    }

    else if (size < 1024**5) {
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size}TiB`;
    }
} 