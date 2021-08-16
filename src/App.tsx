import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File, query } from "dslib";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';
import { Button, ThemeProvider, Typography } from '@material-ui/core'
import { createTheme } from "@material-ui/core/styles";

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
            return (
                <div style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                    <ThemeProvider theme={createTheme({ palette: { type: "dark" } })}>
                        <ul style={{ width: "100%", height: "100%" }}>
                            <li style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                                <Typography variant="h1" color="textPrimary">
                                    Drive Scan
                                </Typography>
                            </li>
                            <li style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                                <Typography variant="h4" color="textSecondary">
                                    Find the things chewing up your disk space
                                </Typography>
                            </li>
                            <li style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                                <br/>
                            </li>
                            <li style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                                <Button variant="contained" color="primary" onClick={this.getDirectory.bind(this)}>
                                    Scan Folder
                                </Button>
                            </li>
                        </ul>
                    </ThemeProvider>
                </div>)
        }
        else if(this.state.currentPage == "mainviewpage") {
            return (
            <ThemeProvider theme={createTheme({ palette: { type: "dark" } })}>
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
            </ThemeProvider>)
        }
    }
}