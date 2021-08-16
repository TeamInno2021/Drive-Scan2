import React, { Component, ReactElement } from "react";
import * as Utility from "./utility";
import * as Scan from "./scan";
import * as Electron from "electron";
import { File } from "dslib";
import open from "open";

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

// const renderCustomizedLabel = ({ cx, cy, midAngle, innerRadius, outerRadius, percent, index }) => {
//     const radius = innerRadius + (outerRadius - innerRadius) * 0.5;
//     const x = cx + radius * Math.cos(-midAngle * RADIAN);
//     const y = cy + radius * Math.sin(-midAngle * RADIAN);
  
//     return (
//       <text x={x} y={y} fill="white" textAnchor={x > cx ? 'start' : 'end'} dominantBaseline="central">
//         {`${(percent * 100).toFixed(0)}%`}
//       </text>
//     );
//   };

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

interface PieProps { 
    //getCurrentFolder: () => dslib.File, 
    //setCurrentFolder: (newFolder: dslib.File) => any
    appComponent: App 
}

export class FolderPie extends Component<PieProps, { hovered: number }> {    
    
    constructor(props: PieProps) {
        super(props)
        this.state = {
            hovered: null
        }
    }

    renderCustomizedLabel = (entry: Utility.dsutils.PieData) => {
        return entry.name + ': ' + entry.strSize
        // return (<p>
        //     <text fontSize={12}>
        //         {`${entry.fileName}\n${entry.strSize}`}
        //     </text>  
        // </p>)
    };

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
            
            return (
                <PieChart width={1200} height={600}>
                    <Pie
                        cx={"50%"} 
                        cy={"50%"}
                        // width={50}
                        // height={50}
                        data={pieData}
                        alignmentBaseline="central"
                        labelLine={false}
                        label={this.renderCustomizedLabel}
                        outerRadius={"50%"}
                        fill="#8884d8"
                        dataKey="value"
                        nameKey="fileName"
                        sectors={PIESECTORS}
                        onMouseEnter={(e, hovered) => this.setState({ hovered }) }
                        onMouseLeave={() => this.setState({ hovered: -1 }) }
                        onClick={(entry, e) => {
                            //If to catch whether the user has clicked on the "Others" slice, in which case we want to do nothing
                            if (entry.directory != null) {
                                console.log(entry);
                                console.log(`Pie: User clicked on pie sector \"${entry.name}\"`);
                                if (entry.directory == true) {
                                    console.log(`Pie: Changing to selected directory:\n\"${entry.path}\"`);
                                    let newCurrentFolder = Scan.query(entry.path);
                                    //Change the currentfolder to this new path
                                    this.props.appComponent.setCurrentFolder(newCurrentFolder);
                                } else {
                                    console.log(`Pie: Opening selected file in default application:\n\"${entry.path}\"`);
                                    //Open the file in the default application
                                    open(entry.path);
                                }
                            } 
                        }}
                    >
                        {/* <Tooltip
                            active={true}
                            wrapperStyle={{
                                visibility: 'visible',
                            }}
                            content={<CustomTooltip />}
                        /> */}
                        {pieData.map((entry, index) => (
                            <Cell key={`cell-${index}`} fill={PIECOLOURS[index]}/>
                        ))}
                    </Pie>
                </PieChart>
              );
        }
        else {
            return <div>Select a folder for a pie breakdown</div>
        }
    }
}
