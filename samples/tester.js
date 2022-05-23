"use strict";

const path = require("path");

const addon = require('../neon/native');

let input = path.resolve("../samples/data.csv");
let output = path.resolve("../samples/data.tsv");

const readArray = async(i) => {
    return new Promise((resolve, reject) => {
        try {
            resolve(addon.fileArray(i))
        }
        catch (err) {
            reject(err);
        }
    })
};

const readObject = async(i) => {
    return new Promise((resolve, reject) => {
        try {
            resolve(addon.fileObject(i))
        }
        catch (err) {
            reject(err);
        }
    })
};

const convFile = (i, o) => {
    return new Promise((resolve, reject) => {
        try {
            resolve(addon.convFile(i, o))
        }
        catch (err) {
            reject(err);
        }
    })
};

readArray(input)
    .then(res => {
        console.log("Read to array:", res);
    })
    .catch(err => 
        console.error('Read Array error: ', err)
    );
 
readObject(input)
    .then(res => {
        console.log("Read to object:", res);
    })
    .catch(err => 
        console.error('Read Object error: ', err)
    );

convFile(input, output)
    .then(res => {
        console.log("Convert format:", res);
    })
    .catch(err => 
        console.error('File Convert error: ', err)
    );

console.log("End of process");
