mod app;
mod render;
mod types;

use eframe::{Error, NativeOptions, run_native};
use eframe::egui::{Vec2, ViewportBuilder};
use env_logger::init as init_env_logger;

use app::App;

fn main() -> Result<(), Error> {
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