"use strict";

const fs = require("fs");
const http = require("http");
const path = require("path");

const { addonBuilt } = require("../test-support/addon");
const describeWithAddon = addonBuilt ? describe : describe.skip;

if (process.env.REQUIRE_ADDON === "1") {
    it("has a built the napi addon (*.node) (REQUIRE_ADDON=1)", () => {
        expect(addonBuilt).toBe(true);
    });
}

describeWithAddon("demo server (needs a built the napi addon (*.node))", () => {
    let server;
    let base;

    beforeAll(async () => {
        const { createServer } = require("../demo/server");

        server = createServer();
        await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
        base = `http://127.0.0.1:${server.address().port}`;
    });

    afterAll(() => new Promise((resolve) => server.close(resolve)));

    const JSON_HEADERS = { "Content-Type": "application/json" };

    const post = (url, body, headers = JSON_HEADERS) => fetch(base + url, {
        method: "POST",
        headers,
        body: typeof body === "string" ? body : JSON.stringify(body),
    });

    const statusWithHost = (host) => new Promise((resolve, reject) => {
        const req = http.request(base + "/api/types", { headers: { Host: host } }, (res) => {
            res.resume();
            resolve(res.statusCode);
        });

        req.on("error", reject);
        req.end();
    });

    const errorOf = async (res) => (await res.json()).error;

    it("serves the page", async () => {
        const res = await fetch(base + "/");

        expect(res.status).toBe(200);
        expect(await res.text()).toContain("Text Formatter Demo");
    });

    it("parses a multi-line quoted csv field", async () => {
        const res = await post("/api/parse", { text: "\"a\nb\",c", delim: "," });

        expect(await res.json()).toEqual({ rows: [["a\nb", "c"]] });
    });

    it("converts csv text to tsv with the line break flattened", async () => {
        const res = await post("/api/convert", { text: "\"a\nb\",c", delim: ",", target: "tsv" });

        expect(await res.json()).toEqual({ output: "a b\tc" });
    });

    it("lists the file types", async () => {
        const res = await fetch(base + "/api/types");

        expect(await res.json()).toEqual({ csv: ",", tsv: "\t", psv: "|" });
    });

    it("answers 400 with the addon error for an unsupported target", async () => {
        const res = await post("/api/convert", { text: "a,b", delim: ",", target: "txt" });

        expect(res.status).toBe(400);
        expect((await res.json()).error).toMatch(/Unsupported file type/);
    });

    it("answers 400 for an unknown delimiter", async () => {
        const res = await post("/api/parse", { text: "a;b", delim: ";" });

        expect(res.status).toBe(400);
    });

    it("answers the browser favicon request without an error", async () => {
        const res = await fetch(base + "/favicon.ico");

        expect(res.status).toBe(204);
    });

    it("answers 404 for an unknown path", async () => {
        const res = await fetch(base + "/nope");

        expect(res.status).toBe(404);
    });

    it("answers 400 for an unknown delimiter on convert too", async () => {
        const res = await post("/api/convert", { text: "a;b", delim: ";", target: "csv" });

        expect(res.status).toBe(400);
        expect(await errorOf(res)).toMatch(/Unsupported delimiter/);
    });

    it("answers 400 for a delimiter named like an object property", async () => {
        const res = await post("/api/parse", { text: "a", delim: "toString" });

        expect(res.status).toBe(400);
        expect(await errorOf(res)).toMatch(/Unsupported delimiter/);
    });

    it("rejects a target that tries to leave the temp folder", async () => {
        const res = await post("/api/convert", { text: "x,y", delim: ",", target: "csv/../../ntfprobe.csv" });

        expect(res.status).toBe(400);
        expect(await errorOf(res)).toMatch(/Unsupported file type/);
    });

    it("accepts an upper-case target", async () => {
        const res = await post("/api/convert", { text: "a,b", delim: ",", target: "TSV" });

        expect(await res.json()).toEqual({ output: "a\tb" });
    });

    it("answers 400 with a plain message for a body that is not an object", async () => {
        const res = await post("/api/parse", "null");

        expect(res.status).toBe(400);
        expect(await errorOf(res)).toBe("Send a JSON object with a text string");
    });

    it("answers 400 when text is not a string", async () => {
        const res = await post("/api/parse", { text: 5, delim: "," });

        expect(res.status).toBe(400);
        expect(await errorOf(res)).toBe("Send a JSON object with a text string");
    });

    it("answers 400 for malformed JSON", async () => {
        const res = await post("/api/parse", "{nope");

        expect(res.status).toBe(400);
    });

    it("answers 415 when the content type is not json", async () => {
        const res = await post("/api/parse", { text: "a", delim: "," }, { "Content-Type": "text/plain" });

        expect(res.status).toBe(415);
    });

    it("answers 413 for a body over 1 MB and keeps serving", async () => {
        const res = await post("/api/parse", { text: "a".repeat(1100000), delim: "," });

        expect(res.status).toBe(413);
        expect((await fetch(base + "/api/types")).status).toBe(200);
    });

    it("refuses a request from another host name", async () => {
        expect(await statusWithHost("evil.example:4321")).toBe(403);
        expect(await statusWithHost("localhost:4321")).toBe(200);
    });

    it("refuses a request from another origin", async () => {
        const res = await fetch(base + "/api/types", { headers: { Origin: "http://evil.example" } });

        expect(res.status).toBe(403);
    });

    it("allows a request from its own origin", async () => {
        const res = await fetch(base + "/api/types", { headers: { Origin: base } });

        expect(res.status).toBe(200);
    });

    it("ignores a query string on a route and on the page", async () => {
        expect((await fetch(base + "/?x=1")).status).toBe(200);
        expect((await fetch(base + "/api/types?x")).status).toBe(200);
    });

    it("answers a HEAD request for the page", async () => {
        const res = await fetch(base + "/", { method: "HEAD" });

        expect(res.status).toBe(200);
    });
});
