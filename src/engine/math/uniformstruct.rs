use self::{mat4::Mat4, vec2::Vec2, vec3::Vec3, vec4::Vec4};

use super::*;

#[derive(Clone, Copy)]
pub enum Usages{
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat,
    Mvpmat,
    Smvpmat
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Uniformstruct{
    pub usage: Usages,
    pub float: f32,
    pub vec2: Vec2,
    pub vec3: Vec3,
    pub vec4: Vec4,
    pub mat: Mat4
}

#[allow(dead_code)]
pub fn createfloat(value: f32) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Float,
        float: value,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn createvec2(value: Vec2) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec2,
        float: 0.0f32,
        vec2: value,
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn createvec3(value: Vec3) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec3,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: value,
        vec4: Vec4::new(),
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn createvec4(value: Vec4) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Vec4,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: value,
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn createmat(value: Mat4) -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Mat,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: value,
    }
}

#[allow(dead_code)]
pub fn createmvpmat() -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Mvpmat,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn createsmvpmat() -> Uniformstruct{
    Uniformstruct{
        usage: Usages::Smvpmat,
        float: 0.0f32,
        vec2: Vec2::new(),
        vec3: Vec3::new(),
        vec4: Vec4::new(),
        mat: Mat4::new(),
    }
}

#[allow(dead_code)]
pub fn getsize(uniforms: &Vec<Uniformstruct>) -> i32{
    let mut size: i32 = 0;
    for i in 0..uniforms.len(){
        match uniforms[i].usage {
            Usages::Float => size += 4,
            Usages::Vec2 => size += 8,
            Usages::Vec3 => size += 12,
            Usages::Vec4 => size += 16,
            Usages::Mat => size += 64,
            Usages::Mvpmat => size += 64,
            Usages::Smvpmat => size += 64,
        }
    }
    return size;
}