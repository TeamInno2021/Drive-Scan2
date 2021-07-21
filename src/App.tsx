import React, { Component } from "react";
import * as Utility from "./utility";

export class App extends Component {      
    render(): JSX.Element {
        return <h3>
            Select
            <input id="button" type="button" value="Directory" onClick={Utility.getDirectory}/>
            </h3>;
    }
}