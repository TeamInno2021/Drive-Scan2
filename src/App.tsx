import React, { Component } from "react";
import * as Scan from "./scan";
import * as Electron from "electron";
import { File } from "dslib"

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';
import TreeView from '@material-ui/lab/TreeView';
import dslib from "dslib";
import path from "path";
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';
import TreeItem from '@material-ui/lab/TreeItem';
import { strConvert } from "./conversion";
import { Button, ThemeProvider, Typography } from '@material-ui/core';
import { createTheme } from "@material-ui/core/styles";;

//Pie Chart
import { FolderPie } from "./FolderPie";

export class App extends Component<{}, { currentPage: string, currentFolder: dslib.File, folderTree: dslib.File, rootPath: string }> { 
    constructor(props:{}) {
        super(props)
        this.state = {
            currentPage:"scanpage", 
            currentFolder: { 
                path: "",
                size: 0, 
                children: [], 
            },
            folderTree: { 
                path: "",
                size: 0, 
                children: [],
            }, 
            rootPath: "", 
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

    renderTree = (nodes: dslib.File) => (
        <TreeItem key={nodes.path} nodeId={nodes.path} label={path.basename(nodes.path ) + " - " + strConvert(nodes.size)}>
            {Array.isArray(nodes.children) ? nodes.children.map((node) => this.renderTree(node)) : null}
            {/* nodesFromChildren(nodes.children); */}
        </TreeItem>
    );

    //Method to allow the pie chart and folder view to update the currentfolder
    async setCurrentFolder(newFolder: File) {
        this.setState({ currentFolder: newFolder });
    }

    //Method to allow the pie chart and folder view to get the currentfolder
    getCurrentFolder(): File {
        return this.state.currentFolder;
    }

    render(): JSX.Element {
        if(this.state.currentPage == "scanpage") {
            return (
                <div style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                    <ThemeProvider theme={createTheme({ palette: { type: "dark" } })}>
                        <ul style={{ width: "100%", height: "100%" }}>
                            <li style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
                                <Typography variant="h1" color="textPrimary">
                                    DriveScan
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

                    <TreeView
                        style={{color: "#ffffff"}}
                        defaultCollapseIcon={<ExpandMoreIcon />}
                        defaultExpanded={['root']}
                        defaultExpandIcon={<ChevronRightIcon />}
                    >
                        {this.renderTree(this.state.currentFolder)}
                    </TreeView>
                
                    </div>
                    {/* Folder and Pie Views In Horizontal Splitter */}
                    <div>
                        <SplitterLayout vertical={true}>
                            {/* FolderView */}
                            <div>
                                {/* Alex Put the Folder View Here */}
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
                                    >Back</Button>
                                </li>
                                <li>
                                    <FolderPie appComponent={this}/>
                                </li>
                            </ul>                       
                        </SplitterLayout>
                    </div>
                </SplitterLayout>
            </ThemeProvider>)
        }
    }
}