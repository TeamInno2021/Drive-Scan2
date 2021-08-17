import React, { Component } from "react";
import * as Scan from "./scan";
import * as Electron from "electron";

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
        Scan.scan(result[0]);
        console.log(Scan.query(result[0]));
        await this.setState({currentPage:"mainviewpage", rootPath: result[0], currentFolder: Scan.query(result[0]), folderTree: Scan.query(result[0])})
    }
    
    renderTree = (nodes: dslib.File) => (
        <TreeItem key={nodes.path} nodeId={nodes.path} label={path.basename(nodes.path ) + " - " + strConvert(nodes.size)}>
            {Array.isArray(nodes.children) ? nodes.children.map((node) => this.renderTree(node)) : null}
            {/* nodesFromChildren(nodes.children); */}
        </TreeItem>
    );

    // nodesFromChildren (children?: [dslib.File]) {
    //     let nodeChildren = [];

    //     if (children) {
    //         children.forEach(child => {
    //             nodeChildren.push(this.renderTree(child)); 
    //         });
    //     }
    //     return nodeChildren;   
    // }

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
            if (this.state.folderTree.size != 0) {                                     
                return <h1>
                    <SplitterLayout vertical={false}>
                        {/* TreeView */}
                        <div>

                        <TreeView
                            defaultCollapseIcon={<ExpandMoreIcon />}
                            defaultExpanded={['root']}
                            defaultExpandIcon={<ChevronRightIcon />}
                        >
                            {this.renderTree(this.state.folderTree)}
                        </TreeView>
                
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
            else {
                return null;
            }

        }
    }
}