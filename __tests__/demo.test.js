"use strict";

const fs = require("fs");
const path = require("path");

const addonBuilt = fs.existsSync(path.resolve(__dirname, "../neon/native/index.node"));
const describeWithAddon = addonBuilt ? describe : describe.skip;

describeWithAddon("demo server (needs a built neon/native/index.node)", () => {
    let server;
    let base;

    beforeAll(async () => {
        const { createServer } = require("../demo/server");

        server = createServer();
        await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
        base = `http://127.0.0.1:${server.address().port}`;
    });

    afterAll(() => new Promise((resolve) => server.close(resolve)));

    const post = (url, body) => fetch(base + url, { method: "POST", body: JSON.stringify(body) });

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

    it("answers 404 for an unknown path", async () => {
        const res = await fetch(base + "/nope");

        expect(res.status).toBe(404);
    });
});
