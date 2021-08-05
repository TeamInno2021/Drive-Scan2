const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed
 */
function scan(_dir) {
    return;
}

function query(dir) {
    return {
        path: "C:\\",
        size: 163,
        files: [
            {
                path: "a_large_video.mp4",
                size: 23,
                directory: false,
            },
            {
                path: "",
                size: 64,
                directory: false,
            },
            {
                path: "",
                size: 76,
                directory: false,
            },
        ],
    };
}

module.exports = {
    init: dslib.init,
    scan,
    query,
    // scan: dslib.scan,
    // query: dslib.query,
};
