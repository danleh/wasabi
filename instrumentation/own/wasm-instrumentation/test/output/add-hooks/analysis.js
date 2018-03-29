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
    executedFunctions.add(targetFunc);
}
