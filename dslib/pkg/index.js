const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed */
async function scan(dir) {}

function query(dir) {
    return {
        path: "",
        size: 23,
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
                size: 23,
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
