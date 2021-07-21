import { scan } from "dslib";
import { app, BrowserWindow, ipcMain } from "electron";

ipcMain.on("scan", (event, dir) => {
    event.sender.send("scan-data", scan(dir));
});

app.whenReady().then(() => {
    const win = new BrowserWindow({
        width: 1920,
        height: 1080,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
        },
    });

    win.webContents.openDevTools();
    win.loadFile("index.html");
});
