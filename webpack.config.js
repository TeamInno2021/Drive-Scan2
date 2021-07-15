const reactConfigs = require("./webpack.react.js");
const electronConfigs = require("./webpack.electron.js");

module.exports = [electronConfigs, reactConfigs];
