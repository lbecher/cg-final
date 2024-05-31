use nalgebra::{DMatrix, Matrix4, Matrix4x1, Vector3};

pub type Vec3 = Vector3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Mat4x1 = Matrix4x1<f32>;
pub type Mat = DMatrix<f32>;

pub fn vec3_to_mat4x1(vec3: &Vec3) -> Mat4x1 {
    Mat4x1::from_vec(vec![
        vec3[0],
        vec3[1],
        vec3[2],
        1.0,
    ])
}

pub fn mat4x1_to_vec3(mat4x1: &Mat4x1) -> Vec3 {
    if mat4x1[3] != 1.0 {
        Vec3::new(
            mat4x1[0] / mat4x1[3],
            mat4x1[1] / mat4x1[3],
            mat4x1[2],
        )
    } else {
        Vec3::new(
            mat4x1[0],
            mat4x1[1],
            mat4x1[2],
        )
    }
}