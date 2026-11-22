"use strict";

const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");

const api = require("../index");

const PORT = Number(process.env.PORT) || 4321;
const EXT_BY_DELIM = { ",": "csv", "\t": "tsv", "|": "psv" };
const PAGE = path.join(__dirname, "index.html");

const extFor = (delim) => {
    if (!EXT_BY_DELIM[delim]) {
        throw new Error(`Unsupported delimiter ${JSON.stringify(delim)}`);
    }

    return EXT_BY_DELIM[delim];
};

const withTempDir = (work) => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "ntf-demo-"));

    try {
        return work(dir);
    } finally {
        fs.rmSync(dir, { recursive: true, force: true });
    }
};

const parseText = (text, delim) => withTempDir((dir) => {
    const file = path.join(dir, `in.${extFor(delim)}`);

    fs.writeFileSync(file, text);

    return api.readArray(file);
});

const convertText = (text, delim, target) => withTempDir((dir) => {
    const file = path.join(dir, `out.${target}`);

    api.convText(text, file, delim);

    return api.readText(file);
});

const readBody = (req) => new Promise((resolve, reject) => {
    const chunks = [];

    req.on("data", (chunk) => chunks.push(chunk));
    req.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
    req.on("error", reject);
});

const sendJson = (res, status, body) => {
    res.writeHead(status, { "Content-Type": "application/json" });
    res.end(JSON.stringify(body));
};

const handleParse = ({ text, delim }) => ({ rows: parseText(text, delim) });

const handleConvert = ({ text, delim, target }) => ({ output: convertText(text, delim, target) });

const ROUTES = {
    "POST /api/parse": handleParse,
    "POST /api/convert": handleConvert,
    "GET /api/types": () => api.fileTypes(),
};

const runRoute = async (req, res, route) => {
    try {
        const raw = req.method === "POST" ? await readBody(req) : "{}";

        sendJson(res, 200, route(JSON.parse(raw || "{}")));
    } catch (err) {
        sendJson(res, 400, { error: err.message });
    }
};

const sendPage = (res) => {
    res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
    res.end(fs.readFileSync(PAGE));
};

const sendNoContent = (res) => {
    res.writeHead(204);
    res.end();
};

const handle = (req, res) => {
    const route = ROUTES[`${req.method} ${req.url}`];

    if (req.url === "/favicon.ico") {
        return sendNoContent(res);
    }

    if (route) {
        return runRoute(req, res, route);
    }

    if (req.method === "GET" && req.url === "/") {
        return sendPage(res);
    }

    sendJson(res, 404, { error: "Not found" });
};

const createServer = () => http.createServer(handle);

if (require.main === module) {
    createServer().listen(PORT, "127.0.0.1", () => {
        console.log(`neon-text-formatter demo on http://127.0.0.1:${PORT}`);
    });
}

module.exports = { createServer, parseText, convertText };
