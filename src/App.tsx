import React, { Component } from "react";
import * as Utility from "./utility";

export class App extends Component {      
    render(): JSX.Element {
        console.log("Test");
        Utility.myFunction();
        return <h1>Hello, World!</h1>;
    }
}