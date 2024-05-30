use crate::types::Vec3;

pub struct Camera {
    pub vrp: Vec3,
    pub p: Vec3,
    pub y: Vec3,

    pub dp: f32,

    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let vrp: Vec3 = Vec3::new(25.0, 15.0, 80.0);
        let p: Vec3 = Vec3::new(20.0, 10.0, 25.0);
        let y: Vec3 = Vec3::new(0.0, 1.0, 0.0);

        let dp = 40.0;

        let xmin = -8.0;
        let xmax = 8.0;
        let ymin = -6.0;
        let ymax = 6.0;
        
        Self {
            vrp,
            p,
            y,

            dp,
    
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }
}