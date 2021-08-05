import { Directory } from "dslib";
import { ipcRenderer } from "electron";

export function scan(dir: string): void {
    ipcRenderer.sendSync("scan", dir);
    return;

    // return new Promise((resolve) => {
    //     ipcRenderer.once("scan-complete", () => {
    //         resolve();
    //     });
    // });
}

export function query(dir: string): Directory | undefined {
    return ipcRenderer.sendSync("query", dir);
}
