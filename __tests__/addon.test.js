"use strict";

const fs = require("fs");
const os = require("os");
const path = require("path");

const addonPath = path.resolve(__dirname, "../neon/native/index.node");
const describeWithAddon = fs.existsSync(addonPath) ? describe : describe.skip;

const makeTempDir = () => fs.mkdtempSync(path.join(os.tmpdir(), "ntf-"));

describeWithAddon("native addon (needs a built neon/native/index.node)", () => {
    let api;
    let dir;

    beforeAll(() => {
        api = require("../index");
    });

    beforeEach(() => {
        dir = makeTempDir();
    });

    afterEach(() => {
        fs.rmSync(dir, { recursive: true, force: true });
    });

    const write = (name, text) => {
        const file = path.join(dir, name);
        fs.writeFileSync(file, text);
        return file;
    };

    it("readArray splits a tsv file into rows and columns", () => {
        const file = write("in.tsv", "a\tb\n1\t2");

        expect(api.readArray(file)).toEqual([["a", "b"], ["1", "2"]]);
    });

    it("readArray strips quotes from csv fields and keeps commas inside them", () => {
        const file = write("in.csv", "\"a\",\"b,c\"\n\"1\",\"2\"");

        expect(api.readArray(file)).toEqual([["a", "b,c"], ["1", "2"]]);
    });

    it("readObject uses the first row as keys", () => {
        const file = write("in.psv", "name|age\nann|30\nbob|41");

        expect(api.readObject(file)).toEqual([
            { name: "ann", age: "30" },
            { name: "bob", age: "41" },
        ]);
    });

    it("convFile turns csv into tsv", () => {
        const source = write("in.csv", "\"a\",\"b\"\n\"1\",\"2\"");
        const target = path.join(dir, "out.tsv");

        api.convFile(source, target);

        expect(fs.readFileSync(target, "utf8")).toBe("a\tb\r\n1\t2");
    });

    it("convFile turns tsv into quoted csv", () => {
        const source = write("in.tsv", "a\tb");
        const target = path.join(dir, "out.csv");

        api.convFile(source, target);

        expect(fs.readFileSync(target, "utf8")).toBe("\"a\",\"b\"");
    });

    it("convText writes parsed text to the target format", () => {
        const target = path.join(dir, "out.psv");

        api.convText("a\tb\n1\t2", target, "\t");

        expect(fs.readFileSync(target, "utf8")).toBe("a|b\r\n1|2");
    });

    it("writeText and readText round-trip the same text", () => {
        const target = path.join(dir, "note.txt");

        expect(api.writeText("hello", target)).toBe(5);
        expect(api.readText(target)).toBe("hello");
    });

    it("convFile throws on a missing source and leaves the target untouched", () => {
        const target = write("keep.tsv", "keep me");

        expect(() => api.convFile(path.join(dir, "missing.csv"), target)).toThrow(/Cannot read/);
        expect(fs.readFileSync(target, "utf8")).toBe("keep me");
    });

    it("readText throws on a missing file", () => {
        expect(() => api.readText(path.join(dir, "missing.txt"))).toThrow(/Cannot read/);
    });
});
