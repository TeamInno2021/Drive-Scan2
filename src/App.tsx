import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File, query } from "dslib";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';

//Pie Chart
import { Chart } from 'react-charts'
import { FolderPie } from "./FolderPie";
import { ResponsiveContainer } from "recharts";

export class App extends Component<{}, { currentPage: string, currentFolder: dslib.File, }> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {
            currentPage: "scanpage",
            currentFolder: {
                path: "",
                size: 0,
                children: []
            }
        }
    }

    async getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        Scan.scan(result[0]);
        console.log({currentPage:"mainviewpage", currentFolder:Scan.query(result[0])});
        await this.setState({currentPage:"mainviewpage", currentFolder:Scan.query(result[0])});
        console.log(this.state.currentFolder);
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
                            <div style={{display: "flex", justifyContent: "center", alignItems: "center"}}>
                                    <FolderPie appComponent={this}/>
                            </div>                       
                        </SplitterLayout>
                    </div>
                </SplitterLayout>
            </h1>
        }
    }
}