use crate::types::Vec3;

pub struct Lighting {
    pub ila: Vec3,
    pub il: Vec3,
    pub l: Vec3,
}

impl Default for Lighting {
    fn default() -> Self {
        let ila: Vec3 = Vec3::new(120.0, 120.0, 120.0);
        let il: Vec3 = Vec3::new(150.0, 150.0, 150.0);
        let l: Vec3 = Vec3::new(70.0, 20.0, 35.0);

        Self {
            ila,
            il,
            l,
        }
    }
}