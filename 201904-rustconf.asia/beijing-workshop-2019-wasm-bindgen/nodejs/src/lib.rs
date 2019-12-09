use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = updateArray)]
pub fn update_array(array: Vec<i32>) -> Vec<i32> {
    array.iter().map(|x| x * x).collect()
}
