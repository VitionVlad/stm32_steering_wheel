use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/mouse.js")]
extern {
    fn jgetx() -> i32;
    fn jgety() -> i32;
}

#[allow(dead_code)]
pub fn get_mouse_x() -> i32{
    jgetx()
}

#[allow(dead_code)]
pub fn get_mouse_y() -> i32{
    jgety()
}