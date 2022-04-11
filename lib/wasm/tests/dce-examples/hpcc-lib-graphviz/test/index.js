global.Wasabi = require('./node_modules/@hpcc-js/wasm/dist/graphvizlib.wasabi.js');

let analysis = require('./../../analysis.js');

var hpccWasm = require("@hpcc-js/wasm")

const dot = `
    digraph G {
        node [shape=rect];
        subgraph cluster_0 {
            style=filled;
            color=lightgrey;
            node [style=filled,color=white];
            a0 -> a1 -> a2 -> a3;
            label = "process #1";
        }
        subgraph cluster_1 {
            node [style=filled];
            b0 -> b1 -> b2 -> b3;
            label = "process #2";
            color=blue
        }
        start -> a0;
        start -> b0;
        a1 -> b3;
        b2 -> a3;
        a3 -> a0;
        a3 -> end;
        b3 -> end;
        start [shape=Mdiamond,label=<<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0" >
                <TR>
                <TD COLSPAN="3" BGCOLOR="azure3" >
                    <b><FONT FACE="Times-Roman" POINT-SIZE="12.0" >one obj</FONT></b>
                </TD>
                </TR>
                    <TR>
                        <TD PORT="in1" BGCOLOR="white">in1<br/>:In1</TD>
                        <TD ROWSPAN="2" BGCOLOR="azure3"> some text </TD>
                        <TD PORT="out1"  BGCOLOR="0.5 0.5 0.5">out1<br/>:Out1</TD>
                    </TR>
                    <TR>
                        <TD PORT="in2"   HREF="somepage.html" BGCOLOR="0.45 0.5 0.75">in2<br/>:In2</TD>
                        <TD PORT="out2"  HREF="somepage.html"   BGCOLOR="0.8 0.5 1">out2<br/>:Out2</TD>
                    </TR>
                </TABLE>>];
        end [shape=Msquare];
    }
`;

hpccWasm.graphvizSync().then(graphviz => {
    graphviz.layout(dot, "svg", "dot")
    graphviz.layout('digraph { a[image="https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png"]; }', "svg", "dot", { images: [{ path: "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png", width: "272px", height: "92px" }] })    
    require("./../../collect-data.js")
}); 
