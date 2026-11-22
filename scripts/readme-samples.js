"use strict";

const fs = require("fs");
const os = require("os");
const path = require("path");

const ROOT = path.resolve(__dirname, "..");
const README = path.join(ROOT, "README.md");
const START = "<!-- samples:start -->";
const END = "<!-- samples:end -->";

const lines = (rows) => rows.map((row) => `  ${JSON.stringify(row)}`).join(",\n");

const listing = (rows) => `[\n${lines(rows)}\n]`;

const fence = (lang, body) => `\`\`\`${lang}\n${body}\n\`\`\``;

const section = (title, call, lang, output) => `**${title}**\n\n${fence("js", call)}\n\n${fence(lang, output)}`;

const tempTarget = (name) => path.join(fs.mkdtempSync(path.join(os.tmpdir(), "ntf-samples-")), name);

const convertedText = (api, source, name) => {
    const target = tempTarget(name);
    api.convFile(source, target);
    const text = fs.readFileSync(target, "utf8");
    fs.rmSync(path.dirname(target), { recursive: true, force: true });
    return text;
};

const shown = (text) => text.replace(/\t/g, "<TAB>").replace(/\r\n/g, "\\r\\n\n");

const render = () => {
    const api = require("../index");
    const source = path.join(ROOT, "samples", "people.csv");
    const rows = api.readArray(source);
    const objects = api.readObject(source);
    return [
        section("Read rows", 'readArray("samples/people.csv")', "js", listing(rows)),
        section("Read objects", 'readObject("samples/people.csv")', "js", `[\n${lines(objects)}\n]`),
        section("Convert to TSV", 'convFile("samples/people.csv", "people.tsv")', "text", shown(convertedText(api, source, "people.tsv"))),
        section("Convert to PSV", 'convFile("samples/people.csv", "people.psv")', "text", shown(convertedText(api, source, "people.psv"))),
    ].join("\n\n");
};

const block = () => `${START}\n${render()}\n${END}`;

const spliced = (readme) => {
    const from = readme.indexOf(START);
    const to = readme.indexOf(END) + END.length;
    return readme.slice(0, from) + block() + readme.slice(to);
};

const currentBlock = (readme) => readme.slice(readme.indexOf(START), readme.indexOf(END) + END.length);

if (require.main === module) {
    const readme = fs.readFileSync(README, "utf8");
    if (process.argv.includes("--write")) {
        fs.writeFileSync(README, spliced(readme));
    } else {
        console.log(render());
    }
}

module.exports = { block, currentBlock };
