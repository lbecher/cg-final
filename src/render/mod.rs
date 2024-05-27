pub mod pipelines;

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

    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
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

        let m_sru_srt = Mat4::zeros();

        Self {
            clear_color,
            image,

            m_sru_srt,
        }
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