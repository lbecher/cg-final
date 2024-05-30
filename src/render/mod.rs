pub mod edges;
pub mod pipelines;
pub mod shading;
pub mod visibility;

use eframe::egui::ColorImage;
use image::{Rgb, RgbImage};
use shading::constant;
use visibility::filter_faces;

use crate::camera::Camera;
use crate::lighting::Lighting;
use crate::object::Object;
use crate::types::{
    vec3_to_mat4x1,
    Mat, Mat4, Vec3,
};

pub struct Render {
    clear_color: [u8; 3],
    image: RgbImage,
    zbuffer: Mat,

    m_sru_srt: Mat4,
}

impl Render {
    pub fn new(
        width: u32,
        height: u32,
        clear_color: [u8; 3],
    ) -> Self {
        let pixel: Rgb<u8> = Rgb::from(clear_color);
        let image: RgbImage = RgbImage::from_pixel(width, height, pixel);

        let zbuffer: Mat = Mat::from_element(
            height as usize,
            width as usize,
            f32::INFINITY,
        );

        let m_sru_srt = Mat4::zeros();

        Self {
            clear_color,
            image,
            zbuffer,

            m_sru_srt,
        }
    }

    pub fn redraw(
        &mut self,
        camera: &Camera,
        lighting: &Lighting,
        objects: &Vec<Object>,
    ) -> ColorImage {
        self.clean();

        for object in objects.iter() {
            let mut faces = object.get_faces();
            faces = filter_faces(&faces, &camera.vrp);

            for face in faces.iter() {
                constant(
                    face,
                    &camera.vrp,
                    lighting,
                    &object.get_material(),
                    &mut self.zbuffer,
                    &mut self.image,
                );
            }
        }
                
        self.egui_image()
    }

    pub fn set_clear_color(&mut self, clear_color: [u8; 3]) {
        self.clear_color = clear_color;
    }

    fn clean(&mut self) {
        let width = self.image.width();
        let height = self.image.height();

        let pixel: Rgb<u8> = Rgb::from(self.clear_color);
        let image: RgbImage = RgbImage::from_pixel(width, height, pixel);

        let zbuffer: Mat = Mat::from_element(
            height as usize,
            width as usize,
            f32::INFINITY,
        );

        self.image = image;
        self.zbuffer = zbuffer;
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