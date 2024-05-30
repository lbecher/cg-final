use debug_print::debug_println;

use crate::object::face::Face;
use crate::types::Vec3;

pub fn filter_faces(
    faces: &Vec<Face>,
    vrp: &Vec3,
) -> Vec<Face> {
    let mut visible_faces: Vec<Face> = Vec::new();

    for face in faces.iter() {
        let face_visibility = calculate_visibility(face, vrp);
        if face_visibility {
            visible_faces.push(face.clone());
        }
    }

    visible_faces
}

fn calculate_visibility(
    face: &Face,
    vrp: &Vec3,
) -> bool {
    let nn: Vec3 = face.get_nn();

    let o: Vec3 = vrp - face.get_cent();
    let on: Vec3 = o.normalize();

    let dot = on.dot(&nn);

    //debug_println!("{}", dot);

    dot > 0.0
}