use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = updateArray)]
pub fn update_array(array: Vec<i32>) -> Vec<i32> {
    array.iter().map(|x| x * x).collect()
}

#[wasm_bindgen(js_name = insertDiv)]
pub fn insert_div() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should be a window");
    let document = window.document().expect("should be a document");
    let body = document.body().expect("document should have a body");

    let div = document.create_element("div")?;
    div.set_inner_html("Hello from Rust!");

    body.append_child(&div)?;

    Ok(())
}
