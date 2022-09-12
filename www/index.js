import * as wasm from "wasmtest";

const w = window.innerWidth;
const h = window.innerHeight;

document.getElementById("canvas").width  = w;
document.getElementById("canvas").height = h;
wasm.start();