use super::vec4::Vec4;
use super::vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Mat4{
    pub mat: [f32; 16]
}

impl Mat4{
    #[allow(dead_code)]
    pub fn new() -> Mat4{
        Mat4 { mat: [0.0f32; 16] }
    }
    #[allow(dead_code)]
    pub fn sum(&mut self, mat: Mat4){
        for i in 0..16 {
            self.mat[i] += mat.mat[i];
        }
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, mat: Mat4){
        for i in 0..16 {
            self.mat[i] -= mat.mat[i];
        }
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, mat: &Mat4){
        let t: Mat4 = self.clone();
        self.mat[0] = t.mat[0] * mat.mat[0] + t.mat[1] * mat.mat[4] + t.mat[2] * mat.mat[8] + t.mat[3] * mat.mat[12];
        self.mat[1] = t.mat[0] * mat.mat[1] + t.mat[1] * mat.mat[5] + t.mat[2] * mat.mat[9] + t.mat[3] * mat.mat[13];
        self.mat[2] = t.mat[0] * mat.mat[2] + t.mat[1] * mat.mat[6] + t.mat[2] * mat.mat[10] +t.mat[3] * mat.mat[14];
        self.mat[3] = t.mat[0] * mat.mat[3] + t.mat[1] * mat.mat[7] + t.mat[2] * mat.mat[11] +t.mat[3] * mat.mat[15];

        self.mat[4] = t.mat[4] * mat.mat[0] + t.mat[5] * mat.mat[4] + t.mat[6] * mat.mat[8] + t.mat[7] * mat.mat[12];
        self.mat[5] = t.mat[4] * mat.mat[1] + t.mat[5] * mat.mat[5] + t.mat[6] * mat.mat[9] + t.mat[7] * mat.mat[13];
        self.mat[6] = t.mat[4] * mat.mat[2] + t.mat[5] * mat.mat[6] + t.mat[6] * mat.mat[10] + t.mat[7] * mat.mat[14];
        self.mat[7] = t.mat[4] * mat.mat[3] + t.mat[5] * mat.mat[7] + t.mat[6] * mat.mat[11] + t.mat[7] * mat.mat[15];

        self.mat[8] = t.mat[8] * mat.mat[0] + t.mat[9] * mat.mat[4] + t.mat[10] * mat.mat[8] + t.mat[11] * mat.mat[12];
        self.mat[9] = t.mat[8] * mat.mat[1] + t.mat[9] * mat.mat[5] + t.mat[10] * mat.mat[9] + t.mat[11] * mat.mat[13];
        self.mat[10] = t.mat[8] * mat.mat[2] + t.mat[9] * mat.mat[6] + t.mat[10] * mat.mat[10] + t.mat[11] * mat.mat[14];
        self.mat[11] = t.mat[8] * mat.mat[3] + t.mat[9] * mat.mat[7] + t.mat[10] * mat.mat[11] + t.mat[11] * mat.mat[15];

        self.mat[12] = t.mat[12] * mat.mat[0] + t.mat[13] * mat.mat[4] + t.mat[14] * mat.mat[8] + t.mat[15] * mat.mat[12];
        self.mat[13] = t.mat[12] * mat.mat[1] + t.mat[13] * mat.mat[5] + t.mat[14] * mat.mat[9] + t.mat[15] * mat.mat[13];
        self.mat[14] = t.mat[12] * mat.mat[2] + t.mat[13] * mat.mat[6] + t.mat[14] * mat.mat[10] + t.mat[15] * mat.mat[14];
        self.mat[15] = t.mat[12] * mat.mat[3] + t.mat[13] * mat.mat[7] + t.mat[14] * mat.mat[11] + t.mat[15] * mat.mat[15];
    }
    #[allow(dead_code)]
    pub fn vec4mul(&self, vec: Vec4) -> Vec4{
        Vec4 { 
            x: vec.x * self.mat[0] + vec.y * self.mat[1] + vec.z * self.mat[2] + vec.w * self.mat[3], 
            y: vec.x * self.mat[4] + vec.y * self.mat[5] + vec.z * self.mat[6] + vec.w * self.mat[7], 
            z: vec.x * self.mat[8] + vec.y * self.mat[9] + vec.z * self.mat[10] + vec.w * self.mat[11], 
            w: vec.x * self.mat[12] + vec.y * self.mat[13] + vec.z * self.mat[14] + vec.w * self.mat[15] 
        }
    }
    #[allow(dead_code)]
    pub fn perspective(&mut self, fov: f32, far: f32, near: f32, aspect: f32){
        let scale = f32::tan((fov/2.0f32)*(3.1415f32 / 180f32));
        self.mat[0] = 1.0f32/ (scale*aspect);
        self.mat[5] = 1.0f32/ scale;
        self.mat[10] = -far / (far-near);
        self.mat[11] = -(far * near) / (far-near);
        self.mat[14] = -1.0f32;
    }
    #[allow(dead_code)]
    pub fn orthographic(&mut self, r: f32, l: f32, t: f32, b: f32, z_near: f32, z_far: f32){
        self.mat[0] = 2f32/(r-l);
        self.mat[5] = 2f32/(t-b);
        self.mat[10] = -2f32/(z_far-z_near);
        self.mat[15] = 1f32;
        self.mat[12] = -(r+l)/(r-l);
        self.mat[13] = -(t+b)/(t-b);
        self.mat[14] = -(z_far+z_near)/(z_far-z_near);
    }
    #[allow(dead_code)]
    pub fn trans(&mut self, pos: Vec3){
        self.mat[0] = 1.0f32;
        self.mat[5] = 1.0f32;
        self.mat[10] = 1.0f32;
        self.mat[15] = 1.0f32;
        self.mat[3] = pos.x;
        self.mat[7] = pos.y;
        self.mat[11] = pos.z;
    }
    #[allow(dead_code)]
    pub fn scale(&mut self, pos: Vec3){
        self.mat[0] = pos.x;
        self.mat[5] = pos.y;
        self.mat[10] = pos.z;
        self.mat[15] = 1.0f32;
    }
    #[allow(dead_code)]
    pub fn xrot(&mut self, rot: f32){
        self.mat[0] = 1.0f32;
        self.mat[5] = f32::cos(rot);
        self.mat[6] = -f32::sin(rot);
        self.mat[10] = f32::cos(rot);
        self.mat[9] = f32::sin(rot);
        self.mat[15] = 1.0f32;
    }
    #[allow(dead_code)]
    pub fn yrot(&mut self, rot: f32){
        self.mat[5] = 1.0f32;
        self.mat[0] = f32::cos(rot);
        self.mat[8] = -f32::sin(rot);
        self.mat[10] = f32::cos(rot);
        self.mat[2] = f32::sin(rot);
        self.mat[15] = 1.0f32;
    }
    #[allow(dead_code)]
    pub fn zrot(&mut self, rot: f32){
        self.mat[0] = f32::cos(rot);
        self.mat[1] = -f32::sin(rot);
        self.mat[4] = f32::sin(rot);
        self.mat[5] = f32::cos(rot);
        self.mat[15] = 1.0f32;
        self.mat[10] = 1.0f32;
    }
    #[allow(dead_code)]
    pub fn transpose(&mut self){
        let t = self.clone();
        for x in 0..4 {
            for y in 0..4 {
                self.mat[y*4+x] = t.mat[x*4+y];
            }
        }
    }
}