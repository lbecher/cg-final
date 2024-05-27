use debug_print::debug_println;

use crate::render::Camera;
use crate::types::{
    vec3_to_mat4x1,
    Vec3, Mat4, Mat4x1,
};

fn nn(
    camera: &Camera,
) -> Vec3 {
    let n: Vec3 = camera.vrp - camera.p;
    n.normalize()
}

/// Função que calcula a matriz de transformação de câmera.
fn m_sru_src(
    camera: &Camera,
    nn: &Vec3,
) -> Mat4 {
    let v: Vec3 = camera.y - (camera.y.dot(&nn) * nn);
    let vn: Vec3 = v.normalize();

    let un: Vec3 = vn.cross(&nn);

    let m_r: Mat4 = Mat4::new(
        un[0], un[1], un[2], 0.0,
        vn[0], vn[1], vn[2], 0.0,
        nn[0], nn[1], nn[2], 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let m_t: Mat4 = Mat4::new(
        1.0, 0.0, 0.0, -camera.vrp[0],
        0.0, 1.0, 0.0, -camera.vrp[1],
        0.0, 0.0, 1.0, -camera.vrp[2],
        0.0, 0.0, 0.0, 1.0,
    );

    let m_sru_src: Mat4 = m_r * m_t;
    
    debug_println!("{}", m_sru_src);

    m_sru_src
}


/// Função que calcula a matriz de transformação perspectiva.
fn m_pers(
    camera: &Camera,
    nn: &Vec3,
    m_sru_src: &Mat4,
) -> Mat4 {
    let vp: Vec3 = camera.vrp + (camera.dp * (-nn));

    let src_vp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&vp);
    let src_prp: Mat4x1 = m_sru_src * vec3_to_mat4x1(&camera.vrp);

    let m_pers: Mat4 = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, (-src_vp[2]) / camera.dp, src_vp[2] * (src_prp[2] / camera.dp),
        0.0, 0.0, (-1.0) / camera.dp, src_prp[2] / camera.dp,
    );

    debug_println!("{}", m_pers);

    m_pers
}

/// Função que calcula a matriz de transformação de mapeamento.
fn m_jp(
    camera: &Camera,
    width: f32,
    height: f32,
) -> Mat4 {
    let umin = 0.0;
    let umax = width - 1.0;
    let vmin = 0.0;
    let vmax = height - 1.0;

    let xmin = camera.xmin;
    let xmax = camera.xmax;
    let ymin = camera.ymin;
    let ymax = camera.ymax;

    let a = (umax - umin) / (xmax - xmin);

    let m_jp: Mat4 = Mat4::new(
        a, 0.0, 0.0, -xmin * a + umin,
        0.0, (vmin - vmax) / (ymax - ymin), 0.0, ymin * (vmax - vmin) / (ymax - ymin) + vmax,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    debug_println!("{}", m_jp);

    m_jp
}

/// Função que calcula a matriz composta do pipeline para
/// as vistas axonométricas.
pub fn axonometric_view_pipeline(
    camera: &Camera,
    width: f32,
    height: f32,
) -> Mat4 {
    let nn: Vec3 = nn(camera);

    let m_sru_src: Mat4 = m_sru_src(camera, &nn);
    let m_jp: Mat4 = m_jp(camera, width, height);

    let m_sru_srt: Mat4 = m_jp * m_sru_src;

    debug_println!("{}", m_sru_srt);

    m_sru_srt
}

/// Função que calcula a matriz composta do pipeline para
/// a vista em perspectiva.
pub fn perspective_view_pipeline(
    camera: &Camera,
    width: f32, 
    height: f32, 
) -> Mat4 {
    let nn: Vec3 = nn(camera);

    let m_sru_src: Mat4 = m_sru_src(camera, &nn);
    let m_pers: Mat4 = m_pers(camera, &nn, &m_sru_src);
    let m_jp: Mat4 = m_jp(camera, width, height);

    let m_sru_srt: Mat4 = m_jp * (m_pers * m_sru_src);

    debug_println!("{}", m_sru_srt);

    m_sru_srt
}