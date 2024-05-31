pub mod edges;
pub mod pipelines;
pub mod shading;
pub mod visibility;

use eframe::egui::ColorImage;
use image::{Rgb, RgbImage};

use crate::camera::{self, Camera};
use crate::lighting::Lighting;
use crate::object::face::Face;
use crate::object::{face, Object};
use crate::render::pipelines::{axonometric_view_pipeline, perspective_view_pipeline};
use crate::render::shading::constant;
use crate::render::visibility::filter_faces;
use crate::types::{
    mat4x1_to_vec3, vec3_to_mat4x1, Mat, Mat4, Mat4x1, Vec3
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
        perspective_view: bool,
        width: u32,
        height: u32,
    ) -> ColorImage {
        self.clean();

        if perspective_view {
            self.m_sru_srt = perspective_view_pipeline(camera, width as f32, height as f32);
        } else {
            self.m_sru_srt = axonometric_view_pipeline(camera, width as f32, height as f32);
        }

        for object in objects.iter() {
            let mut faces = object.get_faces();
            faces = filter_faces(&faces, &camera.vrp);

            for face in faces.iter() {
                let mut face = face.clone();
                face.set_vertices(self.apply_m_sru_srt(&face.get_vertices()));
                constant(
                    &face,
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

    fn apply_m_sru_srt(
        &self,
        face: &[Vec3; 3],
    ) -> [Vec3; 3] {
        let a: Mat4x1 = self.m_sru_srt * vec3_to_mat4x1(&face[0]);
        let b: Mat4x1 = self.m_sru_srt * vec3_to_mat4x1(&face[1]);
        let c: Mat4x1 = self.m_sru_srt * vec3_to_mat4x1(&face[2]);

        let a: Vec3 = mat4x1_to_vec3(&a);
        let b: Vec3 = mat4x1_to_vec3(&b);
        let c: Vec3 = mat4x1_to_vec3(&c);

        [a, b, c]
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