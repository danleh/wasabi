const fs = require("fs");
const Wasabi = require('./node_modules/wasm-rsa/wasm/nodejs/rsa_lib_bg.wasabi.js');

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


const RSASetup = require('wasm-rsa')

async function main() {
  firstInstance = await RSASetup.default()
  secondInstance = await RSASetup.default()

  const private_keys = firstInstance.generateRSAPrivate(2048)
  const public_keys = secondInstance.createRSAPublic(private_keys.n, private_keys.e)

  console.log("PRIVATE KEYS:", { d: Buffer.from(private_keys.d).toString('base64') })
  console.log("PRIVATE PEM:",  private_keys.pem)
  console.log("PUBLIC KEYS:", { n: Buffer.from(public_keys.n).toString('base64') })
  console.log("PUBLIC PEM:",  public_keys.pem)

  const message = "test_message"

  // Sign/Verify
  const signature = firstInstance.signMessage(message)
  console.log("SIGNATURE:", signature)
  const verify = secondInstance.verify(message, signature)
  console.log("VERIFY SIGNATURE:", verify)

  // Encrypt / Decrypt
  const cipher = secondInstance.publicEncrypt(message)
  console.log("CIPHER TEXT:", cipher)
  const decryptedMessage = firstInstance.privateDecrypt(cipher)
  console.log("DECRYPTED MESSAGE", decryptedMessage)
}

main()



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