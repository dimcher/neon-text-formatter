"use strict";

const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");

const hasNodeFile = () => fs.readdirSync(root).some((name) => name.endsWith(".node"));

const loads = () => {
    try {
        require("../binding");
        return true;
    } catch (error) {
        console.warn(`addon failed to load: ${error.message}`);
        return false;
    }
};

const addonBuilt = hasNodeFile() && loads();

module.exports = { addonBuilt };
