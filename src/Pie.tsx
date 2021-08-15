import React, { Component, ReactElement } from "react";
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
//import { PieChart } from 'react-minimal-pie-chart';
import { ResponsiveContainer, Sector, Cell, Pie, PieChart, Tooltip } from "recharts";
import { TableRow } from "@material-ui/core";
import { contextIsolated } from "process";
import { App } from "./App"

interface Props { 
    //getCurrentFolder: () => dslib.File, 
    //setCurrentFolder: (newFolder: dslib.File) => any
    appComponent: App 
}

const PIESECTORS = [
    { cx: 250, cy: 250, startAngle: 0, endAngle: 60, innerRadius: 100, outerRadius: 200 },
    { cx: 250, cy: 250, startAngle: 60, endAngle: 120, innerRadius: 100, outerRadius: 200 },
    { cx: 250, cy: 250, startAngle: 120, endAngle: 180, innerRadius: 100, outerRadius: 200 },
    { cx: 250, cy: 250, startAngle: 180, endAngle: 240, innerRadius: 100, outerRadius: 200 },
    { cx: 250, cy: 250, startAngle: 240, endAngle: 300, innerRadius: 100, outerRadius: 200 },
    { cx: 250, cy: 250, startAngle: 300, endAngle: 360, innerRadius: 100, outerRadius: 200 },
  ];

//Temporary hardcoded segment colours
const PIECOLOURS = ['#f07178','#F78C6C','#FFCB6B','#C3E88D','#82AAFF','#C792EA']


const CustomTooltip = ({ active, payload, label }: any) => {
    console.log("e")
    if (payload && payload.length) {
        return (
        <div className="custom-tooltip">
            <p className="label">{`${label} : ${payload[0].value}`}</p>
            <p className="desc">{payload[0].value}</p>
        </div>
        );
    }
    return null;
};

export class FolderPie extends Component<Props, { hovered: number }> {    
    
    constructor(props: Props) {
        super(props)
        this.state = {
            hovered: null
        }
    }

    render(): JSX.Element {
        if (this.props.appComponent.state.currentFolder.children) {
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
            let pieData = Utility.dsutils.pieDataFromFileChildren(this.props.appComponent.state.currentFolder);
            //console.log("Rendering Pie with the following: " + pieData);
            // return <PieChart
            //     data={pieData}
            //     radius={PieChart.defaultProps.radius - 6}
            //     segmentsStyle={{ transition: 'stroke .3s', cursor: 'pointer' }}
            //     //Set the segmentsShift to be 0 for anything that isn't hovered and X 
            //     // segmentsShift={(index: number): number => {
            //     //     if (this.state.hovered && this.state.hovered == index)  { return 6 } 
            //     //     else                                                    { return 0 }
            //     // }}
            //     onClick={(event, index) => {
            //         //TODO: Change app's state
            //     }}
            //     onMouseOver={(_, index) => {
            //         this.setState({ hovered: index });
            //     }}
            //     onMouseOut={() => {
            //         this.setState({ hovered: undefined });
            //     }}
            // />


            

            console.log("PieData: " + pieData)
            return (
                <ResponsiveContainer width={500} height={500}>
                    <PieChart width={500} height={500}>
                        <Pie
                            data={pieData}
                            cx="50%"
                            cy="50%"
                            // labelLine={false}
                            // label={renderCustomizedLabel}
                            outerRadius={"50%"}
                            fill="#8884d8"
                            dataKey="value"
                            nameKey="fileName"
                            sectors={PIESECTORS}
                            onMouseEnter={(e, hovered) => this.setState({ hovered }) }
                            onMouseLeave={() => this.setState({ hovered: -1 }) }
                            onClick={(data, index, e) => {
                                
                            }}
                        >
                            <Tooltip
                                active={true}
                                wrapperStyle={{
                                    visibility: 'visible',
                                }}
                                content={<CustomTooltip />}
                            />
                            {pieData.map((entry, index) => (
                                <Cell key={`cell-${index}`} fill={PIECOLOURS[index]} />
                            ))}
                        </Pie>
                    </PieChart>
                </ResponsiveContainer>
              );
        }
        else {
            return <div>Select a folder for a pie breakdown</div>
        }
    }
}
