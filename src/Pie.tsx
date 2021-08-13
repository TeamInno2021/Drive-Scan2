import React, { Component } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import dslib, { File, query } from "dslib";

//Splitter Layout
import SplitterLayout from 'react-splitter-layout';
import 'react-splitter-layout/lib/index.css';

//Pie Chart
//import Chart from 'bk-react-charts'
//import 'bk-react-charts/dist/index.css'
import { PieChart } from 'react-minimal-pie-chart';
import { TableRow } from "@material-ui/core";
import { contextIsolated } from "process";

export class Pie extends Component<{ getCurrentFolder: () => dslib.File , setCurrentFolder: (newFolder: dslib.File) => any }, { hovered?: number }> {    
    render(): JSX.Element {
        if (this.props.getCurrentFolder().children) {
            // return <Chart
            // height='400px'
            // width='400px'
            // outerBorderWidth='1px'
            // dataSource={Utility.dsutils.pfsFromFileChildren(this.props.currentFolderHook())}
            // xName='name'
            // yName='perc'
            // pieChartExplode={true}
            // pieChartExplodeOffset='10%'
            // pieChartExplodeIndex={1}
            // pieChartRadius={150}
            // title='Expense details'
            // tooltip={{ enable: true }}
            // type='PieChart'
            // />
            console.log("Rendering Pie with the following: " + this.props.getCurrentFolder().children);
            return <PieChart
                data={Utility.dsutils.pieDataFromFileChildren(this.props.getCurrentFolder())}
                radius={PieChart.defaultProps.radius - 6}
                segmentsStyle={{ transition: 'stroke .3s', cursor: 'pointer' }}
                segmentsShift={(index) => (index === this.state.hovered ? 6 : 1)}
                onClick={(event, index) => {
                    //TODO: Change app's state
                }}
                onMouseOver={(_, index) => {
                    this.setState({ hovered: index });
                }}
                onMouseOut={() => {
                    this.setState({ hovered: undefined });
                }}
            />
        }
        else {
            return <div>Select a folder for a pie breakdown</div>
        }
    }
}
