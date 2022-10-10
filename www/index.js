import * as wasm from "wasmtest";

const canvas = document.getElementById("canvas");

document.getElementById("start").addEventListener("click", () => {
    start.style.display = "none";
    canvas.style.display = "block";

    const w = window.innerWidth;
    const h = window.innerHeight;

    canvas.width  = w;
    canvas.height = h;

    wasm.start();
});