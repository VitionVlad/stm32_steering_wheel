#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3{
    #[allow(dead_code)]
    pub fn new() -> Vec3{
        Vec3 { x: 0.0f32, y: 0.0f32, z: 0.0f32 }
    }
    #[allow(dead_code)]
    pub fn newdefined(x: f32, y: f32, z:f32) -> Vec3{
        Vec3 { x:x, y: y, z: z }
    }
    #[allow(dead_code)]
    pub fn sum(&mut self, v2: Vec3){
        self.x += v2.x;
        self.y += v2.y;
        self.z += v2.z;
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, v2: Vec3){
        self.x -= v2.x;
        self.y -= v2.y;
        self.z -= v2.z;
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, v2: Vec3){
        self.x *= v2.x;
        self.y *= v2.y;
        self.z *= v2.z;
    }
    #[allow(dead_code)]
    pub fn div(&mut self, v2: Vec3){
        self.x /= v2.x;
        self.y /= v2.y;
        self.z /= v2.z;
    }
}