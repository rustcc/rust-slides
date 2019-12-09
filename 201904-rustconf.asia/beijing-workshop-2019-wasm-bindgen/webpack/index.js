import('./pkg') // webpack
    .then(runExample)
    .catch(console.error);

function runExample(wasm) {
    const array = [2, 5];

    document.body.innerHTML += array.join(', ') + '<br>';

    const newArray = wasm.updateArray(array);

    document.body.innerHTML += newArray.join(', ') + '<br>';

    wasm.insertDiv();
}
