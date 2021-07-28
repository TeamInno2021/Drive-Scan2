const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed,
 * note that this method will wait an arbitrary number of seconds before resolving.
 */
async function scan(_dir) {
    return new Promise((resolve) => {
        // Wait somewhere between 2 to 8 seconds before resolving
        setTimeout(resolve, Math.floor(Math.random() * 8 - 2) * 1000);
    });
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
