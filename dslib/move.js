// Move the built library from ./index.node to ./pkg/dslib.node

const { rename } = require("fs");
const { join } = require("path");

rename(
    join(__dirname, "index.node"),
    join(__dirname, "pkg", "dslib.node"),
    (err) => {
        if (err) {
            throw new Error(err);
        }
    }
);
