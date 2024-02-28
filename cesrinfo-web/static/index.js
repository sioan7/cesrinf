import init, { CesrParser } from "/static/cesrinf_wasm.js";

await init();

document.getElementById("parse-btn").addEventListener("click", () => {
    let input = document.getElementById("parse-ta").value;
    let parser = new CesrParser(input);
    let parsed_data = parser.parse();
    console.log(parsed_data);
    console.table(parsed_data.msgs[0].codeage.description);
    document.getElementById("parse-fin").value = JSON.stringify(parsed_data);
});
