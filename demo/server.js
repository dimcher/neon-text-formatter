"use strict";

const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");

const api = require("../index");

const DEFAULT_PORT = 4321;
const MAX_BODY = 1024 * 1024;
const LOCAL_HOSTS = ["127.0.0.1", "localhost"];
const EXT_BY_DELIM = { ",": "csv", "\t": "tsv", "|": "psv" };
const TARGETS = Object.values(EXT_BY_DELIM);
const PAGE = path.join(__dirname, "index.html");

class HttpError extends Error {
    constructor(status, message) {
        super(message);
        this.status = status;
    }
}

const readPort = (value) => {
    if (value === undefined) {
        return DEFAULT_PORT;
    }

    const port = Number(value);

    if (!Number.isInteger(port) || port < 1 || port > 65535) {
        throw new Error("PORT must be a whole number from 1 to 65535");
    }

    return port;
};

const extFor = (delim) => {
    if (typeof delim !== "string" || !Object.hasOwn(EXT_BY_DELIM, delim)) {
        throw new Error(`Unsupported delimiter ${JSON.stringify(delim)}`);
    }

    return EXT_BY_DELIM[delim];
};

const checkTarget = (target) => {
    const name = typeof target === "string" ? target.toLowerCase() : "";

    if (!TARGETS.includes(name)) {
        throw new Error(`Unsupported file type ${JSON.stringify(target)}. Use csv, tsv or psv`);
    }

    return name;
};

const checkBody = (body) => {
    if (body === null || typeof body !== "object" || typeof body.text !== "string") {
        throw new Error("Send a JSON object with a text string");
    }

    return body;
};

const withTempDir = (work) => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "ntf-demo-"));

    try {
        return work(dir);
    } finally {
        fs.rmSync(dir, { recursive: true, force: true });
    }
};

const parseText = (text, delim) => {
    const ext = extFor(delim);

    return withTempDir((dir) => {
        const file = path.join(dir, `in.${ext}`);

        fs.writeFileSync(file, text);

        return api.readArray(file);
    });
};

const convertText = (text, delim, target) => {
    extFor(delim);
    const ext = checkTarget(target);

    return withTempDir((dir) => {
        const file = path.join(dir, `out.${ext}`);

        api.convText(text, file, delim);

        return api.readText(file);
    });
};

const readBody = (req) => new Promise((resolve, reject) => {
    const chunks = [];
    let size = 0;

    const onData = (chunk) => {
        size += chunk.length;

        if (size > MAX_BODY) {
            req.off("data", onData);
            req.resume();
            reject(new HttpError(413, "Body is too large"));
            return;
        }

        chunks.push(chunk);
    };

    req.on("data", onData);
    req.on("end", () => resolve(Buffer.concat(chunks).toString("utf8")));
    req.on("error", reject);
});

const sendJson = (res, status, body) => {
    res.writeHead(status, { "Content-Type": "application/json" });
    res.end(JSON.stringify(body));
};

const handleParse = (body) => {
    const { text, delim } = checkBody(body);

    return { rows: parseText(text, delim) };
};

const handleConvert = (body) => {
    const { text, delim, target } = checkBody(body);

    return { output: convertText(text, delim, target) };
};

const ROUTES = {
    "POST /api/parse": handleParse,
    "POST /api/convert": handleConvert,
    "GET /api/types": () => api.fileTypes(),
};

const hostName = (host) => String(host || "").split(":")[0];

const badHost = (req) => !LOCAL_HOSTS.includes(hostName(req.headers.host));

const sameOrigin = (req) => {
    const origin = req.headers.origin;

    if (origin === undefined) {
        return true;
    }

    try {
        return new URL(origin).host === req.headers.host;
    } catch (err) {
        return false;
    }
};

const isJson = (req) => String(req.headers["content-type"] || "").startsWith("application/json");

const refusalFor = (req) => {
    if (badHost(req)) {
        return new HttpError(403, "Use http://127.0.0.1 or http://localhost");
    }

    if (!sameOrigin(req)) {
        return new HttpError(403, "Cross-site requests are not allowed");
    }

    if (req.method === "POST" && !isJson(req)) {
        return new HttpError(415, "Send Content-Type: application/json");
    }

    return null;
};

const runRoute = async (req, res, route) => {
    try {
        const raw = req.method === "POST" ? await readBody(req) : "{}";

        sendJson(res, 200, route(JSON.parse(raw || "{}")));
    } catch (err) {
        sendJson(res, err.status || 400, { error: err.message });
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

const pathOf = (req) => req.url.split("?")[0];

const isPageRequest = (req, pathname) => ["GET", "HEAD"].includes(req.method) && pathname === "/";

const handle = (req, res) => {
    const refusal = refusalFor(req);
    const pathname = pathOf(req);
    const route = ROUTES[`${req.method} ${pathname}`];

    if (refusal) {
        return sendJson(res, refusal.status, { error: refusal.message });
    }

    if (pathname === "/favicon.ico") {
        return sendNoContent(res);
    }

    if (route) {
        return runRoute(req, res, route);
    }

    if (isPageRequest(req, pathname)) {
        return sendPage(res);
    }

    sendJson(res, 404, { error: "Not found" });
};

const createServer = () => http.createServer(handle);

const reportListenError = (err, port) => {
    const busy = err.code === "EADDRINUSE";

    console.error(busy ? `Port ${port} is in use. Set PORT to another number.` : err.message);
    process.exitCode = 1;
};

const start = () => {
    const port = readPort(process.env.PORT);
    const server = createServer();

    server.on("error", (err) => reportListenError(err, port));
    server.listen(port, "127.0.0.1", () => {
        console.log(`neon-text-formatter demo on http://127.0.0.1:${port}`);
    });
};

if (require.main === module) {
    try {
        start();
    } catch (err) {
        console.error(err.message);
        process.exitCode = 1;
    }
}

module.exports = { createServer, parseText, convertText };
