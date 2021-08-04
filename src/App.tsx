import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";

export class App extends Component<{}, { currentpage: string }> { 
    
    constructor(props:{}) {
        super(props)
        this.state = {currentpage:"directorypage"}
    }

    getDirectory() {
        let result = Electron.remote.dialog.showOpenDialogSync({
            properties: ["openDirectory"],
        });
        Scan.scan(result[0]);
        console.log(Scan.query(result[0]));
        this.setState({currentpage:"treeviewpage"})
    }
    

    render(): JSX.Element {
        if(this.state.currentpage == "directorypage") {
            return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={this.getDirectory.bind(this)}/>
            </h3>;
        }
        else {
            return <h1>
                something
            </h1>
        }
    }
}