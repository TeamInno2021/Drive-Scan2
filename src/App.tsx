import React, { Component, DetailsHTMLAttributes } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';
import TreeView from '@material-ui/lab/TreeView';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';
import TreeItem from '@material-ui/lab/TreeItem';
import Path from 'path';
import dslib, {File, scan} from "dslib";
import path from "path";
import { strConvert } from "./conversion";

export class App extends Component<{}, { currentpage: string, currentFolder: dslib.File, folderTree: dslib.File, rootPath: string }> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {
            currentpage:"scanpage", 
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
        await this.setState({currentpage:"mainviewpage", rootPath: result[0], currentFolder: Scan.query(result[0]), folderTree: Scan.query(result[0])})
    }
    
    renderTree = (nodes: dslib.File) => (
        <TreeItem key={nodes.path} nodeId={nodes.path} label={path.basename(nodes.path ) + strConvert(nodes.size)}>
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
        if(this.state.currentpage == "scanpage") {
            return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={this.getDirectory.bind(this)}/>
            </h3>;
        }
        else if(this.state.currentpage == "mainviewpage") {   
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