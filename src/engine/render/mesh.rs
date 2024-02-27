use super::render::*;
use js_sys::Float32Array;

pub struct Mesh{
    jsmesh: Gfxmesh,
}

impl Mesh{
    #[allow(dead_code)]
    pub fn create(gfx: &Render, vertices: Float32Array, uv: Float32Array, normals: Float32Array, lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, ubol: i32, texid: &str, magfilter: &str, minfilter: &str, forpost: bool) -> Mesh{
        Mesh{
            jsmesh: Gfxmesh::create(&gfx.jsren, vertices, uv, normals, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, magfilter, minfilter, forpost)
        }
    }
    #[allow(dead_code)]
    pub fn draw(&self, gfx: &Render, uniform_values: Float32Array){
        self.jsmesh.draw(&gfx.jsren, uniform_values);
    }
}