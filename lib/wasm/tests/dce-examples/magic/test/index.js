global.Wasabi = require('./node_modules/@npcz/magic/dist/magic-js.wasabi.js');

let analysis = require('./../../analysis.js');


var fs = require("fs")
var path = require('path')
var { MagicBindingModule, MagicBindingInterface } = require('@npcz/magic');

const createBindingModule = require('@npcz/magic/dist/magic-js');

const magicFile = require.resolve('@npcz/magic/dist/magic.mgc');

createBindingModule().then((binding) => {
    console.log(binding.MagicBinding);
    console.log(`Magic version : ${binding.MagicBinding.version()}`);
  
    // We can only use MAGIC_PRESERVE_ATIME on operating suystems that support
    // it and that includes OS X for example. It's a good practice as we don't
    // want to change the last access time because we are just checking the file
    // contents type
    let flags = 0;
    if (process.platform === 'darwin' || process.platform === 'linux') {
      flags = binding.MAGIC_PRESERVE_ATIME;
    }
  
    if (-1 === binding.MagicBinding.init(magicFile, flags)) {
      console.error('Initialization failed!');
      return;
    }
  
    const magic = new binding.MagicBinding();
  
    const files = fs.readdirSync('.');
    console.log(`${files.length} files to check`);
    files.forEach((file) => {
      console.log(
        file,
        ' : ',
        magic.detect(file, binding.MagicBinding.flags() | binding.MAGIC_MIME)
      );
      console.log(file, ' : ', magic.detect(file, -1));
    });
  
    binding.MagicBinding.destroy();

    require("./../../collect-data.js")
  });