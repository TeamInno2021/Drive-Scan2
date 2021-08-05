// const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed
 */
function scan(_dir) {
    return;
}

function query(_dir) {
    return {
        path: "C:\\",
        size: 23 + 64 + 76 + 64954689456,
        children: [
            {
                path: "C:\\a_large_video.mp4",
                size: 23,
                children: undefined,
            },
            {
                path: "C:\\file.txt",
                size: 64,
                children: undefined,
            },
            {
                path: "C:\\moosic.mp5",
                size: 76,
                children: undefined,
            },
            {
                path: "C:\\system32",
                size: 64954689456,
                children: [],
            },
        ],
    };
}

module.exports = {
    // init: dslib.init,
    scan,
    query,
    // scan: dslib.scan,
    // query: dslib.query,
};
