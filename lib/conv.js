const fs = require("fs");
const { Interface } = require("readline");

const addon = require('../native');
const isodates = require("./iso-dates");

const joiner = '\t';

const extensions = {
    csv: "csv",
    tsv: "tsv",
    psv: "psv"
};

const filepath = "c:\\temp\\test";

const sample = `"A","bAA"`;

const csvToTsv = (data) => {
    const tab = '\t';
    const rows = data.split('\n');

    let arr = [];

    rows.forEach(row => {
        while (row.match(/^,{2,}\t*/)) { // initial delimiters at the begining
            row = row.replace(/^(,+),(\t*)/, `$1${tab}$2`);
        };
        
        row = row.replace(/^,/, tab); // begining line character as delimiter

        while (row.match(/,\t*$/)) { // initial delimiters at the end
            row = row.replace(/,(\t*)$/, `${tab}$1`);
        };

        row = row.replace(/^(\t*)"|"(\t*)$/g, `$1`); // begining and end line quotes

        while (row.match(/",+,\t*"/)) { // all delimiters outside quoted string
            row = row.replace(/(")(,+),(\t*")/, `$1$2${tab}$3`);
        };

        row = row.replace(/",(\t+)"/g, `${tab}$1`); // single initial delimiter and new delimeter outside quoted string

        row = row.replace(/","/g, tab); // single initial delimiter outside quoted string

        arr.push(row);
    });

    return arr.join("\r\n");
};

let date;
let diff;
let data;

let input = `${filepath}.${extensions.csv}`;
let output = `${filepath}.${extensions.psv}`;

console.log(`Start:`, `${input}`);
//const csv = fs.readFileSync(`${filepath}.${extensions.csv}`, "utf-8");
//const csv = addon.readtext(`${filepath}.${extensions.csv}`);
//console.log(`CSV:`, csv);
//date =  isodates.toDate();
//const tsv = csvToTsv(csv);
//diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);

console.log(`Diff: ${diff} ....`);


//---------------------------
console.log("............................");

date =  isodates.toDate();
data = addon.readArray(input);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Diff: ${diff} ....`);

console.log(data);
//process.exit(1);

console.log("............................");

date =  isodates.toDate();
data = addon.readObject(input);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Diff: ${diff} ....`);

console.log(data);
process.exit(1);
console.log("............................");


date =  isodates.toDate();
data = addon.convert(input, output);
diff = isodates.diffSeconds(date, isodates.toDate()).toFixed(3);
console.log(`Diff: ${diff} ....`);


//s.writeFileSync(`${filepath}.${extensions.tsv}`, data);
//console.log(tsv);
//process.exit();

