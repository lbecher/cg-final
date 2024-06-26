use crate::object::face::Face;
use crate::types::Vec3;

pub fn face_edges(
    face: &Face,
) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut ab: Vec<Vec3> = interpolate_edges(
        &face.vertices[0],
        &face.vertices[1],
    );
    let mut bc: Vec<Vec3> = interpolate_edges(
        &face.vertices[1],
        &face.vertices[2],
    );
    let mut ca: Vec<Vec3> = interpolate_edges(
        &face.vertices[2],
        &face.vertices[0],
    );

    if ca.len() > bc.len() {
        let tmp: Vec<Vec3> = ca;
        ca = bc;
        bc = tmp;
    }
    if bc.len() > ab.len() {
        let tmp: Vec<Vec3> = bc;
        bc = ab;
        ab = tmp;
    }

    if ab[ab.len() - 1].y < ab[0].y {
        ab.reverse();
    }
    if bc[bc.len() - 1].y < bc[0].y {
        bc.reverse();
    }
    if ca[ca.len() - 1].y < ca[0].y {
        ca.reverse();
    }

    if bc[bc.len() - 1].y > ca[0].y {
        let tmp: Vec<Vec3> = ca;
        ca = bc;
        bc = tmp;
    }

    bc.pop();
    bc.extend(ca);

    (ab, bc)
}

fn interpolate_edges(
    start: &Vec3,
    end: &Vec3,
) -> Vec<Vec3> {
    let mut points: Vec<Vec3> = Vec::new();

    let (
        x0, y0, z0,
        x1, y1, z1,
    ) = if start[1] > end[1] {
        (
            end[0], end[1], end[2],
            start[0], start[1], start[2],
        )
    } else {
        (
            start[0], start[1], start[2],
            end[0], end[1], end[2],
        )
    };

    let dx = x1 - x0;
    let dy = y1 - y0;
    let dz = z1 - z0;

    let tx = dx / dy;
    let tz = dz / dy;

    let y_max = y1.round();

    let mut x = x0;
    let mut y = y0.round();
    let mut z = z0;

    while y < y_max {
        let point: Vec3 = Vec3::new(x, y, z);
        points.push(point);

        x += tx;
        z += tz;

        y += 1.0;
    }

    let point: Vec3 = Vec3::new(x, y, z);
    points.push(point);

    points
}
