import dslib from "dslib";
import { app, BrowserWindow, ipcMain } from "electron";

//dslib.init();

ipcMain.on("scan", (event, dir) => {
    dslib.scan(dir);
    event.returnValue = "";
    // event.sender.send("scan-complete");
});

ipcMain.on("query", (event, dir) => {
    event.returnValue = dslib.query(dir);
});

app.whenReady().then(() => {
    const win = new BrowserWindow({
        width: 1920,
        height: 1080,
        webPreferences: {
            enableRemoteModule: true,
            nodeIntegration: true,
            contextIsolation: false,
        },
    });

    // win.webContents.openDevTools();
    win.loadFile("index.html");
});
