pub mod face;
pub mod material;

use std::f32::consts::PI;

use crate::object::face::Face;
use crate::object::material::Material;
use crate::types::{mat4x1_to_vec3, vec3_to_mat4x1, Mat4, Mat4x1, Vec3};

pub struct Object {
    faces: Vec<Face>,
    pub material: Material,
}

impl Default for Object {
    fn default() -> Self {
        let mut faces: Vec<Face> = Vec::new();

        let a: Vec3 = Vec3::new(21.2, 0.7, 42.3);
        let b: Vec3 = Vec3::new(34.0, 3.4, 27.2);
        let c: Vec3 = Vec3::new(18.8, 5.6, 14.6);
        let d: Vec3 = Vec3::new(5.9, 2.9, 29.0);
        let e: Vec3 = Vec3::new(20.0, 20.9, 31.6);

        faces.push(Face::new([
            a.clone(),
            b.clone(),
            e.clone(),
        ]));

        faces.push(Face::new([
            b.clone(),
            c.clone(),
            e.clone(),
        ]));

        faces.push(Face::new([
            c.clone(),
            d.clone(),
            e.clone(),
        ]));

        faces.push(Face::new([
            d.clone(),
            a.clone(),
            e.clone(),
        ]));

        let material = Material::default();

        Self {
            faces,
            material,
        }
    }
}

impl Object {
    pub fn _new(
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

    pub fn get_cent(&self) -> Vec3 {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;

        for face in self.faces.iter() {
            let vertices: [Vec3; 3] = face.get_vertices();

            x += vertices[0].x + vertices[1].x + vertices[2].x;
            y += vertices[0].y + vertices[1].y + vertices[2].y;
            z += vertices[0].z + vertices[1].z + vertices[2].z;
        }

        let n: f32 = self.faces.len() as f32 * 3.0;

        Vec3::new(x / n, y / n, z / n)
    }

    fn gen_x_rotation_matriz(&self,  rotation: f32) -> Mat4 {
        let rotation = (rotation * PI) / 180.0;
        Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, rotation.cos(), -rotation.sin(), 0.0,
            0.0, rotation.sin(), rotation.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    fn gen_y_rotation_matriz(&self, rotation: f32) -> Mat4 {
        let rotation = (rotation * PI) / 180.0;
        Mat4::new(
            rotation.cos(), 0.0, rotation.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -rotation.sin(), 0.0, rotation.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    
    fn gen_z_rotation_matriz(&self, rotation: f32) -> Mat4 {
        let rotation = (rotation * PI) / 180.0;
        Mat4::new(
            rotation.cos(), -rotation.sin(), 0.0, 0.0,
            rotation.sin(), rotation.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn gen_scaling_matriz(&self, scale: f32) -> Mat4 {
        Mat4::new(
            scale, 0.0, 0.0, 0.0,
            0.0, scale, 0.0, 0.0,
            0.0, 0.0, scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    fn gen_translation_matriz(&self, position: Vec3) -> Mat4 {
        Mat4::new(
            1.0, 0.0, 0.0, position.x,
            0.0, 1.0, 0.0, position.y,
            0.0, 0.0, 1.0, position.z,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scale(
        &mut self,
        factor: f32,
    ) {
        let cent: Vec3 = self.get_cent();

        let t: Mat4 = self.gen_translation_matriz(-cent.clone());

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let s: Mat4 = self.gen_scaling_matriz(factor);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = s * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let t: Mat4 = self.gen_translation_matriz(cent);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
            face.update();
        }
    }

    pub fn rotate_x(
        &mut self,
        rotation: f32,
    ) {
        let cent: Vec3 = self.get_cent();

        let t: Mat4 = self.gen_translation_matriz(-cent.clone());

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let r: Mat4 = self.gen_x_rotation_matriz(rotation);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = r * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let t: Mat4 = self.gen_translation_matriz(cent);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
            face.update();
        }
    }

    pub fn rotate_y(
        &mut self,
        rotation: f32,
    ) {
        let cent: Vec3 = self.get_cent();

        let t: Mat4 = self.gen_translation_matriz(-cent.clone());

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let r: Mat4 = self.gen_y_rotation_matriz(rotation);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = r * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let t: Mat4 = self.gen_translation_matriz(cent);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
            face.update();
        }
    }

    pub fn rotate_z(
        &mut self,
        rotation: f32,
    ) {
        let cent: Vec3 = self.get_cent();

        let t: Mat4 = self.gen_translation_matriz(-cent.clone());

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let r: Mat4 = self.gen_z_rotation_matriz(rotation);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = r * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
        }

        let t: Mat4 = self.gen_translation_matriz(cent);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
            face.update();
        }
    }

    pub fn translate(
        &mut self,
        position: Vec3,
    ) {
        let t: Mat4 = self.gen_translation_matriz(position);

        for face in self.faces.iter_mut() {
            let mut vertices: [Vec3; 3] = face.get_vertices();
            for vertex in vertices.iter_mut() {
                let v: Mat4x1 = t * vec3_to_mat4x1(&vertex);
                *vertex = mat4x1_to_vec3(&v);
            }
            face.set_vertices(vertices);
            face.update();
        }
    }

    pub fn _get_vertex_nn(&self, vertex: &Vec3) -> Vec3 {
        let mut nn: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        for face in self.faces.iter() {
            let vertices: [Vec3; 3] = face.get_vertices();

            if vertices.contains(vertex) {
                nn += face.get_nn();
            }
        }

        nn.normalize()
    }
}