import * as Electron from 'electron'
import * as Scan from "./scan"

export function getDirectory() {
    let result = Electron.remote.dialog.showOpenDialogSync({properties: ["openDirectory"]})
    Scan.scan(result[0]).then(console.log)
}