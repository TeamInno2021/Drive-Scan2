const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    mode: process.env.NODE_ENV || "development",
    entry: "./src/renderer.tsx",
    target: "electron-renderer",
    devtool: "source-map",
    devServer: {
        contentBase: path.join(__dirname, "dist/renderer.js"),
        compress: true,
        port: 9000,
    },
    resolve: {
        extensions: [".tsx", ".ts", ".jsx", ".js"],
    },
    module: {
        rules: [
            {
                test: /\.ts(x?)$/,
                include: /src/,
                use: [{ loader: "ts-loader" }],
            },
            {
                test: /\.css$/i,
                use: ["style-loader", "css-loader", "sass-loader"],
            },
            {
                test: /\.svg$/,
                use: ["@svgr/webpack"],
            },
        ],
    },
    output: {
        path: __dirname + "/dist",
        filename: "renderer.js",
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html",
        }),
    ],
};
