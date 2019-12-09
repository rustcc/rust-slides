const wasm = require('./pkg');

const array = [1, 2, 5];
console.log(array);

const newArray = wasm.updateArray(array);
console.log(newArray);
