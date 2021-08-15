// Move the built library from ./index.node to ./pkg/dslib-<platform>.node

const { rename } = require("fs");
const { join } = require("path");

rename(
    join(__dirname, "index.node"),
    join(__dirname, "pkg", `dslib-${process.platform}.node`),
    (err) => {
        if (err) {
            throw new Error(err);
        }
    }
);
