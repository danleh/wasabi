global.Wasabi = require('./node_modules/wasm-rsa/wasm/nodejs/rsa_lib_bg.wasabi.js');

let analysis = require('./../../analysis.js');

const RSASetup = require('wasm-rsa')

async function main() {
  firstInstance = await RSASetup.default()
  secondInstance = await RSASetup.default()

  const private_keys = firstInstance.generateRSAPrivate(512)
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

  console.log(calledFuncs)
    
  require("./../../collect-data.js")
}

main()


