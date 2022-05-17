const addon = require('../native');
const isodates = require("./iso-dates");

const filepath = "c:\\temp\\test";

let date;
let data;
let diff;

let input = `${filepath}.tsv`;
let output = `${filepath}1.csv`;

console.log("............................");

date =  isodates.toDate();
data = addon.readArray(input);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Read Array Diff: ${diff} ....`);

console.log("............................");

date =  isodates.toDate();
data = addon.readObject(input);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Read Object Diff: ${diff} ....`);

console.log("............................");

date =  isodates.toDate();
data = addon.convFile(input, output);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Convert File Diff: ${diff} ....`);
