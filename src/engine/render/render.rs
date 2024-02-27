use js_sys::Float32Array;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/engine/render/gfx.js")]
extern {
    pub type Gfxrender;
    #[wasm_bindgen(constructor)]
    pub fn new(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Gfxrender;

    #[wasm_bindgen(method)]
    pub fn gfxgetcanvassizex(this: &Gfxrender) -> i32;

    #[wasm_bindgen(method)]
    pub fn gfxgetcanvassizey(this: &Gfxrender) -> i32;

    #[wasm_bindgen(method)]
    pub fn gfxsetrenderscale(this: &Gfxrender,renderscale: f32);

    #[wasm_bindgen(method)]
    pub fn gfxsetshadowmapres(this: &Gfxrender,shadowmapres: i32);

    #[wasm_bindgen(method)]
    pub fn gfxbeginpass(this: &Gfxrender, lop: &str, dlop: &str);

    #[wasm_bindgen(method)]
    pub fn gfxbeginmainpass(this: &Gfxrender, lop: &str, dlop: &str);

    #[wasm_bindgen(method)]
    pub fn gfxbeginshadowpass(this: &Gfxrender, dlop: &str);

    #[wasm_bindgen(method)]
    pub fn gfxendpass(this: &Gfxrender);

    #[wasm_bindgen(method)]
    fn gfxfinishrender(this: &Gfxrender);

    pub type Gfxmesh;
    #[wasm_bindgen(constructor)]
    pub fn create(gfx: &Gfxrender, vertices: Float32Array, uv: Float32Array, normals: Float32Array, lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, magfilter: &str, minfilter: &str, forpost: bool) -> Gfxmesh;

    #[wasm_bindgen(method)]
    pub fn draw(this: &Gfxmesh, gfx: &Gfxrender, uniformValues: Float32Array);

    pub type Gpucompute;
    #[wasm_bindgen(constructor)]
    pub fn createcompute(ibs: i32, obs: i32, code: &str) -> Gpucompute;

    #[wasm_bindgen(method)]
    pub fn execute(this: &Gpucompute, ib: &Float32Array, workgroupsize: i32);

    #[wasm_bindgen(method)]
    pub fn getstate(this: &Gpucompute) -> bool;

    #[wasm_bindgen(method)]
    pub fn getresult(this: &Gpucompute) -> Float32Array;
}

pub struct Render{
    pub jsren: Gfxrender,
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
      .request_animation_frame(f.as_ref().unchecked_ref())
      .expect("should register `requestAnimationFrame` OK");
}

#[allow(dead_code)]
pub fn drawloopexec(mut f: impl FnMut() + 'static){
    let mut execfunc = move || {
      f();
    };
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
      execfunc();
      request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

impl Render{
    #[allow(dead_code)]
    pub fn init(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Render{
        Render{
            jsren: Gfxrender::new(canvasid, renderscale, shadowmapres)
        }
    }
    #[allow(dead_code)]
    pub fn get_canvas_size_x(&self) -> i32{
        self.jsren.gfxgetcanvassizex()
    }
    #[allow(dead_code)]
    pub fn get_canvas_size_y(&self) -> i32{
        self.jsren.gfxgetcanvassizey()
    }
    #[allow(dead_code)]
    pub fn change_render_scale(&self, renderscale: f32){
        self.jsren.gfxsetrenderscale(renderscale);
    }
    #[allow(dead_code)]
    pub fn change_shadow_map_resolution(&self, renderscale: i32){
        self.jsren.gfxsetshadowmapres(renderscale);
    }
    #[allow(dead_code)]
    pub fn begin_shadow_pass(&self, dlop: &str){
        self.jsren.gfxendpass();
        self.jsren.gfxbeginshadowpass(dlop);
    }
    #[allow(dead_code)]
    pub fn begin_main_pass(&self, lop: &str, dlop: &str){
        self.jsren.gfxendpass();
        self.jsren.gfxbeginmainpass(lop, dlop);
    }
    #[allow(dead_code)]
    pub fn begin_post_pass(&self, lop: &str, dlop: &str){
        self.jsren.gfxendpass();
        self.jsren.gfxbeginpass(lop, dlop);
    }
    #[allow(dead_code)]
    pub fn end_render(&self){
        self.jsren.gfxendpass();
        self.jsren.gfxfinishrender();
    }
}