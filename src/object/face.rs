use crate::types::Vec3;

#[derive(Clone)]
pub struct Face {
    pub nn: Vec3,
    pub cent: Vec3,
    pub vertices: [Vec3; 3],
}

impl Face {
    pub fn new(vertices: [Vec3; 3]) -> Self {
        let b: Vec3 = vertices[1];

        let ba: Vec3 = vertices[0] - b;
        let bc: Vec3 = vertices[2] - b;

        let cent: Vec3 = b + (0.5 * ba) + (0.5 * bc);

        let n: Vec3 = bc.cross(&ba);
        let nn: Vec3 = n.normalize();

        Self {
            nn,
            cent,
            vertices,
        }
    }

    pub fn get_nn(&self) -> Vec3 {
        self.nn.clone()
    }

    pub fn get_cent(&self) -> Vec3 {
        self.cent.clone()
    }

    pub fn get_vertices(&self) -> [Vec3; 3] {
        self.vertices.clone()
    }
}