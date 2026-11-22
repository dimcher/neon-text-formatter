"use strict";

const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const neonDir = path.resolve(__dirname, "../neon");
const output = path.join(neonDir, "native", "index.node");

const libraryName = () => {
    const names = { darwin: "libneon.dylib", linux: "libneon.so", win32: "neon.dll" };
    const name = names[process.platform];

    if (!name) {
        throw new Error(`Unsupported platform: ${process.platform}`);
    }

    return name;
};

const cargoBuild = () => {
    const manifest = path.join(neonDir, "native", "Cargo.toml");
    const result = spawnSync("cargo", ["build", "--release", "--manifest-path", manifest], { stdio: "inherit" });

    if (result.status !== 0) {
        throw new Error("cargo build failed");
    }
};

const copyLibrary = () => {
    const built = path.join(neonDir, "target", "release", libraryName());

    fs.copyFileSync(built, output);
    console.log(`Wrote ${output}`);
};

try {
    cargoBuild();
    copyLibrary();
} catch (err) {
    console.error(err.message);
    process.exit(1);
}
