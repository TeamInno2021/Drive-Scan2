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
        console.log(Scan.query(result[0]));
    }

    export interface PieFileSlice {
        path: string,   //Path of the file
        name: string,   //Name of the file
        size: string,   //Size of the file as a string with units
        perc: string,   //Percentage of the parent folder
    }

    export type PieData = {
        color:   string,
        value:   number,
        key:     string,
        title:   string,
        strsize: string
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
        let pieDataArray = new Array(root.children.length);
        //Temporary hardcoded segment colours
        let pieColours = ['#f07178','#F78C6C','#FFCB6B','#C3E88D','#82AAFF','#C792EA']
        let totalSize = 0
        //Shallow clone the children
        let sortedChildren = [...root.children];
        //Then sort the children by their sizes
        sortedChildren.sort((a, b) => (a.size > b.size) ? 1 : -1)
        for (let i = 0; (i<sortedChildren.length && i<=6); i++) {
            if (i<6) {
                let file = sortedChildren[i];
                let slice = {
                    color: pieColours[i],
                    value: file.size/root.size,
                    key: file.path,
                    title: path.basename(file.path),
                    strsize: strConvert(file.size)
                }
                console.log(slice);
                pieDataArray.push(slice);
                //Append the size of this folder to the list
                totalSize += root.children[i].size;
            }
            else {
                //Append one slice to stand for all the others
                pieDataArray.push({
                    color: pieColours[i],
                    value: root.size-totalSize,
                    key: "",
                    title: "Others",
                    strsize: strConvert(root.size-totalSize)
                })
            }
        }
        return pieDataArray
    }
}