use super::{engine::Engine, math::{mat4::Mat4, uniformstruct::{getsize, Uniformstruct, Usages}, vec3::Vec3}, render::mesh::Mesh};
use js_sys::Float32Array;

#[allow(dead_code)]
pub struct Object{
    pub mesh: Mesh,
    jsarr: Float32Array,
    inuniform: u32,
    pub pos: Vec3,
    pub rot: Vec3,
    pub scale: Vec3,
    mat: Mat4,
    smat: Mat4,
}

impl Object {
    #[allow(dead_code)]
    pub fn new(eng: &Engine, vertices: &[f32], uv: &[f32], normals: &[f32], lenght: i32, vertexcode: &str, shadowvertexcode: &str, fragmentcode: &str, unifroms: &Vec<Uniformstruct>, texid: &str, magfilter: &str, minfilter: &str, forpost: bool) -> Object{
        let jsvert = js_sys::Float32Array::new_with_length((lenght*4) as u32);
        jsvert.copy_from(&vertices);

        let jsuv = js_sys::Float32Array::new_with_length((lenght*2) as u32);
        jsuv.copy_from(&uv);

        let jsn = js_sys::Float32Array::new_with_length((lenght*3) as u32);
        jsn.copy_from(&normals);
        let ubol: i32 = getsize(unifroms);
        Object { 
            mesh: Mesh::create(&eng.ren, jsvert, jsuv, jsn, lenght, vertexcode, shadowvertexcode, fragmentcode, ubol, texid, magfilter, minfilter, forpost),
            jsarr: Float32Array::new_with_length((ubol/4) as u32),
            inuniform: 0,
            pos: Vec3::new(),
            rot: Vec3::new(),
            scale: Vec3::newdefined(1f32, 1f32, 1f32),
            mat: Mat4::new(),
            smat: Mat4::new()
        }
    }
    #[allow(dead_code)]
    pub fn draw(&mut self, eng: &Engine, unifroms: &Vec<Uniformstruct>){
        self.inuniform = 0;
        for i in 0..unifroms.len(){
            self.mat = eng.projection;
            self.smat = eng.shadowprojection;

            let mut t: Mat4 = Mat4::new();
            t.scale(self.scale);
            self.mat.mul(&t);
            self.smat.mul(&t);

            t = Mat4::new();
            t.xrot(self.rot.x);
            self.mat.mul(&t);
            self.smat.mul(&t);

            t = Mat4::new();
            t.yrot(self.rot.y);
            self.mat.mul(&t);
            self.smat.mul(&t);

            t = Mat4::new();
            t.zrot(self.rot.z);
            self.mat.mul(&t);
            self.smat.mul(&t);

            t = Mat4::new();
            t.trans(self.pos);
            self.mat.mul(&t);
            self.smat.mul(&t);

            self.mat.transpose();
            self.smat.transpose();
            match unifroms[i].usage {
                Usages::Float => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].float);
                    self.inuniform+=1;
                },
                Usages::Vec2 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec2.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec2.y);
                    self.inuniform+=1;
                },
                Usages::Vec3 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.y);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec3.z);
                    self.inuniform+=1;
                },
                Usages::Vec4 => {
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.x);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.y);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.z);
                    self.inuniform+=1;
                    self.jsarr.set_index(self.inuniform, unifroms[i].vec4.w);
                    self.inuniform+=1;
                },
                Usages::Mat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, unifroms[i].mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
                Usages::Mvpmat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, self.mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
                Usages::Smvpmat => {
                    for b in 0..16{
                        self.jsarr.set_index(b+self.inuniform, self.mat.mat[b as usize]);
                    }
                    self.inuniform+=16;
                },
            }
        }
        self.mesh.draw(&eng.ren, self.jsarr.clone())
    }
}