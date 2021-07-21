import { app, BrowserWindow } from "electron";

app.whenReady().then(() => {
    const win = new BrowserWindow({
        width: 1920,
        height: 1080,
    });

    win.loadFile("index.html");
});
