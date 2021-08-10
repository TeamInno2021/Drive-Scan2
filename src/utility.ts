import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File } from "dslib";
import path from "path";

export function getDirectory() {
    let result = Electron.remote.dialog.showOpenDialogSync({
        properties: ["openDirectory"],
    });
    Scan.scan(result[0]);
    console.log(Scan.query(result[0]));
}

export interface PieFileSlice {
    path: string,   //Path of the file
    name: string,   //Name of the file
    size: string,   //Size of the file as a string with units
    perc: string,   //Percentage of the parent folder
}

export function pfsFromFileChildren(root: dslib.File): Array<PieFileSlice> {
    let pfsArray = new Array(root.children.length);
    root.children.forEach(function (file: dslib.File) {
        pfsArray.push({
            path: file.path,
            name: path.basename(file.path),
            size: file.size,
            perc: '${file.size/root.size}%'
        });
    });
    return pfsArray;
}