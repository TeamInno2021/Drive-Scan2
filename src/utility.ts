import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File } from "dslib";
import path from "path";
import { strConvert } from "./conversion";
import { fileURLToPath } from "url";

export namespace dsutils {
    export function getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        Scan.scan(result[0]);
        //console.log(Scan.query(result[0]));
    }

    export interface PieFileSlice {
        path: string,   //Path of the file
        name: string,   //Name of the file
        size: string,   //Size of the file as a string with units
        perc: string,   //Percentage of the parent folder
    }

    export type PieData = {
        value:      number,
        fileName:   string,
        path:       string,
        strSize:    string
    }

    export function pfsFromFileChildren(root: dslib.File): Array<PieFileSlice> {
        let pfsArray = new Array(root.children.length);
        root.children.forEach(function (file: dslib.File) {
            pfsArray.push({
                path: file.path,
                name: path.basename(file.path),
                size: strConvert(file.size),
                perc: '${file.size/root.size}%'
            });
        });
        return pfsArray;
    }

    export function pieDataFromFileChildren(root: dslib.File): Array<PieData> {
        let pieDataArray = new Array(0);
        let totalSize = 0
        //Shallow clone the children
        let sortedChildren = [...root.children];
        //Then sort the children by their sizes, (largest to smallest)
        sortedChildren.sort((a, b) => (a.size < b.size) ? 1 : -1)
        console.log(sortedChildren)
        for (let i = 0; (i<sortedChildren.length && i<6); i++) {
            if (i<5) {
                let file = sortedChildren[i];
                let slice = {
                    value: file.size/root.size,
                    fileName: path.basename(file.path),
                    path: file.path,
                    strSize: strConvert(file.size)
                }
                //console.log(slice);
                pieDataArray.push(slice);
                //Append the size of this folder to the list
                totalSize += sortedChildren[i].size;
            }
            else {
                //Append one slice to stand for all the others
                pieDataArray.push({
                    value: (root.size-totalSize)/(root.size),
                    fileName: "Others",
                    path: "",
                    strSize: strConvert(root.size-totalSize)
                })
            }
        }
        return pieDataArray
    }
}