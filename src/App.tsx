import React, { Component } from "react";
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

export class App extends Component<{}, { currentpage: string }> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {currentpage:"scanpage"}
    }

    getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        Scan.scan(result[0]);
        console.log(Scan.query(result[0]));
        this.setState({currentpage:"mainviewpage"})
    }
    

    render(): JSX.Element {
        if(this.state.currentpage == "scanpage") {
            return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={this.getDirectory.bind(this)}/>
            </h3>;
        }
        else if(this.state.currentpage == "mainviewpage") {

            let scandata = Scan.query("C:\\")
                interface RenderTree {
                id: string;
                name: string;
                children?: RenderTree[]; 
                }

                    let children = [];

                    for (let i  = 0; i < scandata.files.length; i++) {
                        console.log(i);
                        children.push( {
                        id: `${i}`,
                        name: Path.basename(scandata.files[i].path) + " - " + scandata.files[i].size,
                    },);
                }

                const data: RenderTree = {
                    id: 'root',
                    name: scandata.path + " - " + scandata.size,
                    children:children
                };

                const renderTree = (nodes: RenderTree) => (
                    <TreeItem key={nodes.id} nodeId={nodes.id} label={nodes.name}>
                        {Array.isArray(nodes.children) ? nodes.children.map((node) => renderTree(node)) : null}
                    </TreeItem>
                );
            return <h1>
                <SplitterLayout vertical={false}>
                    {/* TreeView */}
                    <div>

                    <TreeView
                        defaultCollapseIcon={<ExpandMoreIcon />}
                        defaultExpanded={['root']}
                        defaultExpandIcon={<ChevronRightIcon />}
                    >
                        {renderTree(data)}
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
    }
}