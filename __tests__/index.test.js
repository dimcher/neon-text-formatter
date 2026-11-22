"use strict";

jest.mock("../binding", () => ({
    convFile: jest.fn(),
    convText: jest.fn(),
    readText: jest.fn(),
    writeText: jest.fn(),
    fileTypes: jest.fn(),
    readArray: jest.fn(),
    readObject: jest.fn(),
}), { virtual: true });

const binding = require("../binding");
const api = require("../index");

describe("index.js", () => {
    it.each(["convFile", "convText", "readText", "writeText", "fileTypes", "readArray", "readObject"])(
        "exports %s straight from the binding",
        (name) => {
            expect(api[name]).toBe(binding[name]);
        }
    );

    it("exports nothing else", () => {
        expect(Object.keys(api).sort()).toEqual(
            ["convFile", "convText", "fileTypes", "readArray", "readObject", "readText", "writeText"]
        );
    });
});
