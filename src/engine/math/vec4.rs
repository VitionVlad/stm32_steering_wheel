#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4{
    #[allow(dead_code)]
    pub fn new() -> Vec4{
        Vec4 { x: 0.0f32, y: 0.0f32, z: 0.0f32, w: 0.0f32 }
    }
    #[allow(dead_code)]
    pub fn newdefined(x: f32, y: f32, z:f32, w:f32) -> Vec4{
        Vec4 { x:x, y: y, z: z, w: w }
    }
    #[allow(dead_code)]
    pub fn sum(&mut self, v2: Vec4){
        self.x += v2.x;
        self.y += v2.y;
        self.z += v2.z;
        self.w += v2.w;
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, v2: Vec4){
        self.x -= v2.x;
        self.y -= v2.y;
        self.z -= v2.z;
        self.w -= v2.w;
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, v2: Vec4){
        self.x *= v2.x;
        self.y *= v2.y;
        self.z *= v2.z;
        self.w *= v2.w;
    }
    #[allow(dead_code)]
    pub fn div(&mut self, v2: Vec4){
        self.x /= v2.x;
        self.y /= v2.y;
        self.z /= v2.z;
        self.w /= v2.w;
    }
}