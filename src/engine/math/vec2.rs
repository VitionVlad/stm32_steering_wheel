#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec2{
    pub x: f32,
    pub y: f32
}

impl Vec2{
    #[allow(dead_code)]
    pub fn new() -> Vec2{
        Vec2 { x: 0.0f32, y: 0.0f32 }
    }
    #[allow(dead_code)]
    pub fn newdefined(x: f32, y: f32) -> Vec2{
        Vec2 { x:x, y: y }
    }
    #[allow(dead_code)]
    pub fn sum(&mut self, v2: Vec2){
        self.x += v2.x;
        self.y += v2.y;
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, v2: Vec2){
        self.x -= v2.x;
        self.y -= v2.y;
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, v2: Vec2){
        self.x *= v2.x;
        self.y *= v2.y;
    }
    #[allow(dead_code)]
    pub fn div(&mut self, v2: Vec2){
        self.x /= v2.x;
        self.y /= v2.y;
    }
}