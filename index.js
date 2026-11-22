"use strict";

const addon = require('./neon/native');

const fileTypes = () => {
    return addon.fileTypes();
};
const convFile = (source, target) => {
    return addon.convFile(source, target);
};
const convText = (text, target, delim) => {
    return addon.convText(text, delim, target);
};
const readText = (source) => {
    return addon.readText(source);
};
const writeText = (text, target) => {
    return addon.writeText(target, text);
};
const readArray = (source) => {
    return addon.fileArray(source);
};
const readObject = (source) => {
    return addon.fileObject(source);
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
