"use strict";

const fs = require("fs");
const path = require("path");

const addonBuilt = fs.existsSync(path.resolve(__dirname, "../neon/native/index.node"));
const describeWithAddon = addonBuilt ? describe : describe.skip;

describeWithAddon("README live samples", () => {
    it("matches what the real addon returns", () => {
        const { block, currentBlock } = require("../scripts/readme-samples");
        const readme = fs.readFileSync(path.resolve(__dirname, "../README.md"), "utf8");

        expect(currentBlock(readme)).toBe(block());
    });
});
