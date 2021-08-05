import * as Scan from "./scan";
import * as Electron from "electron";

export function getDirectory() {
    let result = Electron.remote.dialog.showOpenDialogSync({properties: ["openDirectory"]})
    Scan.scan(result[0]).then(console.log)
}