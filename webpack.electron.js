const path = require("path");

module.exports = {
    mode: process.env.NODE_ENV || "development",
    entry: "./src/main.ts",
    target: "electron-main",
    resolve: {
        extensions: [".ts", ".js"],
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                include: /src/,
                use: [{ loader: "ts-loader" }],
            },
        ],
    },
    output: {
        path: __dirname + "/dist",
        filename: "main.js",
    },
};
