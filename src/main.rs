mod app;
mod render;
mod types;

use eframe::{Error, NativeOptions, run_native};
use eframe::egui::{Vec2, ViewportBuilder};
use env_logger::init as init_env_logger;

use app::App;


// coisas de teste ------------------------

fn coisas_de_teste() {
    use crate::types::Vec3;
    use crate::render::pipeline::calculate_pipeline;
    use crate::render::Camera;

    let cam = Camera {
        dp: 40.0,
        vrp: Vec3::new(25.0, 15.0, 80.0),
        p: Vec3::new(20.0, 10.0, 25.0),
        y: Vec3::new(0.0, 0.0, 0.0),

        xmin: -8.0,
        xmax: 8.0,
        ymin: -6.0,
        ymax: 6.0,
    };

    calculate_pipeline(cam, 320.0, 240.0);
}

// coisas de teste ------------------------

fn main() -> Result<(), Error> {
    coisas_de_teste();

    init_env_logger();
    
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::new(960.0, 580.0)),
            resizable: Some(false),
            maximize_button: Some(false),
            minimize_button: Some(false),
            ..Default::default()
        },
        ..Default::default()
    };

    run_native(
        "Modelador por Revolução",
        options,
        Box::new(|_cc| {
            Box::<App>::default()
        }),
    )
}