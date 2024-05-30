use crate::types::Vec3;

#[derive(Clone)]
pub struct Material {
    pub ka: Vec3,
    pub kd: Vec3,
    pub ks: Vec3,
    pub n: f32,
}

impl Default for Material {
    fn default() -> Self {
        let ka: Vec3 = Vec3::new(0.4, 0.0, 0.0);
        let kd: Vec3 = Vec3::new(0.7, 0.0, 0.0);
        let ks: Vec3 = Vec3::new(0.5, 0.0, 0.0);

        let n = 2.0;

        Self {
            ka,
            kd,
            ks,
            n
        }
    }
}