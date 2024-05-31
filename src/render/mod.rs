pub mod edges;
pub mod pipelines;
pub mod shading;
pub mod transform;
pub mod visibility;

use eframe::egui::ColorImage;
use image::{Rgb, RgbImage};

use crate::camera::Camera;
use crate::lighting::Lighting;
use crate::app::{RenderType, View};
use crate::object::Object;
use crate::render::pipelines::{axonometric_view_pipeline, perspective_view_pipeline};
use crate::render::shading::constant;
use crate::types::{Mat, Mat4};
use crate::render::transform::transform_positions;

pub struct Render {
    clear_color: [u8; 3],
    image: RgbImage,
    zbuffer: Mat,
    
    pub m_sru_srt: Mat4,
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

        let m_sru_srt: Mat4 = Mat4::zeros();

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
        view: View,
        render_type: RenderType,
        width: u32,
        height: u32,
    ) -> ColorImage {
        self.clean();

        if view == View::Perspective {
            self.m_sru_srt = perspective_view_pipeline(camera, width as f32, height as f32);
        } else {
            self.m_sru_srt = axonometric_view_pipeline(camera, width as f32, height as f32);
        }

        for object in objects.iter() {
            let faces = transform_positions(&self.m_sru_srt, &object.get_faces(), camera, view.clone());

            for face in faces.iter() {
                match render_type {
                    RenderType::Constant => {
                        constant(
                            &face,
                            &camera.vrp,
                            lighting,
                            &object.get_material(),
                            &mut self.zbuffer,
                            &mut self.image,
                        );
                    }
                    _ => {}
                }
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