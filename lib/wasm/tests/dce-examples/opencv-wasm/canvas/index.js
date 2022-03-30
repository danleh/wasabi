const fs = require("fs");
const Wasabi = require('./node_modules/opencv-wasm/opencv.wasabi.js');

// analysis.js 

let get_reachable_exports = false; 
let check_if_consts_are_written_to = false; 
for (const arg of process.argv.slice(2)) {
    if (arg == '--reachable-exports') {
        get_reachable_exports = true;           
    } else if (arg == "--check-constants") {
        check_if_consts_are_written_to = true; 
    } else if (arg == "--help") {
      console.log("\
OPTIONS:\n\
      --reachable-exports\tto get the exported functions that reachable\n\
      --check-constants\t\tto check that the constants in file call_indirect_load_const_addr.json are never written to\
      "); 
      process.exit(0);
  } else {
    console.log("\
OPTIONS:\n\
      --reachable-exports\tto get the exported functions that reachable\n\
      --check-constants\t\tto check that the constants in file call_indirect_load_const_addr.json are never written to\
      ");
      process.exit(0); 
  }
}

// Read array of constant addresses from which call_indirects load their function pointer.
let call_indirect_load_const_addrs;
if (check_if_consts_are_written_to) {
  call_indirect_load_const_addrs = JSON.parse(fs.readFileSync("./call_indirect_load_const_addr.json")); 
}

const calledFuncs = new Set();

function function_info(func_idx) {
    const fct = Wasabi.module.info.functions[func_idx];
    if (fct !== undefined) {
      if (get_reachable_exports && fct.export[0] != undefined) {
          return {
              function_idx: func_idx,
              export: fct.export,
              import: fct.import,
          };
      } else if (!get_reachable_exports && fct.export[0] != undefined) {
        return {
              function_idx: func_idx,
              export: fct.export,
              import: fct.import,
          };
      }
  }
}

Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        // const caller = function_info(location.func);
        const callee = function_info(targetFunc);
        // console.log(`${caller} -> ${callee}`);
        // calledFuncs.add(JSON.stringify(caller));
        calledFuncs.add(JSON.stringify(callee));
    },

    begin({func, instr}, type) {
        if (instr == -1) {
            let funcName = function_info(func);
            calledFuncs.add(JSON.stringify(funcName));
        }
    },
    
    store(location, op, memarg, value) {
        // Ensure that no store goes to these addresses above
        if (check_if_consts_are_written_to && call_indirect_load_const_addrs.includes(memarg.addr)) {
            console.error(`${location.func}:${location.instr}: violated assumption, address of function pointer is written to! addr: ${memarg.addr}`);
            console.error(function_info(location.func));
            console.error(op, JSON.stringify(memarg), value);
        }
    },
    
};


const { Canvas, createCanvas, Image, ImageData, loadImage } = require('canvas');
const { JSDOM } = require('jsdom');
const { writeFileSync, readFileSync } = require('fs');
const { cv, cvTranslateError } = require('opencv-wasm');

// This is our program. This time we use JavaScript async / await and promises to handle asynchronicity.
(async () => {
    // before loading opencv.js we emulate a minimal HTML DOM. See the function declaration below.
    installDOM();
    // using node-canvas, we an image file to an object compatible with HTML DOM Image and therefore with cv.imread()
    const image = await loadImage('./input/image-sample-1.jpg');
    const src = cv.imread(image);
    const dst = new cv.Mat();
    const M = cv.Mat.ones(5, 5, cv.CV_8U);
    const anchor = new cv.Point(-1, -1);
    cv.dilate(src, dst, M, anchor, 1, cv.BORDER_CONSTANT, cv.morphologyDefaultBorderValue());
    // we create an object compatible HTMLCanvasElement
    const canvas = createCanvas(300, 300);
    cv.imshow(canvas, dst);
    writeFileSync('output.jpg', canvas.toBuffer('image/jpeg'));
    src.delete();
    dst.delete();
  })();
  // Load opencv.js just like before but using Promise instead of callbacks:
  function loadOpenCV() {
    return new Promise(resolve => {
      global.Module = {
        onRuntimeInitialized: resolve
      };
      global.cv = require('./opencv.js');
    });
  }
  // Using jsdom and node-canvas we define some global variables to emulate HTML DOM.
  // Although a complete emulation can be archived, here we only define those globals used
  // by cv.imread() and cv.imshow().
  function installDOM() {
    const dom = new JSDOM();
    global.document = dom.window.document;
    // The rest enables DOM image and canvas and is provided by node-canvas
    global.Image = Image;
    global.HTMLCanvasElement = Canvas;
    global.ImageData = ImageData;
    global.HTMLImageElement = Image;
  }

  if (get_reachable_exports) {
    let reachable_exports = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        if (func_info != undefined) {
            count+=1; 
            reachable_exports += JSON.parse(func_info).export[0]; 
            reachable_exports += "\n";
        }
    })
    console.log(count+" exported functions are reachable.")
    let f = fs.writeFileSync("reachable-exports.txt", reachable_exports);
} else {
    let lowerbound = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        if (func_info != undefined) {
            count += 1; 
            lowerbound += JSON.parse(func_info).function_idx; 
            lowerbound += "\n";
        }
    })
    console.log(count+" functions are the lower bound for the analysis.")
    let f = fs.writeFileSync("lowerbound-reachable-functions.txt", lowerbound);
}

