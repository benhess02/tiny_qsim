const fs = require("fs");
const path = require("path");
const esbuild = require("esbuild");

esbuild.buildSync({
    entryPoints: [path.join("src", "index.ts")],
    outfile: path.join("dist", "bundle.js")
});

fs.cpSync("static", "dist", { recursive: true });

fs.cpSync(
    path.join("..", "target", "wasm32-unknown-unknown", "release", "demo_core.wasm"),
    path.join("dist", "core.wasm")
);