"use strict";

const fs = require("fs");
const path = require("path");

const { addonBuilt } = require("../test-support/addon");
const describeWithAddon = addonBuilt ? describe : describe.skip;

if (process.env.REQUIRE_ADDON === "1") {
    it("has a loadable napi addon (REQUIRE_ADDON=1)", () => {
        expect(addonBuilt).toBe(true);
    });
}

describeWithAddon("README live samples", () => {
    it("matches what the real addon returns", () => {
        const { block, currentBlock } = require("../scripts/readme-samples");
        const readme = fs.readFileSync(path.resolve(__dirname, "../README.md"), "utf8").replace(/\r\n/g, "\n");

        expect(currentBlock(readme)).toBe(block());
    });
});
