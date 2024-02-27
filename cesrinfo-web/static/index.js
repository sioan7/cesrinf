import init, { constant } from "/static/cesrinf_wasm.js";

await init();

document.getElementById("parse-btn").addEventListener("click", () => {
    document.getElementById("parse-fin").value = constant();
});
