// Due to the way the webpack loader bundles native node modules,
// the path must be hardcoded to allow the renaming of the target file
let dslib;
switch (process.platform) {
    case "win32": {
        dslib = require("./dslib-win32.node");
        break;
    }

    // case "linux": {
    //     dslib = require("./dslib-linux.node");
    //     break;
    // }

    default: {
        throw new Error("unsupported platform");
    }
}

module.exports = {
    init: dslib.init,
    scan: dslib.scan,
    query: dslib.query,
};
