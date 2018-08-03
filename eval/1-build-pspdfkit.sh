#!/bin/bash
cd programs/pspdfkit-webassembly-benchmark-master/

# needs download key, register for trail to get
npm install --save https://customers.pspdfkit.com/npm/TRIAL-oL6A8_VBUi1QGq68jzqPZ7B6OgMoqr7oAOQ8pXKOtZJUY4r-77rdz_fp3kTBbfUX9aOtbNz-PZF7nadrRWcItA/latest.tar.gz

# from their installation instructions
mkdir -p public/vendor
cp -r node_modules/pspdfkit/dist public/vendor/pspdfkit
npm install

# needs license key, register for trail to get
echo "kffP4ACBJVXTetVoyMiq0PAYaDCJKUDLwScSXoktG0mKLgSE9vtTFfvwvrSgEwdrZFkp4k4_4oJjSnMP7L9KAuD3yZj50OUQR9zFs9exY3gj3O56ft9XZ2R-_QAs1wqeZdp95zJ3V6bjo_DwqNmM9t8o6zLM1-6E45pXqRQnVEpcOumVMxacTn15_FwGlikMEnLHQux0oobMuO0n3yx7zhS6OHUZsRnaEZzfJe6xnw5fd9Nb5FJhP5yf96EnNt33NNyogHnkYON2cOSZ_FYGmu17c6W31xk7qSUGPeMlCdvOEhg-BRUEtLjolUjZMscxWEP0WJXnNSQEBUyfkyrcyCPT_hreJBmGrpMqPekT5NSo-7-sY7aWuBOxZQYAhXkKL72R20irjU_6Ginni_B_JnKbeFT4Dytn7jfAAFqwHWYxz-d1hcOABqdcl6J5Tatq" > public/license-key

# insert emrun compatible reporting of benchmark result (exclusive loading, since that is how PolyBench and Unreal Engine also report their results)
sed -i '62i\    fetch("stdio.html", { method: "POST", body: "^out^0^"+ score.rest }).then(() => fetch("stdio.html", { method: "POST", body: "^exit^0" })).then(() => window.close());' src/index.js

# build minified version
npm run build

cd - > /dev/null