const addon = require('../neon/native');
const isodates = require("./iso-dates");

const filepath = "c:\\temp\\test";

let date;
let diff;

let input = `${filepath}.csv`;
let output = `${filepath}.tsv`;

const readArray = async(i) => {
    return new Promise((resolve, reject) => {
        try {
            addon.readArray(i);
            resolve()
        }
        catch (err) {
            reject(err);
        }
    })
};

const readObject = async(i) => {
    return new Promise((resolve, reject) => {
        try {
            addon.readObject(i);
            resolve()
        }
        catch (err) {
            reject(err);
        }
    })
};

const convFile = (i, o) => {
    return new Promise((resolve, reject) => {
        try {
            addon.convFile(i, o);
            resolve()
        }
        catch (err) {
            reject(err);
        }
    })
};

date =  isodates.toDate();
 
readArray(input)
    .then(() => {
        diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
        console.log(`Read Array Diff: ${diff} ....`);
        console.log("............................");
    })
    .catch(err => 
        console.error('Read Array error: ', err)
    );

readObject(input)
    .then(() => {
        diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
        console.log(`Read Object Diff: ${diff} ....`);
        console.log("............................");
    })
    .catch(err => 
        console.error('Read Object error: ', err)
    );

convFile(input, output)
    .then(() => {
        diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
        console.log(`File Convert Diff: ${diff} ....`);
        console.log("............................");
    })
    .catch(err => 
        console.error('File Convert error: ', err)
    );

console.log("End of process");
