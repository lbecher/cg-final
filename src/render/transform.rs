
use crate::app::View;
use crate::camera::Camera;
use crate::object::face::Face;
use crate::render::visibility::filter_faces;
use crate::types::{mat4x1_to_vec3, vec3_to_mat4x1, Mat4, Mat4x1, Vec3};

pub fn transform_positions(
    m_sru_srt: &Mat4,
    faces: &Vec<Face>,
    camera: &Camera,
    view: View,
) -> Vec<Face> {
    let mut faces: Vec<Face> = faces.clone();

    if view == View::Side {
        for face in faces.iter_mut() {
            let vertices: [Vec3; 3] = face.get_vertices();
            let new_vertices: [Vec3; 3] = [
                Vec3::new(vertices[0].z, vertices[0].y, vertices[0].x),
                Vec3::new(vertices[1].z, vertices[1].y, vertices[1].x),
                Vec3::new(vertices[2].z, vertices[2].y, vertices[2].x),
            ];
            face.set_vertices(new_vertices);
        }
    } else if view == View::Top {
        for face in faces.iter_mut() {
            let vertices: [Vec3; 3] = face.get_vertices();
            let new_vertices: [Vec3; 3] = [
                Vec3::new(vertices[0].x, vertices[0].z, vertices[0].y),
                Vec3::new(vertices[1].x, vertices[1].z, vertices[1].y),
                Vec3::new(vertices[2].x, vertices[2].z, vertices[2].y),
            ];
            face.set_vertices(new_vertices);
        }
    }

    faces = filter_faces(&faces, &camera.vrp);

    for face in faces.iter_mut() {
        face.set_vertices(apply_m_sru_srt(m_sru_srt, &face.get_vertices()));
    }

    faces
}

fn apply_m_sru_srt(
    m_sru_srt: &Mat4,
    face: &[Vec3; 3],
) -> [Vec3; 3] {
    let a: Mat4x1 = m_sru_srt * vec3_to_mat4x1(&face[0]);
    let b: Mat4x1 = m_sru_srt * vec3_to_mat4x1(&face[1]);
    let c: Mat4x1 = m_sru_srt * vec3_to_mat4x1(&face[2]);

    let a: Vec3 = mat4x1_to_vec3(&a);
    let b: Vec3 = mat4x1_to_vec3(&b);
    let c: Vec3 = mat4x1_to_vec3(&c);

    [a, b, c]
}