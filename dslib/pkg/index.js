const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed
 */
function scan(_dir) {
    return;
}

function query(_dir) {
    return {
        path: "C:\\",
        size: 163,
        files: [
            {
                name: "a_large_video.mp4",
                size: 23,
                directory: false,
            },
            {
                name: "file.txt",
                size: 64,
                directory: false,
            },
            {
                name: "moosic.mp5",
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
