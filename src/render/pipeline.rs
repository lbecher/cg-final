use crate::render::Camera;
use crate::types::{
    vec3_to_mat4x1,
    Vec3, Mat4, Mat4x1,
};

pub fn calculate_pipeline(
    camera: Camera,
    width: f32, 
    height: f32, 
) -> Mat4 {

    // Transformação de câmera

    let n: Vec3 = camera.vrp - camera.p;
    let nn: Vec3 = n.normalize();

    let v: Vec3 = camera.y - (camera.y.dot(&nn) * nn);
    let vn: Vec3 = v.normalize();

    let u: Vec3 = v.cross(&n);
    let un: Vec3 = u.normalize();

    let m_r: Mat4 = Mat4::from_vec(vec![
        un[0], un[1], un[2], 0.0,
        vn[0], vn[1], vn[2], 0.0,
        nn[0], nn[1], nn[2], 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]).transpose();

    let m_t: Mat4 = Mat4::from_vec(vec![
        1.0, 0.0, 0.0, -camera.vrp[0],
        0.0, 1.0, 0.0, -camera.vrp[1],
        0.0, 0.0, 1.0, -camera.vrp[2],
        0.0, 0.0, 0.0, 1.0,
    ]).transpose();

    let m_sru_src: Mat4 = m_r * m_t;

    println!("{}", m_sru_src);


    // Transformação perspectiva

    let vp: Vec3 = camera.vrp + (camera.dp * (-nn));

    let src_vp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&vp);
    let src_prp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&camera.vrp);

    let m_pers: Mat4 = Mat4::from_vec(vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, (-src_vp[2]) / camera.dp, src_vp[2] * (src_prp[2] / camera.dp),
        0.0, 0.0, (-1.0) / camera.dp, src_prp[2] / camera.dp,
    ]).transpose();

    println!("{}", m_pers);


    // Transformação de mapeamento

    let umin = 0.0;
    let umax = width - 1.0;
    let vmin = 0.0;
    let vmax = height - 1.0;

    let xmin = camera.xmin;
    let xmax = camera.xmax;
    let ymin = camera.ymin;
    let ymax = camera.ymax;

    let a = (umax - umin) / (xmax - xmin);

    let m_jp: Mat4 = Mat4::from_vec(vec![
        a, 0.0, 0.0, -xmin * a + umin,
        0.0, (vmin - vmax) / (ymax - ymin), 0.0, ymin * (vmax - vmin) / (ymax - ymin) + vmax,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]).transpose();

    println!("{}", m_jp);


    // Matriz composta

    let m_sru_srt: Mat4 = m_jp * (m_pers * m_sru_src);

    println!("{}", m_sru_srt);

    m_sru_srt
}