use self::mat4::Mat4;
use self::vec2::Vec2;
use self::vec3::Vec3;

use super::render::render::Render;
use super::math::{self, *};

#[allow(dead_code)]
pub struct Engine{
    pub ren: Render,
    pub projection: math::mat4::Mat4,
    pub pos: math::vec3::Vec3,
    pub rot: math::vec2::Vec2,
    pub shadowprojection: math::mat4::Mat4,
    pub shadowpos: math::vec3::Vec3,
    pub shadowrot: math::vec2::Vec2,
    pub orthographic: bool,
    pub shadoworthographic: bool,
    pub fov: f32,
    pub shadowfov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub shadow_z_near: f32,
    pub shadow_z_far: f32,
}

impl Engine{
    #[allow(dead_code)]
    pub fn new(canvasid: &str, renderscale: f32, shadowmapres: i32) -> Engine{      
        Engine{
            ren: Render::init(canvasid, renderscale, shadowmapres),
            projection: Mat4::new(),
            pos: Vec3::new(),
            rot: Vec2::new(),
            orthographic: false,
            fov: 90.0f32,
            shadowprojection: Mat4::new(),
            shadowpos: Vec3::new(),
            shadowrot: Vec2::new(),
            shadoworthographic: false,
            shadowfov: 90.0f32,
            z_near: 0.1f32,
            z_far: 100f32,
            shadow_z_near: 0.1f32,
            shadow_z_far: 100f32,
        }
    }
    #[allow(dead_code)]
    pub fn calculate_projection(&mut self){
        self.projection = Mat4::new();
        if !self.orthographic{
            self.projection.perspective(self.fov, self.z_far, self.z_near, self.ren.get_canvas_size_x() as f32/self.ren.get_canvas_size_y() as f32);
        }else{
            self.projection.orthographic(self.fov, -self.fov, self.fov, -self.fov, self.z_near, self.z_far);
        }
        let mut t: Mat4 = Mat4::new();
        t.xrot(self.rot.x);
        self.projection.mul(&t);

        t = Mat4::new();
        t.yrot(self.rot.y);
        self.projection.mul(&t);

        t = Mat4::new();
        t.trans(Vec3::newdefined(self.pos.x, self.pos.y, self.pos.z));
        self.projection.mul(&t);
    }
    #[allow(dead_code)]
    pub fn calculate_shadow_projection(&mut self){
        self.projection = Mat4::new();
        if !self.orthographic{
            self.shadowprojection.perspective(self.shadowfov, self.shadow_z_far, self.shadow_z_near, 1f32);
        }else{
            self.shadowprojection.orthographic(self.shadowfov, -self.shadowfov, self.shadowfov, -self.shadowfov, self.shadow_z_near, self.shadow_z_far);
        }

        let mut t: Mat4 = Mat4::new();
        t.xrot(self.shadowrot.x);
        self.shadowprojection.mul(&t);

        t = Mat4::new();
        t.yrot(self.shadowrot.y);
        self.shadowprojection.mul(&t);

        t = Mat4::new();
        t.trans(Vec3::newdefined(self.shadowpos.x, self.shadowpos.y, self.shadowpos.z));
        self.shadowprojection.mul(&t);
    }
    #[allow(dead_code)]
    pub fn begin_shadow(&mut self, loadop: &str){
        self.ren.begin_shadow_pass(loadop);
        self.calculate_shadow_projection();
    }
    #[allow(dead_code)]
    pub fn begin_main(&mut self, loadop: &str, depthloadop: &str){
        self.ren.begin_main_pass(loadop, depthloadop);
        self.calculate_projection();
    }
    #[allow(dead_code)]
    pub fn begin_post(&mut self, loadop: &str, depthloadop: &str){
        self.ren.begin_post_pass(loadop, depthloadop);
    }
    #[allow(dead_code)]
    pub fn end(&self){
        self.ren.end_render();
    }
}