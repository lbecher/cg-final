use std::collections::HashSet;

use crate::object::face::Face;
use crate::types::Vec3;

#[derive(Clone)]
pub struct Vertex {
    vertices: Vec3,
    faces: HashSet<usize>,
    nn: Vec3,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let vertices: Vec3 = Vec3::new(x, y, z);
        let faces = HashSet::new();
        let nn: Vec3 = Vec3::zeros();
        Self {
            vertices,
            faces,
            nn,
        }
    }

    pub fn add_face(&mut self, face: usize) {
        self.faces.insert(face);
    }

    pub fn update(&mut self, faces: &Vec<Face>) {
        let mut n: Vec3 = Vec3::zeros();

        for i in self.faces.iter() {
            n += faces[*i].get_nn();
        }

        self.nn = n.normalize();
    }

    /*pub fn rotate(&mut self, x: f32, y: f32, z: f32, cent: Vec3) {

    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {

    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {

    }*/

    pub fn get_vertices(&self) -> Vec3 {
        self.vertices.clone()
    }

    pub fn get_nn(&self) -> Vec3 {
        self.nn.clone()
    }
}