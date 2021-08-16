import React, { Component } from "react";
import * as Scan from "./scan";
import * as Electron from "electron";
import dslib from "dslib";
import path from "path";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';

//Pie Chart
import { FolderPie } from "./FolderPie";
import { Button } from "@material-ui/core";

export class App extends Component<{}, { currentPage: string, currentFolder: dslib.File, rootPath: string}> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {
            currentPage: "scanpage",
            currentFolder: {
                path: "",
                size: 0,
                children: []
            },
            rootPath: ""
        }
    }

    async getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        //If a folder is actually selected
        if (result[0] !== undefined) {
            Scan.scan(result[0]);
            console.log({currentPage:"mainviewpage", currentFolder:Scan.query(result[0])});
            await this.setState({currentPage:"mainviewpage", currentFolder:Scan.query(result[0]), rootPath: result[0]});
            console.log(this.state.currentFolder);
        }
    }

    //Method to allow the pie chart and folder view to update the currentfolder
    async setCurrentFolder(newFolder: dslib.File) {
        this.setState({ currentFolder: newFolder });
    }

    //Method to allow the pie chart and folder view to get the currentfolder
    getCurrentFolder(): dslib.File {
        return this.state.currentFolder;
    }

    render(): JSX.Element {
        if(this.state.currentPage == "scanpage") {
            return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={this.getDirectory.bind(this)}/>
            </h3>;
        }
        else if(this.state.currentFolder != undefined && this.state.currentPage == "mainviewpage") {
            return <h1>
                <SplitterLayout vertical={false}>
                    {/* TreeView */}
                    <div>
                        Tom Put the TreeView Here
                    </div>
                    {/* Folder and Pie Views In Horizontal Splitter */}
                    <div>
                        <SplitterLayout vertical={true}>
                            {/* FolderView */}
                            <div>
                                Alex Put the Folder View Here
                            </div>
                            {/* PieView */}
                            <ul style={{ display: "flex", flexDirection: "column", justifyContent: "center", alignItems:"center" }}>
                                <li style={{ display: "flex", justifyContent: "right", alignItems: "right", height: 40 }}>
                                    <Button
                                        color="primary"
                                        size="medium"
                                        variant="outlined"
                                        onClick={(e) => {
                                            //Check that we aren't already at the root of the scanned folder
                                            if (this.state.currentFolder.path !== this.state.rootPath) {
                                                console.log(path.resolve(path.join(this.state.currentFolder.path, "..")));
                                                //Traverse one folder up
                                                this.setState({currentFolder: Scan.query(path.resolve(path.join(this.state.currentFolder.path, "..")))});
                                            }
                                        }}
                                        //Disable the button if we are at the root scanned folder
                                        disabled={this.state.currentFolder.path == this.state.rootPath}
                                    >Up</Button>
                                </li>
                                <li>
                                    <FolderPie appComponent={this}/>
                                </li>
                            </ul>                       
                        </SplitterLayout>
                    </div>
                </SplitterLayout>
            </h1>
        }
    }
}