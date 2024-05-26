use eframe::egui::ColorImage;
use image::{Rgb, RgbImage};

use crate::types::{
    vec3_to_mat4x1,
    Vec3, Mat4, Mat4x1,
};

pub struct Camera {
    pub vrp: Vec3,
    pub p: Vec3,
    pub y: Vec3,
    pub dp: f32,
}

pub struct Render {
    clear_color: [u8; 3],
    image: RgbImage,

    m_sru_srt: Mat4,
}

impl Render {
    pub fn new(
        width: u32, 
        height: u32, 
        clear_color: [u8; 3], 
        
    ) -> Self {
        let pixel = Rgb::from(clear_color);
        let image = RgbImage::from_pixel(width, height, pixel);

        Self {
            clear_color,
            image,
        }
    }

    pub fn generate_pipeline(
        camera: Camera,
        width: f32, 
        height: f32, 
    ) -> Mat4 {
        let n: Vec3 = camera.vrp - camera.p;
        let nn: Vec3 = n.normalize();

        let v: Vec3 = camera.y - (camera.y.dot(&nn) * nn);
        let vn: Vec3 = v.normalize();

        let u: Vec3 = v.cross(&n);
        let un: Vec3 = u.normalize();

        let m_sru_src: Mat4 = Mat4::from_vec(vec![
            un[0], un[1], un[2], -un.dot(camera.vrp[0]),
            vn[0], vn[1], vn[2], -vn.dot(camera.vrp[1]),
            nn[0], nn[1], nn[2], -nn.dot(camera.vrp[2]),
            0.0, 0.0, 0.0, 1.0,
        ]).transpose();

        let cvp: Vec3 = camera.vrp + (camera.dp * (-nn));

        let src_vrp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&camera.vrp);
        let src_cvp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&cvp);

        let m_pers: Mat4 = Mat4::from_vec(vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, (-src_cvp[2]) / camera.dp, src_cvp[2] * (src_vrp[2] / camera.dp),
            0.0, 0.0, (-1.0) / camera.dp, src_vrp[2] / camera.dp,
        ]).transpose();

        let a = (UMAX - UMIN) / (XMAX - XMIN);

        let m_jp: Mat4 = Mat4::from_vec(vec![
            a, 0.0, 0.0, -XMIN * a + UMIN,
            0.0, (VMIN - VMAX) / (YMAX - YMIN), 0.0, YMIN * (VMAX - VMIN) / (YMAX - YMIN) + VMAX,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]).transpose();

        m_jp * (m_pers * m_sru_src)
    }

    pub fn redraw(&mut self) -> ColorImage {
        let pixel = Rgb::from(self.clear_color);
        let width = self.image.width();
        let height = self.image.height();
        self.image = RgbImage::from_pixel(width, height, pixel);

                
        self.egui_image()
    }

    pub fn set_clear_color(&mut self, clear_color: [u8; 3]) {
        self.clear_color = clear_color;
    }

    fn egui_image(&self) -> ColorImage {
        let mut buffer: Vec<u8> = Vec::new();
        for pixel in self.image.pixels() {
            buffer.extend(pixel.0.iter());
        }
        let size = [self.image.width() as _, self.image.height() as _];
        ColorImage::from_rgb(size, &buffer.as_slice())
    }
}