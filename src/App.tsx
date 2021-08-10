import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File, query } from "dslib";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';

export class App extends Component<{}, { currentPage: string, currentFolder: dslib.File, }> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {
            currentPage: "scanpage",
            currentFolder: { path: "", size: 0, children: [] }
        }
    }

    getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        Scan.scan(result[0]);
        console.log(Scan.query(result[0]));
        this.setState({currentPage:"mainviewpage"})
    }
    

    render(): JSX.Element {
        if(this.state.currentPage == "scanpage") {
            return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={this.getDirectory.bind(this)}/>
            </h3>;
        }
        else if(this.state.currentPage == "mainviewpage") {
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
                            <div>
                                Ben Put the Pie Here
                            </div>
                        </SplitterLayout>
                    </div>
                </SplitterLayout>
            </h1>
        }
    }
}