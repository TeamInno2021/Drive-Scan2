import { Directory } from "dslib";
import { ipcRenderer } from "electron";

export async function scan(dir: string): Promise<void> {
    ipcRenderer.send("scan", dir);

    return new Promise((resolve) => {
        ipcRenderer.once("scan-complete", () => {
            resolve();
        });
    });
}

export function query(dir: string): Directory | undefined {
    return ipcRenderer.sendSync("query", dir);
}
