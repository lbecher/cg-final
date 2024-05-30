pub mod face;
pub mod material;
pub mod vertex;

use std::f32::consts::PI;

use crate::object::face::Face;
use crate::object::material::Material;
use crate::types::Vec3;

pub struct Object {
    faces: Vec<Face>,
    material: Material,
}

impl Default for Object {
    fn default() -> Self {
        let mut faces: Vec<Face> = Vec::new();

        faces.push(Face::new([
            Vec3::new(93.0, 251.0, -22.0),
            Vec3::new(198.0, 241.0, -22.0),
            Vec3::new(125.0, 107.0, -22.0),
        ]));

        faces.push(Face::new([
            Vec3::new(85.0, 192.0, -22.0),
            Vec3::new(93.0, 251.0, -22.0),
            Vec3::new(125.0, 107.0, -22.0),
        ]));

        let material = Material::default();

        Self {
            faces,
            material,
        }
    }
}

impl Object {
    pub fn new(
        segments: u32,
        perfil: Vec<Vec3>,
    ) -> Self {
        let perfil_len = perfil.len();

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let tr = (2.0 * PI) / segments as f32;
        let mut r = tr;

        let mut index = 0;
        let mut last_segment_indices: Vec<usize> = Vec::new();
        let mut new_segment_indices: Vec<usize> = Vec::new();

        for i in 0..perfil.len() {
            let x = perfil[i].x;
            let y = perfil[i].y;
            let z = perfil[i].z;

            vertices.push(Vec3::new(x, y, z));

            last_segment_indices.push(index);
            index += 1;
        }

        for i in 0..perfil_len {
            let x = perfil[i].x * r.cos();
            let y = perfil[i].y;
            let z = perfil[i].z * r.sin();

            let origin: Vec3 = vertices[last_segment_indices[i]].clone();
            let offset: Vec3 = Vec3::new(x, y, z);
            let new_vertex: Vec3 = origin + offset;

            vertices.push(new_vertex);

            new_segment_indices.push(index);
            index += 1;
        }

        for i in 0..perfil_len {
            let v1: Vec3 = vertices[last_segment_indices[i]];
            let v2: Vec3 = vertices[last_segment_indices[(i + 1) % perfil_len]];
            let v3: Vec3 = vertices[new_segment_indices[i]];
            let v4: Vec3 = vertices[new_segment_indices[(i + 1) % perfil_len]];

            faces.push(Face::new([v1, v2, v3]));
            faces.push(Face::new([v2, v3, v4]));
        }

        for _ in 1..segments {
            for i in 0..perfil_len {
                let x = perfil[i].x * r.cos();
                let y = perfil[i].y;
                let z = perfil[i].z * r.sin();
    
                let origin: Vec3 = vertices[last_segment_indices[i]].clone();
                let offset: Vec3 = Vec3::new(x, y, z);
                let new_vertex: Vec3 = origin + offset;
    
                vertices.push(new_vertex);
    
                new_segment_indices.push(index);
                index += 1;
            }
    
            for i in 0..perfil_len {
                let v1: Vec3 = vertices[last_segment_indices[i]];
                let v2: Vec3 = vertices[last_segment_indices[(i + 1) % perfil_len]];
                let v3: Vec3 = vertices[new_segment_indices[i]];
                let v4: Vec3 = vertices[new_segment_indices[(i + 1) % perfil_len]];
    
                faces.push(Face::new([v1, v2, v3]));
                faces.push(Face::new([v2, v3, v4]));
            }

            last_segment_indices = new_segment_indices;
            new_segment_indices = Vec::new();

            r += tr;
        
        }

        let material = Material::default();

        Self {
            faces,
            material,
        }
    }

    pub fn get_faces(&self) -> Vec<Face> {
        self.faces.clone()
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }
}