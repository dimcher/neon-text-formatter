"use strict";

const fs = require("fs");
const os = require("os");
const path = require("path");

const addonPath = path.resolve(__dirname, "../neon/native/index.node");
const addonBuilt = fs.existsSync(addonPath);
const describeWithAddon = addonBuilt ? describe : describe.skip;

if (process.env.REQUIRE_ADDON === "1") {
    it("has a built neon/native/index.node (REQUIRE_ADDON=1)", () => {
        expect(addonBuilt).toBe(true);
    });
}

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

    it("readArray and readObject throw on a missing file", () => {
        const missing = path.join(dir, "missing.csv");

        expect(() => api.readArray(missing)).toThrow(/Cannot read/);
        expect(() => api.readObject(missing)).toThrow(/Cannot read/);
    });

    it("readArray throws a clear error for a name without an extension", () => {
        expect(() => api.readArray("")).toThrow(/file extension/);
        expect(() => api.readArray(path.join(dir, "data."))).toThrow(/file extension/);
    });

    it("convFile throws a clear error for a target without an extension", () => {
        const source = write("in.csv", "\"a\"");

        expect(() => api.convFile(source, path.join(dir, "out."))).toThrow(/file extension/);
    });

    it("readText throws on a file that is not valid utf-8", () => {
        const file = path.join(dir, "binary.txt");
        fs.writeFileSync(file, Buffer.from([0xff, 0xfe, 0x41]));

        expect(() => api.readText(file)).toThrow(/Cannot read/);
    });

    it("writeText throws when the target folder does not exist", () => {
        expect(() => api.writeText("x", path.join(dir, "no", "such", "out.txt"))).toThrow(/Cannot write/);
    });

    it("convFile throws when the target folder does not exist", () => {
        const source = write("in.tsv", "a\tb");

        expect(() => api.convFile(source, path.join(dir, "no", "such", "out.csv"))).toThrow(/Cannot write/);
    });

    it("convFile keeps quotes and commas inside csv values", () => {
        const source = write("in.tsv", "say \"hi\"\ta,b");
        const target = path.join(dir, "out.csv");

        api.convFile(source, target);

        expect(api.readArray(target)).toEqual([["say \"hi\"", "a,b"]]);
    });

    it("readText throws on a missing file", () => {
        expect(() => api.readText(path.join(dir, "missing.txt"))).toThrow(/Cannot read/);
    });
});
