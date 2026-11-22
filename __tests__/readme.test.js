"use strict";

const fs = require("fs");
const path = require("path");

const { addonBuilt } = require("../test-support/addon");
const describeWithAddon = addonBuilt ? describe : describe.skip;

describeWithAddon("README live samples", () => {
    it("matches what the real addon returns", () => {
        const { block, currentBlock } = require("../scripts/readme-samples");
        const readme = fs.readFileSync(path.resolve(__dirname, "../README.md"), "utf8");

        expect(currentBlock(readme)).toBe(block());
    });
});
