use image::RgbImage;

use crate::object::face::Face;
use crate::lighting::Lighting;
use crate::object::material::Material;
use crate::render::edges::face_edges;
use crate::types::{Mat, Vec3};

fn calculate_color(
    nn: &Vec3,
    sn: &Vec3,
    lighting: &Lighting,
    material: &Material,
) -> (u8, u8, u8) {
    let mut color = [0, 0, 0];

    for i in 0..3 {
        let mut it = 0.0;

        let ia = lighting.ila[i] * material.ka[i];
        it += ia;

        let ln: Vec3 = lighting.l.normalize();

        let dot = nn.dot(&ln);
        if dot > 0.0 {
            let id = lighting.il[i] * material.kd[i] * dot;
            it += id;
        }

        let r: Vec3 = ((2.0 * ln.dot(nn)) * nn) - ln;
        let dot = r.dot(&sn);
        if dot > 0.0 {
            let is = lighting.il[i] * material.ks[i] * (dot.powf(material.n));
            it += is;
        }

        if it > 255.0 {
            it = 255.0;
        }
        else if it <= 0.0 {
            it = 0.0;
        }

        color[i] = it as u8;
    }

    (color[0], color[1], color[2])
}

pub fn constant(
    face: &Face,
    vrp: &Vec3,
    lighting: &Lighting,
    material: &Material,
    zbuffer: &mut Mat,
    image: &mut RgbImage,
) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    let egdes_points: (Vec<Vec3>, Vec<Vec3>) = face_edges(face);
    let (start_edge, end_edge) = egdes_points;

    let nn: Vec3 = face.get_nn();

    let s: Vec3 = vrp - face.get_cent();
    let sn: Vec3 = s.normalize();

    let (r, g, b) = calculate_color(&nn, &sn, lighting, material);

    for i in 0..start_edge.len() {
        let x0 = start_edge[i][0].floor();
        let x1 = end_edge[i][0].ceil();

        let z0 = start_edge[i][2];
        let z1 = end_edge[i][2];

        let dx = x1 - x0;
        let dz = z1 - z0;

        let mut tz: f32 = 0.0;
        // Evitar divisão por zero
        if dz.abs() > f32::EPSILON {
            tz = dz / dx;
        }

        let y = start_edge[i][1] as i32;
        let mut z = z0;

        for x in x0 as i32..x1 as i32 {

            if x >= 0 && x < width && y >= 0 && y < height {
                let index = (y as usize * width as usize) + x as usize;

                if z < zbuffer[index] {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);

                    pixel.0[0] = r;
                    pixel.0[1] = g;
                    pixel.0[2] = b;

                    zbuffer[index] = z;
                }
            }

            z += tz;
        }
    }
}

/*
pub fn gouraud(
    face: &Face,
    vrp: &Vec3,
    lighting: &Lighting,
    material: &Material,
    zbuffer: &mut Mat,
    image: &mut RgbImage,
) {
    
}

pub fn phong(
    zbuffer: &Mat,
) {

}
*/