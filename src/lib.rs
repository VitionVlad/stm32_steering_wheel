use engine::engine::Engine;
use engine::render::compute::Compute;
use engine::math::uniformstruct::{createmvpmat, createsmvpmat, createvec4, Uniformstruct};
use engine::math::vec4::Vec4;
use engine::object::Object;
use engine::input::keyboard::is_key_pressed;
use engine::input::mouse::{get_mouse_x, get_mouse_y};
use engine::input::touch::*;
use wasm_bindgen::prelude::*;
mod engine;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    const SPEED: f32 = 0.1f32;
    let mut eng: Engine = Engine::new("render", 1f32, 4000);

    let vertices: [f32; 24] = [
        -1.0, -1.0, -0.5, 1.0,
        -1.0, 1.0, -0.5, 1.0,
        1.0, 1.0, -0.5, 1.0,

        -1.0, -1.0, -0.8, 1.0,
        1.0, 1.0, -0.8, 1.0,
        1.0, -1.0, -0.8, 1.0
    ];

    let uv: [f32; 12] = [
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,

        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
    ];

    let normals: [f32; 18] = [
        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0
    ];

    let vertc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
    }

    @vertex
    fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f) -> OUT {
      var out: OUT;
      out.position = ourStruct.mvp * vec4f(pos.xyz, 1);
      out.uv = uv;
      out.norm = n;
      return out;
    }";

    let pvertc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
    }

    @vertex
    fn vertexMain(@location(0) pos: vec4f, @location(1) uv: vec2f, @location(2) n: vec3f) -> OUT {
      var out: OUT;
      out.position = vec4f(pos.xy, 0.5, 1);
      out.uv = uv;
      out.norm = n;
      return out;
    }";

    let vertsc: &str = "
    struct OurStruct {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

    @vertex
    fn vertexMain(@location(0) pos: vec4f) -> @builtin(position) vec4f {
      return vec4f(pos.xyz, 1);
    }
    ";

    let fragc: &str = "
    struct UBO {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ubo: UBO;

    @group(0) @binding(1) var mySampler: sampler;

    @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(textureSample(myTexture, mySampler, in.uv, 0).rgb, 1);
    }";

    let pfragc: &str = "
    struct UBO {
      mvp: mat4x4<f32>,
      smvp: mat4x4<f32>,
      lightpos: vec4f,
      lightcolor: vec4f,
      playerpos: vec4f,
    };

    @group(0) @binding(0) var<uniform> ubo: UBO;

    @group(0) @binding(1) var mySampler: sampler;

    @group(0) @binding(2) var myTexture: texture_2d_array<f32>;

    @group(0) @binding(3) var shadowMap: texture_depth_2d;

    @group(0) @binding(4) var mainMap: texture_2d<f32>;

    @group(0) @binding(5) var mainDepthMap: texture_depth_2d;

    struct OUT{
      @builtin(position) position: vec4f,
      @location(0) uv: vec2f,
      @location(1) norm: vec3f,
    }

    @fragment
    fn fragmentMain(in: OUT) -> @location(0) vec4f {
      return vec4f(textureSample(mainMap, mySampler, in.uv).rgb, 1);
    }";

    let mut uniforms: Vec<Uniformstruct> = vec![];
    uniforms.push(createmvpmat());
    uniforms.push(createsmvpmat());
    uniforms.push(createvec4(Vec4::new()));
    uniforms.push(createvec4(Vec4::new()));
    uniforms.push(createvec4(Vec4::new()));

    let mut mesh: Object = Object::new(&eng, &vertices, &uv, &normals, 6, vertc, vertsc, fragc, &uniforms, "tex;spec", "linear", "linear", false);
    mesh.rot.x = 0.24f32;
    mesh.pos.x = 0.5f32;
    mesh.scale.y = 1.5f32;

    let mut renquad: Object = Object::new(&eng, &vertices, &uv, &normals, 6, pvertc, vertsc, pfragc, &uniforms, "tex", "nearest", "nearest", true);
    let mut rd = 1.0f32;

    let compute: &str = "
    @group(0) @binding(0) var<storage> in: array<f32>;
    @group(0) @binding(1) var<storage, read_write> out: array<f32>;

    @compute @workgroup_size(1) fn computeMain() {
        out[0] = in[0];
        out[1] = in[4] * in[5] * in[6] * in[7];
        out[2] = in[8] * in[9] * in[10] * in[11];
        out[3] = in[12] * in[13] * in[14] * in[15];
    }";
    let inbuf: [f32; 16] = [1.2f32; 16];
    let mut com: Compute = Compute::create(16, 4, compute);

    let drawloop = move || {
      com.execute(&inbuf);
      log(&com.out_buf[0].to_string());
      eng.rot.x += get_mouse_y() as f32/eng.ren.get_canvas_size_y()as f32;
      eng.rot.y += get_mouse_x() as f32/eng.ren.get_canvas_size_x()as f32;
      if is_key_pressed(87){
        eng.pos.z += f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
        eng.pos.x -= f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
      }
      if is_key_pressed(83){
        eng.pos.z -= f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
        eng.pos.x += f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
      }
      if is_key_pressed(65){
        eng.pos.x += f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
        eng.pos.z += f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
      }
      if is_key_pressed(68){
        eng.pos.x -= f32::cos(eng.rot.x) * f32::cos(eng.rot.y) * SPEED;
        eng.pos.z -= f32::cos(eng.rot.x) * f32::sin(eng.rot.y) * SPEED;
      }
      if is_key_pressed(75){
        if rd > 0.1f32{
          rd-=0.1;
        }
        eng.ren.change_render_scale(rd);
      }
      if is_key_pressed(76){
        rd+=0.1;
        eng.ren.change_render_scale(rd);
      }
      set_touch_index(0);
      if get_is_touching(){
        eng.rot.y += ((get_touch_x() as f32/eng.ren.get_canvas_size_x()as f32)*2.0f32 - 1.0f32) / 100f32;
      }

      eng.begin_shadow("clear");

      mesh.draw(&eng, &uniforms);

      eng.begin_main("clear", "clear");

      mesh.draw(&eng, &uniforms);

      eng.begin_post("clear", "clear");

      renquad.draw(&eng, &uniforms);

      eng.end();
    };

    engine::render::render::drawloopexec(drawloop)
}