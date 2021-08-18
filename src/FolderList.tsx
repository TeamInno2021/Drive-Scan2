import React, { Component, ReactElement } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import { App } from "./App";
import { DataGrid, GridCellParams, GridColDef, GridRowParams, GridValueFormatterParams, GridValueGetterParams } from "@material-ui/data-grid";
import { strConvert } from "./conversion";
import path from "path";
import dslib from "dslib";

//Mui Stuff
import { LinearProgress, Box, Typography } from '@material-ui/core';

interface FolderListProps { 
    //getCurrentFolder: () => dslib.File, 
    //setCurrentFolder: (newFolder: dslib.File) => any
    appComponent: App 
}

interface FolderListData {
    id:         string,
    size:       number,
    strSize:    string,
    children?:  Array<dslib.File>
}

export class FolderList extends Component<FolderListProps> {    

    constructor(props: FolderListProps) {
        super(props)

    }

    getFileSize(params): string {
        return params.getValue
    }

    folderListDataFromFiles (files: Array<dslib.File>): Array<FolderListData> {
        let data: Array<FolderListData> = [];
        files.forEach((file: dslib.File) => {
            data.push({
                id:         file.path,
                size:       file.size,
                strSize:    strConvert(file.size),
                children:   file.children
            });
        });
        //Sort them by size
        data.sort((a, b) => (a.size < b.size) ? 1 : -1)
        return data;
    }

    render(): JSX.Element {
        if (this.props.appComponent.state.currentFolder.children) {

            let FolderListColumns: Array<GridColDef> = [
                {   //FileName Field (id=path)
                    field: "id", 
                    headerName: "File Name", 
                    description: "The path of the file",
                    flex: 1,
                    //Value getter to turn the path into a filename
                    valueGetter: (params: GridValueGetterParams) => {
                        return path.basename(params.value as string);
                    },
                    sortComparator: (v1, v2, cellParams1, cellParams2) => {
                        return v1!.toString().localeCompare(v2!.toString());
                    }
                },
                { 
                    field: "strSize", 
                    headerName: "Size", 
                    description: "The size of the file",
                    width: 150,
                    // valueFormatter: (params: GridValueFormatterParams) => {
                    //     return strConvert(params.value as number)
                    // },
                    // sortComparator: (v1, v2) => {
                    //     return Math.max((v1.valueOf() as number), (v2.valueOf() as number));
                    // },
                },
                {
                    field: "size",
                    headerName: "Percentage of Parent",
                    description: "The size of the file as a percentage of its parent folder",
                    width: 250,
                    //Valueformatter to get the size as a percentage of the parent folder"s size
                    valueGetter: (params: GridValueGetterParams) => {
                        return (((params.value as number)/this.props.appComponent.state.currentFolder.size)*100);
                    },
                    sortComparator: (v1, v2) => {
                        return Math.max((v1.valueOf() as number), (v2.valueOf() as number));
                    },
                    renderCell: (params: GridCellParams) => (
                        <Box display="flex" width="100%" alignItems="center">
                            <Box width="100%" mr={1}>
                                <LinearProgress variant="determinate" value={params.value.valueOf() as number}/>
                            </Box>
                            <Box minWidth={35}>
                                <Typography variant="body2" color="textSecondary">{`${Math.round(
                                    params.value.valueOf() as number,
                                )}%`}</Typography>
                            </Box>
                        </Box>
                    )
                },
            ]
            
            let FolderListRows = this.folderListDataFromFiles(this.props.appComponent.state.currentFolder.children);
           
            return (
                <DataGrid
                    columns={FolderListColumns}
                    rows={FolderListRows}
                    onRowDoubleClick={(params: GridRowParams) => {
                        //If it is a folder
                        if (params.getValue(params.id, "children") !== null) {
                            this.props.appComponent.setState({ currentFolder: Scan.query(params.id.toString()) });
                        }
                        else {
                            Electron.shell.openPath(params.id.toString());
                        }
                    }}
                />
            );
        }
        else {
            return <div>Select a folder to display files</div>
        }
    }
}
