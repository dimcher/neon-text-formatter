"use strict";

const addon = require('./neon/native');

const fileTypes = () => {
    return addon.fileTypes();
};
const convFile = (source, target) => {
    return addon.convFile(source, target);
};
const convText = (text, target, delim) => {
    return addon.convText(text, target, delim);
};
const readText = (source) => {
    return addon.readText(source);
};
const writeText = (text, target) => {
    return addon.writeText(text, target);
};
const readArray = (source) => {
    return addon.readArray(source);
};
const readObject = (source) => {
    return addon.readObject(source);
};
module.exports = {
    convFile,
    convText,
    readText,
    writeText,
    fileTypes,
    readArray,
    readObject,
};
