mod app;
mod types;

use eframe::{Error, NativeOptions, run_native};
use eframe::egui::{Vec2, ViewportBuilder};
use env_logger::init as init_env_logger;

use app::App;

fn main() -> Result<(), Error> {
    init_env_logger();
    
    let options = NativeOptions {
        viewport: ViewportBuilder {
            min_inner_size: Some(Vec2::new(600.0, 400.0)),
            inner_size: Some(Vec2::new(960.0, 640.0)),
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