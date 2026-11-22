"use strict";

jest.mock("../neon/native", () => ({
    convFile: jest.fn(() => 10),
    convText: jest.fn(() => 20),
    readText: jest.fn(() => "text"),
    writeText: jest.fn(() => 4),
    fileTypes: jest.fn(() => ({ csv: ",", tsv: "\t", psv: "|" })),
    fileArray: jest.fn(() => [["a", "b"]]),
    fileObject: jest.fn(() => [{ a: "1" }]),
}), { virtual: true });

const addon = require("../neon/native");
const api = require("../index");

beforeEach(() => jest.clearAllMocks());

describe("index.js wrapper", () => {
    it("readArray calls the exported fileArray and returns its rows", () => {
        expect(api.readArray("in.csv")).toEqual([["a", "b"]]);
        expect(addon.fileArray).toHaveBeenCalledWith("in.csv");
    });

    it("readObject calls the exported fileObject and returns its rows", () => {
        expect(api.readObject("in.csv")).toEqual([{ a: "1" }]);
        expect(addon.fileObject).toHaveBeenCalledWith("in.csv");
    });

    it("writeText passes the target file first and the text second", () => {
        expect(api.writeText("hello", "out.txt")).toBe(4);
        expect(addon.writeText).toHaveBeenCalledWith("out.txt", "hello");
    });

    it("convText passes text, delimiter, then target", () => {
        expect(api.convText("a,b", "out.tsv", ",")).toBe(20);
        expect(addon.convText).toHaveBeenCalledWith("a,b", ",", "out.tsv");
    });

    it("convFile passes source then target", () => {
        expect(api.convFile("in.csv", "out.tsv")).toBe(10);
        expect(addon.convFile).toHaveBeenCalledWith("in.csv", "out.tsv");
    });

    it("readText passes the source file through", () => {
        expect(api.readText("in.txt")).toBe("text");
        expect(addon.readText).toHaveBeenCalledWith("in.txt");
    });

    it("fileTypes returns the delimiter map", () => {
        expect(api.fileTypes()).toEqual({ csv: ",", tsv: "\t", psv: "|" });
    });

    it("lets addon errors reach the caller", () => {
        addon.fileArray.mockImplementationOnce(() => {
            throw new Error("Cannot read missing.csv");
        });

        expect(() => api.readArray("missing.csv")).toThrow("Cannot read missing.csv");
    });
});
