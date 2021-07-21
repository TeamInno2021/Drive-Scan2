import { ipcRenderer } from "electron";

export async function scan(dir: string): Promise<any> {
    ipcRenderer.send("scan", dir);

    return new Promise((resolve, _reject) => {
        ipcRenderer.once("scan-data", (_, data) => {
            resolve(data);
        });
    });
} 
