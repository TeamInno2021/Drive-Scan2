import React, { Component } from "react";
import * as Utility from "./utility";

export class App extends Component {      
    render(): JSX.Element {
        console.log(Utility.strConvert(650000000));
        return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={Utility.getDirectory}/>
            </h3>;
    }
}