/*
 * User-facing API for dynamic analyses.
 */

const executedFunctions = new Set();

function begin(location, type) {
    if (type === "function") {
        executedFunctions.add(location.func);
    }
}

function call_(location, targetFunc, indirect, args) {
    if (indirect)
        console.log(location, targetFunc, args);
    executedFunctions.add(targetFunc);
}

// function analysisResults() {
//     const fns = [...executedFunctions].sort((a, b) => a - b);
//     for (const fn of fns) {
//         console.log(fn, staticInfo.functions[fn].export);
//     }
// }
//
// setTimeout(analysisResults, 2000);