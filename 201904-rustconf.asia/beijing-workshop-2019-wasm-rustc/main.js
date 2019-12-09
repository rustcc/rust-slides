async function main() {
    const module = await WebAssembly.instantiateStreaming(
        fetch('lib.wasm'),
        { env: {
            consume_array(offset, length) {
                const { memory } = module.instance.exports;
                const array = new Int32Array(memory.buffer).subarray(
                    offset / 4,
                    offset / 4 + length,
                );
                document.body.innerHTML += array.toString();
            }
        }}
    );
    const { add, array_example } = module.instance.exports;
    document.body.innerHTML = `add(2, 3) is ${add(2, 3)}<br>`;
    array_example();
}

window.addEventListener('load', async () => {
    await main();
});
