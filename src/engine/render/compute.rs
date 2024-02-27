use super::render::*;
use js_sys::Float32Array;

pub struct Compute{
    jsc: Gpucompute,
    workgroupsize: i32,
    ibs: u32,
    obs: u32,
    returnval: Float32Array,
    in_exec: bool,
    pub out_buf: Vec<f32>,
}

impl Compute{
    #[allow(dead_code)]
    pub fn create(ibs: u32, obs: u32, code: &str) -> Compute{
        Compute{
            jsc: Gpucompute::createcompute(ibs as i32, obs as i32, code),
            workgroupsize: 1,
            ibs: ibs,
            obs: obs,
            returnval: Float32Array::new_with_length(obs),
            in_exec: false,
            out_buf: vec![0.0f32],
        }
    }
    #[allow(dead_code)]
    pub fn execute(&mut self, ib: &[f32]){
        if !self.in_exec{
            let jsi = js_sys::Float32Array::new_with_length((self.ibs) as u32);
            jsi.copy_from(&ib);
            self.jsc.execute(&jsi, self.workgroupsize);
            self.in_exec = true;
        }
        if self.jsc.getstate() && self.in_exec{
            self.returnval = self.jsc.getresult();
            self.out_buf.resize(self.obs as usize, 0.0f32);
            self.returnval.copy_to(&mut self.out_buf);
            self.in_exec = !self.in_exec;
        }
    }
}