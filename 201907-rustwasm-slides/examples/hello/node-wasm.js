const fs = require('fs');
const bytes = fs.readFileSync('./lib.wasm');

(async () => {
  const module = await WebAssembly.compile(bytes);
  const instance = await WebAssembly.instantiate(module);

  console.log(instance.exports.add_one(8));
})();
