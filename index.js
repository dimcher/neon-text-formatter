"use strict";

const binding = require("./binding");

module.exports = {
    convFile: binding.convFile,
    convText: binding.convText,
    readText: binding.readText,
    writeText: binding.writeText,
    fileTypes: binding.fileTypes,
    readArray: binding.readArray,
    readObject: binding.readObject,
};
