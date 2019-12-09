import init, { updateArray, insertDiv } from './pkg/es6.js';

window.addEventListener('load', async () => {
    await init('./pkg/es6_bg.wasm');

    const array = [1, 2, 3];
    document.body.innerHTML = array.join(', ') + '<br>';

    const newArray = updateArray(array);
    document.body.innerHTML += newArray.join(', ') + '<br>';

    insertDiv();
});
