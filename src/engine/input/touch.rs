use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/engine/input/touch.js")]
extern {
    fn jgettx() -> i32;
    fn jgetty() -> i32;
    fn jgetuse() -> i32;
    fn jsettouchindex(lindex: i32);
}


#[allow(dead_code)]
pub fn get_touch_x() -> i32{
    jgettx()
}

#[allow(dead_code)]
pub fn get_touch_y() -> i32{
    jgetty()
}

#[allow(dead_code)]
pub fn get_is_touching() -> bool{
    jgetuse() == 1
}

#[allow(dead_code)]
pub fn set_touch_index(lindex: i32){
    jsettouchindex(lindex);
}