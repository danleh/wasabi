// Add console.save() function that opens a download dialog with given data and filename.
// See https://stackoverflow.com/questions/11849562/how-to-save-the-output-of-a-console-logobject-to-a-file
(function (console) {
    console.save = function (data, filename) {
        if (!data) {
            console.error('Console.save: No data')
            return;
        }

        if (!filename) filename = 'console.json'

        if (typeof data === "object") {
            data = JSON.stringify(data, undefined, 4)
        }

        var blob = new Blob([data], { type: 'text/json' }),
            e = document.createEvent('MouseEvents'),
            a = document.createElement('a')

        a.download = filename
        a.href = window.URL.createObjectURL(blob)
        a.dataset.downloadurl = ['text/json', a.download, a.href].join(':')
        e.initMouseEvent('click', true, false, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null)
        a.dispatchEvent(e)
    }
})(console);

// Convert a set of (string) edges of the form "a -> b" to a call graph in .dot format
// and open the download dialog to save it.
function saveCallGraph(edgeSet) {
    // Start a directed graph.
    let dotFile = "digraph {\n";
    // Remove special characters in vertex names.
    for (const edge of edgeSet) {
        let [wholeMatch, from, to] = edge.match(/([^ ]*) *-> *([^ ]*)/);
        from = from.replace(/[^a-zA-Z0-9]/g,'_');
        to = to.replace(/[^a-zA-Z0-9]/g,'_');
        dotFile += `  ${from} -> ${to};\n`;
    }
    dotFile += "}\n";
    
    console.save(dotFile, "call-graph.dot");
}
