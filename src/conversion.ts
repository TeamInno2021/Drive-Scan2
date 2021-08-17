export function strConvert(size:number) {
    if (size < 1000) {
        size = Math.ceil(size);
        return `${size}B`;
    }

    else if (size < 1000**2) {
        size = size/1000;
        size = Math.ceil(size);
        return `${size}KB`;
    }

    else if (size < 1000**3) {
        size = size/1000;
        size = size/1000;
        size = Math.ceil(size);
        return `${size}MB`;
    }

    else if (size < 1000**4) {
        size = size/1000;
        size = size/1000;
        size = size/1000;
        size = Math.ceil(size);
        return `${size}GB`;
    }

    else if (size < 1000**5) {
        size = size/1000;
        size = size/1000;
        size = size/1000;
        size = size/1000;
        size = Math.ceil(size);
        return `${size}TB`;
    }
} 