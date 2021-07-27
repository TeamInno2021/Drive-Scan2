const dslib = require("./dslib.node");

/** Dummy scan implementation while the real implementation is being developed */
function scan(dir) {
    if (dir == "C:\\") {
        return {
            size: 4324324,
            files: [
                {
                    name: "test",
                    directory: true,
                    size: 4324320,
                },
                {
                    name: "sad",
                    directory: false,
                    size: 4,
                },
            ],
        };
    } else if (dir == "C:\\test") {
        return {
            size: 32,
            files: [
                {
                    name: "a",
                    directory: false,
                    size: 32,
                },
            ],
        };
    }
    // return JSON.parse(
    //     '{"base":"C:\\\\","files":{"files":{"system32":{"files":{"wow, why is system32 empty thats weird":{"path":"C:\\\\system32\\\\wow, why is system32 empty thats weird","size":0}},"path":"C:\\\\system32","size":1},"dslib":{"files":{"src":{"files":{"audio.mp3":{"path":"C:\\\\dslib\\\\src\\\\audio.mp3","size":97612073},"video.mp4":{"path":"C:\\\\dslib\\\\src\\\\video.mp4","size":439598709}},"path":"C:\\\\dslib\\\\src","size":537210782},"package-lock.json":{"path":"C:\\\\dslib\\\\package-lock.json","size":48784},"build.rs":{"path":"C:\\\\dslib\\\\build.rs","size":42},"move.js":{"path":"C:\\\\dslib\\\\move.js","size":320}},"path":"C:\\\\dslib","size":537259928}},"path":"C:\\\\","size":537259929}}'
    // );
}

module.exports = {
    scan,
    _scan: dslib.scan,
};
