import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';

//Pie Chart
import Chart from 'bk-react-charts'
import 'bk-react-charts/dist/index.css'

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
                                <Chart
                                    height='400px'
                                    width='400px'
                                    outerBorderWidth='1px'

                                    xName='item'
                                    yName='spending'
                                    pieChartExplode={true}
                                    pieChartExplodeOffset='10%'
                                    pieChartExplodeIndex={1}
                                    pieChartRadius={150}
                                    title='Expense details'
                                    tooltip={{ enable: true }}
                                    type='PieChart'
                                />  
                            </div>
                        </SplitterLayout>
                    </div>
                </SplitterLayout>
            </h1>
        }
    }
}