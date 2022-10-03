git clone https://github.com/kripken/BananaBread.git
pushd BananaBread/cube2/src/web
emmake make
cd ../..
zip -9 -r ../../bb.zip assets base.data bb.html bb.js bb.wasm character.data css game js low.data
popd
