import * as Electron from 'electron'
import * as Scan from "./scan"

export function getDirectory() {
    let result = Electron.remote.dialog.showOpenDialogSync({properties: ["openDirectory"]})
    Scan.scan(result[0]).then(console.log)
}

export function convertData() {
    Scan.scan("result").then((data) => {
        console.log(data);
    });
}

export function strConvert(size:number) {
    if (size < 1024) {
        size = Math.ceil(size);
        return `${size} bytes`;
    }

    else if (size < 1024**2) {
        size = size/1024;
        size = Math.ceil(size);
        return `${size} kilobytes`;
    }

    else if (size < 1024**3) {
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size} megabytes`;
    }

    else if (size < 1024**4) {
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size} gigabytes`;
    }

    else if (size < 1024**5) {
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = size/1024;
        size = Math.ceil(size);
        return `${size} terabytes`;
    }
}